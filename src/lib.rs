#[macro_use]
extern crate lazy_static;

use std::ffi::CString;
use std::num::NonZeroU32;
use std::sync::RwLock;
use std::time::Instant;

use euclid::Vector2D;
use gl::types::*;
use gl_rs as gl;
use glutin::{
  config::{ConfigTemplateBuilder, GlConfig},
  context::{ContextApi, ContextAttributesBuilder, PossiblyCurrentContext},
  display::{GetGlDisplay, GlDisplay},
  prelude::{GlSurface, NotCurrentGlContext},
  surface::{Surface as GlutinSurface, SurfaceAttributesBuilder, SwapInterval, WindowSurface},
};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasWindowHandle;
use skia_safe::{
  gpu::{self, backend_render_targets, gl::FramebufferInfo, SurfaceOrigin},
  ColorType, Surface,
};
use winit::{
  application::ApplicationHandler,
  dpi::LogicalSize,
  event::{ElementState, KeyEvent, MouseScrollDelta, WindowEvent},
  event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
  keyboard::{Key, ModifiersState, NamedKey, PhysicalKey},
  window::{Window, WindowAttributes, WindowId},
};

mod color;
mod extracter;
mod ffi;
mod handlers;
mod key_listener;
mod primes;
mod renderer;
mod touches;

calcit_native_ffi::export_buffer_abi_v1!();
calcit_native_ffi::export_async_abi_v1!();

use cirru_edn::Edn;

struct Env {
  surface: Surface,
  gl_surface: GlutinSurface<WindowSurface>,
  gr_context: skia_safe::gpu::DirectContext,
  gl_context: PossiblyCurrentContext,
  window: Window,
}

impl Drop for Env {
  fn drop(&mut self) {
    self.gr_context.release_resources_and_abandon();
  }
}

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 600;

lazy_static! {
  static ref NEXT_DRAWING_DATA: RwLock<Vec<(Box<str>, Edn)>> = RwLock::new(vec![]);
}

fn create_event_loop() -> Result<EventLoop<()>, String> {
  let mut builder = EventLoop::builder();
  #[cfg(target_os = "linux")]
  winit::platform::x11::EventLoopBuilderExtX11::with_any_thread(&mut builder, true);
  builder
    .build()
    .map_err(|error| format!("failed to create event loop: {error}"))
}

struct PaintApplication<F> {
  env: Env,
  handler: F,
  fb_info: FramebufferInfo,
  num_samples: usize,
  stencil_size: usize,
  input: handlers::InputState,
  started_at: Instant,
  scale_factor: f32,
  first_paint: bool,
  smoke_once: bool,
}

impl<F> PaintApplication<F>
where
  F: Fn(Vec<Edn>) -> Result<Edn, String>,
{
  fn dispatch(&self, event: Edn) {
    if let Err(error) = (self.handler)(vec![event]) {
      eprintln!("error in handling paint event: {error}");
    }
  }

  fn resize(&mut self, width: u32, height: u32) -> Result<(), String> {
    let width = width.max(1);
    let height = height.max(1);
    self.env.gl_surface.resize(
      &self.env.gl_context,
      NonZeroU32::new(width).expect("clamped width is non-zero"),
      NonZeroU32::new(height).expect("clamped height is non-zero"),
    );
    self.env.surface = create_surface(
      &self.env.window,
      self.fb_info,
      &mut self.env.gr_context,
      self.num_samples,
      self.stencil_size,
    )?;
    Ok(())
  }

  fn draw_frame(&mut self, event_loop: &ActiveEventLoop) {
    match take_drawing_data() {
      Ok(messages) => {
        let canvas = self.env.surface.canvas();
        canvas.clear(renderer::get_bg_color());
        canvas.reset_matrix();
        canvas.scale((self.scale_factor, self.scale_factor));
        if let Err(error) = renderer::draw_page(canvas, messages, true) {
          eprintln!("failed drawing paint scene: {error}");
        }
      }
      Err(error) => eprintln!("failed extracting paint messages: {error}"),
    }

    self.env.gr_context.flush_and_submit();
    if let Err(error) = self.env.gl_surface.swap_buffers(&self.env.gl_context) {
      eprintln!("failed to swap OpenGL buffers: {error}");
      event_loop.exit();
    } else if self.smoke_once {
      event_loop.exit();
    }
  }
}

impl<F> ApplicationHandler for PaintApplication<F>
where
  F: Fn(Vec<Edn>) -> Result<Edn, String>,
{
  fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
    if self.first_paint {
      self.dispatch(Edn::Nil);
      self.env.window.request_redraw();
      self.first_paint = false;
    }
  }

  fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
    if window_id != self.env.window.id() {
      return;
    }

    match event {
      WindowEvent::CloseRequested => event_loop.exit(),
      WindowEvent::Resized(size) => {
        if let Err(error) = self.resize(size.width, size.height) {
          eprintln!("failed to resize paint surface: {error}");
          event_loop.exit();
          return;
        }
        let width = size.width as f64 / self.scale_factor as f64;
        let height = size.height as f64 / self.scale_factor as f64;
        self.dispatch(handlers::handle_resize(width, height));
        self.env.window.request_redraw();
      }
      WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
        self.scale_factor = scale_factor as f32;
        self.env.window.request_redraw();
      }
      WindowEvent::CursorMoved { position, .. } => {
        let event = handlers::handle_mouse_move(
          Vector2D::new(
            position.x as f32 / self.scale_factor,
            position.y as f32 / self.scale_factor,
          ),
          &mut self.input,
        );
        if let Some(event) = event {
          self.dispatch(event);
          self.env.window.request_redraw();
        }
      }
      WindowEvent::CursorLeft { .. } => {
        self.dispatch(handlers::handle_mouse_leave(&self.input));
        self.env.window.request_redraw();
      }
      WindowEvent::MouseInput { state, button, .. } => {
        let event = match state {
          ElementState::Pressed => handlers::handle_mouse_down(&mut self.input, button, self.started_at.elapsed()),
          ElementState::Released => handlers::handle_mouse_up(&self.input, button),
        };
        self.dispatch(event);
        self.env.window.request_redraw();
      }
      WindowEvent::MouseWheel { delta, .. } => {
        let event = match delta {
          MouseScrollDelta::LineDelta(dx, dy) => {
            handlers::handle_mouse_wheel(&self.input, dx as f64, dy as f64, "line")
          }
          MouseScrollDelta::PixelDelta(position) => handlers::handle_mouse_wheel(
            &self.input,
            position.x / self.scale_factor as f64,
            position.y / self.scale_factor as f64,
            "pixel",
          ),
        };
        self.dispatch(event);
        self.env.window.request_redraw();
      }
      WindowEvent::ModifiersChanged(modifiers) => {
        self.input.set_modifiers(modifiers.state());
      }
      WindowEvent::KeyboardInput {
        event: KeyEvent {
          state,
          logical_key,
          physical_key,
          ..
        },
        ..
      } => {
        if logical_key == Key::Named(NamedKey::Escape) {
          event_loop.exit();
          return;
        }
        let name = handlers::name_key(&logical_key);
        let key_code = match physical_key {
          PhysicalKey::Code(code) => code as u32 as f64,
          PhysicalKey::Unidentified(_) => 0.0,
        };
        for event in handlers::handle_keyboard(&name, key_code, &physical_key, state, self.input.modifiers()) {
          self.dispatch(event);
        }
        self.env.window.request_redraw();
      }
      WindowEvent::RedrawRequested => self.draw_frame(event_loop),
      _ => {}
    }
  }

  fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
    event_loop.set_control_flow(ControlFlow::Wait);
  }
}

fn launch_canvas_impl(handler: impl Fn(Vec<Edn>) -> Result<Edn, String>) -> Result<Edn, String> {
  let _ = env_logger::try_init();
  let event_loop = create_event_loop()?;
  let window_attributes = WindowAttributes::default()
    .with_inner_size(LogicalSize::new(WIDTH, HEIGHT))
    .with_title("Calcit Paint");
  let template = ConfigTemplateBuilder::new().with_alpha_size(8);
  let display_builder = DisplayBuilder::new().with_window_attributes(window_attributes.into());
  let (window, gl_config) = display_builder
    .build(&event_loop, template, |configs| {
      configs
        .reduce(|best, config| {
          if config.num_samples() < best.num_samples() {
            config
          } else {
            best
          }
        })
        .expect("glutin returned no OpenGL configurations")
    })
    .map_err(|error| format!("failed to create paint window: {error}"))?;
  let window = window.ok_or_else(|| "glutin did not create a paint window".to_owned())?;
  let raw_window_handle = window
    .window_handle()
    .map_err(|error| format!("failed to retrieve paint window handle: {error}"))?
    .as_raw();
  let context_attributes = ContextAttributesBuilder::new().build(Some(raw_window_handle));
  let fallback_context_attributes = ContextAttributesBuilder::new()
    .with_context_api(ContextApi::Gles(None))
    .build(Some(raw_window_handle));
  let not_current_context = unsafe {
    gl_config
      .display()
      .create_context(&gl_config, &context_attributes)
      .or_else(|_| {
        gl_config
          .display()
          .create_context(&gl_config, &fallback_context_attributes)
      })
      .map_err(|error| format!("failed to create OpenGL context: {error}"))?
  };
  let size = window.inner_size();
  let surface_attributes = SurfaceAttributesBuilder::<WindowSurface>::new().build(
    raw_window_handle,
    NonZeroU32::new(size.width.max(1)).expect("clamped width is non-zero"),
    NonZeroU32::new(size.height.max(1)).expect("clamped height is non-zero"),
  );
  let gl_surface = unsafe {
    gl_config
      .display()
      .create_window_surface(&gl_config, &surface_attributes)
      .map_err(|error| format!("failed to create OpenGL window surface: {error}"))?
  };
  let gl_context = not_current_context
    .make_current(&gl_surface)
    .map_err(|error| format!("failed to activate OpenGL context: {error}"))?;
  let _ = gl_surface.set_swap_interval(
    &gl_context,
    SwapInterval::Wait(NonZeroU32::new(1).expect("one is non-zero")),
  );

  gl::load_with(|name| {
    gl_config
      .display()
      .get_proc_address(CString::new(name).expect("OpenGL symbol has no NUL").as_c_str())
  });
  let skia_interface = skia_safe::gpu::gl::Interface::new_load_with(|name| {
    if name == "eglGetCurrentDisplay" {
      return std::ptr::null();
    }
    gl_config
      .display()
      .get_proc_address(CString::new(name).expect("OpenGL symbol has no NUL").as_c_str())
  })
  .ok_or_else(|| "failed to load Skia OpenGL interface".to_owned())?;
  let mut gr_context = skia_safe::gpu::direct_contexts::make_gl(skia_interface, None)
    .ok_or_else(|| "failed to create Skia OpenGL context".to_owned())?;
  let fb_info = {
    let mut fboid: GLint = 0;
    unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };
    FramebufferInfo {
      fboid: fboid as u32,
      format: skia_safe::gpu::gl::Format::RGBA8.into(),
      ..Default::default()
    }
  };
  let num_samples = gl_config.num_samples() as usize;
  let stencil_size = gl_config.stencil_size() as usize;
  let surface = create_surface(&window, fb_info, &mut gr_context, num_samples, stencil_size)?;
  let scale_factor = window.scale_factor() as f32;
  let env = Env {
    surface,
    gl_surface,
    gr_context,
    gl_context,
    window,
  };
  let mut application = PaintApplication {
    env,
    handler,
    fb_info,
    num_samples,
    stencil_size,
    input: handlers::InputState::new(Vector2D::new(0.0, 0.0), ModifiersState::empty()),
    started_at: Instant::now(),
    scale_factor,
    first_paint: true,
    smoke_once: std::env::var_os("CALCIT_PAINT_SMOKE_ONCE").is_some(),
  };
  event_loop
    .run_app(&mut application)
    .map_err(|error| format!("paint event loop failed: {error}"))?;
  Ok(Edn::Nil)
}

fn take_drawing_data() -> Result<Vec<(Box<str>, Edn)>, String> {
  let mut pending = NEXT_DRAWING_DATA
    .write()
    .map_err(|_| "drawing-data queue lock is poisoned".to_owned())?;
  Ok(std::mem::take(&mut *pending))
}

fn push_drawing_data(args: Vec<Edn>) -> Result<Edn, String> {
  let [Edn::Str(op), data] = args.as_slice() else {
    return Err(format!(
      "push-drawing-data expected an operation string and data, got: {args:?}"
    ));
  };
  let mut pending = NEXT_DRAWING_DATA
    .write()
    .map_err(|_| "drawing-data queue lock is poisoned".to_owned())?;
  pending.push((op.to_string().into_boxed_str(), data.to_owned()));
  Ok(Edn::Nil)
}

calcit_native_ffi::export_edn_buffer_method_v1!(push_drawing_data_calcit_ffi_v1, push_drawing_data);

/// Own the host thread while the paint event loop is running.
///
/// # Safety
///
/// Request bytes and descriptors must remain readable and `output` writable
/// for this call. Host function pointers must follow blocking protocol v1.
#[no_mangle]
pub unsafe extern "C" fn launch_canvas_calcit_ffi_blocking_v1(
  request_ptr: *const u8,
  request_len: usize,
  task: *const ffi::CalcitFfiAsyncTaskV1,
  host: *const ffi::CalcitFfiBlockingHostV1,
  output: *mut ffi::CalcitFfiBuffer,
) -> i32 {
  // SAFETY: the adapter validates descriptors and owns the call until the event loop returns.
  unsafe {
    ffi::run_blocking_adapter(request_ptr, request_len, task, host, output, |_args, task, host| {
      launch_canvas_impl(|args| ffi::invoke_blocking_callback(host, task, args))
    })
  }
}

fn create_surface(
  window: &Window,
  fb_info: FramebufferInfo,
  gr_context: &mut skia_safe::gpu::DirectContext,
  num_samples: usize,
  stencil_size: usize,
) -> Result<skia_safe::Surface, String> {
  let size = window.inner_size();
  let width = size.width.max(1);
  let height = size.height.max(1);
  let backend_render_target =
    backend_render_targets::make_gl((width as i32, height as i32), num_samples, stencil_size, fb_info);
  gpu::surfaces::wrap_backend_render_target(
    gr_context,
    &backend_render_target,
    SurfaceOrigin::BottomLeft,
    ColorType::RGBA8888,
    None,
    None,
  )
  .ok_or_else(|| format!("failed to wrap a {width}x{height} Skia backend surface"))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn drawing_queue_validates_and_drains_commands() {
    assert!(push_drawing_data(vec![Edn::Nil]).is_err());
    assert!(push_drawing_data(vec![Edn::Number(1.0), Edn::Nil]).is_err());

    push_drawing_data(vec![Edn::Str("render-canvas!".into()), Edn::Number(3.0)]).unwrap();
    assert_eq!(
      take_drawing_data().unwrap(),
      vec![(Box::<str>::from("render-canvas!"), Edn::Number(3.0))]
    );
    assert!(take_drawing_data().unwrap().is_empty());
  }
}

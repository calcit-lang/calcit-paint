#[macro_use]
extern crate lazy_static;

use std::cell::RefCell;
// use std::time::Instant;

use euclid::Vector2D;

mod color;
mod extracter;
mod ffi;
mod handlers;
mod key_listener;
mod primes;
mod renderer;
mod touches;

use std::sync::RwLock;
use std::{thread, time};

use cirru_edn::Edn;

use winit::dpi::LogicalSize;
use winit::event::Event;
use winit::event::WindowEvent;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::run_return::EventLoopExtRunReturn;
#[cfg(target_os = "linux")]
use winit::platform::unix::EventLoopExtUnix;
use winit::window::WindowBuilder;

use gl::types::*;
use gl_rs as gl;
use skia_safe::{
  gpu::{backend_render_targets, gl::FramebufferInfo, surfaces, SurfaceOrigin},
  ColorType, Surface,
};

type WindowedContext = glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>;

struct Env {
  surface: Surface,
  gr_context: skia_safe::gpu::DirectContext,
  windowed_context: WindowedContext,
}

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 600;

lazy_static! {
  static ref NEXT_DRAWING_DATA: RwLock<Vec<(Box<str>, Edn)>> = RwLock::new(vec![]);
}

fn create_event_loop() -> EventLoop<()> {
  #[cfg(target_os = "linux")]
  {
    EventLoop::new_any_thread()
  }
  #[cfg(not(target_os = "linux"))]
  {
    EventLoop::new()
  }
}

fn launch_canvas_impl(handler: impl Fn(Vec<Edn>) -> Result<Edn, String>) -> Result<Edn, String> {
  let _ = env_logger::try_init();

  println!("\nCalcit Paint event loop started.\n");

  let mut event_loop = create_event_loop();

  let area_size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);

  let wb = WindowBuilder::new()
    .with_inner_size(area_size)
    .with_title("Calcit Paint");

  let cb = glutin::ContextBuilder::new().with_vsync(true);

  let windowed_context = unsafe {
    cb.build_windowed(wb, &event_loop)
      .map_err(|error| format!("failed to create paint window: {error}"))?
      .make_current()
      .map_err(|(_, error)| format!("failed to activate OpenGL context: {error}"))?
  };

  let window = windowed_context.window();
  let pixel_format = windowed_context.get_pixel_format();

  println!("Pixel format of the window's GL context: {:?}", pixel_format);

  gl::load_with(|s| windowed_context.get_proc_address(s));

  let skia_interface = skia_safe::gpu::gl::Interface::new_load_with(|name| windowed_context.get_proc_address(name))
    .ok_or_else(|| "failed to load Skia OpenGL interface from the active window context".to_owned())?;
  let mut gr_context = skia_safe::gpu::DirectContext::new_gl(skia_interface, None)
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

  window.set_inner_size(glutin::dpi::Size::new(glutin::dpi::LogicalSize::new(WIDTH, HEIGHT)));

  let surface = create_surface(&windowed_context, &fb_info, &mut gr_context)?;
  let scale_factor = window.scale_factor() as f32;

  let mut env = Env {
    surface,
    gr_context,
    windowed_context,
  };

  let canvas = env.surface.canvas();
  canvas.scale((scale_factor, scale_factor));

  let mut first_paint = true;
  let track_mouse = RefCell::new(Vector2D::new(0.0, 0.0));
  let track_scale: RefCell<f32> = RefCell::new(scale_factor);
  let smoke_once = std::env::var_os("CALCIT_PAINT_SMOKE_ONCE").is_some();
  // Handle events. Refer to `winit` docs for more information.
  event_loop.run_return(move |event, _window_target, control_flow| {
    // println!("Event: {:?}", event);
    *control_flow = ControlFlow::Wait;
    let scaled = track_scale.clone().into_inner();
    let window = env.windowed_context.window();

    if first_paint {
      if let Err(err) = handler(vec![Edn::Nil]) {
        println!("error in handling event: {}", err);
      } else {
        // Update internal state and request a redraw
        window.request_redraw();
        first_paint = false
      }
    }

    match event {
      Event::WindowEvent { event, .. } => match event {
        WindowEvent::Resized(physical_size) => {
          env.windowed_context.resize(physical_size);
          if physical_size.width > 0 && physical_size.height > 0 {
            match create_surface(&env.windowed_context, &fb_info, &mut env.gr_context) {
              Ok(surface) => env.surface = surface,
              Err(error) => {
                eprintln!("failed to resize paint surface: {error}");
                *control_flow = ControlFlow::Exit;
                return;
              }
            }
          }
          // println!("Window size changed: {:?}", size);
          let scale = track_scale.to_owned().into_inner();
          let w = physical_size.width as f32 / scale;
          let h = physical_size.height as f32 / scale;
          let e = handlers::handle_resize(w as f64, h as f64).unwrap();

          if let Err(err) = handler(vec![e]) {
            eprintln!("error in handling event: {}", err);
          } else {
            window.request_redraw();
          }
        }
        WindowEvent::ScaleFactorChanged {
          scale_factor: factor,
          new_inner_size: size,
        } => {
          println!("DPI scale change {} {:?}", factor, size);
          track_scale.replace(factor as f32);
          window.request_redraw();
        }
        WindowEvent::CursorMoved { position, .. } => {
          // let scale = track_scale.to_owned().into_inner();
          let event_info = handlers::handle_mouse_move(
            Vector2D::new(position.x as f32 / scaled, position.y as f32 / scaled),
            &track_mouse,
          );

          if let Some(e) = event_info {
            if let Err(err) = handler(vec![e]) {
              println!("error in handling event: {}", err);
            } else {
              window.request_redraw();
            }
          }
        }
        WindowEvent::MouseInput { state, .. } => {
          // println!("mouse button: {:?}", button);
          let event_info = match state {
            winit::event::ElementState::Pressed => handlers::handle_mouse_down(&track_mouse),
            winit::event::ElementState::Released => handlers::handle_mouse_up(&track_mouse),
          };

          if let Err(err) = handler(vec![event_info]) {
            println!("error in handling event: {}", err);
          } else {
            window.request_redraw();
          }
        }
        WindowEvent::KeyboardInput {
          input:
            winit::event::KeyboardInput {
              state: key_state,
              scancode: _c, // unknown order
              virtual_keycode: Some(keycode),
              ..
            },
          ..
        } => match keycode {
          winit::event::VirtualKeyCode::Escape => *control_flow = ControlFlow::Exit,
          _ => {
            // println!("keyboard event: {:?} {:?}", keycode, scancode);
            let event_infos = handlers::handle_keyboard(keycode, key_state);
            for event_info in event_infos {
              if let Err(err) = handler(vec![event_info]) {
                eprintln!("error in handling event: {}", err);
              }
            }
            window.request_redraw();
          }
        },
        WindowEvent::CloseRequested => {
          *control_flow = ControlFlow::Exit;
          println!("User Close.");
        }
        // `CloseRequested` and `KeyboardInput` events won't appear here.
        x => println!("Other window event fired: {:?}", x),
      },

      Event::MainEventsCleared => {
        // println!("main events cleared");
        thread::sleep(time::Duration::from_millis(50));
      }
      Event::RedrawRequested(_wid) => {
        match take_drawing_data() {
          Ok(messages) => {
            let canvas = env.surface.canvas();
            canvas.clear(renderer::get_bg_color());
            canvas.reset_matrix();
            canvas.scale((scaled, scaled));
            if let Err(e) = renderer::draw_page(canvas, messages, true) {
              println!("Failed drawing: {:?}", e);
            }
          }
          Err(e) => {
            println!("failed extracting messages: {}", e)
          }
        }

        env.gr_context.flush_and_submit();
        if let Err(error) = env.windowed_context.swap_buffers() {
          eprintln!("failed to swap OpenGL buffers: {error}");
          *control_flow = ControlFlow::Exit;
        }
        if smoke_once {
          *control_flow = ControlFlow::Exit;
        }
      }
      Event::RedrawEventsCleared => {
        // println!("redraw events cleared");
      }
      Event::NewEvents(winit::event::StartCause::Poll) => {
        // nothing
      }
      Event::DeviceEvent { event: _event, .. } => {
        // println!("Device event fired: {:?}", event);
      }
      Event::NewEvents(_e) => {
        // println!("New events fired: {:?}", e);
      }
      e => {
        eprintln!("unknown event: {:?}", e)
      }
    }
  });

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
  pending.push((op.to_owned(), data.to_owned()));
  Ok(Edn::Nil)
}

/// Push one drawing command through C-safe buffer protocol v1.
///
/// # Safety
///
/// Request bytes must remain readable and `output` writable for this call.
#[no_mangle]
pub unsafe extern "C" fn push_drawing_data_calcit_ffi_v1(
  request_ptr: *const u8,
  request_len: usize,
  output: *mut ffi::CalcitFfiBuffer,
) -> i32 {
  // SAFETY: the shared adapter validates and copies all call-scoped inputs.
  unsafe { ffi::run_buffer_adapter(request_ptr, request_len, output, push_drawing_data) }
}

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
  windowed_context: &WindowedContext,
  fb_info: &FramebufferInfo,
  gr_context: &mut skia_safe::gpu::DirectContext,
) -> Result<skia_safe::Surface, String> {
  let pixel_format = windowed_context.get_pixel_format();
  let size = windowed_context.window().inner_size();
  let backend_render_target = backend_render_targets::make_gl(
    (size.width as i32, size.height as i32),
    pixel_format.multisampling.map(|s| s as usize),
    pixel_format.stencil_bits as usize,
    *fb_info,
  );
  surfaces::wrap_backend_render_target(
    gr_context,
    &backend_render_target,
    SurfaceOrigin::BottomLeft,
    ColorType::RGBA8888,
    None,
    None,
  )
  .ok_or_else(|| format!("failed to wrap a {}x{} Skia backend surface", size.width, size.height))
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

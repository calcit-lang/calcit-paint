#[macro_use]
extern crate lazy_static;

use std::collections::VecDeque;
use std::ffi::CString;
use std::num::NonZeroU32;
use std::sync::RwLock;
use std::time::Instant;

use accesskit::{Action, ActionRequest};
use accesskit_winit::{Adapter as AccessKitAdapter, Event as AccessKitEvent, WindowEvent as AccessKitWindowEvent};
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
  event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy},
  keyboard::{Key, ModifiersState, NamedKey, PhysicalKey},
  window::{CursorIcon, Window, WindowAttributes, WindowId},
};

mod accessibility;
mod clipboard;
mod color;
mod extracter;
mod ffi;
mod file_dialog;
mod focus;
mod frame;
mod handlers;
mod hit_test;
mod key_listener;
mod primes;
mod renderer;
mod touches;
mod typed_events;
mod window_lifecycle;

calcit_native_ffi::export_buffer_abi_v1!();
calcit_native_ffi::export_async_abi_v1!();

use cirru_edn::{Edn, EdnListView};

struct Env {
  surface: Surface,
  gl_surface: GlutinSurface<WindowSurface>,
  gr_context: skia_safe::gpu::DirectContext,
  gl_context: PossiblyCurrentContext,
  window: Window,
  fb_info: FramebufferInfo,
  num_samples: usize,
  stencil_size: usize,
}

impl Drop for Env {
  fn drop(&mut self) {
    self.gr_context.release_resources_and_abandon();
  }
}

lazy_static! {
  static ref NEXT_DRAWING_DATA: RwLock<Vec<(Box<str>, Edn)>> = RwLock::new(vec![]);
  static ref NEXT_FOCUS_EVENTS: RwLock<Vec<Edn>> = RwLock::new(vec![]);
}

#[derive(Debug)]
pub enum PaintUserEvent {
  AccessKit(AccessKitEvent),
  FileDialogResult(file_dialog::FileDialogResult),
}

impl From<AccessKitEvent> for PaintUserEvent {
  fn from(event: AccessKitEvent) -> Self {
    Self::AccessKit(event)
  }
}

fn create_event_loop() -> Result<EventLoop<PaintUserEvent>, String> {
  let mut builder = EventLoop::<PaintUserEvent>::with_user_event();
  #[cfg(target_os = "linux")]
  winit::platform::x11::EventLoopBuilderExtX11::with_any_thread(&mut builder, true);
  builder
    .build()
    .map_err(|error| format!("failed to create event loop: {error}"))
}

struct PaintApplication<F> {
  env: Option<Env>,
  accessibility_adapter: Option<AccessKitAdapter>,
  options: window_lifecycle::WindowStartupOptions,
  handler: F,
  input: handlers::InputState,
  started_at: Instant,
  frame_clock: frame::FrameClock,
  scale_factor: f32,
  first_paint: bool,
  initial_theme_dispatched: bool,
  smoke_once: bool,
  ime_allowed: bool,
  cursor_icon: CursorIcon,
  occluded: bool,
  minimized: bool,
  suspended: bool,
  close_dispatched: bool,
  event_proxy: EventLoopProxy<PaintUserEvent>,
}

impl<F> PaintApplication<F>
where
  F: Fn(Vec<Edn>) -> Result<Edn, String>,
{
  fn env(&self) -> &Env {
    self.env.as_ref().expect("paint environment is initialized in resumed")
  }

  fn env_mut(&mut self) -> &mut Env {
    self.env.as_mut().expect("paint environment is initialized in resumed")
  }

  fn dispatch(&mut self, event: Edn) {
    let mut events = VecDeque::from([event]);
    while let Some(event) = events.pop_front() {
      if let Err(error) = (self.handler)(vec![event]) {
        eprintln!("error in handling paint event: {error}");
      }
      match take_focus_events() {
        Ok(pending) => events.extend(pending),
        Err(error) => eprintln!("failed reading programmatic focus events: {error}"),
      }
    }
    self.sync_ime();
    self.schedule_requested_frame();
  }

  fn dispatch_all(&mut self, events: Vec<Edn>) {
    for event in events {
      self.dispatch(event);
    }
  }

  fn update_accessibility_tree(&mut self) {
    if let Some(adapter) = self.accessibility_adapter.as_mut() {
      adapter.update_if_active(accessibility::tree_update);
    }
  }

  fn handle_accesskit_event(&mut self, event: AccessKitEvent) {
    let Some(env) = self.env.as_ref() else {
      return;
    };
    if event.window_id != env.window.id() {
      return;
    }
    match event.window_event {
      AccessKitWindowEvent::InitialTreeRequested => self.update_accessibility_tree(),
      AccessKitWindowEvent::ActionRequested(ActionRequest {
        action, target_node, ..
      }) => {
        let Some(node) = accessibility::node_for_id(target_node) else {
          return;
        };
        if !node.properties.enabled {
          return;
        }
        match action {
          Action::Focus if node.properties.focusable => {
            if let Some(focus_id) = node.focus_id.as_deref() {
              match focus::request_focus(focus_id, focus::FocusReason::Programmatic) {
                Ok(Some(transition)) => self.dispatch_all(handlers::handle_focus_transition(transition)),
                Ok(None) => {}
                Err(error) => {
                  eprintln!("failed applying accessibility focus action: {error}");
                  return;
                }
              }
              self.dispatch(handlers::handle_accessibility_action(&node, "focus"));
            }
          }
          Action::Click => self.dispatch(handlers::handle_accessibility_action(&node, "activate")),
          _ => return,
        }
        self.update_accessibility_tree();
        self.env().window.request_redraw();
      }
      AccessKitWindowEvent::AccessibilityDeactivated => {}
    }
  }

  fn request_exit(&mut self, event_loop: &ActiveEventLoop, reason: &str) {
    if !self.close_dispatched {
      self.close_dispatched = true;
      match window_lifecycle::begin_close() {
        Ok(_) => self.dispatch(handlers::handle_window_close(reason)),
        Err(error) => eprintln!("failed marking paint window as closing: {error}"),
      }
    }
    event_loop.exit();
  }

  fn apply_window_requests(&mut self, event_loop: &ActiveEventLoop) {
    loop {
      let mut requests = match window_lifecycle::take_requests() {
        Ok(requests) => requests,
        Err(error) => {
          eprintln!("failed reading paint window requests: {error}");
          return;
        }
      };
      if requests.is_empty() {
        return;
      }
      while let Some(request) = requests.pop_front() {
        match request {
          window_lifecycle::WindowRequest::SetTitle(title) => {
            self.env().window.set_title(&title);
            self.dispatch(handlers::handle_window_title_request(&title));
          }
          window_lifecycle::WindowRequest::RequestSize { width, height } => {
            let actual = self.env().window.request_inner_size(LogicalSize::new(width, height));
            if let Some(size) = actual {
              let minimized = size.width == 0 || size.height == 0;
              if self.minimized != minimized {
                self.minimized = minimized;
                self.reset_frame_timing();
              }
              if !minimized {
                if let Err(error) = self.resize(size.width, size.height) {
                  eprintln!("failed to apply confirmed paint window size: {error}");
                  self.request_exit(event_loop, "render-error");
                  return;
                }
                self.env().window.request_redraw();
              }
            }
            self.dispatch(handlers::handle_window_size_request(
              width,
              height,
              self.scale_factor as f64,
              actual.map(|size| (size.width, size.height)),
            ));
          }
          window_lifecycle::WindowRequest::FileDialog(request) => {
            if let Err(error) = file_dialog::launch(request.clone(), self.event_proxy.clone()) {
              if let Err(completion_error) = window_lifecycle::complete_file_dialog() {
                eprintln!("failed completing native file dialog request: {completion_error}");
              }
              self.dispatch(handlers::handle_file_dialog_result(file_dialog::failed_result(
                &request, error,
              )));
            }
          }
          window_lifecycle::WindowRequest::Close => {
            self.request_exit(event_loop, "requested");
            return;
          }
        }
      }
    }
  }

  fn sync_cursor(&mut self) {
    let cursor = touches::pointer_cursor();
    if self.cursor_icon != cursor {
      self.env().window.set_cursor(cursor);
      self.cursor_icon = cursor;
    }
  }

  fn sync_ime(&mut self) {
    let allowed = focus::text_input_enabled();
    if self.ime_allowed != allowed {
      self.env().window.set_ime_allowed(allowed);
      self.ime_allowed = allowed;
    }
  }

  fn frame_paused(&self) -> bool {
    self.occluded || self.minimized || self.suspended
  }

  fn reset_frame_timing(&mut self) {
    self.frame_clock.reset_delta();
  }

  fn schedule_requested_frame(&self) {
    if self.frame_paused() {
      return;
    }
    match frame::pending() {
      Ok(true) => self.env().window.request_redraw(),
      Ok(false) => {}
      Err(error) => eprintln!("failed reading paint frame request: {error}"),
    }
  }

  fn dispatch_requested_frame(&mut self) {
    let requested = match frame::take_request() {
      Ok(requested) => requested,
      Err(error) => {
        eprintln!("failed consuming paint frame request: {error}");
        return;
      }
    };
    if !requested {
      return;
    }
    let size = self.env().window.inner_size();
    let width = size.width as f64 / self.scale_factor as f64;
    let height = size.height as f64 / self.scale_factor as f64;
    let timing = self.frame_clock.next_at(Instant::now());
    self.dispatch(handlers::handle_frame(timing, width, height, self.scale_factor as f64));
  }

  fn resize(&mut self, width: u32, height: u32) -> Result<(), String> {
    let width = width.max(1);
    let height = height.max(1);
    let env = self.env_mut();
    env.gl_surface.resize(
      &env.gl_context,
      NonZeroU32::new(width).expect("clamped width is non-zero"),
      NonZeroU32::new(height).expect("clamped height is non-zero"),
    );
    env.surface = create_surface(
      &env.window,
      env.fb_info,
      &mut env.gr_context,
      env.num_samples,
      env.stencil_size,
    )?;
    Ok(())
  }

  fn draw_frame(&mut self, event_loop: &ActiveEventLoop) {
    if self.frame_paused() {
      return;
    }
    self.dispatch_requested_frame();
    focus::begin_frame();
    accessibility::begin_frame();
    match take_drawing_data() {
      Ok(messages) => {
        let scale_factor = self.scale_factor;
        let canvas = self.env_mut().surface.canvas();
        canvas.clear(renderer::get_bg_color());
        canvas.reset_matrix();
        canvas.scale((scale_factor, scale_factor));
        if let Err(error) = renderer::draw_page(canvas, messages, true) {
          eprintln!("failed drawing paint scene: {error}");
        }
      }
      Err(error) => eprintln!("failed extracting paint messages: {error}"),
    }

    let pointer_events = handlers::handle_pointer_scene_change(&self.input);
    if !pointer_events.is_empty() {
      self.dispatch_all(pointer_events);
      self.env().window.request_redraw();
    }
    self.sync_cursor();

    if let Some(transition) = focus::finish_frame() {
      for event in handlers::handle_focus_transition(transition) {
        self.dispatch(event);
      }
    }
    self.update_accessibility_tree();
    self.sync_ime();

    if !self.initial_theme_dispatched {
      self.initial_theme_dispatched = true;
      self.dispatch(handlers::handle_window_theme(self.env().window.theme(), true));
      self.env().window.request_redraw();
    }

    self.env_mut().gr_context.flush_and_submit();
    let env = self.env();
    if let Err(error) = env.gl_surface.swap_buffers(&env.gl_context) {
      eprintln!("failed to swap OpenGL buffers: {error}");
      self.request_exit(event_loop, "render-error");
    } else if self.smoke_once {
      self.request_exit(event_loop, "smoke");
    }
  }
}

impl<F> ApplicationHandler<PaintUserEvent> for PaintApplication<F>
where
  F: Fn(Vec<Edn>) -> Result<Edn, String>,
{
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    if self.env.is_none() {
      match create_env(event_loop, self.options.clone()) {
        Ok(env) => {
          self.scale_factor = env.window.scale_factor() as f32;
          self.started_at = Instant::now();
          self.frame_clock = frame::FrameClock::new(self.started_at);
          let adapter = AccessKitAdapter::with_event_loop_proxy(event_loop, &env.window, self.event_proxy.clone());
          self.env = Some(env);
          self.accessibility_adapter = Some(adapter);
          self.env().window.set_visible(true);
        }
        Err(error) => {
          eprintln!("failed initializing paint window in resumed: {error}");
          self.request_exit(event_loop, "startup-error");
          return;
        }
      }
    }
    if self.suspended {
      self.suspended = false;
      self.reset_frame_timing();
    }
    if self.first_paint {
      self.dispatch(Edn::Nil);
      self.env().window.request_redraw();
      self.first_paint = false;
    } else {
      self.schedule_requested_frame();
    }
  }

  fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
    self.suspended = true;
    self.reset_frame_timing();
  }

  fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
    let Some(env) = self.env.as_ref() else {
      return;
    };
    if window_id != env.window.id() {
      return;
    }
    if let Some(adapter) = self.accessibility_adapter.as_mut() {
      adapter.process_event(&env.window, &event);
    }

    match event {
      WindowEvent::CloseRequested => self.request_exit(event_loop, "system"),
      WindowEvent::Resized(size) => {
        let minimized = size.width == 0 || size.height == 0;
        if self.minimized != minimized {
          self.minimized = minimized;
          self.reset_frame_timing();
        }
        let width = size.width as f64 / self.scale_factor as f64;
        let height = size.height as f64 / self.scale_factor as f64;
        self.dispatch(handlers::handle_resize(width, height, self.scale_factor as f64));
        if minimized {
          return;
        }
        if let Err(error) = self.resize(size.width, size.height) {
          eprintln!("failed to resize paint surface: {error}");
          self.request_exit(event_loop, "render-error");
          return;
        }
        self.env().window.request_redraw();
      }
      WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
        self.scale_factor = scale_factor as f32;
        let size = self.env().window.inner_size();
        self.dispatch(handlers::handle_scale_factor(
          size.width as f64 / scale_factor,
          size.height as f64 / scale_factor,
          scale_factor,
        ));
        self.env().window.request_redraw();
      }
      WindowEvent::ThemeChanged(theme) => {
        self.dispatch(handlers::handle_window_theme(Some(theme), false));
        self.env().window.request_redraw();
      }
      WindowEvent::CursorMoved { position, .. } => {
        let events = handlers::handle_mouse_move(
          Vector2D::new(
            position.x as f32 / self.scale_factor,
            position.y as f32 / self.scale_factor,
          ),
          &mut self.input,
        );
        if !events.is_empty() {
          self.dispatch_all(events);
          self.sync_cursor();
          self.env().window.request_redraw();
        }
      }
      WindowEvent::CursorLeft { .. } => {
        let events = handlers::handle_mouse_leave(&mut self.input);
        self.dispatch_all(events);
        self.sync_cursor();
        self.env().window.request_redraw();
      }
      WindowEvent::MouseInput { state, button, .. } => {
        let events = match state {
          ElementState::Pressed => handlers::handle_mouse_down(&mut self.input, button, self.started_at.elapsed()),
          ElementState::Released => handlers::handle_mouse_up(&self.input, button),
        };
        self.dispatch_all(events);
        if state == ElementState::Pressed {
          self.dispatch_all(handlers::handle_pointer_focus(self.input.position(), button));
        }
        self.sync_cursor();
        self.env().window.request_redraw();
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
        self.env().window.request_redraw();
      }
      WindowEvent::HoveredFile(path) => match handlers::handle_file_hover(&path, &self.input) {
        Ok(event) => {
          self.dispatch(event);
          self.env().window.request_redraw();
        }
        Err(error) => eprintln!("failed handling hovered paint file: {error}"),
      },
      WindowEvent::DroppedFile(path) => match handlers::handle_file_drop(&path, &self.input) {
        Ok(event) => {
          self.dispatch(event);
          self.env().window.request_redraw();
        }
        Err(error) => eprintln!("failed handling dropped paint file: {error}"),
      },
      WindowEvent::HoveredFileCancelled => {
        self.dispatch(handlers::handle_file_hover_cancel(&self.input));
        self.env().window.request_redraw();
      }
      WindowEvent::ModifiersChanged(modifiers) => {
        self.input.set_modifiers(modifiers.state());
      }
      WindowEvent::Occluded(occluded) => {
        if self.occluded != occluded {
          self.occluded = occluded;
          self.reset_frame_timing();
        }
        if !occluded {
          self.schedule_requested_frame();
        }
      }
      WindowEvent::Focused(focused) => {
        if !focused {
          self.dispatch_all(handlers::handle_pointer_blur(&self.input));
          self.sync_cursor();
        }
        self.dispatch_all(handlers::handle_window_focus(focused));
        self.env().window.request_redraw();
      }
      WindowEvent::Ime(ime) => {
        for event in handlers::handle_ime(ime) {
          self.dispatch(event);
        }
        self.env().window.request_redraw();
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
        if state == ElementState::Pressed
          && logical_key == Key::Named(NamedKey::Escape)
          && !focus::has_focus()
          && !focus::is_composing()
        {
          self.request_exit(event_loop, "escape");
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
        self.env().window.request_redraw();
      }
      WindowEvent::RedrawRequested => self.draw_frame(event_loop),
      _ => {}
    }
  }

  fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: PaintUserEvent) {
    match event {
      PaintUserEvent::AccessKit(event) => self.handle_accesskit_event(event),
      PaintUserEvent::FileDialogResult(result) => {
        if let Err(error) = window_lifecycle::complete_file_dialog() {
          eprintln!("failed completing native file dialog request: {error}");
        }
        self.dispatch(handlers::handle_file_dialog_result(result));
        self.env().window.request_redraw();
      }
    }
  }

  fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
    self.apply_window_requests(event_loop);
    event_loop.set_control_flow(ControlFlow::Wait);
  }

  fn exiting(&mut self, event_loop: &ActiveEventLoop) {
    if !self.close_dispatched {
      self.request_exit(event_loop, "event-loop");
    }
    if let Err(error) = clipboard::release() {
      eprintln!("failed releasing text clipboard: {error}");
    }
  }
}

fn create_env(event_loop: &ActiveEventLoop, options: window_lifecycle::WindowStartupOptions) -> Result<Env, String> {
  let mut window_attributes = WindowAttributes::default()
    .with_inner_size(LogicalSize::new(options.width, options.height))
    .with_title(options.title)
    .with_resizable(options.resizable)
    .with_visible(false);
  if let (Some(min_width), Some(min_height)) = (options.min_width, options.min_height) {
    window_attributes = window_attributes.with_min_inner_size(LogicalSize::new(min_width, min_height));
  }
  let template = ConfigTemplateBuilder::new().with_alpha_size(8);
  let display_builder = DisplayBuilder::new().with_window_attributes(window_attributes.into());
  let (window, gl_config) = display_builder
    .build(event_loop, template, |configs| {
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
  let env = Env {
    surface,
    gl_surface,
    gr_context,
    gl_context,
    window,
    fb_info,
    num_samples,
    stencil_size,
  };
  Ok(env)
}

fn launch_canvas_impl(
  options: window_lifecycle::WindowStartupOptions,
  handler: impl Fn(Vec<Edn>) -> Result<Edn, String>,
) -> Result<Edn, String> {
  let _ = env_logger::try_init();
  let _active_window = window_lifecycle::activate()?;
  let event_loop = create_event_loop()?;
  let event_proxy = event_loop.create_proxy();
  let started_at = Instant::now();
  let mut application = PaintApplication {
    env: None,
    accessibility_adapter: None,
    options,
    handler,
    input: handlers::InputState::new(Vector2D::new(0.0, 0.0), ModifiersState::empty()),
    started_at,
    frame_clock: frame::FrameClock::new(started_at),
    scale_factor: 1.0,
    first_paint: true,
    initial_theme_dispatched: false,
    smoke_once: std::env::var_os("CALCIT_PAINT_SMOKE_ONCE").is_some(),
    ime_allowed: false,
    cursor_icon: CursorIcon::default(),
    occluded: false,
    minimized: false,
    suspended: false,
    close_dispatched: false,
    event_proxy,
  };
  touches::reset_pointer_state();
  let _active_frame_loop = frame::activate()?;
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

fn queue_focus_events(events: Vec<Edn>) -> Result<(), String> {
  NEXT_FOCUS_EVENTS
    .write()
    .map_err(|_| "focus-event queue lock is poisoned".to_owned())?
    .extend(events);
  Ok(())
}

fn take_focus_events() -> Result<Vec<Edn>, String> {
  let mut pending = NEXT_FOCUS_EVENTS
    .write()
    .map_err(|_| "focus-event queue lock is poisoned".to_owned())?;
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

fn request_frame(args: Vec<Edn>) -> Result<Edn, String> {
  if !args.is_empty() {
    return Err(format!("request-frame expected no arguments, got: {args:?}"));
  }
  frame::request()?;
  Ok(Edn::Nil)
}

calcit_native_ffi::export_edn_buffer_method_v1!(request_frame_calcit_ffi_v1, request_frame);

fn request_focus(args: Vec<Edn>) -> Result<Edn, String> {
  let [Edn::Str(id)] = args.as_slice() else {
    return Err(format!("request-focus expected one focus-id string, got: {args:?}"));
  };
  if let Some(transition) = focus::request_focus(id, focus::FocusReason::Programmatic)? {
    queue_focus_events(handlers::handle_focus_transition(transition))?;
  }
  Ok(Edn::Nil)
}

calcit_native_ffi::export_edn_buffer_method_v1!(request_focus_calcit_ffi_v1, request_focus);

fn clear_focus(args: Vec<Edn>) -> Result<Edn, String> {
  if !args.is_empty() {
    return Err(format!("clear-focus expected no arguments, got: {args:?}"));
  }
  if let Some(transition) = focus::clear_focus(focus::FocusReason::Programmatic) {
    queue_focus_events(handlers::handle_focus_transition(transition))?;
  }
  Ok(Edn::Nil)
}

calcit_native_ffi::export_edn_buffer_method_v1!(clear_focus_calcit_ffi_v1, clear_focus);

fn focused(args: Vec<Edn>) -> Result<Edn, String> {
  let [Edn::Str(id)] = args.as_slice() else {
    return Err(format!("focused expected one focus-id string, got: {args:?}"));
  };
  Ok(Edn::Bool(focus::focused(id)))
}

calcit_native_ffi::export_edn_buffer_method_v1!(focused_calcit_ffi_v1, focused);

fn read_clipboard_text(args: Vec<Edn>) -> Result<Edn, String> {
  if !args.is_empty() {
    return Err(format!("read-clipboard-text expected no arguments, got: {args:?}"));
  }
  Ok(Edn::str(clipboard::read_text()?))
}

calcit_native_ffi::export_edn_buffer_method_v1!(read_clipboard_text_calcit_ffi_v1, read_clipboard_text);

fn write_clipboard_text(args: Vec<Edn>) -> Result<Edn, String> {
  let [Edn::Str(text)] = args.as_slice() else {
    return Err(format!("write-clipboard-text expected one text string, got: {args:?}"));
  };
  clipboard::write_text(text)?;
  Ok(Edn::Nil)
}

calcit_native_ffi::export_edn_buffer_method_v1!(write_clipboard_text_calcit_ffi_v1, write_clipboard_text);

fn measure_text(args: Vec<Edn>) -> Result<Edn, String> {
  let [data] = args.as_slice() else {
    return Err(format!("measure-text expected one text options map, got: {args:?}"));
  };
  renderer::measure_text(data)
}

calcit_native_ffi::export_edn_buffer_method_v1!(measure_text_calcit_ffi_v1, measure_text);

fn measure_paragraph(args: Vec<Edn>) -> Result<Edn, String> {
  let [data] = args.as_slice() else {
    return Err(format!(
      "measure-paragraph expected one paragraph options map, got: {args:?}"
    ));
  };
  renderer::measure_paragraph(data)
}

calcit_native_ffi::export_edn_buffer_method_v1!(measure_paragraph_calcit_ffi_v1, measure_paragraph);

fn render_to_png(args: Vec<Edn>) -> Result<Edn, String> {
  let [data] = args.as_slice() else {
    return Err(format!("render-to-png expected one options map, got: {args:?}"));
  };
  renderer::render_to_png(data)?;
  Ok(Edn::Nil)
}

calcit_native_ffi::export_edn_buffer_method_v1!(render_to_png_calcit_ffi_v1, render_to_png);

fn validate_scene(args: Vec<Edn>) -> Result<Edn, String> {
  let [scene] = args.as_slice() else {
    return Err(format!("validate-scene expected one scene value, got: {args:?}"));
  };
  Ok(Edn::List(EdnListView(
    renderer::validate_scene(scene)
      .into_iter()
      .map(|message| Edn::Str(message.into()))
      .collect(),
  )))
}

calcit_native_ffi::export_edn_buffer_method_v1!(validate_scene_calcit_ffi_v1, validate_scene);

fn set_window_title(args: Vec<Edn>) -> Result<Edn, String> {
  let [Edn::Str(title)] = args.as_slice() else {
    return Err(format!("set-window-title expected one title string, got: {args:?}"));
  };
  window_lifecycle::queue_title(title.to_string())?;
  Ok(Edn::Nil)
}

calcit_native_ffi::export_edn_buffer_method_v1!(set_window_title_calcit_ffi_v1, set_window_title);

fn request_window_size(args: Vec<Edn>) -> Result<Edn, String> {
  let [Edn::Number(width), Edn::Number(height)] = args.as_slice() else {
    return Err(format!(
      "request-window-size expected logical width and height numbers, got: {args:?}"
    ));
  };
  window_lifecycle::queue_size(*width, *height)?;
  Ok(Edn::Nil)
}

calcit_native_ffi::export_edn_buffer_method_v1!(request_window_size_calcit_ffi_v1, request_window_size);

fn close_window(args: Vec<Edn>) -> Result<Edn, String> {
  if !args.is_empty() {
    return Err(format!("close-window expected no arguments, got: {args:?}"));
  }
  window_lifecycle::queue_close()?;
  Ok(Edn::Nil)
}

calcit_native_ffi::export_edn_buffer_method_v1!(close_window_calcit_ffi_v1, close_window);

fn open_file_dialog(args: Vec<Edn>) -> Result<Edn, String> {
  window_lifecycle::queue_file_dialog(file_dialog::FileDialogRequest::Open(file_dialog::parse_options(&args)?))?;
  Ok(Edn::Nil)
}

calcit_native_ffi::export_edn_buffer_method_v1!(open_file_dialog_calcit_ffi_v1, open_file_dialog);

fn save_file_dialog(args: Vec<Edn>) -> Result<Edn, String> {
  window_lifecycle::queue_file_dialog(file_dialog::FileDialogRequest::Save(file_dialog::parse_options(&args)?))?;
  Ok(Edn::Nil)
}

calcit_native_ffi::export_edn_buffer_method_v1!(save_file_dialog_calcit_ffi_v1, save_file_dialog);

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
      launch_canvas_impl(window_lifecycle::WindowStartupOptions::default(), |args| {
        ffi::invoke_blocking_callback(host, task, args)
      })
    })
  }
}

/// Own the host thread while a configured paint event loop is running.
///
/// # Safety
///
/// Request bytes and descriptors must remain readable and `output` writable
/// for this call. Host function pointers must follow blocking protocol v1.
#[no_mangle]
pub unsafe extern "C" fn launch_canvas_with_options_calcit_ffi_blocking_v1(
  request_ptr: *const u8,
  request_len: usize,
  task: *const ffi::CalcitFfiAsyncTaskV1,
  host: *const ffi::CalcitFfiBlockingHostV1,
  output: *mut ffi::CalcitFfiBuffer,
) -> i32 {
  // SAFETY: the adapter validates descriptors and owns the call until the event loop returns.
  unsafe {
    ffi::run_blocking_adapter(request_ptr, request_len, task, host, output, |args, task, host| {
      let options = window_lifecycle::parse_startup_options(&args)?;
      launch_canvas_impl(options, |args| ffi::invoke_blocking_callback(host, task, args))
    })
  }
}

/// Own the host thread while a configured paint event loop delivers typed envelopes.
///
/// # Safety
///
/// Request bytes and descriptors must remain readable and `output` writable
/// for this call. Host function pointers must follow blocking protocol v1.
#[no_mangle]
pub unsafe extern "C" fn launch_canvas_typed_calcit_ffi_blocking_v1(
  request_ptr: *const u8,
  request_len: usize,
  task: *const ffi::CalcitFfiAsyncTaskV1,
  host: *const ffi::CalcitFfiBlockingHostV1,
  output: *mut ffi::CalcitFfiBuffer,
) -> i32 {
  // SAFETY: the adapter validates descriptors and owns the call until the event loop returns.
  unsafe {
    ffi::run_blocking_adapter(request_ptr, request_len, task, host, output, |args, task, host| {
      let options = window_lifecycle::parse_startup_options(&args)?;
      launch_canvas_impl(options, |args| {
        let [event] = args.as_slice() else {
          return Err(format!("typed paint callback expected one event, got: {args:?}"));
        };
        let event = typed_events::from_legacy(event.clone())?;
        ffi::invoke_blocking_callback(host, task, vec![event])
      })
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

  #[test]
  fn text_measurement_validates_its_argument_shape() {
    assert!(measure_text(vec![]).is_err());
    assert!(measure_text(vec![Edn::Nil]).is_err());
    assert!(measure_paragraph(vec![]).is_err());
    assert!(measure_paragraph(vec![Edn::Nil]).is_err());
    assert!(render_to_png(vec![]).is_err());
    assert!(render_to_png(vec![Edn::Nil]).is_err());
    assert!(validate_scene(vec![]).is_err());
    assert_eq!(validate_scene(vec![Edn::Nil]).unwrap(), Edn::List(EdnListView(vec![])));
  }

  #[test]
  fn focus_ffi_validates_argument_shapes() {
    assert!(request_frame(vec![Edn::Nil]).is_err());
    assert!(request_focus(vec![]).is_err());
    assert!(request_focus(vec![Edn::Nil]).is_err());
    assert!(clear_focus(vec![Edn::Nil]).is_err());
    assert!(focused(vec![]).is_err());
    assert!(focused(vec![Edn::Nil]).is_err());
    assert!(read_clipboard_text(vec![Edn::Nil]).is_err());
    assert!(write_clipboard_text(vec![]).is_err());
    assert!(write_clipboard_text(vec![Edn::Nil]).is_err());
  }

  #[test]
  fn window_lifecycle_ffi_rejects_invalid_requests() {
    assert!(set_window_title(vec![Edn::Nil]).is_err());
    assert!(request_window_size(vec![Edn::Number(640.0)]).is_err());
    assert!(request_window_size(vec![Edn::Number(0.0), Edn::Number(480.0)])
      .unwrap_err()
      .contains("finite positive"));
    assert!(close_window(vec![Edn::Nil]).is_err());
  }
}

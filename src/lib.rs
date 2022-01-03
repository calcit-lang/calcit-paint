#[macro_use]
extern crate lazy_static;

use std::cell::RefCell;
// use std::time::Instant;

use euclid::Vector2D;

mod color;
mod extracter;
mod handlers;
mod key_listener;
mod primes;
mod renderer;
mod touches;

use std::sync::Arc;
use std::sync::RwLock;
// use std::time::Duration;
use std::{thread, time};

use cirru_edn::Edn;

use winit::dpi::LogicalSize;
use winit::event::Event;
use winit::event::WindowEvent;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

use gl::types::*;
use gl_rs as gl;
use glutin::GlProfile;
use skia_safe::{
  gpu::{gl::FramebufferInfo, BackendRenderTarget, SurfaceOrigin},
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

#[no_mangle]
pub fn launch_canvas(
  _args: Vec<Edn>,
  handler: Arc<dyn Fn(Vec<Edn>) -> Result<Edn, String> + Send + Sync + 'static>,
  _finish: Box<dyn FnOnce() + Send + Sync + 'static>,
) -> Result<Edn, String> {
  env_logger::init();

  // let duration = Instant::now().duration_since(started_time);
  // let initial_cost: f64 = duration.as_micros() as f64 / 1000.0; // in ms

  println!("\nRunner: in watch mode...\n");

  let el = EventLoop::new();

  let area_size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);

  let wb = WindowBuilder::new()
    .with_inner_size(area_size)
    .with_title("Calcit Paint");

  let cb = glutin::ContextBuilder::new()
    .with_depth_buffer(0)
    .with_stencil_buffer(8)
    .with_pixel_format(24, 8)
    .with_gl_profile(GlProfile::Core);

  let cb = cb.with_double_buffer(Some(true));

  let windowed_context = unsafe { cb.build_windowed(wb, &el).unwrap().make_current().unwrap() };

  let window = windowed_context.window();
  let pixel_format = windowed_context.get_pixel_format();

  println!("Pixel format of the window's GL context: {:?}", pixel_format);

  gl::load_with(|s| windowed_context.get_proc_address(s));

  let mut gr_context = skia_safe::gpu::DirectContext::new_gl(None, None).unwrap();

  let fb_info = {
    let mut fboid: GLint = 0;
    unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };

    FramebufferInfo {
      fboid: fboid as u32,
      format: skia_safe::gpu::gl::Format::RGBA8.into(),
    }
  };

  window.set_inner_size(glutin::dpi::Size::new(glutin::dpi::LogicalSize::new(WIDTH, HEIGHT)));

  let surface = create_surface(&windowed_context, &fb_info, &mut gr_context);
  let scale_factor = window.scale_factor() as f32;

  let mut env = Env {
    surface,
    gr_context,
    windowed_context,
  };

  let canvas = env.surface.canvas();
  canvas.scale((scale_factor, scale_factor));

  let event_loop = EventLoop::new();

  let mut first_paint = true;
  let track_mouse = RefCell::new(Vector2D::new(0.0, 0.0));
  let track_scale: RefCell<f32> = RefCell::new(scale_factor);
  // Handle events. Refer to `winit` docs for more information.
  event_loop.run(move |event, _window_target, control_flow| {
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
          env.surface = create_surface(&env.windowed_context, &fb_info, &mut env.gr_context);
          env.windowed_context.resize(physical_size);
          // println!("Window size changed: {:?}", size);
          let scale = track_scale.to_owned().into_inner();
          let w = physical_size.width as f32 / scale;
          let h = physical_size.height as f32 / scale;
          let _e = handlers::handle_resize(w as f64, h as f64).unwrap();

          // if let Err(err) = handler(vec![e]) {
          //   println!("error in handling event: {}", err);
          // } else {
          //   match take_drawing_data() {
          //     Ok(None) => {
          //       // nothing
          //     }
          //     Ok(Some(messages)) => {
          //       if let Err(e) = renderer::draw_page(&mut draw_target, messages, 2.2, true) {
          //         println!("Failed drawing: {:?}", e);
          //       }
          //     }
          //     Err(e) => {
          //       println!("failed extracting messages: {}", e)
          //     }
          //   }
          //   window.request_redraw();
          // }
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
        WindowEvent::MouseInput { state, button: _, .. } => {
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
                println!("error in handling event: {}", err);
              }
            }
            window.request_redraw();
          }
        },
        WindowEvent::CloseRequested => {
          *control_flow = ControlFlow::Exit;
          println!("User Close.");
          std::process::exit(0)
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
            let mut canvas = env.surface.canvas();
            canvas.clear(renderer::get_bg_color());
            canvas.reset_matrix();
            canvas.scale((scaled, scaled));
            if let Err(e) = renderer::draw_page(&mut canvas, messages, 2.2, true) {
              println!("Failed drawing: {:?}", e);
            }
          }
          Err(e) => {
            println!("failed extracting messages: {}", e)
          }
        }

        env.surface.flush();
        env.windowed_context.swap_buffers().unwrap();
      }
      Event::RedrawEventsCleared => {
        // println!("redraw events cleared");
      }
      Event::NewEvents(e) if e == winit::event::StartCause::Poll => {
        // nothing
      }
      Event::DeviceEvent { event: _event, .. } => {
        // println!("Device event fired: {:?}", event);
      }
      Event::NewEvents(_e) => {
        // println!("New events fired: {:?}", e);
      }
      e => {
        println!("unknown event: {:?}", e)
      }
    }
  });
}

fn take_drawing_data() -> Result<Vec<(Box<str>, Edn)>, String> {
  let mut m = NEXT_DRAWING_DATA.write().unwrap();
  let ret = m.to_owned();
  *m = vec![];
  if ret.is_empty() {
    Ok(vec![])
  } else {
    let mut ys: Vec<(Box<str>, Edn)> = vec![];
    for (op, data) in ret {
      ys.push((op.to_owned(), data.to_owned()));
    }
    Ok(ys)
  }
}

#[no_mangle]
pub fn push_drawing_data(args: Vec<Edn>) -> Result<Edn, String> {
  if args.len() == 2 {
    if let Edn::Str(s) = &args[0] {
      let mut m = NEXT_DRAWING_DATA.write().unwrap();
      m.push((s.to_owned(), args[1].to_owned()));
    }
  } else {
    return Err(format!("expected 2 arguments, got: {:?}", args));
  }
  Ok(Edn::Nil)
}

#[no_mangle]
pub fn abi_version() -> String {
  String::from("0.0.6")
}

fn create_surface(
  windowed_context: &WindowedContext,
  fb_info: &FramebufferInfo,
  gr_context: &mut skia_safe::gpu::DirectContext,
) -> skia_safe::Surface {
  let pixel_format = windowed_context.get_pixel_format();
  let size = windowed_context.window().inner_size();
  let backend_render_target = BackendRenderTarget::new_gl(
    (size.width as i32, size.height as i32),
    pixel_format.multisampling.map(|s| s as usize),
    pixel_format.stencil_bits as usize,
    *fb_info,
  );
  Surface::from_backend_render_target(
    gr_context,
    &backend_render_target,
    SurfaceOrigin::BottomLeft,
    ColorType::RGBA8888,
    None,
    None,
  )
  .unwrap()
}

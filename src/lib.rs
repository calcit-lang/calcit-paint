#[macro_use]
extern crate lazy_static;

use log::error;

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
// use std::{thread, time};

use cirru_edn::Edn;

use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::Event;
use winit::event::WindowEvent;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

use raqote::DrawTarget;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 600;

lazy_static! {
  static ref NEXT_DRAWING_DATA: RwLock<Vec<(String, Edn)>> = RwLock::new(vec![]);
}

#[no_mangle]
pub fn launch_canvas(
  args: Vec<Edn>,
  handler: Arc<dyn Fn(Vec<Edn>) -> Result<Edn, String> + Send + Sync + 'static>,
  _finish: Box<dyn FnOnce() + Send + Sync + 'static>,
) -> Result<Edn, String> {
  env_logger::init();

  // let duration = Instant::now().duration_since(started_time);
  // let initial_cost: f64 = duration.as_micros() as f64 / 1000.0; // in ms

  println!("\nRunner: in watch mode...\n");

  let event_loop = EventLoop::new();

  let window = {
    let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
    WindowBuilder::new()
      .with_title("Calcit Paint")
      .with_inner_size(size)
      .build(&event_loop)
      .unwrap()
  };

  let mut pixels = {
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap() // TODO handle error
  };

  let mut draw_target = DrawTarget::new(WIDTH as i32, HEIGHT as i32);

  let mut first_paint = true;
  let track_mouse = RefCell::new(Vector2D::new(0.0, 0.0));
  let track_scale: RefCell<f32> = RefCell::new(window.scale_factor() as f32);
  // Handle events. Refer to `winit` docs for more information.
  event_loop.run(move |event, _window_target, control_flow| {
    // println!("Event: {:?}", event);

    // TODO
    // if first_paint {
    //   if let Err(e) = renderer::draw_page(&mut draw_target, initial_cost, false) {
    //     println!("failed first paint: {:?}", e);
    //   }

    //   // Update internal state and request a redraw
    //   window.request_redraw();
    //   first_paint = false
    // }

    match event {
      Event::WindowEvent { event, .. } => match event {
        WindowEvent::Resized(size) => {
          println!("Window size changed: {:?}", size);
          let scale = track_scale.to_owned().into_inner();
          pixels.resize_surface(size.width, size.height);
          let w = size.width as f32 / scale;
          let h = size.height as f32 / scale;
          pixels.resize_buffer(w as u32, h as u32);
          draw_target = DrawTarget::new(w as i32, h as i32);
          let e = handlers::handle_resize(w as f64, h as f64).unwrap();

          if let Err(err) = handler(vec![e]) {
            println!("error in handling event: {}", err);
          } else {
            match take_drawing_data() {
              Ok(None) => {
                // nothing
              }
              Ok(Some(messages)) => {
                if let Err(e) = renderer::draw_page(&mut draw_target, messages, 2.2, true) {
                  println!("Failed drawing: {:?}", e);
                }
              }
              Err(e) => {
                println!("failed extracting messages: {}", e)
              }
            }
            window.request_redraw();
          }
        }
        WindowEvent::ScaleFactorChanged {
          scale_factor: factor,
          new_inner_size: size,
        } => {
          println!("DPI scale change {} {:?}", factor, size);
          track_scale.replace(factor as f32);
          pixels.resize_surface(size.width, size.height);
          window.request_redraw();
        }
        WindowEvent::CursorMoved { position, .. } => {
          let scale = track_scale.to_owned().into_inner();
          let event_info = handlers::handle_mouse_move(
            Vector2D::new((position.x as f32) / scale, (position.y as f32) / scale),
            &track_mouse,
          );

          if let Some(e) = event_info {
            if let Err(err) = handler(vec![e]) {
              println!("error in handling event: {}", err);
            }
            window.request_redraw();
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
          }
          window.request_redraw();
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
          println!("User Close.");
          std::process::exit(0)
        }
        // `CloseRequested` and `KeyboardInput` events won't appear here.
        x => println!("Other window event fired: {:?}", x),
      },

      Event::MainEventsCleared => {
        // println!("main events cleared");

        // some break

        todo!(); // check if rendered
      }
      Event::RedrawRequested(_wid) => {
        for (dst, &src) in pixels
          .get_frame()
          .chunks_exact_mut(4)
          .zip(draw_target.get_data().iter())
        {
          dst[0] = (src >> 16) as u8;
          dst[1] = (src >> 8) as u8;
          dst[2] = src as u8;
          dst[3] = (src >> 24) as u8;
        }

        if pixels
          .render()
          .map_err(|e| error!("pixels.render() failed: {}", e))
          .is_err()
        {
          *control_flow = ControlFlow::Exit;
        }
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
      e => {
        println!("unkwnon event: {:?}", e)
      }
    }
  });

  Ok(Edn::Nil)
}

fn take_drawing_data() -> Result<Option<Vec<(String, Edn)>>, String> {
  let mut m = NEXT_DRAWING_DATA.write().unwrap();
  let ret = m.to_owned();
  *m = vec![];
  if ret.is_empty() {
    Ok(None)
  } else {
    let mut ys: Vec<(String, Edn)> = vec![];
    for (op, data) in ret {
      ys.push((op.to_owned(), data.to_owned()));
    }
    Ok(Some(ys))
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
  String::from("0.0.1")
}

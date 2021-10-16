#[macro_use]
extern crate lazy_static;

use log::error;

use std::cell::RefCell;
use std::time::Instant;

use euclid::Vector2D;

mod color;
mod extracter;
mod handlers;
mod injection;
mod key_listener;
mod primes;
mod renderer;
mod touches;

use std::sync::mpsc::TryRecvError;

use std::fs;
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::{thread, time};

use notify::{RecommendedWatcher, RecursiveMode, Watcher};

use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::Event;
use winit::event::WindowEvent;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

use raqote::DrawTarget;

use calcit_runner::CalcitItems;
use calcit_runner::{builtins, call_stack, cli_args, program, program::ProgramCodeData, snapshot};

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 600;

pub fn main() -> Result<(), String> {
  env_logger::init();
  builtins::effects::init_effects_states();
  let cli_matches = cli_args::parse_cli();

  println!("calcit_runner version: {}", cli_args::CALCIT_VERSION);

  let core_snapshot = calcit_runner::load_core_snapshot()?;

  injection::inject_platform_apis();

  // load entry file
  let entry_path = Path::new(cli_matches.value_of("input").unwrap());
  let content = fs::read_to_string(entry_path).unwrap_or_else(|_| panic!("expected Cirru snapshot: {:?}", entry_path));
  let data = cirru_edn::parse(&content)?;
  // println!("reading: {}", content);
  let mut snapshot = snapshot::load_snapshot_data(data, entry_path.to_str().unwrap())?;
  let init_fn = cli_matches
    .value_of("init-fn")
    .unwrap_or(&snapshot.configs.init_fn)
    .to_owned();
  let reload_fn = cli_matches
    .value_of("reload-fn")
    .unwrap_or(&snapshot.configs.reload_fn)
    .to_owned();

  // attach modules
  for module_path in &snapshot.configs.modules {
    let module_data = calcit_runner::load_module(module_path, entry_path.parent().unwrap())?;
    for (k, v) in &module_data.files {
      snapshot.files.insert(k.to_owned(), v.to_owned());
    }
  }

  // attach core
  for (k, v) in core_snapshot.files {
    snapshot.files.insert(k.to_owned(), v.to_owned());
  }
  let mut program_code = program::extract_program_data(&snapshot)?;
  let check_warnings: &RefCell<Vec<String>> = &RefCell::new(vec![]);

  // make sure builtin classes are touched
  calcit_runner::runner::preprocess::preprocess_ns_def(
    calcit_runner::primes::CORE_NS,
    calcit_runner::primes::BUILTIN_CLASSES_ENTRY,
    &program_code,
    calcit_runner::primes::BUILTIN_CLASSES_ENTRY,
    None,
    check_warnings,
  )
  .map_err(|e| {
    for w in e.warnings {
      println!("{}", w);
    }
    e.msg
  })?;

  let warnings = check_warnings.to_owned().into_inner();
  if !warnings.is_empty() {
    for message in &warnings {
      println!("{}", message);
    }

    return Err(format!("Found {} warnings, runner blocked", warnings.len()));
  }

  let started_time = Instant::now();
  let _v = calcit_runner::run_program(&init_fn, im::vector![], &program_code).map_err(|e| {
    for w in e.warnings {
      println!("{}", w);
    }
    e.msg
  })?;
  let duration = Instant::now().duration_since(started_time);
  let initial_cost: f64 = duration.as_micros() as f64 / 1000.0; // in ms

  println!("\nRunner: in watch mode...\n");
  let (tx, rx) = channel();
  let entry_path = Path::new(cli_matches.value_of("input").unwrap());
  let event_entry = cli_matches.value_of("event-entry").unwrap().to_owned();

  let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_millis(200)).unwrap();
  let event_loop = EventLoop::new();

  let inc_path = entry_path.parent().unwrap().join(".compact-inc.cirru").to_owned();
  if !inc_path.exists() {
    let _todo = fs::write(&inc_path, "");
  }
  watcher.watch(&inc_path, RecursiveMode::NonRecursive).unwrap();

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

    if first_paint {
      if let Err(e) = renderer::draw_page(&mut draw_target, initial_cost) {
        println!("failed first paint: {:?}", e);
      }

      // Update internal state and request a redraw
      window.request_redraw();
      first_paint = false
    }

    match event {
      Event::WindowEvent { event, .. } => match event {
        WindowEvent::Resized(size) => {
          println!("Window size change change {:?}", size);
          pixels.resize_surface(size.width, size.height);
          pixels.resize_buffer(size.width, size.height);
          draw_target = DrawTarget::new(size.width as i32, size.height as i32);
          window.request_redraw();
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
            handle_calcit_event(&mut draw_target, &mut program_code, &event_entry, im::vector![e]);
            window.request_redraw();
          }
        }
        WindowEvent::MouseInput { state, button: _, .. } => {
          // println!("mouse button: {:?}", button);
          let event_info = match state {
            winit::event::ElementState::Pressed => handlers::handle_mouse_down(&track_mouse),
            winit::event::ElementState::Released => handlers::handle_mouse_up(&track_mouse),
          };
          handle_calcit_event(
            &mut draw_target,
            &mut program_code,
            &event_entry,
            im::vector![event_info],
          );
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
              handle_calcit_event(
                &mut draw_target,
                &mut program_code,
                &event_entry,
                im::vector![event_info],
              );
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
        match rx.try_recv() {
          Err(TryRecvError::Empty) => {
            thread::sleep(time::Duration::from_millis(50));
          } // most of the time
          Ok(event) => {
            // println!("event: {:?}", event);
            match event {
              notify::DebouncedEvent::NoticeWrite(..) => {
                // response later
                // some break
              }
              notify::DebouncedEvent::Write(_) => {
                let reload_libs = cli_matches.is_present("reload-libs");
                handle_code_change(
                  &mut draw_target,
                  &mut program_code,
                  &init_fn,
                  &reload_fn,
                  &inc_path,
                  reload_libs,
                )
                .unwrap();
                window.request_redraw();
              }
              _ => println!("other file event: {:?}, ignored", event),
            }
          }
          Err(e) => println!("watch error: {:?}", e),
        }

        // some break
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
}

fn handle_code_change(
  draw_target: &mut DrawTarget,
  program_code: &mut ProgramCodeData,
  init_fn: &str,
  reload_fn: &str,
  inc_path: &Path,
  reload_libs: bool,
) -> Result<(), String> {
  println!("\n-------- file change --------\n");
  call_stack::clear_stack();
  // load new program code
  let content = fs::read_to_string(inc_path).unwrap();
  if content.trim() == "" {
    println!("failed re-compiling, got empty inc file");
  } else {
    let started_time = Instant::now();
    let data = cirru_edn::parse(&content)?;
    let changes = snapshot::load_changes_info(data.to_owned())?;
    let new_code = program::apply_code_changes(program_code, &changes)?;
    // println!("\nprogram code: {:?}", new_code);
    // clear data in evaled states
    program::clear_all_program_evaled_defs(init_fn, reload_fn, reload_libs)?;
    builtins::meta::force_reset_gensym_index()?;
    // run from `reload_fn` after reload
    calcit_runner::run_program(reload_fn, im::vector![], &new_code).map_err(|e| {
      for w in e.warnings {
        println!("{}", w);
      }
      e.msg
    })?;
    // overwrite previous state
    let duration = Instant::now().duration_since(started_time);
    let cost: f64 = duration.as_micros() as f64 / 1000.0;
    if let Err(e) = renderer::draw_page(draw_target, cost) {
      println!("Failed drawing: {:?}", e);
    }

    // Update internal state and request a redraw
    *program_code = new_code;
  }
  Ok(())
}

fn handle_calcit_event(
  draw_target: &mut DrawTarget,
  program_code: &mut ProgramCodeData,
  event_entry: &str,
  params: CalcitItems,
) {
  let started_time = Instant::now();
  let mut cost: f64 = 0.0; // in ms

  call_stack::clear_stack();
  match calcit_runner::run_program(event_entry, params, program_code) {
    Ok(_v) => {
      let duration = Instant::now().duration_since(started_time);
      cost = duration.as_micros() as f64 / 1000.0;
    }
    Err(e) => println!("failed falling on-window-event: {}", e),
  }

  if let Err(e) = renderer::draw_page(draw_target, cost) {
    println!("Failed drawing: {:?}", e);
  }
}

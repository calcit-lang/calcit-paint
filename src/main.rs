#[macro_use]
extern crate lazy_static;

use glam::Vec2;

use std::cell::RefCell;
use std::env;
use std::path;
use std::time::Instant;

mod color;
mod extracter;
mod handlers;
mod key_listener;
mod primes;
mod renderer;
mod touches;

use std::sync::mpsc::TryRecvError;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event;
use ggez::event::winit_event::{Event, KeyboardInput, WindowEvent};
use ggez::graphics::{self};
use ggez::{Context, GameResult};
use winit::event_loop::ControlFlow;

use renderer::to_game_err;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::{thread, time};

use notify::{RecommendedWatcher, RecursiveMode, Watcher};

use calcit_runner;
use calcit_runner::CalcitItems;
use calcit_runner::{builtins, call_stack, cli_args, program, program::ProgramCodeData, snapshot};

pub fn main() -> GameResult {
  builtins::effects::init_effects_states();
  let cli_matches = cli_args::parse_cli();

  println!("calcit_runner version: {}", cli_args::CALCIT_VERSION);

  let core_snapshot = calcit_runner::load_core_snapshot().map_err(to_game_err)?;

  // load entry file
  let entry_path = Path::new(cli_matches.value_of("input").unwrap());
  let content = fs::read_to_string(entry_path).expect(&format!("expected Cirru snapshot: {:?}", entry_path));
  let data = cirru_edn::parse(&content).map_err(to_game_err)?;
  // println!("reading: {}", content);
  let mut snapshot = snapshot::load_snapshot_data(data).map_err(to_game_err)?;
  let init_fn = cli_matches
    .value_of("init-fn")
    .or(Some(&snapshot.configs.init_fn))
    .unwrap()
    .to_owned();
  let reload_fn = cli_matches
    .value_of("reload-fn")
    .or(Some(&snapshot.configs.reload_fn))
    .unwrap()
    .to_owned();

  // attach modules
  for module_path in &snapshot.configs.modules {
    let module_data = calcit_runner::load_module(&module_path, entry_path.parent().unwrap()).map_err(to_game_err)?;
    for (k, v) in &module_data.files {
      snapshot.files.insert(k.clone(), v.clone());
    }
  }

  // attach core
  for (k, v) in core_snapshot.files {
    snapshot.files.insert(k.clone(), v.clone());
  }
  let mut program_code = program::extract_program_data(&snapshot).map_err(to_game_err)?;
  // make sure builtin classes are touched
  calcit_runner::runner::preprocess::preprocess_ns_def(
    &calcit_runner::primes::CORE_NS,
    &calcit_runner::primes::BUILTIN_CLASSES_ENTRY,
    &program_code,
    &calcit_runner::primes::BUILTIN_CLASSES_ENTRY,
    None,
  )
  .map_err(to_game_err)?;

  let started_time = Instant::now();
  let _v = calcit_runner::run_program(&init_fn, im::vector![], &program_code).map_err(to_game_err)?;
  let duration = Instant::now().duration_since(started_time);
  let initial_cost: f64 = duration.as_micros() as f64 / 1000.0; // in ms

  println!("\nRunner: in watch mode...\n");
  let (tx, rx) = channel();
  let entry_path = Path::new(cli_matches.value_of("input").unwrap());
  let event_entry = cli_matches.value_of("event-entry").unwrap().to_owned();

  let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_millis(200)).unwrap();

  let inc_path = entry_path.parent().unwrap().join(".compact-inc.cirru").to_owned();
  if !inc_path.exists() {
    fs::write(&inc_path, "").map_err(|e| -> ggez::GameError { to_game_err(e.to_string()) })?;
  }
  watcher.watch(&inc_path, RecursiveMode::NonRecursive).unwrap();

  let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
    let mut path = path::PathBuf::from(manifest_dir);
    path.push("resources");
    path
  } else {
    path::PathBuf::from("./resources")
  };

  let (mut ctx, events_loop) = ggez::ContextBuilder::new("eventloop", "ggez")
    .add_resource_path(resource_dir)
    .window_setup(WindowSetup::default().title("Painter driven by Calcit"))
    .window_mode(WindowMode::default().dimensions(1000.0, 600.0).resizable(true))
    .build()?;

  let mut first_paint = true;
  let track_mouse = RefCell::new(Vec2::new(0.0, 0.0));
  // Handle events. Refer to `winit` docs for more information.
  events_loop.run(move |event, _window_target, control_flow| {
    // println!("Event: {:?}", event);
    if !ctx.continuing {
      *control_flow = ControlFlow::Exit;
      return;
    }

    *control_flow = ControlFlow::Poll;
    let ctx = &mut ctx;
    event::process_event(ctx, &event);
    if first_paint {
      ctx.timer_context.tick();

      if let Err(e) = renderer::draw_page(ctx, initial_cost) {
        println!("failed first paint: {:?}", e);
      }
      first_paint = false
    }

    match event {
      Event::WindowEvent { event, .. } => match event {
        WindowEvent::CloseRequested => event::quit(ctx),
        WindowEvent::Resized(_logical_size) => {
          // goto request_redraw
        }
        WindowEvent::ScaleFactorChanged {
          scale_factor: _f,
          new_inner_size: _size,
        } => {
          // goto request_redraw
        }
        WindowEvent::CursorMoved { position, .. } => {
          let event_info = handlers::handle_mouse_move(Vec2::new(position.x as f32, position.y as f32), &track_mouse);
          match event_info {
            Some(e) => handle_calcit_event(ctx, &mut program_code, &event_entry, im::vector![e]),
            None => (),
          }
        }
        WindowEvent::MouseInput { state, button, .. } => {
          // println!("mouse button: {:?}", button);
          let event_info = match state {
            winit::event::ElementState::Pressed => handlers::handle_mouse_down(&track_mouse),
            winit::event::ElementState::Released => handlers::handle_mouse_up(&track_mouse),
          };
          handle_calcit_event(ctx, &mut program_code, &event_entry, im::vector![event_info]);
        }
        WindowEvent::KeyboardInput {
          input:
            KeyboardInput {
              state: key_state,
              scancode: _c, // unknown order
              virtual_keycode: Some(keycode),
              ..
            },
          ..
        } => match keycode {
          event::KeyCode::Escape => *control_flow = ControlFlow::Exit,
          _ => {
            // println!("keyboard event: {:?} {:?}", keycode, scancode);
            let event_infos = handlers::handle_keyboard(keycode, key_state);
            for event_info in event_infos {
              handle_calcit_event(ctx, &mut program_code, &event_entry, im::vector![event_info]);
            }
          }
        },
        // `CloseRequested` and `KeyboardInput` events won't appear here.
        x => println!("Other window event fired: {:?}", x),
      },

      Event::MainEventsCleared => {
        // println!("main events cleared");
        match rx.try_recv() {
          Err(TryRecvError::Empty) => {
            ggez::timer::yield_now();
            thread::sleep(time::Duration::from_millis(50));
          } // most of the time
          Ok(event) => {
            // println!("event: {:?}", event);
            match event {
              notify::DebouncedEvent::NoticeWrite(..) => {
                // response later
                ggez::timer::yield_now();
              }
              notify::DebouncedEvent::Write(_) => {
                let reload_libs = cli_matches.is_present("reload-libs");
                handle_code_change(ctx, &mut program_code, &init_fn, &reload_fn, &inc_path, reload_libs);
              }
              _ => println!("other file event: {:?}, ignored", event),
            }
          }
          Err(e) => println!("watch error: {:?}", e),
        }

        ggez::timer::yield_now();
      }
      Event::RedrawRequested(_wid) => {
        let size = graphics::window(ctx).inner_size();
        let new_rect = graphics::Rect::new(0.0, 0.0, size.width as f32, size.height as f32);
        graphics::set_screen_coordinates(ctx, new_rect).unwrap();
        let event_info = handlers::handle_redraw();
        handle_calcit_event(ctx, &mut program_code, &event_entry, im::vector![event_info]);
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
  ctx: &mut Context,
  program_code: &mut ProgramCodeData,
  init_fn: &str,
  reload_fn: &str,
  inc_path: &PathBuf,
  reload_libs: bool,
) {
  println!("\n-------- file change --------\n");
  call_stack::clear_stack();
  // load new program code
  let content = fs::read_to_string(inc_path).unwrap();
  if content.trim() == "" {
    println!("failed re-compiling, got empty inc file");
  } else {
    let started_time = Instant::now();
    let data = cirru_edn::parse(&content).unwrap();
    let changes = snapshot::load_changes_info(data.clone()).unwrap();
    let new_code = program::apply_code_changes(&program_code, &changes).unwrap();
    // println!("\nprogram code: {:?}", new_code);
    // clear data in evaled states
    program::clear_all_program_evaled_defs(init_fn, reload_fn, reload_libs).unwrap();
    builtins::meta::force_reset_gensym_index().unwrap();
    // run from `reload_fn` after reload
    calcit_runner::run_program(reload_fn, im::vector![], &new_code).unwrap();
    // overwrite previous state
    let duration = Instant::now().duration_since(started_time);
    let cost: f64 = duration.as_micros() as f64 / 1000.0;
    if let Err(e) = renderer::draw_page(ctx, cost) {
      println!("Failed drawing: {:?}", e);
    }
    *program_code = new_code;
    ctx.timer_context.tick();
  }
}

fn handle_calcit_event(ctx: &mut Context, program_code: &mut ProgramCodeData, event_entry: &str, params: CalcitItems) {
  let started_time = Instant::now();
  let mut cost: f64 = 0.0; // in ms

  call_stack::clear_stack();
  match calcit_runner::run_program(event_entry, params, &program_code) {
    Ok(_v) => {
      let duration = Instant::now().duration_since(started_time);
      cost = duration.as_micros() as f64 / 1000.0;
    }
    Err(e) => println!("failed falling on-window-event: {}", e),
  }
  ctx.timer_context.tick();

  if let Err(e) = renderer::draw_page(ctx, cost) {
    println!("Failed drawing: {:?}", e);
  }
}

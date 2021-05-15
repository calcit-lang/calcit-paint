#[macro_use]
extern crate lazy_static;

use ggez;
use glam::Vec2;

use std::cell::RefCell;
use std::env;
use std::path;

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
use ggez::GameResult;

use renderer::to_game_err;
use std::fs;
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::{thread, time};

use notify::{RecommendedWatcher, RecursiveMode, Watcher};

use calcit_runner;
use calcit_runner::CalcitItems;
use calcit_runner::{builtins, call_stack, cli_args, program, snapshot};

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

  calcit_runner::run_program(&init_fn, im::vector![], &program_code).map_err(to_game_err)?;

  println!("\nRunner: in watch mode...\n");
  let (tx, rx) = channel();
  let entry_path = Path::new(cli_matches.value_of("input").unwrap());
  let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_millis(200)).unwrap();

  let inc_path = entry_path.parent().unwrap().join(".compact-inc.cirru").to_owned();
  if inc_path.exists() {
    watcher.watch(&inc_path, RecursiveMode::NonRecursive).unwrap();
  } else {
    return Err(to_game_err(format!("path {:?} not existed", &inc_path)));
  }

  let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
    let mut path = path::PathBuf::from(manifest_dir);
    path.push("resources");
    path
  } else {
    path::PathBuf::from("./resources")
  };

  let (ref mut ctx, mut events_loop) = ggez::ContextBuilder::new("eventloop", "ggez")
    .add_resource_path(resource_dir)
    .window_setup(WindowSetup::default().title("Painter driven by Calcit"))
    .window_mode(WindowMode::default().dimensions(1000.0, 600.0).resizable(true))
    .build()?;

  let mut first_paint = true;
  let track_mouse = RefCell::new(Vec2::new(0.0, 0.0));

  while ctx.continuing {
    // Handle events. Refer to `winit` docs for more information.
    use winit::platform::run_return::EventLoopExtRunReturn;
    events_loop.run_return(|event, _window_target, control_flow| {
      // println!("Event: {:?}", event);
      ctx.process_event(&event);
      if first_paint {
        if let Err(e) = renderer::draw_page(ctx) {
          println!("failed first paint: {:?}", e);
        }
        first_paint = false
      }

      let handle_calcit_event =
        |params: CalcitItems| match calcit_runner::run_program("app.main/on-window-event", params, &program_code) {
          Ok(..) => (),
          Err(e) => println!("failed falling on-window-event: {}", e),
        };

      match event {
        Event::MainEventsCleared => {
          ctx.timer_context.tick();
        }
        Event::WindowEvent { event, .. } => match event {
          WindowEvent::CloseRequested => event::quit(ctx),
          WindowEvent::Resized(logical_size) => {
            let new_rect = graphics::Rect::new(0.0, 0.0, logical_size.width as f32, logical_size.height as f32);
            graphics::set_screen_coordinates(ctx, new_rect).unwrap();
            // TODO call rerender
          }
          WindowEvent::CursorMoved { position, .. } => {
            let event_info = handlers::handle_mouse_move(Vec2::new(position.x as f32, position.y as f32), &track_mouse);
            match event_info {
              Some(e) => handle_calcit_event(im::vector![e]),
              None => (),
            }
          }
          WindowEvent::MouseInput { state, button, .. } => {
            println!("mouse button: {:?}", button);
            let event_info = match state {
              winit::event::ElementState::Pressed => handlers::handle_mouse_down(&track_mouse),
              winit::event::ElementState::Released => handlers::handle_mouse_up(&track_mouse),
            };
            handle_calcit_event(im::vector![event_info]);
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
            event::KeyCode::Escape => *control_flow = winit::event_loop::ControlFlow::Exit,
            _ => {
              // println!("keyboard event: {:?} {:?}", keycode, scancode);
              let event_infos = handlers::handle_keyboard(keycode, key_state);
              for event_info in event_infos {
                handle_calcit_event(im::vector![event_info]);
              }
            }
          },
          // `CloseRequested` and `KeyboardInput` events won't appear here.
          x => println!("Other window event fired: {:?}", x),
        },

        _x => {
          // println!("Device event fired: {:?}", x);
          match rx.try_recv() {
            Err(TryRecvError::Empty) => {
              thread::sleep(time::Duration::from_millis(140));
            } // most of the time
            Ok(event) => {
              println!("event: {:?}", event);
              match event {
                notify::DebouncedEvent::NoticeWrite(..) => {
                  // idle, sleep for a while
                }
                notify::DebouncedEvent::Write(_) => {
                  println!("\n-------- file change --------\n");
                  call_stack::clear_stack();
                  touches::reset_touches_stack();
                  key_listener::reset_listeners_stack();
                  // load new program code
                  let content = fs::read_to_string(&inc_path).unwrap();
                  if content.trim() == "" {
                    println!("failed re-compiling, got empty inc file");
                  } else {
                    let data = cirru_edn::parse(&content).unwrap();
                    let changes = snapshot::load_changes_info(data.clone()).unwrap();
                    let new_code = program::apply_code_changes(&program_code, &changes).unwrap();
                    // println!("\nprogram code: {:?}", new_code);
                    // clear data in evaled states
                    let reload_libs = cli_matches.is_present("reload-libs");
                    program::clear_all_program_evaled_defs(&init_fn, &reload_fn, reload_libs).unwrap();
                    builtins::meta::force_reset_gensym_index().unwrap();
                    // run from `reload_fn` after reload
                    calcit_runner::run_program(&reload_fn, im::vector![], &new_code).unwrap();
                    // overwrite previous state
                    program_code = new_code;
                  }
                  if let Err(e) = renderer::draw_page(ctx) {
                    println!("Failed drawing: {:?}", e);
                  }
                }
                _ => println!("other file event: {:?}, ignored", event),
              }
            }
            Err(e) => println!("watch error: {:?}", e),
          }
        }
      }
    });

    // ggez::timer::yield_now();
  }
  Ok(())
}

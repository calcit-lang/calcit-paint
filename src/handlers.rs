use glam::Vec2;
use std::cell::RefCell;

use calcit_runner::Calcit;
use ggez::event::KeyCode;

use crate::{key_listener, primes::kwd, touches};

// TODO track position

pub fn handle_mouse_down(mouse: &RefCell<Vec2>) -> Calcit {
  // println!("mouse down: {:?}", mouse.to_owned().into_inner());
  let mut info: im::HashMap<Calcit, Calcit> = im::HashMap::new();
  let position = mouse.to_owned().into_inner();

  info.insert(kwd("type"), kwd("mouse-down"));
  info.insert(kwd("clicks"), Calcit::Number(1.0)); // TODO
  info.insert(kwd("x"), Calcit::Number(position.x as f64));
  info.insert(kwd("y"), Calcit::Number(position.y as f64));

  match touches::find_touch_area(position) {
    Some(target) => {
      info.insert(kwd("action"), target.action.to_owned());
      info.insert(kwd("path"), target.path.to_owned());
      info.insert(kwd("data"), target.data.to_owned());
      touches::track_mouse_drag(
        position,
        target.action.to_owned(),
        target.path.to_owned(),
        target.data.to_owned(),
      );
    }
    None => (),
  }

  Calcit::Map(info)
}

pub fn handle_mouse_up(mouse: &RefCell<Vec2>) -> Calcit {
  // println!("mouse up: {:?}", mouse.to_owned().into_inner());
  let mut info: im::HashMap<Calcit, Calcit> = im::HashMap::new();
  let position = mouse.to_owned().into_inner();

  info.insert(kwd("type"), kwd("mouse-up"));
  info.insert(kwd("x"), Calcit::Number(position.x as f64));
  info.insert(kwd("y"), Calcit::Number(position.y as f64));
  info.insert(kwd("clicks"), Calcit::Number(1.0)); // TODO

  match touches::read_mouse_tracked_state() {
    Some(tracked_state) => {
      let p0 = tracked_state.position;
      info.insert(kwd("action"), tracked_state.action);
      info.insert(kwd("path"), tracked_state.path);
      info.insert(kwd("data"), tracked_state.data);
      info.insert(kwd("dx"), Calcit::Number((position.x - p0.x) as f64));
      info.insert(kwd("dy"), Calcit::Number((position.y - p0.y) as f64));

      touches::release_mouse_drag();
    }
    None => (),
  }
  Calcit::Map(info)
}

pub fn handle_mouse_move(position: Vec2, mouse: &RefCell<Vec2>) -> Option<Calcit> {
  if position == mouse.to_owned().into_inner() {
    // triggered a same position, ignored
    None
  } else {
    mouse.replace(position);
    // println!("mouse move: {:?}", position);
    let mut info: im::HashMap<Calcit, Calcit> = im::HashMap::new();
    info.insert(kwd("type"), kwd("mouse-move"));
    info.insert(kwd("clicks"), Calcit::Number(1.0)); // TODO
    info.insert(kwd("x"), Calcit::Number(position.x as f64));
    info.insert(kwd("y"), Calcit::Number(position.y as f64));

    match touches::read_mouse_tracked_state() {
      Some(tracked_state) => {
        let p0 = tracked_state.position;
        info.insert(kwd("action"), tracked_state.action);
        info.insert(kwd("path"), tracked_state.path);
        info.insert(kwd("data"), tracked_state.data);
        info.insert(kwd("dx"), Calcit::Number((position.x - p0.x) as f64));
        info.insert(kwd("dy"), Calcit::Number((position.y - p0.y) as f64));
      }
      None => (),
    }

    Some(Calcit::Map(info))
  }
}

pub fn handle_keyboard(keycode: KeyCode, key_state: winit::event::ElementState) -> Vec<Calcit> {
  let targets = key_listener::find_key_listeners(&name_key(keycode));
  if targets.is_empty() {
    let mut info: im::HashMap<Calcit, Calcit> = im::HashMap::new();
    info.insert(
      kwd("type"),
      match key_state {
        winit::event::ElementState::Pressed => kwd("key-down"),
        winit::event::ElementState::Released => kwd("key-up"),
      },
    );
    info.insert(kwd("key-code"), Calcit::Number(keycode as usize as f64));
    info.insert(kwd("name"), Calcit::Str(name_key(keycode)));
    vec![Calcit::Map(info)]
  } else {
    let mut hits: Vec<Calcit> = vec![];
    for target in targets {
      let mut info: im::HashMap<Calcit, Calcit> = im::HashMap::new();
      info.insert(
        kwd("type"),
        match key_state {
          winit::event::ElementState::Pressed => kwd("key-down"),
          winit::event::ElementState::Released => kwd("key-up"),
        },
      );
      info.insert(kwd("key-code"), Calcit::Number(keycode as usize as f64));
      info.insert(kwd("name"), Calcit::Str(name_key(keycode)));
      info.insert(kwd("action"), target.action);
      info.insert(kwd("path"), target.path);
      info.insert(kwd("data"), target.data);
      hits.push(Calcit::Map(info));
    }
    hits
  }
}

pub fn name_key(keycode: KeyCode) -> String {
  format!("{:?}", keycode) // TODO
}

use std::cell::RefCell;

use cirru_edn::Edn;
use euclid::Vector2D;
use std::collections::HashMap;

use crate::{key_listener, primes::kwd, touches};

// TODO track position

pub fn handle_mouse_down(mouse: &RefCell<Vector2D<f32, f32>>) -> Edn {
  // println!("mouse down: {:?}", mouse.to_owned().into_inner());
  let mut info: HashMap<Edn, Edn> = HashMap::new();
  let position = mouse.to_owned().into_inner();

  info.insert(kwd("type"), kwd("mouse-down"));
  info.insert(kwd("clicks"), Edn::Number(1.0)); // TODO
  info.insert(kwd("x"), Edn::Number(position.x as f64));
  info.insert(kwd("y"), Edn::Number(position.y as f64));

  if let Some(target) = touches::find_touch_area(position) {
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

  Edn::Map(info)
}

pub fn handle_mouse_up(mouse: &RefCell<Vector2D<f32, f32>>) -> Edn {
  // println!("mouse up: {:?}", mouse.to_owned().into_inner());
  let mut info: HashMap<Edn, Edn> = HashMap::new();
  let position = mouse.to_owned().into_inner();

  info.insert(kwd("type"), kwd("mouse-up"));
  info.insert(kwd("x"), Edn::Number(position.x as f64));
  info.insert(kwd("y"), Edn::Number(position.y as f64));
  info.insert(kwd("clicks"), Edn::Number(1.0)); // TODO

  if let Some(tracked_state) = touches::read_mouse_tracked_state() {
    let p0 = tracked_state.position;
    info.insert(kwd("action"), tracked_state.action);
    info.insert(kwd("path"), tracked_state.path);
    info.insert(kwd("data"), tracked_state.data);
    info.insert(kwd("dx"), Edn::Number((position.x - p0.x) as f64));
    info.insert(kwd("dy"), Edn::Number((position.y - p0.y) as f64));

    touches::release_mouse_drag();
  }

  Edn::Map(info)
}

pub fn handle_mouse_move(position: Vector2D<f32, f32>, mouse: &RefCell<Vector2D<f32, f32>>) -> Option<Edn> {
  if position == mouse.to_owned().into_inner() {
    // triggered a same position, ignored
    None
  } else {
    mouse.replace(position);
    // println!("mouse move: {:?}", position);
    let mut info: HashMap<Edn, Edn> = HashMap::new();
    info.insert(kwd("type"), kwd("mouse-move"));
    info.insert(kwd("clicks"), Edn::Number(1.0)); // TODO
    info.insert(kwd("x"), Edn::Number(position.x as f64));
    info.insert(kwd("y"), Edn::Number(position.y as f64));

    if let Some(tracked_state) = touches::read_mouse_tracked_state() {
      let p0 = tracked_state.position;
      info.insert(kwd("action"), tracked_state.action);
      info.insert(kwd("path"), tracked_state.path);
      info.insert(kwd("data"), tracked_state.data);
      info.insert(kwd("dx"), Edn::Number((position.x - p0.x) as f64));
      info.insert(kwd("dy"), Edn::Number((position.y - p0.y) as f64));
    }

    Some(Edn::Map(info))
  }
}

pub fn handle_keyboard(keycode: winit::event::VirtualKeyCode, key_state: winit::event::ElementState) -> Vec<Edn> {
  let targets = key_listener::find_key_listeners(&name_key(keycode));
  if targets.is_empty() {
    let mut info: HashMap<Edn, Edn> = HashMap::new();
    info.insert(
      kwd("type"),
      match key_state {
        winit::event::ElementState::Pressed => kwd("key-down"),
        winit::event::ElementState::Released => kwd("key-up"),
      },
    );
    info.insert(kwd("key-code"), Edn::Number(keycode as usize as f64));
    info.insert(kwd("name"), Edn::Str(name_key(keycode).into_boxed_str()));
    vec![Edn::Map(info)]
  } else {
    let mut hits: Vec<Edn> = vec![];
    for target in targets {
      let mut info: HashMap<Edn, Edn> = HashMap::new();
      info.insert(
        kwd("type"),
        match key_state {
          winit::event::ElementState::Pressed => kwd("key-down"),
          winit::event::ElementState::Released => kwd("key-up"),
        },
      );
      info.insert(kwd("key-code"), Edn::Number(keycode as usize as f64));
      info.insert(kwd("name"), Edn::Str(name_key(keycode).into_boxed_str()));
      info.insert(kwd("action"), target.action);
      info.insert(kwd("path"), target.path);
      info.insert(kwd("data"), target.data);
      hits.push(Edn::Map(info));
    }
    hits
  }
}

pub fn name_key(keycode: winit::event::VirtualKeyCode) -> String {
  format!("{:?}", keycode) // TODO
}

pub fn handle_resize(w: f64, h: f64) -> Option<Edn> {
  let mut info: HashMap<Edn, Edn> = HashMap::new();
  info.insert(kwd("type"), kwd("resize"));
  info.insert(kwd("width"), Edn::Number(w));
  info.insert(kwd("height"), Edn::Number(h));

  Some(Edn::Map(info))
}

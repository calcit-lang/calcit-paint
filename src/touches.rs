use std::sync::Mutex;

use calcit_runner::Calcit;
use glam::Vec2;

use crate::primes::TouchAreaShape;

lazy_static! {
  static ref TOUCH_ITEMS_STACK: Mutex<Vec<TouchArea>> = Mutex::new(vec![]);
  static ref MOUSE_DRAG_TRACKED: Mutex<Option<MouseDragState>> = Mutex::new(None);
}

#[derive(Debug, PartialEq, Clone)]
pub struct TouchArea {
  path: Calcit,
  action: Calcit,
  data: Calcit,
  position: Vec2,
  area: TouchAreaShape,
}

#[derive(Debug, PartialEq, Clone)]
struct MouseDragState {
  position: Vec2,
  action: Calcit,
  path: Calcit,
  data: Calcit,
}

pub fn reset_touches_stack() {
  let stack = &mut TOUCH_ITEMS_STACK.lock().unwrap();
  stack.clear()
}

pub fn add_touch_area(position: Vec2, area: TouchAreaShape, action: Calcit, path: Calcit, data: Calcit) {
  let stack = &mut TOUCH_ITEMS_STACK.lock().unwrap();

  let item = TouchArea {
    action: action.to_owned(),
    path: path.to_owned(),
    data: data.to_owned(),
    position: position.to_owned(),
    area: area.to_owned(),
  };
  stack.push(item);
}

pub fn track_mouse_drag(down_position: Vec2, action: Calcit, path: Calcit, data: Calcit) -> Result<(), String> {
  let item = MouseDragState {
    data: data,
    action: action,
    path: path,
    position: down_position,
  };
  let mut state = MOUSE_DRAG_TRACKED.lock().unwrap();
  *state = Some(item);
  Ok(())
}

pub fn release_mouse_drag() {
  let mut state = MOUSE_DRAG_TRACKED.lock().unwrap();
  *state = None;
}

pub fn calc_mouse_move_delta(p: Vec2) -> Result<Vec2, String> {
  let state = MOUSE_DRAG_TRACKED.lock().unwrap();
  if state.is_some() {
    let v = state.to_owned().unwrap();
    Ok(Vec2::new(p.x - v.position.x, p.y - v.position.y))
  } else {
    Err(format!("found no tracked mouse point"))
  }
}

pub fn find_touch_area(p: Vec2) -> Option<TouchArea> {
  let stack = TOUCH_ITEMS_STACK.lock().unwrap();
  let last_idx = stack.len() - 1;
  for idx in 0..last_idx {
    let item = &stack[last_idx - idx];
    match item.area {
      TouchAreaShape::Rect(w, h) => {
        // half of width height
        if (p.x - item.position.x).abs() < w && (p.y - item.position.y).abs() <= h {
          return Some(item.to_owned());
        }
      }
      TouchAreaShape::Circle(r) => {
        if (p.x - item.position.x).powf(2.0) + (p.y - item.position.y).powf(2.0) <= r.powf(2.0) {
          return Some(item.to_owned());
        }
      }
    }
  }
  None
}

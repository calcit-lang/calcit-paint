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
  pub path: Calcit,
  pub action: Calcit,
  pub data: Calcit,
  pub position: Vec2,
  pub area: TouchAreaShape,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MouseDragState {
  pub position: Vec2,
  pub action: Calcit,
  pub path: Calcit,
  pub data: Calcit,
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

pub fn read_mouse_tracked_state() -> Option<MouseDragState> {
  MOUSE_DRAG_TRACKED.lock().unwrap().to_owned()
}

pub fn track_mouse_drag(down_position: Vec2, action: Calcit, path: Calcit, data: Calcit) {
  let item = MouseDragState {
    data,
    action,
    path,
    position: down_position,
  };
  let mut state = MOUSE_DRAG_TRACKED.lock().unwrap();
  *state = Some(item);
}

pub fn release_mouse_drag() {
  let mut state = MOUSE_DRAG_TRACKED.lock().unwrap();
  *state = None;
}

pub fn find_touch_area(p: Vec2) -> Option<TouchArea> {
  let stack = TOUCH_ITEMS_STACK.lock().unwrap();
  let mut reversed = stack.clone();
  reversed.reverse(); // mutable...
                      // println!("Touch Stack: {:?} {:?}", reversed, stack);
  for item in reversed {
    // println!("CHECK touch position: {:?} {}", item, p);
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

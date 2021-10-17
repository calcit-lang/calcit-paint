use std::sync::RwLock;

use calcit_runner::Calcit;
use euclid::{Point2D, Vector2D};
use raqote::Transform;

use crate::primes::TouchAreaShape;

lazy_static! {
  static ref TOUCH_ITEMS_STACK: RwLock<Vec<TouchArea>> = RwLock::new(vec![]);
  static ref MOUSE_DRAG_TRACKED: RwLock<Option<MouseDragState>> = RwLock::new(None);
}

#[derive(Debug, PartialEq, Clone)]
pub struct TouchArea {
  pub path: Calcit,
  pub action: Calcit,
  pub data: Calcit,
  pub position: Vector2D<f32, f32>,
  pub area: TouchAreaShape,
  pub transform: Transform,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MouseDragState {
  pub position: Vector2D<f32, f32>,
  pub action: Calcit,
  pub path: Calcit,
  pub data: Calcit,
}

pub fn reset_touches_stack() {
  let mut stack = TOUCH_ITEMS_STACK.write().unwrap();
  stack.clear()
}

pub fn add_touch_area(
  position: Vector2D<f32, f32>,
  area: TouchAreaShape,
  action: Calcit,
  path: Calcit,
  data: Calcit,
  transform: &Transform,
) {
  let mut stack = TOUCH_ITEMS_STACK.write().unwrap();

  let item = TouchArea {
    action: action.to_owned(),
    path: path.to_owned(),
    data: data.to_owned(),
    position: position.to_owned(),
    area: area.to_owned(),
    transform: transform.to_owned(),
  };
  stack.push(item);
}

pub fn read_mouse_tracked_state() -> Option<MouseDragState> {
  MOUSE_DRAG_TRACKED.read().unwrap().to_owned()
}

pub fn track_mouse_drag(down_position: Vector2D<f32, f32>, action: Calcit, path: Calcit, data: Calcit) {
  let item = MouseDragState {
    data,
    action,
    path,
    position: down_position,
  };
  let mut state = MOUSE_DRAG_TRACKED.write().unwrap();
  *state = Some(item);
}

pub fn release_mouse_drag() {
  let mut state = MOUSE_DRAG_TRACKED.write().unwrap();
  *state = None;
}

pub fn find_touch_area(p0: Vector2D<f32, f32>) -> Option<TouchArea> {
  let stack = TOUCH_ITEMS_STACK.read().unwrap();
  let mut reversed = stack.to_owned();
  reversed.reverse(); // mutable...
                      // println!("Touch Stack: {:?} {:?}", reversed, stack);
  let p1 = Point2D::new(p0.x, p0.y);
  for item in reversed {
    let p = item.transform.inverse().unwrap().transform_point(p1);
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

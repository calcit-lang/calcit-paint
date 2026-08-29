use std::cell::RefCell;

use euclid::{Point2D, Vector2D};
use winit::{event::MouseButton, window::CursorIcon};

use crate::primes::{EventTarget, TouchAreaShape};

pub type Transform = euclid::default::Transform2D<f32>;

thread_local! {
  static TOUCH_ITEMS_STACK: RefCell<Vec<TouchArea>> = const { RefCell::new(vec![]) };
  static POINTER_STATE: RefCell<PointerState> = RefCell::new(PointerState::default());
}

#[derive(Debug, PartialEq, Clone)]
pub struct TouchArea {
  pub id: Box<str>,
  pub target: EventTarget,
  pub position: Vector2D<f32, f32>,
  pub area: TouchAreaShape,
  pub transform: Transform,
  pub cursor: Option<CursorIcon>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PointerCapture {
  pub down_position: Vector2D<f32, f32>,
  pub button: MouseButton,
  pub area: TouchArea,
}

#[derive(Debug, PartialEq, Clone)]
pub struct HoverTransition {
  pub from: Option<TouchArea>,
  pub to: Option<TouchArea>,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct PointerReconcile {
  pub hover: Option<HoverTransition>,
  pub cancelled: Option<PointerCapture>,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct PointerExit {
  pub capture: Option<PointerCapture>,
  pub hovered: Option<TouchArea>,
}

#[derive(Default)]
struct PointerState {
  hovered: Option<TouchArea>,
  captured: Option<PointerCapture>,
}

pub fn reset_touches_stack() {
  TOUCH_ITEMS_STACK.with(|stack| stack.borrow_mut().clear());
}

pub fn reset_pointer_state() {
  POINTER_STATE.with(|state| *state.borrow_mut() = PointerState::default());
}

pub fn add_touch_area(
  id: impl Into<Box<str>>,
  position: Vector2D<f32, f32>,
  area: TouchAreaShape,
  target: EventTarget,
  cursor: Option<CursorIcon>,
  transform: &Transform,
) {
  TOUCH_ITEMS_STACK.with(|stack| {
    stack.borrow_mut().push(TouchArea {
      id: id.into(),
      target,
      position,
      area,
      transform: *transform,
      cursor,
    });
  });
}

fn contains(area: &TouchArea, position: Vector2D<f32, f32>) -> bool {
  let Some(transform) = area.transform.inverse() else {
    return false;
  };
  let point = transform.transform_point(Point2D::new(position.x, position.y));
  match area.area {
    TouchAreaShape::Rect(dx, dy) => (point.x - area.position.x).abs() <= dx && (point.y - area.position.y).abs() <= dy,
    TouchAreaShape::Circle(radius) => {
      (point.x - area.position.x).powi(2) + (point.y - area.position.y).powi(2) <= radius.powi(2)
    }
  }
}

fn find_in(areas: &[TouchArea], position: Vector2D<f32, f32>) -> Option<TouchArea> {
  areas.iter().rev().find(|area| contains(area, position)).cloned()
}

pub fn find_touch_area(position: Vector2D<f32, f32>) -> Option<TouchArea> {
  TOUCH_ITEMS_STACK.with(|stack| find_in(&stack.borrow(), position))
}

fn same_area(left: Option<&TouchArea>, right: Option<&TouchArea>) -> bool {
  match (left, right) {
    (Some(left), Some(right)) => left.id == right.id && left.target == right.target,
    (None, None) => true,
    _ => false,
  }
}

pub fn reconcile_pointer(position: Vector2D<f32, f32>, inside_window: bool) -> PointerReconcile {
  let areas = TOUCH_ITEMS_STACK.with(|stack| stack.borrow().clone());
  let hit = inside_window.then(|| find_in(&areas, position)).flatten();
  POINTER_STATE.with(|state| {
    let mut state = state.borrow_mut();
    let mut cancelled = None;

    if let Some(capture) = state.captured.as_mut() {
      if let Some(current) = areas
        .iter()
        .find(|area| area.id == capture.area.id && area.target == capture.area.target)
      {
        capture.area = current.clone();
      } else {
        cancelled = state.captured.take();
      }
    }

    let next = if inside_window {
      state.captured.as_ref().map(|capture| capture.area.clone()).or(hit)
    } else {
      None
    };
    let hover = if same_area(state.hovered.as_ref(), next.as_ref()) {
      None
    } else {
      Some(HoverTransition {
        from: state.hovered.take(),
        to: next.clone(),
      })
    };
    state.hovered = next;

    PointerReconcile { hover, cancelled }
  })
}

pub fn begin_pointer_capture(position: Vector2D<f32, f32>, button: MouseButton) -> Option<PointerCapture> {
  let area = find_touch_area(position)?;
  let capture = PointerCapture {
    down_position: position,
    button,
    area,
  };
  POINTER_STATE.with(|state| state.borrow_mut().captured = Some(capture.clone()));
  Some(capture)
}

pub fn read_pointer_capture() -> Option<PointerCapture> {
  POINTER_STATE.with(|state| state.borrow().captured.clone())
}

pub fn release_pointer_capture(button: MouseButton) -> Option<PointerCapture> {
  POINTER_STATE.with(|state| {
    let mut state = state.borrow_mut();
    if state.captured.as_ref().is_some_and(|capture| capture.button == button) {
      state.captured.take()
    } else {
      None
    }
  })
}

pub fn cancel_pointer_capture() -> Option<PointerCapture> {
  POINTER_STATE.with(|state| state.borrow_mut().captured.take())
}

pub fn leave_window() -> PointerExit {
  POINTER_STATE.with(|state| {
    let mut state = state.borrow_mut();
    PointerExit {
      capture: state.captured.take(),
      hovered: state.hovered.take(),
    }
  })
}

pub fn pointer_cursor() -> CursorIcon {
  POINTER_STATE.with(|state| {
    let state = state.borrow();
    state
      .captured
      .as_ref()
      .and_then(|capture| capture.area.cursor)
      .or_else(|| state.hovered.as_ref().and_then(|area| area.cursor))
      .unwrap_or_default()
  })
}

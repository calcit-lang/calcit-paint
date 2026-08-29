use std::sync::RwLock;

use euclid::{Point2D, Vector2D};

use crate::{
  hit_test::{clips_contain, ClipRegion},
  primes::{EventTarget, TouchAreaShape},
};

pub type Transform = euclid::default::Transform2D<f32>;

lazy_static! {
  static ref FOCUS_STATE: RwLock<FocusState> = RwLock::new(FocusState::default());
}

#[cfg(test)]
pub static FOCUS_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[derive(Debug, PartialEq, Clone)]
pub struct FocusArea {
  pub id: String,
  pub target: EventTarget,
  pub position: Vector2D<f32, f32>,
  pub area: TouchAreaShape,
  pub transform: Transform,
  pub clips: Vec<ClipRegion>,
  pub tab_index: i32,
  pub text_input: bool,
  pub(crate) order: usize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FocusReason {
  Pointer,
  Tab,
  Programmatic,
  Escape,
  WindowBlur,
  Removed,
}

impl FocusReason {
  pub fn as_str(self) -> &'static str {
    match self {
      Self::Pointer => "pointer",
      Self::Tab => "tab",
      Self::Programmatic => "programmatic",
      Self::Escape => "escape",
      Self::WindowBlur => "window-blur",
      Self::Removed => "removed",
    }
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FocusTransition {
  pub from: Option<FocusArea>,
  pub to: Option<FocusArea>,
  pub reason: FocusReason,
}

#[derive(Debug, Default)]
struct FocusState {
  areas: Vec<FocusArea>,
  focused: Option<FocusArea>,
  composing: bool,
}

pub fn begin_frame() {
  FOCUS_STATE.write().unwrap().areas.clear();
}

pub fn register_focus_area(mut area: FocusArea) -> Result<(), String> {
  let mut state = FOCUS_STATE.write().unwrap();
  if state.areas.iter().any(|existing| existing.id == area.id) {
    return Err(format!("duplicate focus-id in rendered scene: {}", area.id));
  }
  area.order = state.areas.len();
  if state.focused.as_ref().is_some_and(|focused| focused.id == area.id) {
    state.focused = Some(area.clone());
  }
  state.areas.push(area);
  Ok(())
}

pub fn finish_frame() -> Option<FocusTransition> {
  let mut state = FOCUS_STATE.write().unwrap();
  let removed = state
    .focused
    .as_ref()
    .is_some_and(|focused| !state.areas.iter().any(|area| area.id == focused.id));
  if removed {
    let from = state.focused.take();
    Some(FocusTransition {
      from,
      to: None,
      reason: FocusReason::Removed,
    })
  } else {
    None
  }
}

fn transition_to(state: &mut FocusState, to: Option<FocusArea>, reason: FocusReason) -> Option<FocusTransition> {
  if state.focused.as_ref().map(|area| &area.id) == to.as_ref().map(|area| &area.id) {
    if to.is_some() {
      state.focused = to;
    }
    return None;
  }
  let from = state.focused.take();
  state.focused = to.clone();
  Some(FocusTransition { from, to, reason })
}

pub fn request_focus(id: &str, reason: FocusReason) -> Result<Option<FocusTransition>, String> {
  let mut state = FOCUS_STATE.write().unwrap();
  let area = state
    .areas
    .iter()
    .find(|area| area.id == id)
    .cloned()
    .ok_or_else(|| format!("cannot focus unknown focus-id: {id}"))?;
  Ok(transition_to(&mut state, Some(area), reason))
}

pub fn clear_focus(reason: FocusReason) -> Option<FocusTransition> {
  let mut state = FOCUS_STATE.write().unwrap();
  transition_to(&mut state, None, reason)
}

pub fn focus_at(position: Vector2D<f32, f32>) -> Option<FocusTransition> {
  let target = {
    let state = FOCUS_STATE.read().unwrap();
    let point = Point2D::new(position.x, position.y);
    state.areas.iter().rev().find_map(|item| {
      if !clips_contain(&item.clips, position) {
        return None;
      }
      let local = item.transform.inverse()?.transform_point(point);
      let hit = match item.area {
        TouchAreaShape::Rect(dx, dy) => {
          (local.x - item.position.x).abs() < dx && (local.y - item.position.y).abs() <= dy
        }
        TouchAreaShape::Circle(radius) => {
          (local.x - item.position.x).powi(2) + (local.y - item.position.y).powi(2) <= radius.powi(2)
        }
      };
      hit.then(|| item.clone())
    })
  };
  let mut state = FOCUS_STATE.write().unwrap();
  transition_to(&mut state, target, FocusReason::Pointer)
}

pub fn advance(reverse: bool) -> Option<FocusTransition> {
  let mut state = FOCUS_STATE.write().unwrap();
  let mut areas: Vec<FocusArea> = state.areas.iter().filter(|area| area.tab_index >= 0).cloned().collect();
  areas.sort_by_key(|area| (area.tab_index, area.order));
  if areas.is_empty() {
    return transition_to(&mut state, None, FocusReason::Tab);
  }
  let current = state
    .focused
    .as_ref()
    .and_then(|focused| areas.iter().position(|area| area.id == focused.id));
  let next_index = match (current, reverse) {
    (Some(0), true) | (None, true) => areas.len() - 1,
    (Some(index), true) => index - 1,
    (Some(index), false) => (index + 1) % areas.len(),
    (None, false) => 0,
  };
  transition_to(&mut state, Some(areas[next_index].clone()), FocusReason::Tab)
}

pub fn current() -> Option<FocusArea> {
  FOCUS_STATE.read().unwrap().focused.clone()
}

pub fn focused(id: &str) -> bool {
  current().is_some_and(|area| area.id == id)
}

pub fn has_focus() -> bool {
  FOCUS_STATE.read().unwrap().focused.is_some()
}

pub fn text_input_enabled() -> bool {
  current().is_some_and(|area| area.text_input)
}

pub fn begin_composition() -> bool {
  let mut state = FOCUS_STATE.write().unwrap();
  let started = !state.composing;
  state.composing = true;
  started
}

pub fn end_composition() -> bool {
  let mut state = FOCUS_STATE.write().unwrap();
  let ended = state.composing;
  state.composing = false;
  ended
}

pub fn is_composing() -> bool {
  FOCUS_STATE.read().unwrap().composing
}

#[cfg(test)]
pub fn reset_for_test() {
  *FOCUS_STATE.write().unwrap() = FocusState::default();
}

#[cfg(test)]
mod tests {
  use super::*;

  fn area(id: &str, tab_index: i32) -> FocusArea {
    FocusArea {
      id: id.into(),
      target: EventTarget::default(),
      position: Vector2D::new(0.0, 0.0),
      area: TouchAreaShape::Rect(20.0, 10.0),
      transform: Transform::identity(),
      clips: vec![],
      tab_index,
      text_input: false,
      order: 0,
    }
  }

  #[test]
  fn traverses_and_wraps_in_tab_order() {
    let _guard = FOCUS_TEST_LOCK.lock().unwrap();
    reset_for_test();
    begin_frame();
    register_focus_area(area("second", 2)).unwrap();
    register_focus_area(area("first", 1)).unwrap();
    assert_eq!(advance(false).unwrap().to.unwrap().id, "first");
    assert_eq!(advance(false).unwrap().to.unwrap().id, "second");
    assert_eq!(advance(false).unwrap().to.unwrap().id, "first");
    assert_eq!(advance(true).unwrap().to.unwrap().id, "second");
  }

  #[test]
  fn clears_focus_when_the_node_is_removed() {
    let _guard = FOCUS_TEST_LOCK.lock().unwrap();
    reset_for_test();
    begin_frame();
    register_focus_area(area("field", 0)).unwrap();
    request_focus("field", FocusReason::Programmatic).unwrap();
    begin_frame();
    let transition = finish_frame().unwrap();
    assert_eq!(transition.reason, FocusReason::Removed);
    assert_eq!(transition.from.unwrap().id, "field");
    assert!(!has_focus());
  }

  #[test]
  fn rejects_duplicate_focus_ids_in_one_scene() {
    let _guard = FOCUS_TEST_LOCK.lock().unwrap();
    reset_for_test();
    begin_frame();
    register_focus_area(area("field", 0)).unwrap();
    assert!(register_focus_area(area("field", 1))
      .unwrap_err()
      .contains("duplicate focus-id"));
  }

  #[test]
  fn pointer_focus_respects_clip_regions() {
    let _guard = FOCUS_TEST_LOCK.lock().unwrap();
    reset_for_test();
    begin_frame();
    let mut clipped = area("field", 0);
    clipped.position = Vector2D::new(30.0, 20.0);
    clipped.clips.push(ClipRegion {
      shape: crate::hit_test::ClipShape::Rect {
        position: Vector2D::new(20.0, 10.0),
        width: 20.0,
        height: 20.0,
      },
      transform: Transform::identity(),
    });
    register_focus_area(clipped).unwrap();

    assert!(focus_at(Vector2D::new(15.0, 20.0)).is_none());
    assert!(!has_focus());
    assert_eq!(focus_at(Vector2D::new(30.0, 20.0)).unwrap().to.unwrap().id, "field");
  }
}

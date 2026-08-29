use std::collections::BTreeMap;
use std::time::Duration;

use cirru_edn::{Edn, EdnMapView};
use euclid::Vector2D;
use winit::{
  event::{ElementState, MouseButton},
  keyboard::{Key, ModifiersState, PhysicalKey},
};

use crate::{extracter::tag, key_listener, primes::EventTarget, touches};

fn map_view(pairs: impl IntoIterator<Item = (Edn, Edn)>) -> EdnMapView {
  let mut map = EdnMapView::default();
  for (key, value) in pairs {
    map.insert(key, value);
  }
  map
}

fn extend_map(map: &mut EdnMapView, pairs: impl IntoIterator<Item = (Edn, Edn)>) {
  for (key, value) in pairs {
    map.insert(key, value);
  }
}

const MULTI_CLICK_INTERVAL: Duration = Duration::from_millis(500);
const MULTI_CLICK_DISTANCE: f32 = 4.0;

#[derive(Debug, Clone)]
struct ClickRecord {
  position: Vector2D<f32, f32>,
  at: Duration,
  count: u32,
}

#[derive(Debug, Clone)]
pub struct InputState {
  position: Vector2D<f32, f32>,
  modifiers: ModifiersState,
  latest_click_count: u32,
  clicks: BTreeMap<MouseButton, ClickRecord>,
}

impl InputState {
  pub fn new(position: Vector2D<f32, f32>, modifiers: ModifiersState) -> Self {
    Self {
      position,
      modifiers,
      latest_click_count: 0,
      clicks: BTreeMap::new(),
    }
  }

  pub fn modifiers(&self) -> ModifiersState {
    self.modifiers
  }

  pub fn set_modifiers(&mut self, modifiers: ModifiersState) {
    self.modifiers = modifiers;
  }

  fn click_count(&mut self, button: MouseButton, at: Duration) -> u32 {
    let count = match self.clicks.get(&button) {
      Some(previous)
        if at.saturating_sub(previous.at) <= MULTI_CLICK_INTERVAL
          && (self.position - previous.position).square_length() <= MULTI_CLICK_DISTANCE.powi(2) =>
      {
        previous.count + 1
      }
      _ => 1,
    };
    self.clicks.insert(
      button,
      ClickRecord {
        position: self.position,
        at,
        count,
      },
    );
    self.latest_click_count = count;
    count
  }

  fn latest_click_count(&self) -> u32 {
    self.latest_click_count
  }

  fn click_count_for(&self, button: MouseButton) -> u32 {
    self.clicks.get(&button).map_or(0, |click| click.count)
  }
}

fn modifiers_edn(modifiers: ModifiersState) -> Edn {
  Edn::Map(map_view([
    (tag("shift?"), Edn::Bool(modifiers.shift_key())),
    (tag("control?"), Edn::Bool(modifiers.control_key())),
    (tag("alt?"), Edn::Bool(modifiers.alt_key())),
    (tag("super?"), Edn::Bool(modifiers.super_key())),
  ]))
}

fn pointer_event(kind: &str, input: &InputState) -> EdnMapView {
  map_view([
    (tag("type"), tag(kind)),
    (tag("clicks"), Edn::Number(input.latest_click_count() as f64)),
    (tag("x"), Edn::Number(input.position.x as f64)),
    (tag("y"), Edn::Number(input.position.y as f64)),
    (tag("modifiers"), modifiers_edn(input.modifiers())),
  ])
}

fn mouse_button_name(button: MouseButton) -> &'static str {
  match button {
    MouseButton::Left => "primary",
    MouseButton::Right => "secondary",
    MouseButton::Middle => "middle",
    MouseButton::Back => "back",
    MouseButton::Forward => "forward",
    MouseButton::Other(_) => "other",
  }
}

fn add_button_fields(info: &mut EdnMapView, button: MouseButton) {
  info.insert(tag("button"), tag(mouse_button_name(button)));
  if let MouseButton::Other(id) = button {
    info.insert(tag("button-id"), Edn::Number(id as f64));
  }
}

fn add_target_fields(info: &mut EdnMapView, target: &EventTarget) {
  extend_map(
    info,
    [
      (tag("action"), target.action.clone().unwrap_or(Edn::Nil)),
      (tag("path"), target.path.clone().unwrap_or(Edn::Nil)),
      (tag("data"), target.data.clone().unwrap_or(Edn::Nil)),
    ],
  );
}

pub fn handle_mouse_down(input: &mut InputState, button: MouseButton, at: Duration) -> Edn {
  let clicks = input.click_count(button, at);
  let position = input.position;
  let mut info = pointer_event("mouse-down", input);
  info.insert(tag("clicks"), Edn::Number(clicks as f64));
  add_button_fields(&mut info, button);

  if let Some(target) = touches::find_touch_area(position) {
    add_target_fields(&mut info, &target.target);
    touches::track_mouse_drag(position, button, target.target);
  }

  Edn::Map(info)
}

pub fn handle_mouse_up(input: &InputState, button: MouseButton) -> Edn {
  let position = input.position;
  let mut info = pointer_event("mouse-up", input);
  info.insert(tag("clicks"), Edn::Number(input.click_count_for(button) as f64));
  add_button_fields(&mut info, button);

  if let Some(tracked_state) = touches::read_mouse_tracked_state().filter(|state| state.button == button) {
    let p0 = tracked_state.position;
    add_target_fields(&mut info, &tracked_state.target);
    extend_map(
      &mut info,
      [
        (tag("dx"), Edn::Number((position.x - p0.x) as f64)),
        (tag("dy"), Edn::Number((position.y - p0.y) as f64)),
      ],
    );

    touches::release_mouse_drag();
  }

  Edn::Map(info)
}

pub fn handle_mouse_move(position: Vector2D<f32, f32>, input: &mut InputState) -> Option<Edn> {
  if position == input.position {
    // triggered a same position, ignored
    None
  } else {
    input.position = position;
    let mut info = pointer_event("mouse-move", input);

    if let Some(tracked_state) = touches::read_mouse_tracked_state() {
      let p0 = tracked_state.position;
      let button = tracked_state.button;
      add_target_fields(&mut info, &tracked_state.target);
      extend_map(
        &mut info,
        [
          (tag("button"), tag(mouse_button_name(button))),
          (tag("dx"), Edn::Number((position.x - p0.x) as f64)),
          (tag("dy"), Edn::Number((position.y - p0.y) as f64)),
        ],
      );
      if let MouseButton::Other(id) = button {
        info.insert(tag("button-id"), Edn::Number(id as f64));
      }
    }

    Some(Edn::Map(info))
  }
}

pub fn handle_mouse_leave(input: &InputState) -> Edn {
  let mut info = pointer_event("mouse-leave", input);
  if let Some(tracked_state) = touches::take_mouse_drag() {
    let p0 = tracked_state.position;
    let button = tracked_state.button;
    add_target_fields(&mut info, &tracked_state.target);
    extend_map(
      &mut info,
      [
        (tag("button"), tag(mouse_button_name(button))),
        (tag("cancelled?"), Edn::Bool(true)),
        (tag("dx"), Edn::Number((input.position.x - p0.x) as f64)),
        (tag("dy"), Edn::Number((input.position.y - p0.y) as f64)),
      ],
    );
    if let MouseButton::Other(id) = button {
      info.insert(tag("button-id"), Edn::Number(id as f64));
    }
  }
  Edn::Map(info)
}

pub fn handle_keyboard(
  key_name: &str,
  key_code: f64,
  physical_key: &PhysicalKey,
  key_state: ElementState,
  modifiers: ModifiersState,
) -> Vec<Edn> {
  let targets = key_listener::find_key_listeners(key_name);
  let base = map_view([
    (
      tag("type"),
      match key_state {
        ElementState::Pressed => tag("key-down"),
        ElementState::Released => tag("key-up"),
      },
    ),
    (tag("key-code"), Edn::Number(key_code)),
    (tag("physical-key"), Edn::str(physical_key_name(physical_key))),
    (tag("name"), Edn::str(key_name)),
    (tag("modifiers"), modifiers_edn(modifiers)),
  ]);
  if targets.is_empty() {
    vec![Edn::Map(base)]
  } else {
    let mut hits: Vec<Edn> = vec![];
    for target in targets {
      let mut info = base.clone();
      add_target_fields(&mut info, &target.target);
      hits.push(Edn::Map(info));
    }
    hits
  }
}

pub fn name_key(key: &Key) -> String {
  match key {
    Key::Character(value) if value.chars().count() == 1 => value.to_uppercase(),
    Key::Character(value) => value.to_string(),
    Key::Named(value) => format!("{value:?}"),
    Key::Dead(value) => format!("Dead({value:?})"),
    Key::Unidentified(_) => "Unidentified".to_owned(),
  }
}

pub fn physical_key_name(key: &PhysicalKey) -> String {
  match key {
    PhysicalKey::Code(code) => format!("{code:?}"),
    PhysicalKey::Unidentified(code) => format!("Unidentified({code:?})"),
  }
}

pub fn handle_resize(w: f64, h: f64) -> Edn {
  let info = map_view([
    (tag("type"), tag("resize")),
    (tag("width"), Edn::Number(w)),
    (tag("height"), Edn::Number(h)),
  ]);

  Edn::Map(info)
}

pub fn handle_mouse_wheel(input: &InputState, dx: f64, dy: f64, unit: &str) -> Edn {
  Edn::Map(map_view([
    (tag("type"), tag("mouse-wheel")),
    (tag("x"), Edn::Number(input.position.x as f64)),
    (tag("y"), Edn::Number(input.position.y as f64)),
    (tag("clicks"), Edn::Number(input.latest_click_count() as f64)),
    (tag("dx"), Edn::Number(dx)),
    (tag("dy"), Edn::Number(dy)),
    (tag("unit"), tag(unit)),
    (tag("modifiers"), modifiers_edn(input.modifiers())),
  ]))
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::sync::Mutex;
  use winit::keyboard::{KeyCode, NamedKey, NativeKeyCode};

  static POINTER_TEST_LOCK: Mutex<()> = Mutex::new(());

  fn input(position: Vector2D<f32, f32>) -> InputState {
    InputState::new(position, ModifiersState::empty())
  }

  fn event_map(event: Edn) -> EdnMapView {
    let Edn::Map(event) = event else {
      panic!("event must be a map")
    };
    event
  }

  #[test]
  fn keeps_legacy_single_letter_key_names() {
    assert_eq!(name_key(&Key::Character("d".into())), "D");
    assert_eq!(name_key(&Key::Named(NamedKey::Enter)), "Enter");
  }

  #[test]
  fn reports_buttons_and_multi_clicks() {
    let _guard = POINTER_TEST_LOCK.lock().unwrap();
    let mut state = input(Vector2D::new(20.0, 30.0));
    let first = event_map(handle_mouse_down(&mut state, MouseButton::Left, Duration::ZERO));
    assert_eq!(first.get(&tag("button")), Some(&tag("primary")));
    assert_eq!(first.get(&tag("clicks")), Some(&Edn::Number(1.0)));

    let second = event_map(handle_mouse_down(
      &mut state,
      MouseButton::Left,
      Duration::from_millis(200),
    ));
    assert_eq!(second.get(&tag("clicks")), Some(&Edn::Number(2.0)));

    let right = event_map(handle_mouse_down(
      &mut state,
      MouseButton::Right,
      Duration::from_millis(300),
    ));
    assert_eq!(right.get(&tag("button")), Some(&tag("secondary")));
    assert_eq!(right.get(&tag("clicks")), Some(&Edn::Number(1.0)));

    let middle = event_map(handle_mouse_down(
      &mut state,
      MouseButton::Middle,
      Duration::from_millis(400),
    ));
    assert_eq!(middle.get(&tag("button")), Some(&tag("middle")));

    let left_up = event_map(handle_mouse_up(&state, MouseButton::Left));
    assert_eq!(left_up.get(&tag("clicks")), Some(&Edn::Number(2.0)));
  }

  #[test]
  fn resets_click_count_after_distance_or_timeout() {
    let _guard = POINTER_TEST_LOCK.lock().unwrap();
    let mut state = input(Vector2D::new(20.0, 30.0));
    handle_mouse_down(&mut state, MouseButton::Middle, Duration::ZERO);
    let after_timeout = event_map(handle_mouse_down(
      &mut state,
      MouseButton::Middle,
      Duration::from_millis(501),
    ));
    assert_eq!(after_timeout.get(&tag("clicks")), Some(&Edn::Number(1.0)));

    handle_mouse_down(&mut state, MouseButton::Middle, Duration::from_millis(600));
    handle_mouse_move(Vector2D::new(30.0, 30.0), &mut state);
    let after_move = event_map(handle_mouse_down(
      &mut state,
      MouseButton::Middle,
      Duration::from_millis(700),
    ));
    assert_eq!(after_move.get(&tag("clicks")), Some(&Edn::Number(1.0)));
  }

  #[test]
  fn attaches_modifiers_to_keyboard_and_pointer_events() {
    let _guard = POINTER_TEST_LOCK.lock().unwrap();
    let modifiers = ModifiersState::SHIFT | ModifiersState::ALT;
    let mut state = InputState::new(Vector2D::new(20.0, 30.0), modifiers);
    let pointer = event_map(handle_mouse_down(&mut state, MouseButton::Left, Duration::ZERO));
    let Some(Edn::Map(pointer_modifiers)) = pointer.get(&tag("modifiers")) else {
      panic!("pointer modifiers must be a map")
    };
    assert_eq!(pointer_modifiers.get(&tag("shift?")), Some(&Edn::Bool(true)));
    assert_eq!(pointer_modifiers.get(&tag("control?")), Some(&Edn::Bool(false)));
    assert_eq!(pointer_modifiers.get(&tag("alt?")), Some(&Edn::Bool(true)));

    let events = handle_keyboard(
      "D",
      KeyCode::KeyD as u32 as f64,
      &PhysicalKey::Code(KeyCode::KeyD),
      ElementState::Pressed,
      modifiers,
    );
    let key = event_map(events.into_iter().next().unwrap());
    assert_eq!(key.get(&tag("physical-key")), Some(&Edn::str("KeyD")));
    assert!(matches!(key.get(&tag("modifiers")), Some(Edn::Map(_))));
  }

  #[test]
  fn cancels_drag_when_pointer_leaves_window() {
    let _guard = POINTER_TEST_LOCK.lock().unwrap();
    touches::release_mouse_drag();
    touches::reset_touches_stack();
    let mut state = input(Vector2D::new(20.0, 30.0));
    touches::add_touch_area(
      Vector2D::new(20.0, 30.0),
      crate::primes::TouchAreaShape::Circle(10.0),
      EventTarget {
        action: Some(tag("drag")),
        path: Some(tag("path")),
        data: Some(tag("data")),
      },
      &crate::touches::Transform::identity(),
    );
    handle_mouse_down(&mut state, MouseButton::Left, Duration::ZERO);
    handle_mouse_move(Vector2D::new(50.0, 40.0), &mut state);
    let leave = event_map(handle_mouse_leave(&state));
    assert_eq!(leave.get(&tag("type")), Some(&tag("mouse-leave")));
    assert_eq!(leave.get(&tag("cancelled?")), Some(&Edn::Bool(true)));
    assert_eq!(leave.get(&tag("dx")), Some(&Edn::Number(30.0)));
    assert!(touches::read_mouse_tracked_state().is_none());
    touches::reset_touches_stack();
  }

  #[test]
  fn preserves_nil_fields_for_optional_event_targets() {
    let _guard = POINTER_TEST_LOCK.lock().unwrap();
    touches::release_mouse_drag();
    touches::reset_touches_stack();
    let mut state = input(Vector2D::new(20.0, 30.0));
    touches::add_touch_area(
      Vector2D::new(20.0, 30.0),
      crate::primes::TouchAreaShape::Circle(10.0),
      EventTarget::default(),
      &crate::touches::Transform::identity(),
    );

    let down = event_map(handle_mouse_down(&mut state, MouseButton::Left, Duration::ZERO));
    assert_eq!(down.get(&tag("action")), Some(&Edn::Nil));
    assert_eq!(down.get(&tag("path")), Some(&Edn::Nil));
    assert_eq!(down.get(&tag("data")), Some(&Edn::Nil));

    touches::release_mouse_drag();
    touches::reset_touches_stack();
  }

  #[test]
  fn resize_event_is_not_optional() {
    let Edn::Map(event) = handle_resize(640.0, 480.0) else {
      panic!("resize must be an event map");
    };
    assert_eq!(event.get(&tag("type")), Some(&tag("resize")));
    assert_eq!(event.get(&tag("width")), Some(&Edn::Number(640.0)));
  }

  #[test]
  fn wheel_event_preserves_units_and_pointer_context() {
    let mut state = input(Vector2D::new(12.0, 24.0));
    state.set_modifiers(ModifiersState::CONTROL);
    let Edn::Map(event) = handle_mouse_wheel(&state, 1.0, -2.0, "line") else {
      panic!("wheel must be an event map");
    };
    assert_eq!(event.get(&tag("unit")), Some(&tag("line")));
    assert_eq!(event.get(&tag("x")), Some(&Edn::Number(12.0)));
    assert!(matches!(event.get(&tag("modifiers")), Some(Edn::Map(_))));
  }

  #[test]
  fn names_unidentified_physical_keys_without_panicking() {
    assert_eq!(physical_key_name(&PhysicalKey::Code(KeyCode::KeyA)), "KeyA");
    assert!(physical_key_name(&PhysicalKey::Unidentified(NativeKeyCode::Unidentified)).starts_with("Unidentified("));
  }
}

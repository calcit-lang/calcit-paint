use std::collections::BTreeMap;
use std::path::Path;
use std::time::Duration;

use cirru_edn::{Edn, EdnMapView};
use euclid::Vector2D;
use winit::{
  event::{ElementState, Ime, MouseButton},
  keyboard::{Key, ModifiersState, PhysicalKey},
  window::Theme,
};

use crate::{
  accessibility::SemanticNode, extracter::tag, file_dialog::FileDialogResult, focus, frame::FrameTiming, key_listener,
  primes::EventTarget, touches,
};

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
  inside_window: bool,
  modifiers: ModifiersState,
  latest_click_count: u32,
  clicks: BTreeMap<MouseButton, ClickRecord>,
}

impl InputState {
  pub fn new(position: Vector2D<f32, f32>, modifiers: ModifiersState) -> Self {
    Self {
      position,
      inside_window: false,
      modifiers,
      latest_click_count: 0,
      clicks: BTreeMap::new(),
    }
  }

  pub fn modifiers(&self) -> ModifiersState {
    self.modifiers
  }

  pub fn position(&self) -> Vector2D<f32, f32> {
    self.position
  }

  fn move_pointer(&mut self, position: Vector2D<f32, f32>) {
    self.position = position;
    self.inside_window = true;
  }

  fn leave_window(&mut self) {
    self.inside_window = false;
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

fn hover_event(kind: &str, input: &InputState, area: &touches::TouchArea) -> Edn {
  let mut info = pointer_event(kind, input);
  add_target_fields(&mut info, &area.target);
  info.insert(tag("cursor"), tag(area.cursor.unwrap_or_default().name()));
  info.insert(tag("captured?"), Edn::Bool(false));
  Edn::Map(info)
}

fn hover_transition_events(transition: Option<touches::HoverTransition>, input: &InputState) -> Vec<Edn> {
  let Some(transition) = transition else {
    return vec![];
  };
  let mut events = vec![];
  if let Some(from) = transition.from {
    events.push(hover_event("pointer-leave", input, &from));
  }
  if let Some(to) = transition.to {
    events.push(hover_event("pointer-enter", input, &to));
  }
  events
}

fn pointer_cancel_event(input: &InputState, capture: &touches::PointerCapture, reason: &str) -> Edn {
  let mut info = pointer_event("pointer-cancel", input);
  add_button_fields(&mut info, capture.button);
  add_target_fields(&mut info, &capture.area.target);
  extend_map(
    &mut info,
    [
      (tag("captured?"), Edn::Bool(true)),
      (tag("cancelled?"), Edn::Bool(true)),
      (tag("reason"), tag(reason)),
      (
        tag("dx"),
        Edn::Number((input.position.x - capture.down_position.x) as f64),
      ),
      (
        tag("dy"),
        Edn::Number((input.position.y - capture.down_position.y) as f64),
      ),
    ],
  );
  Edn::Map(info)
}

fn reconcile_events(input: &InputState, cancellation_reason: &str) -> Vec<Edn> {
  let change = touches::reconcile_pointer(input.position, input.inside_window);
  let mut events = vec![];
  if let Some(capture) = change.cancelled {
    events.push(pointer_cancel_event(input, &capture, cancellation_reason));
  }
  events.extend(hover_transition_events(change.hover, input));
  events
}

fn focus_id_value(area: Option<&focus::FocusArea>) -> Edn {
  area.map_or(Edn::Nil, |area| Edn::str(area.id.as_str()))
}

fn focus_event(kind: &str, area: &focus::FocusArea, related: Option<&focus::FocusArea>, reason: &str) -> Edn {
  let mut info = map_view([
    (tag("type"), tag(kind)),
    (tag("focus-id"), Edn::str(area.id.as_str())),
    (tag("related-focus-id"), focus_id_value(related)),
    (tag("reason"), tag(reason)),
  ]);
  add_target_fields(&mut info, &area.target);
  Edn::Map(info)
}

pub fn handle_accessibility_action(node: &SemanticNode, action: &str) -> Edn {
  let mut info = map_view([
    (tag("type"), tag("accessibility-action")),
    (tag("id"), Edn::str(node.properties.id.as_str())),
    (tag("operation"), tag(action)),
  ]);
  add_target_fields(&mut info, &node.target);
  Edn::Map(info)
}

fn composition_event(
  kind: &str,
  area: &focus::FocusArea,
  text: &str,
  cursor: Option<(usize, usize)>,
  cancelled: Option<bool>,
) -> Edn {
  let mut info = map_view([
    (tag("type"), tag(kind)),
    (tag("focus-id"), Edn::str(area.id.as_str())),
    (tag("text"), Edn::str(text)),
    (
      tag("cursor-start"),
      cursor.map_or(Edn::Nil, |(start, _)| Edn::Number(start as f64)),
    ),
    (
      tag("cursor-end"),
      cursor.map_or(Edn::Nil, |(_, end)| Edn::Number(end as f64)),
    ),
  ]);
  if let Some(cancelled) = cancelled {
    info.insert(tag("cancelled?"), Edn::Bool(cancelled));
  }
  add_target_fields(&mut info, &area.target);
  Edn::Map(info)
}

pub fn handle_focus_transition(transition: focus::FocusTransition) -> Vec<Edn> {
  let mut events = vec![];
  if let Some(from) = &transition.from {
    if focus::end_composition() {
      events.push(composition_event("composition-end", from, "", None, Some(true)));
    }
    events.push(focus_event(
      "focus-out",
      from,
      transition.to.as_ref(),
      transition.reason.as_str(),
    ));
  }
  if let Some(to) = &transition.to {
    events.push(focus_event(
      "focus-in",
      to,
      transition.from.as_ref(),
      transition.reason.as_str(),
    ));
  }
  events
}

pub fn handle_pointer_focus(position: Vector2D<f32, f32>, button: MouseButton) -> Vec<Edn> {
  if button != MouseButton::Left {
    return vec![];
  }
  focus::focus_at(position).map_or_else(Vec::new, handle_focus_transition)
}

pub fn handle_window_focus(focused: bool) -> Vec<Edn> {
  let mut events = vec![Edn::Map(map_view([(
    tag("type"),
    tag(if focused { "window-focus" } else { "window-blur" }),
  )]))];
  if !focused {
    if let Some(transition) = focus::clear_focus(focus::FocusReason::WindowBlur) {
      events.extend(handle_focus_transition(transition));
    }
  }
  events
}

pub fn handle_frame(timing: FrameTiming, width: f64, height: f64, scale_factor: f64) -> Edn {
  Edn::Map(map_view([
    (tag("type"), tag("frame")),
    (tag("frame"), Edn::Number(timing.number as f64)),
    (
      tag("timestamp-ms"),
      Edn::Number(timing.timestamp.as_secs_f64() * 1000.0),
    ),
    (tag("delta-ms"), Edn::Number(timing.delta.as_secs_f64() * 1000.0)),
    (tag("width"), Edn::Number(width)),
    (tag("height"), Edn::Number(height)),
    (tag("scale-factor"), Edn::Number(scale_factor)),
  ]))
}

pub fn handle_ime(ime: Ime) -> Vec<Edn> {
  let Some(area) = focus::current().filter(|area| area.text_input) else {
    return vec![];
  };
  match ime {
    Ime::Enabled => vec![composition_event("ime-enabled", &area, "", None, None)],
    Ime::Preedit(text, _) if text.is_empty() => {
      if focus::end_composition() {
        vec![composition_event("composition-end", &area, "", None, Some(false))]
      } else {
        vec![]
      }
    }
    Ime::Preedit(text, cursor) => {
      let mut events = vec![];
      if focus::begin_composition() {
        events.push(composition_event("composition-start", &area, &text, cursor, None));
      }
      events.push(composition_event("composition-update", &area, &text, cursor, None));
      events
    }
    Ime::Commit(text) => {
      let mut events = vec![];
      if focus::end_composition() {
        events.push(composition_event("composition-end", &area, "", None, Some(false)));
      }
      events.push(composition_event("text-input", &area, &text, None, None));
      events
    }
    Ime::Disabled => {
      let mut events = vec![];
      if focus::end_composition() {
        events.push(composition_event("composition-end", &area, "", None, Some(true)));
      }
      events.push(composition_event("ime-disabled", &area, "", None, None));
      events
    }
  }
}

pub fn handle_mouse_down(input: &mut InputState, button: MouseButton, at: Duration) -> Vec<Edn> {
  input.inside_window = true;
  let mut events = reconcile_events(input, "target-removed");
  let clicks = input.click_count(button, at);
  let position = input.position;
  let mut info = pointer_event("mouse-down", input);
  info.insert(tag("clicks"), Edn::Number(clicks as f64));
  add_button_fields(&mut info, button);

  let capture = touches::read_pointer_capture().or_else(|| touches::begin_pointer_capture(position, button));
  if let Some(capture) = capture {
    add_target_fields(&mut info, &capture.area.target);
    info.insert(tag("captured?"), Edn::Bool(true));
  } else {
    info.insert(tag("captured?"), Edn::Bool(false));
  }

  events.push(Edn::Map(info));
  events
}

pub fn handle_mouse_up(input: &InputState, button: MouseButton) -> Vec<Edn> {
  let position = input.position;
  let mut info = pointer_event("mouse-up", input);
  info.insert(tag("clicks"), Edn::Number(input.click_count_for(button) as f64));
  add_button_fields(&mut info, button);

  let capture = touches::read_pointer_capture().filter(|capture| capture.button == button);
  if let Some(capture) = &capture {
    let p0 = capture.down_position;
    add_target_fields(&mut info, &capture.area.target);
    extend_map(
      &mut info,
      [
        (tag("captured?"), Edn::Bool(true)),
        (tag("dx"), Edn::Number((position.x - p0.x) as f64)),
        (tag("dy"), Edn::Number((position.y - p0.y) as f64)),
      ],
    );
  } else {
    info.insert(tag("captured?"), Edn::Bool(false));
  }

  let mut events = vec![Edn::Map(info)];
  if capture.is_some() {
    touches::release_pointer_capture(button);
    events.extend(reconcile_events(input, "target-removed"));
  }
  events
}

pub fn handle_mouse_move(position: Vector2D<f32, f32>, input: &mut InputState) -> Vec<Edn> {
  let moved = position != input.position;
  input.move_pointer(position);
  let mut events = reconcile_events(input, "target-removed");
  if moved {
    let mut info = pointer_event("mouse-move", input);

    if let Some(capture) = touches::read_pointer_capture() {
      let p0 = capture.down_position;
      let button = capture.button;
      add_target_fields(&mut info, &capture.area.target);
      extend_map(
        &mut info,
        [
          (tag("captured?"), Edn::Bool(true)),
          (tag("button"), tag(mouse_button_name(button))),
          (tag("dx"), Edn::Number((position.x - p0.x) as f64)),
          (tag("dy"), Edn::Number((position.y - p0.y) as f64)),
        ],
      );
      if let MouseButton::Other(id) = button {
        info.insert(tag("button-id"), Edn::Number(id as f64));
      }
    } else {
      info.insert(tag("captured?"), Edn::Bool(false));
    }

    events.push(Edn::Map(info));
  }
  events
}

pub fn handle_mouse_leave(input: &mut InputState) -> Vec<Edn> {
  input.leave_window();
  let exit = touches::leave_window();
  let mut info = pointer_event("mouse-leave", input);
  if let Some(capture) = &exit.capture {
    let p0 = capture.down_position;
    let button = capture.button;
    add_target_fields(&mut info, &capture.area.target);
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

  let mut events = vec![Edn::Map(info)];
  if let Some(capture) = exit.capture {
    events.push(pointer_cancel_event(input, &capture, "window-leave"));
  }
  if let Some(hovered) = exit.hovered {
    events.push(hover_event("pointer-leave", input, &hovered));
  }
  events
}

pub fn handle_pointer_blur(input: &InputState) -> Vec<Edn> {
  let mut events = vec![];
  if let Some(capture) = touches::cancel_pointer_capture() {
    events.push(pointer_cancel_event(input, &capture, "window-blur"));
  }
  events.extend(reconcile_events(input, "target-removed"));
  events
}

pub fn handle_pointer_scene_change(input: &InputState) -> Vec<Edn> {
  reconcile_events(input, "target-removed")
}

pub fn handle_keyboard(
  key_name: &str,
  key_code: f64,
  physical_key: &PhysicalKey,
  key_state: ElementState,
  modifiers: ModifiersState,
) -> Vec<Edn> {
  let focused = focus::current();
  let targets = key_listener::find_key_listeners(key_name, modifiers, focused.as_ref().map(|area| area.id.as_str()));
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
    (tag("focus-id"), focus_id_value(focused.as_ref())),
  ]);
  let mut events = if targets.is_empty() {
    vec![Edn::Map(base)]
  } else {
    let mut hits: Vec<Edn> = vec![];
    for target in targets {
      let mut info = base.clone();
      add_target_fields(&mut info, &target.target);
      if target.modifiers.is_some() {
        info.insert(tag("shortcut?"), Edn::Bool(true));
      }
      hits.push(Edn::Map(info));
    }
    hits
  };

  if key_state == ElementState::Pressed {
    if key_name == "Tab" {
      if let Some(transition) = focus::advance(modifiers.shift_key()) {
        events.extend(handle_focus_transition(transition));
      }
    } else if key_name == "Escape" {
      if let Some(transition) = focus::clear_focus(focus::FocusReason::Escape) {
        events.extend(handle_focus_transition(transition));
      }
    }
  }
  events
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

pub fn handle_resize(w: f64, h: f64, scale_factor: f64) -> Edn {
  let info = map_view([
    (tag("type"), tag("resize")),
    (tag("width"), Edn::Number(w)),
    (tag("height"), Edn::Number(h)),
    (tag("scale-factor"), Edn::Number(scale_factor)),
  ]);

  Edn::Map(info)
}

pub fn handle_scale_factor(w: f64, h: f64, scale_factor: f64) -> Edn {
  Edn::Map(map_view([
    (tag("type"), tag("scale-factor")),
    (tag("width"), Edn::Number(w)),
    (tag("height"), Edn::Number(h)),
    (tag("scale-factor"), Edn::Number(scale_factor)),
  ]))
}

pub fn handle_window_theme(theme: Option<Theme>, initial: bool) -> Edn {
  let theme = match theme {
    Some(Theme::Light) => "light",
    Some(Theme::Dark) => "dark",
    None => "unknown",
  };
  Edn::Map(map_view([
    (tag("type"), tag("window-theme")),
    (tag("theme"), tag(theme)),
    (tag("initial?"), Edn::Bool(initial)),
  ]))
}

pub fn handle_window_title_request(title: &str) -> Edn {
  Edn::Map(map_view([
    (tag("type"), tag("window-request")),
    (tag("operation"), tag("set-title")),
    (tag("status"), tag("applied")),
    (tag("title"), Edn::str(title)),
  ]))
}

pub fn handle_window_size_request(
  requested_width: f64,
  requested_height: f64,
  scale_factor: f64,
  actual_physical: Option<(u32, u32)>,
) -> Edn {
  let (status, actual_width, actual_height, matched) = match actual_physical {
    Some((width, height)) => {
      let width = width as f64 / scale_factor;
      let height = height as f64 / scale_factor;
      let tolerance = 0.5 / scale_factor;
      (
        tag("confirmed"),
        Edn::Number(width),
        Edn::Number(height),
        Edn::Bool((width - requested_width).abs() <= tolerance && (height - requested_height).abs() <= tolerance),
      )
    }
    None => (tag("pending"), Edn::Nil, Edn::Nil, Edn::Nil),
  };
  Edn::Map(map_view([
    (tag("type"), tag("window-request")),
    (tag("operation"), tag("request-size")),
    (tag("status"), status),
    (tag("requested-width"), Edn::Number(requested_width)),
    (tag("requested-height"), Edn::Number(requested_height)),
    (tag("actual-width"), actual_width),
    (tag("actual-height"), actual_height),
    (tag("matched?"), matched),
    (tag("scale-factor"), Edn::Number(scale_factor)),
  ]))
}

pub fn handle_window_close(reason: &str) -> Edn {
  Edn::Map(map_view([
    (tag("type"), tag("window-close")),
    (tag("reason"), tag(reason)),
  ]))
}

fn file_event(kind: &str, path: &Path, input: &InputState) -> Result<Edn, String> {
  let path = path
    .to_str()
    .ok_or_else(|| format!("paint :{kind} event path is not valid UTF-8"))?;
  Ok(Edn::Map(map_view([
    (tag("type"), tag(kind)),
    (tag("path"), Edn::str(path)),
    (tag("x"), Edn::Number(input.position.x as f64)),
    (tag("y"), Edn::Number(input.position.y as f64)),
    (tag("modifiers"), modifiers_edn(input.modifiers())),
  ])))
}

pub fn handle_file_hover(path: &Path, input: &InputState) -> Result<Edn, String> {
  file_event("file-hover", path, input)
}

pub fn handle_file_drop(path: &Path, input: &InputState) -> Result<Edn, String> {
  file_event("file-drop", path, input)
}

pub fn handle_file_hover_cancel(input: &InputState) -> Edn {
  Edn::Map(map_view([
    (tag("type"), tag("file-hover-cancel")),
    (tag("x"), Edn::Number(input.position.x as f64)),
    (tag("y"), Edn::Number(input.position.y as f64)),
    (tag("modifiers"), modifiers_edn(input.modifiers())),
  ]))
}

pub fn handle_file_dialog_result(result: FileDialogResult) -> Edn {
  Edn::Map(map_view([
    (tag("type"), tag("file-dialog-result")),
    (tag("request-id"), Edn::str(result.request_id)),
    (tag("operation"), tag(result.operation)),
    (tag("status"), tag(result.status)),
    (tag("path"), result.path.map_or(Edn::Nil, Edn::str)),
    (tag("error"), result.error.map_or(Edn::Nil, Edn::str)),
  ]))
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
  use winit::{
    keyboard::{KeyCode, NamedKey, NativeKeyCode},
    window::CursorIcon,
  };

  static POINTER_TEST_LOCK: Mutex<()> = Mutex::new(());

  fn input(position: Vector2D<f32, f32>) -> InputState {
    touches::reset_pointer_state();
    touches::reset_touches_stack();
    InputState::new(position, ModifiersState::empty())
  }

  trait IntoTestEvent {
    fn into_test_event(self) -> Edn;
  }

  impl IntoTestEvent for Edn {
    fn into_test_event(self) -> Edn {
      self
    }
  }

  impl IntoTestEvent for Vec<Edn> {
    fn into_test_event(mut self) -> Edn {
      self.pop().expect("expected at least one event")
    }
  }

  fn event_map(event: impl IntoTestEvent) -> EdnMapView {
    let event = event.into_test_event();
    let Edn::Map(event) = event else {
      panic!("event must be a map")
    };
    event
  }

  fn focus_area(id: &str, x: f32, tab_index: i32, text_input: bool) -> focus::FocusArea {
    focus::FocusArea {
      id: id.into(),
      target: EventTarget {
        action: Some(tag("focus-demo")),
        path: Some(Edn::str(id)),
        data: None,
      },
      position: Vector2D::new(x, 20.0),
      area: crate::primes::TouchAreaShape::Rect(20.0, 10.0),
      transform: focus::Transform::identity(),
      clips: vec![],
      tab_index,
      text_input,
      order: 0,
    }
  }

  fn event_types(events: Vec<Edn>) -> Vec<Edn> {
    events
      .into_iter()
      .map(|event| event_map(event).get(&tag("type")).unwrap().clone())
      .collect()
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
    touches::reset_pointer_state();
    touches::reset_touches_stack();
    let mut state = input(Vector2D::new(20.0, 30.0));
    touches::add_touch_area(
      "test-area",
      Vector2D::new(20.0, 30.0),
      crate::primes::TouchAreaShape::Circle(10.0),
      EventTarget {
        action: Some(tag("drag")),
        path: Some(tag("path")),
        data: Some(tag("data")),
      },
      None,
      &crate::touches::Transform::identity(),
      &[],
    );
    handle_mouse_down(&mut state, MouseButton::Left, Duration::ZERO);
    handle_mouse_move(Vector2D::new(50.0, 40.0), &mut state);
    let events = handle_mouse_leave(&mut state);
    assert_eq!(
      event_types(events.clone()),
      vec![tag("mouse-leave"), tag("pointer-cancel"), tag("pointer-leave")]
    );
    let leave = event_map(events[0].clone());
    assert_eq!(leave.get(&tag("type")), Some(&tag("mouse-leave")));
    assert_eq!(
      event_map(events[1].clone()).get(&tag("reason")),
      Some(&tag("window-leave"))
    );
    assert_eq!(leave.get(&tag("cancelled?")), Some(&Edn::Bool(true)));
    assert_eq!(leave.get(&tag("dx")), Some(&Edn::Number(30.0)));
    assert!(touches::read_pointer_capture().is_none());
    touches::reset_touches_stack();
  }

  #[test]
  fn preserves_nil_fields_for_optional_event_targets() {
    let _guard = POINTER_TEST_LOCK.lock().unwrap();
    touches::reset_pointer_state();
    touches::reset_touches_stack();
    let mut state = input(Vector2D::new(20.0, 30.0));
    touches::add_touch_area(
      "test-area",
      Vector2D::new(20.0, 30.0),
      crate::primes::TouchAreaShape::Circle(10.0),
      EventTarget::default(),
      None,
      &crate::touches::Transform::identity(),
      &[],
    );

    let down = event_map(handle_mouse_down(&mut state, MouseButton::Left, Duration::ZERO));
    assert_eq!(down.get(&tag("action")), Some(&Edn::Nil));
    assert_eq!(down.get(&tag("path")), Some(&Edn::Nil));
    assert_eq!(down.get(&tag("data")), Some(&Edn::Nil));

    touches::reset_pointer_state();
    touches::reset_touches_stack();
  }

  fn add_pointer_area(id: &str, action: &str, position: Vector2D<f32, f32>, cursor: CursorIcon) {
    touches::add_touch_area(
      id,
      position,
      crate::primes::TouchAreaShape::Rect(20.0, 15.0),
      EventTarget {
        action: Some(tag(action)),
        path: Some(Edn::str(id)),
        data: None,
      },
      Some(cursor),
      &crate::touches::Transform::identity(),
      &[],
    );
  }

  #[test]
  fn hover_uses_topmost_area_and_reconciles_scene_removal() {
    let _guard = POINTER_TEST_LOCK.lock().unwrap();
    let mut state = input(Vector2D::new(-100.0, -100.0));
    add_pointer_area("base", "base-hover", Vector2D::new(20.0, 20.0), CursorIcon::Pointer);
    add_pointer_area(
      "overlay",
      "overlay-hover",
      Vector2D::new(20.0, 20.0),
      CursorIcon::Crosshair,
    );

    let entered = handle_mouse_move(Vector2D::new(20.0, 20.0), &mut state);
    assert_eq!(
      event_types(entered.clone()),
      vec![tag("pointer-enter"), tag("mouse-move")]
    );
    let enter = event_map(entered.into_iter().next().unwrap());
    assert_eq!(enter.get(&tag("action")), Some(&tag("overlay-hover")));
    assert_eq!(enter.get(&tag("cursor")), Some(&tag("crosshair")));
    assert_eq!(touches::pointer_cursor(), CursorIcon::Crosshair);

    touches::reset_touches_stack();
    add_pointer_area("base", "base-hover", Vector2D::new(20.0, 20.0), CursorIcon::Pointer);
    let changed = handle_pointer_scene_change(&state);
    assert_eq!(
      event_types(changed.clone()),
      vec![tag("pointer-leave"), tag("pointer-enter")]
    );
    assert_eq!(
      event_map(changed.into_iter().last().unwrap()).get(&tag("action")),
      Some(&tag("base-hover"))
    );
    assert_eq!(touches::pointer_cursor(), CursorIcon::Pointer);

    touches::reset_touches_stack();
    assert_eq!(
      event_types(handle_pointer_scene_change(&state)),
      vec![tag("pointer-leave")]
    );
    assert_eq!(touches::pointer_cursor(), CursorIcon::Default);
  }

  #[test]
  fn capture_routes_drag_until_matching_release() {
    let _guard = POINTER_TEST_LOCK.lock().unwrap();
    let mut state = input(Vector2D::new(-100.0, -100.0));
    add_pointer_area("drag", "drag-demo", Vector2D::new(20.0, 20.0), CursorIcon::Grab);
    handle_mouse_move(Vector2D::new(20.0, 20.0), &mut state);

    let down = event_map(handle_mouse_down(&mut state, MouseButton::Left, Duration::ZERO));
    assert_eq!(down.get(&tag("captured?")), Some(&Edn::Bool(true)));
    assert_eq!(touches::pointer_cursor(), CursorIcon::Grab);

    let moved = handle_mouse_move(Vector2D::new(90.0, 70.0), &mut state);
    assert_eq!(event_types(moved.clone()), vec![tag("mouse-move")]);
    let moved = event_map(moved);
    assert_eq!(moved.get(&tag("action")), Some(&tag("drag-demo")));
    assert_eq!(moved.get(&tag("captured?")), Some(&Edn::Bool(true)));
    assert_eq!(moved.get(&tag("dx")), Some(&Edn::Number(70.0)));

    let released = handle_mouse_up(&state, MouseButton::Left);
    assert_eq!(
      event_types(released.clone()),
      vec![tag("mouse-up"), tag("pointer-leave")]
    );
    assert_eq!(
      event_map(released.into_iter().next().unwrap()).get(&tag("captured?")),
      Some(&Edn::Bool(true))
    );
    assert!(touches::read_pointer_capture().is_none());
    assert_eq!(touches::pointer_cursor(), CursorIcon::Default);
  }

  #[test]
  fn capture_cancels_on_removal_and_window_blur() {
    let _guard = POINTER_TEST_LOCK.lock().unwrap();
    let mut state = input(Vector2D::new(-100.0, -100.0));
    add_pointer_area("drag", "drag-demo", Vector2D::new(20.0, 20.0), CursorIcon::Grab);
    handle_mouse_move(Vector2D::new(20.0, 20.0), &mut state);
    handle_mouse_down(&mut state, MouseButton::Left, Duration::ZERO);

    touches::reset_touches_stack();
    let removed = handle_pointer_scene_change(&state);
    assert_eq!(
      event_types(removed.clone()),
      vec![tag("pointer-cancel"), tag("pointer-leave")]
    );
    assert_eq!(
      event_map(removed.into_iter().next().unwrap()).get(&tag("reason")),
      Some(&tag("target-removed"))
    );

    add_pointer_area("drag", "drag-demo", Vector2D::new(20.0, 20.0), CursorIcon::Grab);
    handle_pointer_scene_change(&state);
    handle_mouse_down(&mut state, MouseButton::Left, Duration::ZERO);
    let blurred = handle_pointer_blur(&state);
    assert_eq!(event_types(blurred.clone()), vec![tag("pointer-cancel")]);
    assert_eq!(
      event_map(blurred.into_iter().next().unwrap()).get(&tag("reason")),
      Some(&tag("window-blur"))
    );
    assert!(touches::read_pointer_capture().is_none());
  }

  #[test]
  fn frame_event_exposes_monotonic_timing_and_window_metrics() {
    let event = event_map(handle_frame(
      FrameTiming {
        number: 7,
        timestamp: Duration::from_millis(1250),
        delta: Duration::from_millis(16),
      },
      640.0,
      480.0,
      2.0,
    ));
    assert_eq!(event.get(&tag("type")), Some(&tag("frame")));
    assert_eq!(event.get(&tag("frame")), Some(&Edn::Number(7.0)));
    assert_eq!(event.get(&tag("timestamp-ms")), Some(&Edn::Number(1250.0)));
    assert_eq!(event.get(&tag("delta-ms")), Some(&Edn::Number(16.0)));
    assert_eq!(event.get(&tag("width")), Some(&Edn::Number(640.0)));
    assert_eq!(event.get(&tag("height")), Some(&Edn::Number(480.0)));
    assert_eq!(event.get(&tag("scale-factor")), Some(&Edn::Number(2.0)));
  }

  #[test]
  fn resize_event_is_not_optional() {
    let Edn::Map(event) = handle_resize(640.0, 480.0, 2.0) else {
      panic!("resize must be an event map");
    };
    assert_eq!(event.get(&tag("type")), Some(&tag("resize")));
    assert_eq!(event.get(&tag("width")), Some(&Edn::Number(640.0)));
    assert_eq!(event.get(&tag("scale-factor")), Some(&Edn::Number(2.0)));
  }

  #[test]
  fn window_lifecycle_events_report_scale_acknowledgement_and_close_reason() {
    let Edn::Map(scale) = handle_scale_factor(640.0, 480.0, 2.0) else {
      panic!("scale factor must be an event map");
    };
    assert_eq!(scale.get(&tag("type")), Some(&tag("scale-factor")));
    assert_eq!(scale.get(&tag("scale-factor")), Some(&Edn::Number(2.0)));

    let Edn::Map(confirmed) = handle_window_size_request(640.0, 480.0, 2.0, Some((1280, 960))) else {
      panic!("size acknowledgement must be an event map");
    };
    assert_eq!(confirmed.get(&tag("status")), Some(&tag("confirmed")));
    assert_eq!(confirmed.get(&tag("actual-width")), Some(&Edn::Number(640.0)));
    assert_eq!(confirmed.get(&tag("matched?")), Some(&Edn::Bool(true)));

    let Edn::Map(rejected) = handle_window_size_request(640.0, 480.0, 2.0, Some((1200, 900))) else {
      panic!("size acknowledgement must be an event map");
    };
    assert_eq!(rejected.get(&tag("matched?")), Some(&Edn::Bool(false)));

    let Edn::Map(pending) = handle_window_size_request(640.0, 480.0, 2.0, None) else {
      panic!("size acknowledgement must be an event map");
    };
    assert_eq!(pending.get(&tag("status")), Some(&tag("pending")));
    assert_eq!(pending.get(&tag("actual-width")), Some(&Edn::Nil));

    for reason in ["requested", "system", "escape", "render-error", "smoke", "event-loop"] {
      let Edn::Map(close) = handle_window_close(reason) else {
        panic!("close must be an event map");
      };
      assert_eq!(close.get(&tag("type")), Some(&tag("window-close")));
      assert_eq!(close.get(&tag("reason")), Some(&tag(reason)));
    }
  }

  #[test]
  fn window_theme_events_distinguish_initial_and_runtime_observations() {
    for (theme, initial, expected) in [
      (Some(Theme::Light), true, "light"),
      (Some(Theme::Dark), false, "dark"),
      (None, true, "unknown"),
    ] {
      let Edn::Map(event) = handle_window_theme(theme, initial) else {
        panic!("theme must be an event map");
      };
      assert_eq!(event.get(&tag("type")), Some(&tag("window-theme")));
      assert_eq!(event.get(&tag("theme")), Some(&tag(expected)));
      assert_eq!(event.get(&tag("initial?")), Some(&Edn::Bool(initial)));
    }
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
  fn file_events_preserve_path_pointer_context_and_modifiers() {
    let mut state = input(Vector2D::new(12.0, 24.0));
    state.set_modifiers(ModifiersState::SHIFT | ModifiersState::ALT);

    for event in [
      handle_file_hover(Path::new("/tmp/paint image.png"), &state).unwrap(),
      handle_file_drop(Path::new("/tmp/paint image.png"), &state).unwrap(),
    ] {
      let event = event_map(event);
      assert_eq!(event.get(&tag("path")), Some(&Edn::str("/tmp/paint image.png")));
      assert_eq!(event.get(&tag("x")), Some(&Edn::Number(12.0)));
      assert_eq!(event.get(&tag("y")), Some(&Edn::Number(24.0)));
      let Some(Edn::Map(modifiers)) = event.get(&tag("modifiers")) else {
        panic!("file event modifiers must be a map")
      };
      assert_eq!(modifiers.get(&tag("shift?")), Some(&Edn::Bool(true)));
      assert_eq!(modifiers.get(&tag("alt?")), Some(&Edn::Bool(true)));
    }

    let cancel = event_map(handle_file_hover_cancel(&state));
    assert_eq!(cancel.get(&tag("type")), Some(&tag("file-hover-cancel")));
    assert!(!cancel.contains_key("path"));
    assert_eq!(cancel.get(&tag("x")), Some(&Edn::Number(12.0)));
  }

  #[cfg(unix)]
  #[test]
  fn file_events_reject_non_utf8_paths_without_lossy_conversion() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    let state = input(Vector2D::new(0.0, 0.0));
    let path = Path::new(OsStr::from_bytes(b"/tmp/paint-\xff.png"));
    let error = handle_file_drop(path, &state).unwrap_err();
    assert!(error.contains("path is not valid UTF-8"));
  }

  #[test]
  fn file_dialog_results_keep_success_cancellation_and_failure_distinct() {
    let selected = handle_file_dialog_result(FileDialogResult {
      request_id: "open-image".to_owned(),
      operation: "open",
      status: "selected",
      path: Some("/tmp/image.png".to_owned()),
      error: None,
    });
    let Edn::Map(selected) = selected else {
      panic!("file dialog result must be a map");
    };
    assert_eq!(selected.get(&tag("type")), Some(&tag("file-dialog-result")));
    assert_eq!(selected.get(&tag("path")), Some(&Edn::str("/tmp/image.png")));
    assert_eq!(selected.get(&tag("error")), Some(&Edn::Nil));

    let cancelled = handle_file_dialog_result(FileDialogResult {
      request_id: "save-image".to_owned(),
      operation: "save",
      status: "cancelled",
      path: None,
      error: None,
    });
    let Edn::Map(cancelled) = cancelled else {
      panic!("file dialog cancellation must be a map");
    };
    assert_eq!(cancelled.get(&tag("path")), Some(&Edn::Nil));
    assert_eq!(cancelled.get(&tag("error")), Some(&Edn::Nil));
  }

  #[test]
  fn names_unidentified_physical_keys_without_panicking() {
    assert_eq!(physical_key_name(&PhysicalKey::Code(KeyCode::KeyA)), "KeyA");
    assert!(physical_key_name(&PhysicalKey::Unidentified(NativeKeyCode::Unidentified)).starts_with("Unidentified("));
  }

  #[test]
  fn transfers_focus_with_pointer_and_wrapping_tab_navigation() {
    let _guard = focus::FOCUS_TEST_LOCK.lock().unwrap();
    focus::reset_for_test();
    focus::begin_frame();
    focus::register_focus_area(focus_area("first", 20.0, 0, true)).unwrap();
    focus::register_focus_area(focus_area("second", 80.0, 1, false)).unwrap();

    assert_eq!(
      event_types(handle_pointer_focus(Vector2D::new(20.0, 20.0), MouseButton::Left)),
      vec![tag("focus-in")]
    );
    assert!(focus::focused("first"));

    let tab = handle_keyboard(
      "Tab",
      KeyCode::Tab as u32 as f64,
      &PhysicalKey::Code(KeyCode::Tab),
      ElementState::Pressed,
      ModifiersState::empty(),
    );
    assert_eq!(
      event_types(tab),
      vec![tag("key-down"), tag("focus-out"), tag("focus-in")]
    );
    assert!(focus::focused("second"));

    handle_keyboard(
      "Tab",
      KeyCode::Tab as u32 as f64,
      &PhysicalKey::Code(KeyCode::Tab),
      ElementState::Pressed,
      ModifiersState::SHIFT,
    );
    assert!(focus::focused("first"));
  }

  #[test]
  fn matches_exact_modifier_chords_and_focus_scope() {
    let _guard = focus::FOCUS_TEST_LOCK.lock().unwrap();
    focus::reset_for_test();
    key_listener::reset_listeners_stack();
    focus::begin_frame();
    focus::register_focus_area(focus_area("editor", 20.0, 0, true)).unwrap();
    focus::request_focus("editor", focus::FocusReason::Programmatic).unwrap();
    key_listener::add_key_listener(
      "K".into(),
      Some(crate::primes::ShortcutModifiers {
        control: true,
        ..Default::default()
      }),
      Some("editor".into()),
      EventTarget {
        action: Some(tag("shortcut")),
        ..Default::default()
      },
    );

    let plain = event_map(
      handle_keyboard(
        "K",
        KeyCode::KeyK as u32 as f64,
        &PhysicalKey::Code(KeyCode::KeyK),
        ElementState::Pressed,
        ModifiersState::empty(),
      )
      .remove(0),
    );
    assert_eq!(plain.get(&tag("action")), None);

    let chord = event_map(
      handle_keyboard(
        "K",
        KeyCode::KeyK as u32 as f64,
        &PhysicalKey::Code(KeyCode::KeyK),
        ElementState::Pressed,
        ModifiersState::CONTROL,
      )
      .remove(0),
    );
    assert_eq!(chord.get(&tag("action")), Some(&tag("shortcut")));
    assert_eq!(chord.get(&tag("shortcut?")), Some(&Edn::Bool(true)));
    assert_eq!(chord.get(&tag("focus-id")), Some(&Edn::str("editor")));
  }

  #[test]
  fn emits_ime_composition_and_committed_text_lifecycle() {
    let _guard = focus::FOCUS_TEST_LOCK.lock().unwrap();
    focus::reset_for_test();
    focus::begin_frame();
    focus::register_focus_area(focus_area("editor", 20.0, 0, true)).unwrap();
    focus::request_focus("editor", focus::FocusReason::Programmatic).unwrap();

    assert_eq!(event_types(handle_ime(Ime::Enabled)), vec![tag("ime-enabled")]);
    let preedit = handle_ime(Ime::Preedit("ni".into(), Some((1, 2))));
    assert_eq!(
      event_types(preedit.clone()),
      vec![tag("composition-start"), tag("composition-update")]
    );
    let update = event_map(preedit.into_iter().last().unwrap());
    assert_eq!(update.get(&tag("cursor-start")), Some(&Edn::Number(1.0)));
    assert_eq!(update.get(&tag("cursor-end")), Some(&Edn::Number(2.0)));

    assert_eq!(
      event_types(handle_ime(Ime::Preedit(String::new(), None))),
      vec![tag("composition-end")]
    );
    let committed = event_map(handle_ime(Ime::Commit("你".into())).remove(0));
    assert_eq!(committed.get(&tag("type")), Some(&tag("text-input")));
    assert_eq!(committed.get(&tag("text")), Some(&Edn::str("你")));
  }

  #[test]
  fn cancels_composition_on_escape_and_window_blur() {
    let _guard = focus::FOCUS_TEST_LOCK.lock().unwrap();
    focus::reset_for_test();
    focus::begin_frame();
    focus::register_focus_area(focus_area("editor", 20.0, 0, true)).unwrap();
    focus::request_focus("editor", focus::FocusReason::Programmatic).unwrap();
    handle_ime(Ime::Preedit("draft".into(), None));

    let escape = handle_keyboard(
      "Escape",
      KeyCode::Escape as u32 as f64,
      &PhysicalKey::Code(KeyCode::Escape),
      ElementState::Pressed,
      ModifiersState::empty(),
    );
    assert_eq!(
      event_types(escape),
      vec![tag("key-down"), tag("composition-end"), tag("focus-out")]
    );
    assert!(!focus::has_focus());
    assert!(!focus::is_composing());

    focus::request_focus("editor", focus::FocusReason::Programmatic).unwrap();
    handle_ime(Ime::Preedit("draft".into(), None));
    let blur = handle_window_focus(false);
    assert_eq!(
      event_types(blur),
      vec![tag("window-blur"), tag("composition-end"), tag("focus-out")]
    );
    assert!(!focus::has_focus());
  }
}

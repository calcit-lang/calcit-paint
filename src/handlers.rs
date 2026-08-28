use std::cell::RefCell;

use cirru_edn::{Edn, EdnMapView};
use euclid::Vector2D;
use winit::{event::ElementState, keyboard::Key};

use crate::{extracter::tag, key_listener, touches};

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

// TODO track position

pub fn handle_mouse_down(mouse: &RefCell<Vector2D<f32, f32>>) -> Edn {
  // println!("mouse down: {:?}", mouse.to_owned().into_inner());
  let position = mouse.to_owned().into_inner();

  let mut info = map_view([
    (tag("type"), tag("mouse-down")),
    (tag("clicks"), Edn::Number(1.0)), // TODO
    (tag("x"), Edn::Number(position.x as f64)),
    (tag("y"), Edn::Number(position.y as f64)),
  ]);

  if let Some(target) = touches::find_touch_area(position) {
    extend_map(
      &mut info,
      [
        (tag("action"), target.action.to_owned()),
        (tag("path"), target.path.to_owned()),
        (tag("data"), target.data.to_owned()),
      ],
    );
    touches::track_mouse_drag(position, target.action.to_owned(), target.path.to_owned(), target.data);
  }

  Edn::Map(info)
}

pub fn handle_mouse_up(mouse: &RefCell<Vector2D<f32, f32>>) -> Edn {
  // println!("mouse up: {:?}", mouse.to_owned().into_inner());
  let position = mouse.to_owned().into_inner();

  let mut info = map_view([
    (tag("type"), tag("mouse-up")),
    (tag("x"), Edn::Number(position.x as f64)),
    (tag("y"), Edn::Number(position.y as f64)),
    (tag("clicks"), Edn::Number(1.0)), // TODO
  ]);

  if let Some(tracked_state) = touches::read_mouse_tracked_state() {
    let p0 = tracked_state.position;
    extend_map(
      &mut info,
      [
        (tag("action"), tracked_state.action),
        (tag("path"), tracked_state.path),
        (tag("data"), tracked_state.data),
        (tag("dx"), Edn::Number((position.x - p0.x) as f64)),
        (tag("dy"), Edn::Number((position.y - p0.y) as f64)),
      ],
    );

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
    let mut info = map_view([
      (tag("type"), tag("mouse-move")),
      (tag("clicks"), Edn::Number(1.0)), // TODO
      (tag("x"), Edn::Number(position.x as f64)),
      (tag("y"), Edn::Number(position.y as f64)),
    ]);

    if let Some(tracked_state) = touches::read_mouse_tracked_state() {
      let p0 = tracked_state.position;
      extend_map(
        &mut info,
        [
          (tag("action"), tracked_state.action),
          (tag("path"), tracked_state.path),
          (tag("data"), tracked_state.data),
          (tag("dx"), Edn::Number((position.x - p0.x) as f64)),
          (tag("dy"), Edn::Number((position.y - p0.y) as f64)),
        ],
      );
    }

    Some(Edn::Map(info))
  }
}

pub fn handle_keyboard(key_name: &str, key_code: f64, key_state: ElementState) -> Vec<Edn> {
  let targets = key_listener::find_key_listeners(key_name);
  if targets.is_empty() {
    let info = map_view([
      (
        tag("type"),
        match key_state {
          ElementState::Pressed => tag("key-down"),
          ElementState::Released => tag("key-up"),
        },
      ),
      (tag("key-code"), Edn::Number(key_code)),
      (tag("name"), Edn::str(key_name)),
    ]);
    vec![Edn::Map(info)]
  } else {
    let mut hits: Vec<Edn> = vec![];
    for target in targets {
      let info = map_view([
        (
          tag("type"),
          match key_state {
            ElementState::Pressed => tag("key-down"),
            ElementState::Released => tag("key-up"),
          },
        ),
        (tag("key-code"), Edn::Number(key_code)),
        (tag("name"), Edn::str(key_name)),
        (tag("action"), target.action),
        (tag("path"), target.path),
        (tag("data"), target.data),
      ]);
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

pub fn handle_resize(w: f64, h: f64) -> Edn {
  let info = map_view([
    (tag("type"), tag("resize")),
    (tag("width"), Edn::Number(w)),
    (tag("height"), Edn::Number(h)),
  ]);

  Edn::Map(info)
}

pub fn handle_mouse_wheel(dx: f64, dy: f64, unit: &str) -> Edn {
  Edn::Map(map_view([
    (tag("type"), tag("mouse-wheel")),
    (tag("dx"), Edn::Number(dx)),
    (tag("dy"), Edn::Number(dy)),
    (tag("unit"), tag(unit)),
  ]))
}

#[cfg(test)]
mod tests {
  use super::*;
  use winit::keyboard::NamedKey;

  #[test]
  fn keeps_legacy_single_letter_key_names() {
    assert_eq!(name_key(&Key::Character("d".into())), "D");
    assert_eq!(name_key(&Key::Named(NamedKey::Enter)), "Enter");
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
  fn wheel_event_preserves_units() {
    let Edn::Map(event) = handle_mouse_wheel(1.0, -2.0, "line") else {
      panic!("wheel must be an event map");
    };
    assert_eq!(event.get(&tag("unit")), Some(&tag("line")));
  }
}

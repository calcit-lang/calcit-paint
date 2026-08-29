use std::sync::RwLock;

use winit::keyboard::ModifiersState;

use crate::primes::{EventTarget, ShortcutModifiers};

lazy_static! {
  static ref KEY_LISTENERS: RwLock<Vec<KeyListenerMark>> = RwLock::new(vec![]);
}

#[derive(Debug, PartialEq, Clone)]
pub struct KeyListenerMark {
  pub key: String,
  pub modifiers: Option<ShortcutModifiers>,
  pub focus_id: Option<String>,
  pub target: EventTarget,
}

pub fn reset_listeners_stack() {
  let mut stack = KEY_LISTENERS.write().unwrap();
  stack.clear();
}

pub fn add_key_listener(
  key: String,
  modifiers: Option<ShortcutModifiers>,
  focus_id: Option<String>,
  target: EventTarget,
) {
  let mut stack = KEY_LISTENERS.write().unwrap();
  stack.push(KeyListenerMark {
    key,
    modifiers,
    focus_id,
    target,
  })
}

fn modifiers_match(expected: &ShortcutModifiers, actual: ModifiersState) -> bool {
  expected.shift == actual.shift_key()
    && expected.control == actual.control_key()
    && expected.alt == actual.alt_key()
    && expected.super_key == actual.super_key()
}

pub fn find_key_listeners(k: &str, modifiers: ModifiersState, focused_id: Option<&str>) -> Vec<KeyListenerMark> {
  let stack = KEY_LISTENERS.read().unwrap();
  let mut marks: Vec<KeyListenerMark> = vec![];
  for item in stack.iter() {
    let focus_matches = item.focus_id.as_deref().is_none_or(|id| Some(id) == focused_id);
    let modifiers_match = item
      .modifiers
      .as_ref()
      .is_none_or(|expected| self::modifiers_match(expected, modifiers));
    if item.key.as_str() == k && focus_matches && modifiers_match {
      marks.push(item.to_owned());
    }
  }

  marks
}

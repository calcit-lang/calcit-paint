use std::sync::RwLock;

use crate::primes::EventTarget;

lazy_static! {
  static ref KEY_LISTENERS: RwLock<Vec<KeyListenerMark>> = RwLock::new(vec![]);
}

#[derive(Debug, PartialEq, Clone)]
pub struct KeyListenerMark {
  pub key: String,
  pub target: EventTarget,
}

pub fn reset_listeners_stack() {
  let mut stack = KEY_LISTENERS.write().unwrap();
  stack.clear();
}

pub fn add_key_listener(key: String, target: EventTarget) {
  let mut stack = KEY_LISTENERS.write().unwrap();
  stack.push(KeyListenerMark { key, target })
}

pub fn find_key_listeners(k: &str) -> Vec<KeyListenerMark> {
  let stack = KEY_LISTENERS.read().unwrap();
  let mut marks: Vec<KeyListenerMark> = vec![];
  for item in stack.iter() {
    if item.key.as_str() == k {
      marks.push(item.to_owned());
    }
  }

  marks
}

use std::sync::RwLock;

use calcit_runner::Calcit;

lazy_static! {
  static ref KEY_LISTENERS: RwLock<Vec<KeyListenerMark>> = RwLock::new(vec![]);
}

#[derive(Debug, PartialEq, Clone)]
pub struct KeyListenerMark {
  pub key: String,
  pub path: Calcit,
  pub action: Calcit,
  pub data: Calcit,
}

pub fn reset_listeners_stack() {
  let mut stack = KEY_LISTENERS.write().unwrap();
  stack.clear();
}

pub fn add_key_listener(key: String, action: Calcit, path: Calcit, data: Calcit) {
  let mut stack = KEY_LISTENERS.write().unwrap();
  stack.push(KeyListenerMark {
    key,
    action,
    path,
    data,
  })
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

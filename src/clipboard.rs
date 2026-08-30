use std::sync::Mutex;

use arboard::Clipboard;

lazy_static! {
  static ref TEXT_CLIPBOARD: Mutex<Option<Clipboard>> = Mutex::new(None);
}

fn with_clipboard<T>(
  operation: &str,
  f: impl FnOnce(&mut Clipboard) -> Result<T, arboard::Error>,
) -> Result<T, String> {
  let mut slot = TEXT_CLIPBOARD
    .lock()
    .map_err(|_| "text clipboard lock is poisoned".to_owned())?;
  if slot.is_none() {
    *slot = Some(Clipboard::new().map_err(|error| format!("failed to initialize text clipboard: {error}"))?);
  }
  f(slot.as_mut().expect("clipboard was initialized"))
    .map_err(|error| format!("failed to {operation} text clipboard: {error}"))
}

pub fn read_text() -> Result<String, String> {
  with_clipboard("read", Clipboard::get_text)
}

pub fn write_text(text: &str) -> Result<(), String> {
  with_clipboard("write", |clipboard| clipboard.set_text(text))
}

pub fn release() -> Result<(), String> {
  let clipboard = {
    TEXT_CLIPBOARD
      .lock()
      .map_err(|_| "text clipboard lock is poisoned".to_owned())?
      .take()
  };
  drop(clipboard);
  Ok(())
}

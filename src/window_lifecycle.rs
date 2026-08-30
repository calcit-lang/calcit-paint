use std::collections::{HashSet, VecDeque};
use std::sync::Mutex;

use cirru_edn::{Edn, EdnStructView};

use crate::file_dialog::FileDialogRequest;

const WINDOW_OPTIONS_NAME: &str = "WindowOptions";
const WINDOW_OPTION_FIELDS: [&str; 6] = ["title", "width", "height", "min-width", "min-height", "resizable?"];

#[derive(Clone, Debug, PartialEq)]
pub struct WindowStartupOptions {
  pub title: String,
  pub width: f64,
  pub height: f64,
  pub min_width: Option<f64>,
  pub min_height: Option<f64>,
  pub resizable: bool,
}

impl Default for WindowStartupOptions {
  fn default() -> Self {
    Self {
      title: "Calcit Paint".to_owned(),
      width: 1100.0,
      height: 760.0,
      min_width: None,
      min_height: None,
      resizable: true,
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum WindowRequest {
  SetTitle(String),
  RequestSize { width: f64, height: f64 },
  FileDialog(FileDialogRequest),
  Close,
}

#[derive(Default)]
struct WindowLifecycleState {
  active: bool,
  closing: bool,
  dialog_pending: bool,
  pending: VecDeque<WindowRequest>,
}

lazy_static! {
  static ref WINDOW_LIFECYCLE: Mutex<WindowLifecycleState> = Mutex::new(WindowLifecycleState::default());
}

#[derive(Debug)]
pub struct ActiveWindow;

impl Drop for ActiveWindow {
  fn drop(&mut self) {
    let mut state = WINDOW_LIFECYCLE.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    state.active = false;
    state.closing = false;
    state.dialog_pending = false;
    state.pending.clear();
  }
}

pub fn activate() -> Result<ActiveWindow, String> {
  let mut state = WINDOW_LIFECYCLE
    .lock()
    .map_err(|_| "window lifecycle lock is poisoned".to_owned())?;
  if state.active {
    return Err("a paint window is already active; multiple windows are not supported".to_owned());
  }
  state.active = true;
  state.closing = false;
  state.dialog_pending = false;
  state.pending.clear();
  Ok(ActiveWindow)
}

fn enqueue(request: WindowRequest) -> Result<(), String> {
  let mut state = WINDOW_LIFECYCLE
    .lock()
    .map_err(|_| "window lifecycle lock is poisoned".to_owned())?;
  if !state.active {
    return Err("window request requires an active paint window callback".to_owned());
  }
  if state.closing {
    return Err("paint window is already closing".to_owned());
  }
  state.pending.push_back(request);
  Ok(())
}

pub fn queue_title(title: String) -> Result<(), String> {
  enqueue(WindowRequest::SetTitle(title))
}

pub fn queue_size(width: f64, height: f64) -> Result<(), String> {
  validate_positive("window width", width)?;
  validate_positive("window height", height)?;
  enqueue(WindowRequest::RequestSize { width, height })
}

pub fn queue_close() -> Result<(), String> {
  enqueue(WindowRequest::Close)
}

pub fn queue_file_dialog(request: FileDialogRequest) -> Result<(), String> {
  let mut state = WINDOW_LIFECYCLE
    .lock()
    .map_err(|_| "window lifecycle lock is poisoned".to_owned())?;
  if !state.active {
    return Err("native file dialog requires an active paint window callback".to_owned());
  }
  if state.closing {
    return Err("paint window is already closing".to_owned());
  }
  if state.dialog_pending {
    return Err("a native file dialog request is already pending".to_owned());
  }
  state.dialog_pending = true;
  state.pending.push_back(WindowRequest::FileDialog(request));
  Ok(())
}

pub fn complete_file_dialog() -> Result<(), String> {
  let mut state = WINDOW_LIFECYCLE
    .lock()
    .map_err(|_| "window lifecycle lock is poisoned".to_owned())?;
  if !state.active {
    return Ok(());
  }
  state.dialog_pending = false;
  Ok(())
}

pub fn take_requests() -> Result<VecDeque<WindowRequest>, String> {
  let mut state = WINDOW_LIFECYCLE
    .lock()
    .map_err(|_| "window lifecycle lock is poisoned".to_owned())?;
  if !state.active {
    return Err("window request queue has no active paint window".to_owned());
  }
  Ok(std::mem::take(&mut state.pending))
}

pub fn begin_close() -> Result<bool, String> {
  let mut state = WINDOW_LIFECYCLE
    .lock()
    .map_err(|_| "window lifecycle lock is poisoned".to_owned())?;
  if !state.active {
    return Ok(false);
  }
  if state.closing {
    return Ok(false);
  }
  state.closing = true;
  state.dialog_pending = false;
  state.pending.clear();
  Ok(true)
}

fn validate_positive(name: &str, value: f64) -> Result<f64, String> {
  if value.is_finite() && value > 0.0 {
    Ok(value)
  } else {
    Err(format!("{name} must be a finite positive number, got {value}"))
  }
}

fn has_window_options_name(value: &str) -> bool {
  value == WINDOW_OPTIONS_NAME || value.rsplit('/').next() == Some(WINDOW_OPTIONS_NAME)
}

fn field<'a>(options: &'a EdnStructView, name: &str) -> Result<&'a Edn, String> {
  options
    .pairs
    .iter()
    .find_map(|(key, value)| (key.ref_str() == name).then_some(value))
    .ok_or_else(|| format!("WindowOptions is missing :{name}"))
}

fn number_field(options: &EdnStructView, name: &str) -> Result<f64, String> {
  let Edn::Number(value) = field(options, name)? else {
    return Err(format!("WindowOptions :{name} must be a number"));
  };
  validate_positive(&format!("WindowOptions :{name}"), *value)
}

pub fn parse_startup_options(args: &[Edn]) -> Result<WindowStartupOptions, String> {
  let [Edn::Struct(options)] = args else {
    return Err(format!(
      "launch-canvas-with-options expected one WindowOptions struct, got: {args:?}"
    ));
  };
  if !has_window_options_name(&options.name) {
    return Err(format!(
      "launch-canvas-with-options expected WindowOptions, got struct {}",
      options.name
    ));
  }

  let mut seen = HashSet::new();
  for (key, _) in &options.pairs {
    let name = key.ref_str();
    if !WINDOW_OPTION_FIELDS.contains(&name) {
      return Err(format!("WindowOptions contains unsupported field :{name}"));
    }
    if !seen.insert(name) {
      return Err(format!("WindowOptions contains duplicate field :{name}"));
    }
  }
  if seen.len() != WINDOW_OPTION_FIELDS.len() {
    for name in WINDOW_OPTION_FIELDS {
      if !seen.contains(name) {
        return Err(format!("WindowOptions is missing :{name}"));
      }
    }
  }

  let Edn::Str(title) = field(options, "title")? else {
    return Err("WindowOptions :title must be a string".to_owned());
  };
  let width = number_field(options, "width")?;
  let height = number_field(options, "height")?;
  let min_width = number_field(options, "min-width")?;
  let min_height = number_field(options, "min-height")?;
  let Edn::Bool(resizable) = field(options, "resizable?")? else {
    return Err("WindowOptions :resizable? must be a bool".to_owned());
  };
  if min_width > width || min_height > height {
    return Err(format!(
      "WindowOptions minimum size {min_width}x{min_height} exceeds initial size {width}x{height}"
    ));
  }

  Ok(WindowStartupOptions {
    title: title.to_string(),
    width,
    height,
    min_width: Some(min_width),
    min_height: Some(min_height),
    resizable: *resizable,
  })
}

#[cfg(test)]
mod tests {
  use std::sync::Mutex;

  use cirru_edn::EdnStructView;

  use super::*;

  static TEST_LOCK: Mutex<()> = Mutex::new(());

  fn options() -> Edn {
    let mut options = EdnStructView::new("calcit-paint.core/WindowOptions");
    options.insert("title", Edn::str("Lifecycle demo"));
    options.insert("width", Edn::Number(960.0));
    options.insert("height", Edn::Number(640.0));
    options.insert("min-width", Edn::Number(480.0));
    options.insert("min-height", Edn::Number(320.0));
    options.insert("resizable?", Edn::Bool(false));
    Edn::Struct(options)
  }

  #[test]
  fn parses_nominal_startup_options_and_rejects_invalid_bounds() {
    assert_eq!(
      parse_startup_options(&[options()]).unwrap(),
      WindowStartupOptions {
        title: "Lifecycle demo".to_owned(),
        width: 960.0,
        height: 640.0,
        min_width: Some(480.0),
        min_height: Some(320.0),
        resizable: false,
      }
    );
    assert!(parse_startup_options(&[Edn::Map(Default::default())])
      .unwrap_err()
      .contains("WindowOptions struct"));

    let Edn::Struct(mut missing) = options() else {
      unreachable!();
    };
    missing.pairs.retain(|(key, _)| key.ref_str() != "title");
    assert!(parse_startup_options(&[Edn::Struct(missing)])
      .unwrap_err()
      .contains("missing :title"));

    let Edn::Struct(mut unknown) = options() else {
      unreachable!();
    };
    unknown.insert("decorations?", Edn::Bool(true));
    assert!(parse_startup_options(&[Edn::Struct(unknown)])
      .unwrap_err()
      .contains("unsupported field :decorations?"));

    let Edn::Struct(mut wrong_type) = options() else {
      unreachable!();
    };
    wrong_type
      .pairs
      .iter_mut()
      .find(|(key, _)| key.ref_str() == "resizable?")
      .unwrap()
      .1 = Edn::Nil;
    assert!(parse_startup_options(&[Edn::Struct(wrong_type)])
      .unwrap_err()
      .contains(":resizable? must be a bool"));

    let Edn::Struct(mut invalid) = options() else {
      unreachable!();
    };
    invalid
      .pairs
      .iter_mut()
      .find(|(key, _)| key.ref_str() == "min-width")
      .unwrap()
      .1 = Edn::Number(1200.0);
    assert!(parse_startup_options(&[Edn::Struct(invalid)])
      .unwrap_err()
      .contains("minimum size 1200x320 exceeds initial size 960x640"));
  }

  #[test]
  fn serializes_requests_and_rejects_duplicate_or_closing_windows() {
    let _lock = TEST_LOCK.lock().unwrap();
    assert!(queue_title("before".to_owned())
      .unwrap_err()
      .contains("active paint window"));

    let active = activate().unwrap();
    assert!(activate().unwrap_err().contains("already active"));
    queue_title("Updated".to_owned()).unwrap();
    queue_size(800.0, 600.0).unwrap();
    queue_close().unwrap();
    assert_eq!(
      take_requests().unwrap(),
      VecDeque::from([
        WindowRequest::SetTitle("Updated".to_owned()),
        WindowRequest::RequestSize {
          width: 800.0,
          height: 600.0,
        },
        WindowRequest::Close,
      ])
    );
    assert!(begin_close().unwrap());
    assert!(!begin_close().unwrap());
    assert!(queue_size(1.0, 1.0).unwrap_err().contains("already closing"));
    drop(active);

    assert!(take_requests().is_err());
    assert!(activate().is_ok());
  }

  #[test]
  fn rejects_non_finite_runtime_sizes_before_queueing() {
    assert!(queue_size(0.0, 1.0).unwrap_err().contains("finite positive"));
    assert!(queue_size(f64::NAN, 1.0).unwrap_err().contains("finite positive"));
  }
}

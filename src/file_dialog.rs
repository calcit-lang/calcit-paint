use std::{collections::HashSet, path::PathBuf, thread};

use cirru_edn::{Edn, EdnEnumView, EdnListView, EdnStructView};
use rfd::FileDialog;
use winit::event_loop::EventLoopProxy;

use crate::PaintUserEvent;

const OPTIONS_NAME: &str = "PaintFileDialogOptions";
const FILTER_NAME: &str = "PaintFileDialogFilter";
const FS_PATH_NAME: &str = "FsPath";
const OPTION_FIELDS: [&str; 5] = ["request-id", "title", "directory", "file-name", "filters"];
const FILTER_FIELDS: [&str; 2] = ["name", "extensions"];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileDialogFilter {
  pub name: String,
  pub extensions: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileDialogOptions {
  pub request_id: String,
  pub title: Option<String>,
  pub directory: Option<PathBuf>,
  pub file_name: Option<String>,
  pub filters: Vec<FileDialogFilter>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FileDialogRequest {
  Open(FileDialogOptions),
  Save(FileDialogOptions),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileDialogResult {
  pub request_id: String,
  pub operation: &'static str,
  pub status: &'static str,
  pub path: Option<String>,
  pub error: Option<String>,
}

fn has_name(actual: &str, expected: &str) -> bool {
  actual == expected || actual.rsplit('/').next() == Some(expected)
}

fn field<'a>(value: &'a EdnStructView, name: &str) -> Result<&'a Edn, String> {
  value
    .pairs
    .iter()
    .find_map(|(key, value)| (key.ref_str() == name).then_some(value))
    .ok_or_else(|| format!("{OPTIONS_NAME} is missing :{name}"))
}

fn validate_fields(value: &EdnStructView, expected_name: &str, fields: &[&str]) -> Result<(), String> {
  if !has_name(&value.name, expected_name) {
    return Err(format!("expected {expected_name}, got struct {}", value.name));
  }
  let mut seen = HashSet::new();
  for (key, _) in &value.pairs {
    let name = key.ref_str();
    if !fields.contains(&name) {
      return Err(format!("{expected_name} contains unsupported field :{name}"));
    }
    if !seen.insert(name) {
      return Err(format!("{expected_name} contains duplicate field :{name}"));
    }
  }
  for name in fields {
    if !seen.contains(name) {
      return Err(format!("{expected_name} is missing :{name}"));
    }
  }
  Ok(())
}

fn string(value: &Edn, label: &str) -> Result<String, String> {
  let Edn::Str(value) = value else {
    return Err(format!("{label} must be a string"));
  };
  if value.is_empty() {
    return Err(format!("{label} must not be empty"));
  }
  Ok(value.to_string())
}

fn optional_string(value: &Edn, label: &str) -> Result<Option<String>, String> {
  let Edn::Enum(EdnEnumView { variant, extra, .. }) = value else {
    return Err(format!("{label} must be an Option<String>"));
  };
  if variant.as_ref() != "_" || extra.len() != 1 {
    return Err(format!("{label} must be an Option<String>"));
  }
  match &extra[0] {
    Edn::Nil => Ok(None),
    value => string(value, label).map(Some),
  }
}

fn optional_path(value: &Edn, label: &str) -> Result<Option<PathBuf>, String> {
  let Edn::Enum(EdnEnumView { variant, extra, .. }) = value else {
    return Err(format!("{label} must be an Option<FsPath>"));
  };
  if variant.as_ref() != "_" || extra.len() != 1 {
    return Err(format!("{label} must be an Option<FsPath>"));
  }
  let Edn::Struct(path) = &extra[0] else {
    return if matches!(extra[0], Edn::Nil) {
      Ok(None)
    } else {
      Err(format!("{label} must be an Option<FsPath>"))
    };
  };
  validate_fields(path, FS_PATH_NAME, &["value"])?;
  let value = path
    .pairs
    .iter()
    .find_map(|(key, value)| (key.ref_str() == "value").then_some(value))
    .expect("validated FsPath contains :value");
  string(value, &format!("{label} FsPath :value"))
    .map(PathBuf::from)
    .map(Some)
}

fn parse_filter(value: &Edn) -> Result<FileDialogFilter, String> {
  let Edn::Struct(value) = value else {
    return Err(format!("{OPTIONS_NAME} :filters must contain {FILTER_NAME} structs"));
  };
  validate_fields(value, FILTER_NAME, &FILTER_FIELDS)?;
  let name = string(field(value, "name")?, &format!("{FILTER_NAME} :name"))?;
  let Edn::List(EdnListView(extensions)) = field(value, "extensions")? else {
    return Err(format!("{FILTER_NAME} :extensions must be a list of strings"));
  };
  if extensions.is_empty() {
    return Err(format!("{FILTER_NAME} :extensions must not be empty"));
  }
  let extensions = extensions
    .iter()
    .map(|extension| {
      let extension = string(extension, &format!("{FILTER_NAME} :extensions item"))?;
      if extension.contains('/') || extension.contains('\\') || extension.starts_with('.') {
        return Err(format!(
          "{FILTER_NAME} extension must omit dots and path separators, got {extension:?}"
        ));
      }
      Ok(extension)
    })
    .collect::<Result<Vec<_>, _>>()?;
  Ok(FileDialogFilter { name, extensions })
}

pub fn parse_options(args: &[Edn]) -> Result<FileDialogOptions, String> {
  let [Edn::Struct(value)] = args else {
    return Err(format!(
      "native file dialog expected one {OPTIONS_NAME} struct, got: {args:?}"
    ));
  };
  validate_fields(value, OPTIONS_NAME, &OPTION_FIELDS)?;
  let request_id = string(field(value, "request-id")?, &format!("{OPTIONS_NAME} :request-id"))?;
  let title = optional_string(field(value, "title")?, &format!("{OPTIONS_NAME} :title"))?;
  let directory = optional_path(field(value, "directory")?, &format!("{OPTIONS_NAME} :directory"))?;
  let file_name = optional_string(field(value, "file-name")?, &format!("{OPTIONS_NAME} :file-name"))?;
  let Edn::List(EdnListView(filters)) = field(value, "filters")? else {
    return Err(format!("{OPTIONS_NAME} :filters must be a list"));
  };
  let filters = filters.iter().map(parse_filter).collect::<Result<Vec<_>, _>>()?;
  Ok(FileDialogOptions {
    request_id,
    title,
    directory,
    file_name,
    filters,
  })
}

fn build_dialog(options: &FileDialogOptions) -> FileDialog {
  let mut dialog = FileDialog::new();
  if let Some(title) = &options.title {
    dialog = dialog.set_title(title);
  }
  if let Some(directory) = &options.directory {
    dialog = dialog.set_directory(directory);
  }
  if let Some(file_name) = &options.file_name {
    dialog = dialog.set_file_name(file_name);
  }
  for filter in &options.filters {
    let extensions = filter.extensions.iter().map(String::as_str).collect::<Vec<_>>();
    dialog = dialog.add_filter(&filter.name, &extensions);
  }
  dialog
}

fn selected_result(request: &FileDialogRequest, path: Option<PathBuf>) -> FileDialogResult {
  let (options, operation) = match request {
    FileDialogRequest::Open(options) => (options, "open"),
    FileDialogRequest::Save(options) => (options, "save"),
  };
  match path {
    Some(path) => match path.into_os_string().into_string() {
      Ok(path) => FileDialogResult {
        request_id: options.request_id.clone(),
        operation,
        status: "selected",
        path: Some(path),
        error: None,
      },
      Err(_) => FileDialogResult {
        request_id: options.request_id.clone(),
        operation,
        status: "failed",
        path: None,
        error: Some("native file dialog returned a path that is not valid UTF-8".to_owned()),
      },
    },
    None => FileDialogResult {
      request_id: options.request_id.clone(),
      operation,
      status: "cancelled",
      path: None,
      error: None,
    },
  }
}

pub fn failed_result(request: &FileDialogRequest, error: String) -> FileDialogResult {
  let (options, operation) = match request {
    FileDialogRequest::Open(options) => (options, "open"),
    FileDialogRequest::Save(options) => (options, "save"),
  };
  FileDialogResult {
    request_id: options.request_id.clone(),
    operation,
    status: "failed",
    path: None,
    error: Some(error),
  }
}

fn run(request: FileDialogRequest) -> FileDialogResult {
  let dialog = match &request {
    FileDialogRequest::Open(options) | FileDialogRequest::Save(options) => build_dialog(options),
  };
  let path = match &request {
    FileDialogRequest::Open(_) => dialog.pick_file(),
    FileDialogRequest::Save(_) => dialog.save_file(),
  };
  selected_result(&request, path)
}

pub fn launch(request: FileDialogRequest, proxy: EventLoopProxy<PaintUserEvent>) -> Result<(), String> {
  thread::Builder::new()
    .name("calcit-paint-file-dialog".to_owned())
    .spawn(move || {
      let fallback = request.clone();
      let result = std::panic::catch_unwind(|| run(request))
        .unwrap_or_else(|_| failed_result(&fallback, "native file dialog worker panicked".to_owned()));
      let _ = proxy.send_event(PaintUserEvent::FileDialogResult(result));
    })
    .map(|_| ())
    .map_err(|error| format!("failed starting native file dialog worker: {error}"))
}

#[cfg(test)]
mod tests {
  use super::*;

  fn option(value: Edn) -> Edn {
    Edn::enum_value("_", vec![value])
  }

  fn path(value: &str) -> Edn {
    let mut path = EdnStructView::new("FsPath");
    path.insert("value", Edn::str(value));
    Edn::Struct(path)
  }

  fn filter() -> Edn {
    let mut filter = EdnStructView::new("calcit-paint.core/PaintFileDialogFilter");
    filter.insert("name", Edn::str("Images"));
    filter.insert(
      "extensions",
      Edn::List(EdnListView(vec![Edn::str("png"), Edn::str("jpg")])),
    );
    Edn::Struct(filter)
  }

  fn options() -> Edn {
    let mut options = EdnStructView::new("calcit-paint.core/PaintFileDialogOptions");
    options.insert("request-id", Edn::str("open-image"));
    options.insert("title", option(Edn::str("Open image")));
    options.insert("directory", option(path("/tmp")));
    options.insert("file-name", option(Edn::Nil));
    options.insert("filters", Edn::List(EdnListView(vec![filter()])));
    Edn::Struct(options)
  }

  #[test]
  fn parses_strict_nominal_options() {
    let parsed = parse_options(&[options()]).unwrap();
    assert_eq!(parsed.request_id, "open-image");
    assert_eq!(parsed.title.as_deref(), Some("Open image"));
    assert_eq!(parsed.directory, Some(PathBuf::from("/tmp")));
    assert_eq!(parsed.filters[0].extensions, ["png", "jpg"]);
  }

  #[test]
  fn rejects_invalid_option_shapes_and_filter_extensions() {
    assert!(parse_options(&[]).unwrap_err().contains("expected one"));
    let Edn::Struct(mut missing) = options() else {
      unreachable!();
    };
    missing.pairs.retain(|(key, _)| key.ref_str() != "title");
    assert!(parse_options(&[Edn::Struct(missing)])
      .unwrap_err()
      .contains("missing :title"));

    let Edn::Struct(mut invalid) = options() else {
      unreachable!();
    };
    let (_, filters) = invalid
      .pairs
      .iter_mut()
      .find(|(key, _)| key.ref_str() == "filters")
      .expect("options contain filters");
    let Edn::List(EdnListView(filters)) = filters else {
      unreachable!();
    };
    let Edn::Struct(filter) = &mut filters[0] else {
      unreachable!();
    };
    filter
      .pairs
      .iter_mut()
      .find(|(key, _)| key.ref_str() == "extensions")
      .unwrap()
      .1 = Edn::List(EdnListView(vec![Edn::str(".png")]));
    assert!(parse_options(&[Edn::Struct(invalid)])
      .unwrap_err()
      .contains("must omit dots"));
  }

  #[test]
  fn results_keep_cancellation_and_non_utf8_failures_distinct() {
    let request = FileDialogRequest::Open(parse_options(&[options()]).unwrap());
    let cancelled = selected_result(&request, None);
    assert_eq!(cancelled.status, "cancelled");
    assert_eq!(cancelled.path, None);
    assert_eq!(cancelled.error, None);
  }
}

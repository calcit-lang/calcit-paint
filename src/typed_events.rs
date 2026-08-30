use cirru_edn::{Edn, EdnMapView};

const FFI_EVENT_TYPE: &str = "calcit-paint.core/PaintEventFfi";

const POINTER_EVENTS: [&str; 8] = [
  "mouse-down",
  "mouse-up",
  "mouse-move",
  "mouse-leave",
  "mouse-wheel",
  "pointer-enter",
  "pointer-leave",
  "pointer-cancel",
];
const KEYBOARD_EVENTS: [&str; 2] = ["key-down", "key-up"];
const FOCUS_EVENTS: [&str; 2] = ["focus-in", "focus-out"];
const TEXT_INPUT_EVENTS: [&str; 6] = [
  "ime-enabled",
  "ime-disabled",
  "composition-start",
  "composition-update",
  "composition-end",
  "text-input",
];
const FILE_EVENTS: [&str; 3] = ["file-hover", "file-drop", "file-hover-cancel"];
const FILE_DIALOG_EVENTS: [&str; 1] = ["file-dialog-result"];

fn take_tag(map: &mut EdnMapView, key: &str) -> Result<String, String> {
  match map.0.remove(&Edn::tag(key)) {
    Some(Edn::Tag(value)) => Ok(value.ref_str().to_owned()),
    Some(value) => Err(format!("typed paint event :{key} must be a tag, got: {value}")),
    None => Err(format!("typed paint event is missing :{key}")),
  }
}

fn move_target_fields(map: &mut EdnMapView) {
  let mut target = EdnMapView::default();
  for key in ["action", "path", "data"] {
    if let Some(value) = map.0.remove(&Edn::tag(key)) {
      if !matches!(value, Edn::Nil) {
        target.insert(Edn::tag(key), value);
      }
    }
  }
  map.insert(Edn::tag("target"), Edn::Map(target));
}

fn typed_event(variant: &str, payload: Option<EdnMapView>) -> Edn {
  Edn::typed_enum(
    FFI_EVENT_TYPE,
    variant,
    payload.map_or_else(Vec::new, |payload| vec![Edn::Map(payload)]),
  )
}

pub fn from_legacy(event: Edn) -> Result<Edn, String> {
  let Edn::Map(mut payload) = event else {
    return if matches!(event, Edn::Nil) {
      Ok(typed_event("ready", None))
    } else {
      Err(format!(
        "typed paint callback expected nil or an event map, got: {event}"
      ))
    };
  };

  let kind = take_tag(&mut payload, "type")?;
  let variant = if kind == "window-request" {
    match take_tag(&mut payload, "operation")?.as_str() {
      "set-title" => {
        let status = take_tag(&mut payload, "status")?;
        if status != "applied" {
          return Err(format!("typed title acknowledgement expected :applied, got :{status}"));
        }
        "window-title-applied"
      }
      "request-size" => "window-size-request",
      operation => return Err(format!("unsupported typed window operation :{operation}")),
    }
  } else {
    kind.as_str()
  };

  let has_target = POINTER_EVENTS.contains(&variant)
    || KEYBOARD_EVENTS.contains(&variant)
    || FOCUS_EVENTS.contains(&variant)
    || TEXT_INPUT_EVENTS.contains(&variant);
  if has_target {
    move_target_fields(&mut payload);
  }
  payload.0.retain(|_, value| !matches!(value, Edn::Nil));

  if matches!(variant, "window-focus" | "window-blur") {
    if payload.is_empty() {
      Ok(typed_event(variant, None))
    } else {
      Err(format!(
        "typed :{variant} event does not accept payload fields: {payload:?}"
      ))
    }
  } else if POINTER_EVENTS.contains(&variant)
    || KEYBOARD_EVENTS.contains(&variant)
    || FOCUS_EVENTS.contains(&variant)
    || TEXT_INPUT_EVENTS.contains(&variant)
    || FILE_EVENTS.contains(&variant)
    || FILE_DIALOG_EVENTS.contains(&variant)
    || matches!(
      variant,
      "frame"
        | "resize"
        | "scale-factor"
        | "window-theme"
        | "window-title-applied"
        | "window-size-request"
        | "window-close"
    )
  {
    Ok(typed_event(variant, Some(payload)))
  } else {
    Err(format!("unsupported typed paint event :{variant}"))
  }
}

#[cfg(test)]
mod tests {
  use cirru_edn::EdnEnumView;

  use super::*;

  fn enum_view(event: Edn) -> EdnEnumView {
    let Edn::Enum(event) = event else {
      panic!("typed event must be an enum");
    };
    event
  }

  #[test]
  fn replaces_legacy_nil_with_ready() {
    let event = enum_view(from_legacy(Edn::Nil).unwrap());
    assert_eq!(event.type_name.as_deref(), Some(FFI_EVENT_TYPE));
    assert_eq!(event.variant.as_ref(), "ready");
    assert!(event.extra.is_empty());
  }

  #[test]
  fn normalizes_pointer_target_and_optional_nil_fields() {
    let mut modifiers = EdnMapView::default();
    modifiers.insert(Edn::tag("shift?"), Edn::Bool(false));
    modifiers.insert(Edn::tag("control?"), Edn::Bool(false));
    modifiers.insert(Edn::tag("alt?"), Edn::Bool(false));
    modifiers.insert(Edn::tag("super?"), Edn::Bool(false));
    let mut legacy = EdnMapView::default();
    legacy.insert(Edn::tag("type"), Edn::tag("mouse-down"));
    legacy.insert(Edn::tag("x"), Edn::Number(12.0));
    legacy.insert(Edn::tag("y"), Edn::Number(8.0));
    legacy.insert(Edn::tag("clicks"), Edn::Number(1.0));
    legacy.insert(Edn::tag("modifiers"), Edn::Map(modifiers));
    legacy.insert(Edn::tag("action"), Edn::tag("select"));
    legacy.insert(Edn::tag("path"), Edn::Nil);
    legacy.insert(Edn::tag("data"), Edn::Nil);
    legacy.insert(Edn::tag("button"), Edn::tag("primary"));

    let event = enum_view(from_legacy(Edn::Map(legacy)).unwrap());
    assert_eq!(event.variant.as_ref(), "mouse-down");
    let [Edn::Map(payload)] = event.extra.as_slice() else {
      panic!("pointer event must contain one map payload");
    };
    assert!(!payload.contains_key("type"));
    assert!(!payload.contains_key("action"));
    let Edn::Map(target) = payload.tag_get("target").unwrap() else {
      panic!("pointer target must be nested");
    };
    assert_eq!(target.tag_get("action"), Some(&Edn::tag("select")));
    assert!(!target.contains_key("path"));
    assert!(!target.contains_key("data"));
  }

  #[test]
  fn splits_window_acknowledgements_and_rejects_unknown_events() {
    let mut title = EdnMapView::default();
    title.insert(Edn::tag("type"), Edn::tag("window-request"));
    title.insert(Edn::tag("operation"), Edn::tag("set-title"));
    title.insert(Edn::tag("status"), Edn::tag("applied"));
    title.insert(Edn::tag("title"), Edn::str("Updated"));
    let title = enum_view(from_legacy(Edn::Map(title)).unwrap());
    assert_eq!(title.variant.as_ref(), "window-title-applied");

    let mut size = EdnMapView::default();
    size.insert(Edn::tag("type"), Edn::tag("window-request"));
    size.insert(Edn::tag("operation"), Edn::tag("request-size"));
    size.insert(Edn::tag("status"), Edn::tag("pending"));
    size.insert(Edn::tag("requested-width"), Edn::Number(800.0));
    size.insert(Edn::tag("requested-height"), Edn::Number(600.0));
    size.insert(Edn::tag("actual-width"), Edn::Nil);
    size.insert(Edn::tag("actual-height"), Edn::Nil);
    size.insert(Edn::tag("matched?"), Edn::Nil);
    size.insert(Edn::tag("scale-factor"), Edn::Number(2.0));
    let size = enum_view(from_legacy(Edn::Map(size)).unwrap());
    assert_eq!(size.variant.as_ref(), "window-size-request");
    let [Edn::Map(payload)] = size.extra.as_slice() else {
      panic!("size event must contain one map payload");
    };
    assert!(!payload.contains_key("actual-width"));
    assert!(!payload.contains_key("matched?"));

    let mut unknown = EdnMapView::default();
    unknown.insert(Edn::tag("type"), Edn::tag("future-event"));
    assert!(from_legacy(Edn::Map(unknown))
      .unwrap_err()
      .contains("unsupported typed paint event :future-event"));
  }

  #[test]
  fn preserves_file_event_payloads_without_target_normalization() {
    let mut dropped = EdnMapView::default();
    dropped.insert(Edn::tag("type"), Edn::tag("file-drop"));
    dropped.insert(Edn::tag("path"), Edn::str("/tmp/paint image.png"));
    dropped.insert(Edn::tag("x"), Edn::Number(12.0));
    dropped.insert(Edn::tag("y"), Edn::Number(24.0));
    let dropped = enum_view(from_legacy(Edn::Map(dropped)).unwrap());
    assert_eq!(dropped.variant.as_ref(), "file-drop");
    let [Edn::Map(payload)] = dropped.extra.as_slice() else {
      panic!("file drop event must contain one map payload");
    };
    assert_eq!(payload.tag_get("path"), Some(&Edn::str("/tmp/paint image.png")));

    let mut cancel = EdnMapView::default();
    cancel.insert(Edn::tag("type"), Edn::tag("file-hover-cancel"));
    cancel.insert(Edn::tag("x"), Edn::Number(12.0));
    cancel.insert(Edn::tag("y"), Edn::Number(24.0));
    let cancel = enum_view(from_legacy(Edn::Map(cancel)).unwrap());
    assert_eq!(cancel.variant.as_ref(), "file-hover-cancel");
    let [Edn::Map(payload)] = cancel.extra.as_slice() else {
      panic!("file hover cancellation must contain one map payload");
    };
    assert!(!payload.contains_key("path"));
  }

  #[test]
  fn preserves_window_theme_payload_for_the_strict_calcit_decoder() {
    let mut legacy = EdnMapView::default();
    legacy.insert(Edn::tag("type"), Edn::tag("window-theme"));
    legacy.insert(Edn::tag("theme"), Edn::tag("dark"));
    legacy.insert(Edn::tag("initial?"), Edn::Bool(false));

    let event = enum_view(from_legacy(Edn::Map(legacy)).unwrap());
    assert_eq!(event.variant.as_ref(), "window-theme");
    let [Edn::Map(payload)] = event.extra.as_slice() else {
      panic!("window theme must contain one map payload");
    };
    assert_eq!(payload.tag_get("theme"), Some(&Edn::tag("dark")));
    assert_eq!(payload.tag_get("initial?"), Some(&Edn::Bool(false)));
  }

  #[test]
  fn preserves_file_dialog_results_for_the_strict_calcit_decoder() {
    let mut legacy = EdnMapView::default();
    legacy.insert(Edn::tag("type"), Edn::tag("file-dialog-result"));
    legacy.insert(Edn::tag("request-id"), Edn::str("open-image"));
    legacy.insert(Edn::tag("operation"), Edn::tag("open"));
    legacy.insert(Edn::tag("status"), Edn::tag("selected"));
    legacy.insert(Edn::tag("path"), Edn::str("/tmp/image.png"));
    legacy.insert(Edn::tag("error"), Edn::Nil);

    let event = enum_view(from_legacy(Edn::Map(legacy)).unwrap());
    assert_eq!(event.variant.as_ref(), "file-dialog-result");
    let [Edn::Map(payload)] = event.extra.as_slice() else {
      panic!("file dialog result must contain one map payload");
    };
    assert_eq!(payload.tag_get("request-id"), Some(&Edn::str("open-image")));
    assert!(!payload.contains_key("error"));
  }
}

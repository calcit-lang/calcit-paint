// use raqote::{Color, Cap, LineJoin};

use cirru_edn::{Edn, EdnListView, EdnMapView};
use euclid::{Point2D, Vector2D};

use skia_safe::paint::{Cap, Join};
use skia_safe::{BlendMode, Color};
use winit::window::CursorIcon;

use crate::{
  color::extract_color,
  primes::{
    AccessibilityProperties, AccessibilityRole, DashPattern, EventTarget, GradientStop, PaintSource, ParagraphLayout,
    ShortcutModifiers, StrokeStyle, TextAlign, TextBaseline, TextDirection, TextSlant, TextStyle, TouchAreaShape,
  },
};

pub fn tag(s: &str) -> Edn {
  Edn::tag(s)
}

pub fn extract_event_target(tree: &EdnMapView) -> EventTarget {
  EventTarget {
    action: read_optional_edn(tree, "action"),
    path: read_optional_edn(tree, "path"),
    data: read_optional_edn(tree, "data"),
  }
}

pub fn extract_accessibility(tree: &EdnMapView) -> Result<Option<AccessibilityProperties>, String> {
  let Some(value) = tree.get(&tag("accessibility")) else {
    return Ok(None);
  };
  let Edn::Map(metadata) = value else {
    return Err(format!(":accessibility must be a map, got {value}"));
  };
  for key in metadata.0.keys() {
    let Edn::Tag(key) = key else {
      return Err(":accessibility keys must be tags".to_owned());
    };
    if !matches!(
      key.ref_str(),
      "id" | "role" | "label" | "value" | "enabled?" | "focusable?"
    ) {
      return Err(format!("unsupported :accessibility field :{key}"));
    }
  }
  let id = read_string(metadata, "id")?;
  if id.is_empty() {
    return Err(":accessibility :id must not be empty".to_owned());
  }
  let role = match metadata.get(&tag("role")) {
    Some(Edn::Tag(role)) => match role.ref_str() {
      "button" => AccessibilityRole::Button,
      "text-input" => AccessibilityRole::TextInput,
      "image" => AccessibilityRole::Image,
      role => return Err(format!("unsupported :accessibility :role :{role}")),
    },
    Some(value) => return Err(format!(":accessibility :role must be a tag, got {value}")),
    None => return Err(":accessibility requires :role".to_owned()),
  };
  let label = read_string(metadata, "label")?;
  if label.is_empty() {
    return Err(":accessibility :label must not be empty".to_owned());
  }
  let value = read_optional_string_field(metadata, "value")?;
  let enabled = match metadata.get(&tag("enabled?")) {
    Some(Edn::Bool(value)) => *value,
    Some(value) => return Err(format!(":accessibility :enabled? must be a bool, got {value}")),
    None => true,
  };
  let focusable = match metadata.get(&tag("focusable?")) {
    Some(Edn::Bool(value)) => *value,
    Some(value) => return Err(format!(":accessibility :focusable? must be a bool, got {value}")),
    None => false,
  };
  Ok(Some(AccessibilityProperties {
    id,
    role,
    label,
    value,
    enabled,
    focusable,
  }))
}

fn read_optional_edn(tree: &EdnMapView, key: &str) -> Option<Edn> {
  match tree.get(&tag(key)) {
    None | Some(Edn::Nil) => None,
    Some(value) => Some(value.to_owned()),
  }
}

pub fn read_f32(tree: &EdnMapView, key: &str) -> Result<f32, String> {
  match tree.get(&tag(key)) {
    Some(Edn::Number(n)) => Ok(*n as f32),
    Some(a) => Err(format!("cannot be used as f32: {}", a)),
    None => Err(format!(
      "cannot read f32 {} from empty from: {}",
      key,
      Edn::Map(tree.to_owned())
    )),
  }
}

pub fn read_optional_f32(tree: &EdnMapView, key: &str) -> Result<Option<f32>, String> {
  match tree.get(&tag(key)) {
    Some(Edn::Number(n)) => Ok(Some(*n as f32)),
    Some(a) => Err(format!("cannot be used as f32: {a}")),
    None => Ok(None),
  }
}

pub fn read_bool(tree: &EdnMapView, key: &str) -> Result<bool, String> {
  match tree.get(&tag(key)) {
    Some(Edn::Bool(b)) => Ok(*b),
    Some(a) => Err(format!("cannot be used as bool: {}", a)),
    None => Ok(false),
  }
}

pub fn read_string(tree: &EdnMapView, key: &str) -> Result<String, String> {
  match tree.get(&tag(key)) {
    Some(Edn::Str(s)) => Ok(s.to_string()),
    Some(Edn::Tag(s)) => Ok(s.to_string()),
    Some(a) => Err(format!(
      "cannot be used as string {} in {}",
      a,
      Edn::Map(tree.to_owned())
    )),
    None => Err(format!("cannot read string from empty from: {}", key)),
  }
}

pub fn read_position(tree: &EdnMapView, key: &str) -> Result<Vector2D<f32, f32>, String> {
  match tree.get(&tag(key)) {
    Some(Edn::List(EdnListView(xs))) if xs.len() == 2 => match (&xs[0], &xs[1]) {
      (Edn::Number(x), Edn::Number(y)) => Ok(Vector2D::new(*x as f32, *y as f32)),
      (a, b) => Err(format!("invalid positon values: {} {}", a, b)),
    },
    Some(Edn::List(EdnListView(xs))) => Err(format!("invalid position length: {:?}", xs)),
    Some(Edn::Nil) => Ok(Vector2D::new(0.0, 0.0)),
    Some(a) => Err(format!(
      "cannot be used as position: {} in {}",
      a,
      Edn::Map(tree.to_owned())
    )),
    None => Ok(Vector2D::new(0.0, 0.0)),
  }
}

// get position from a value
pub fn extract_position(x: &Edn) -> Result<Point2D<f32, f32>, String> {
  match x {
    Edn::List(EdnListView(xs)) if xs.len() == 2 => match (&xs[0], &xs[1]) {
      (Edn::Number(x), Edn::Number(y)) => Ok(Point2D::new(*x as f32, *y as f32)),
      (a, b) => Err(format!("invalid positon values: {} {}", a, b)),
    },
    a => Err(format!("cannot be used as position: {} in {}", a, x)),
  }
}

pub fn read_color(tree: &EdnMapView, key: &str) -> Result<Color, String> {
  match tree.get(&tag(key)) {
    Some(a) => extract_color(a),
    None => Err(format!("cannot read color from empty from: {}", key)),
  }
}

pub fn extract_fill_style(tree: &EdnMapView) -> Result<Option<PaintSource>, String> {
  match (tree.get(&tag("fill")), tree.get(&tag("fill-color"))) {
    (Some(_), Some(_)) => Err("shape cannot use both :fill and :fill-color".to_owned()),
    (Some(fill), None) => extract_paint_source(fill).map(Some),
    (None, Some(color)) => extract_color(color).map(PaintSource::Solid).map(Some),
    (None, None) => Ok(None),
  }
}

pub fn extract_stroke_style(tree: &EdnMapView) -> Result<Option<StrokeStyle>, String> {
  let stroke = tree.get(&tag("stroke"));
  let line_color = tree.get(&tag("line-color"));
  let line_width = tree.get(&tag("line-width"));
  if stroke.is_some() && (line_color.is_some() || line_width.is_some()) {
    return Err("shape cannot combine :stroke with :line-color or :line-width".to_owned());
  }
  match stroke {
    Some(Edn::Map(stroke)) => extract_stroke_map(stroke).map(Some),
    Some(value) => Err(format!(":stroke must be a map, got {value}")),
    None => match (line_color, line_width) {
      (Some(color), Some(Edn::Number(width))) => Ok(Some(StrokeStyle {
        paint: PaintSource::Solid(extract_color(color)?),
        width: validate_non_negative("line-width", *width as f32)?,
        cap: Cap::Round,
        join: Join::Round,
        miter_limit: 4.0,
        dash: None,
      })),
      (Some(color), None) => Ok(Some(StrokeStyle {
        paint: PaintSource::Solid(extract_color(color)?),
        width: 1.0,
        cap: Cap::Round,
        join: Join::Round,
        miter_limit: 4.0,
        dash: None,
      })),
      (None, None) => Ok(None),
      (Some(_), Some(width)) => Err(format!("line-width must be a number, got {width}")),
      (None, Some(width)) => Err(format!("line-width requires line-color, got {width}")),
    },
  }
}

pub fn extract_polyline_stroke_style(tree: &EdnMapView) -> Result<StrokeStyle, String> {
  if let Some(stroke) = tree.get(&tag("stroke")) {
    for legacy_key in ["color", "width", "join", "cap"] {
      if tree.contains_key(legacy_key) {
        return Err(format!("polyline cannot combine :stroke with legacy :{legacy_key}"));
      }
    }
    return match stroke {
      Edn::Map(stroke) => extract_stroke_map(stroke),
      value => Err(format!(":stroke must be a map, got {value}")),
    };
  }

  Ok(StrokeStyle {
    paint: PaintSource::Solid(read_color(tree, "color")?),
    width: validate_non_negative("width", read_f32(tree, "width")?)?,
    cap: read_line_cap(tree, "cap")?,
    join: read_line_join(tree, "join")?,
    miter_limit: 4.0,
    dash: None,
  })
}

pub fn read_blend_mode(tree: &EdnMapView, key: &str) -> Result<BlendMode, String> {
  let Some(value) = tree.get(&tag(key)) else {
    return Err(format!("cannot read blend mode from empty field: {key}"));
  };
  let Edn::Tag(mode) = value else {
    return Err(format!("blend mode must be a tag, got {value}"));
  };
  match mode.ref_str() {
    "src-over" => Ok(BlendMode::SrcOver),
    "multiply" => Ok(BlendMode::Multiply),
    "screen" => Ok(BlendMode::Screen),
    "overlay" => Ok(BlendMode::Overlay),
    "darken" => Ok(BlendMode::Darken),
    "lighten" => Ok(BlendMode::Lighten),
    "difference" => Ok(BlendMode::Difference),
    "exclusion" => Ok(BlendMode::Exclusion),
    "plus" => Ok(BlendMode::Plus),
    _ => Err(format!("unsupported blend mode: {mode}")),
  }
}

fn extract_stroke_map(tree: &EdnMapView) -> Result<StrokeStyle, String> {
  let paint = tree
    .get(&tag("paint"))
    .ok_or_else(|| ":stroke requires :paint".to_owned())?;
  let width = match tree.get(&tag("width")) {
    Some(Edn::Number(width)) => validate_non_negative("stroke width", *width as f32)?,
    Some(value) => return Err(format!("stroke width must be a number, got {value}")),
    None => 1.0,
  };
  let cap = read_optional_cap(tree, "cap")?.unwrap_or(Cap::Round);
  let join = read_optional_join(tree, "join")?.unwrap_or(Join::Round);
  let miter_limit = match tree.get(&tag("miter-limit")) {
    Some(Edn::Number(limit)) => validate_positive("stroke miter-limit", *limit as f32)?,
    Some(value) => return Err(format!("stroke miter-limit must be a number, got {value}")),
    None => 4.0,
  };
  let dash = extract_dash_pattern(tree)?;
  Ok(StrokeStyle {
    paint: extract_paint_source(paint)?,
    width,
    cap,
    join,
    miter_limit,
    dash,
  })
}

fn extract_paint_source(value: &Edn) -> Result<PaintSource, String> {
  let Edn::Map(tree) = value else {
    return Err(format!("paint must be a map, got {value}"));
  };
  let Some(Edn::Tag(kind)) = tree.get(&tag("type")) else {
    return Err("paint requires a tag in :type".to_owned());
  };
  match kind.ref_str() {
    "solid" => Ok(PaintSource::Solid(read_color(tree, "color")?)),
    "linear-gradient" => {
      let from = read_required_point(tree, "from")?;
      let to = read_required_point(tree, "to")?;
      if from == to {
        return Err("linear-gradient :from and :to must be different".to_owned());
      }
      Ok(PaintSource::LinearGradient {
        from,
        to,
        stops: extract_gradient_stops(tree)?,
      })
    }
    "radial-gradient" => Ok(PaintSource::RadialGradient {
      center: read_required_point(tree, "center")?,
      radius: validate_positive("radial-gradient radius", read_f32(tree, "radius")?)?,
      stops: extract_gradient_stops(tree)?,
    }),
    _ => Err(format!("unsupported paint type: {kind}")),
  }
}

fn extract_gradient_stops(tree: &EdnMapView) -> Result<Vec<GradientStop>, String> {
  let Some(Edn::List(EdnListView(stops))) = tree.get(&tag("stops")) else {
    return Err("gradient requires a :stops list".to_owned());
  };
  if stops.len() < 2 {
    return Err("gradient requires at least two color stops".to_owned());
  }
  let mut parsed = Vec::with_capacity(stops.len());
  let mut previous = None;
  for stop in stops {
    let Edn::List(EdnListView(pair)) = stop else {
      return Err(format!("gradient stop must be [offset color], got {stop}"));
    };
    if pair.len() != 2 {
      return Err(format!("gradient stop must contain offset and color, got {stop}"));
    }
    let Edn::Number(offset) = &pair[0] else {
      return Err(format!("gradient stop offset must be a number, got {}", pair[0]));
    };
    let offset = *offset as f32;
    if !offset.is_finite() || !(0.0..=1.0).contains(&offset) {
      return Err(format!("gradient stop offset must be between 0 and 1, got {offset}"));
    }
    if previous.is_some_and(|previous| offset <= previous) {
      return Err("gradient stops must use strictly increasing offsets".to_owned());
    }
    previous = Some(offset);
    parsed.push(GradientStop {
      offset,
      color: extract_color(&pair[1])?,
    });
  }
  Ok(parsed)
}

fn extract_dash_pattern(tree: &EdnMapView) -> Result<Option<DashPattern>, String> {
  let dash = tree.get(&tag("dash"));
  let offset = tree.get(&tag("dash-offset"));
  let Some(dash) = dash else {
    return match offset {
      Some(_) => Err(":dash-offset requires :dash".to_owned()),
      None => Ok(None),
    };
  };
  let Edn::List(EdnListView(values)) = dash else {
    return Err(format!(":dash must be a list, got {dash}"));
  };
  if values.is_empty() || values.len() % 2 != 0 {
    return Err(":dash must contain a non-empty even number of intervals".to_owned());
  }
  let mut intervals = Vec::with_capacity(values.len());
  for value in values {
    let Edn::Number(value) = value else {
      return Err(format!("dash interval must be a number, got {value}"));
    };
    intervals.push(validate_positive("dash interval", *value as f32)?);
  }
  let offset = match offset {
    Some(Edn::Number(offset)) if (*offset as f32).is_finite() => *offset as f32,
    Some(value) => return Err(format!("dash-offset must be a finite number, got {value}")),
    None => 0.0,
  };
  Ok(Some(DashPattern { intervals, offset }))
}

fn read_required_point(tree: &EdnMapView, key: &str) -> Result<Point2D<f32, f32>, String> {
  let point = tree
    .get(&tag(key))
    .ok_or_else(|| format!("paint requires :{key}"))
    .and_then(extract_position)?;
  if point.x.is_finite() && point.y.is_finite() {
    Ok(point)
  } else {
    Err(format!("paint :{key} must contain finite coordinates"))
  }
}

fn read_optional_cap(tree: &EdnMapView, key: &str) -> Result<Option<Cap>, String> {
  match tree.get(&tag(key)) {
    Some(_) => read_line_cap(tree, key).map(Some),
    None => Ok(None),
  }
}

fn read_optional_join(tree: &EdnMapView, key: &str) -> Result<Option<Join>, String> {
  match tree.get(&tag(key)) {
    Some(_) => read_line_join(tree, key).map(Some),
    None => Ok(None),
  }
}

fn validate_non_negative(name: &str, value: f32) -> Result<f32, String> {
  if value.is_finite() && value >= 0.0 {
    Ok(value)
  } else {
    Err(format!("{name} must be a finite non-negative number, got {value}"))
  }
}

fn validate_positive(name: &str, value: f32) -> Result<f32, String> {
  if value.is_finite() && value > 0.0 {
    Ok(value)
  } else {
    Err(format!("{name} must be a finite positive number, got {value}"))
  }
}

pub fn read_text_align(tree: &EdnMapView, key: &str) -> Result<TextAlign, String> {
  match tree.get(&tag(key)) {
    Some(Edn::Tag(k)) => match k.ref_str() {
      "left" => Ok(TextAlign::Left),
      "center" => Ok(TextAlign::Center),
      "right" => Ok(TextAlign::Right),
      _ => Err(format!("unknown align value: {}", k)),
    },
    Some(a) => Err(format!("invalid text align: {}", a)),
    None => Err(format!("cannot read text align from empty from: {}", key)),
  }
}

fn read_optional_text_align(tree: &EdnMapView, key: &str) -> Result<Option<TextAlign>, String> {
  match tree.get(&tag(key)) {
    Some(_) => read_text_align(tree, key).map(Some),
    None => Ok(None),
  }
}

fn read_text_direction(tree: &EdnMapView, key: &str) -> Result<TextDirection, String> {
  match tree.get(&tag(key)) {
    Some(Edn::Tag(direction)) => match direction.ref_str() {
      "ltr" => Ok(TextDirection::Ltr),
      "rtl" => Ok(TextDirection::Rtl),
      _ => Err(format!("unsupported text direction: {direction}")),
    },
    Some(value) => Err(format!("text direction must be a tag, got {value}")),
    None => Ok(TextDirection::Ltr),
  }
}

fn read_optional_string(tree: &EdnMapView, key: &str) -> Result<Option<String>, String> {
  match tree.get(&tag(key)) {
    Some(Edn::Str(value)) => Ok(Some(value.to_string())),
    Some(Edn::Nil) | None => Ok(None),
    Some(value) => Err(format!("{key} must be a string, got {value}")),
  }
}

pub fn read_optional_string_field(tree: &EdnMapView, key: &str) -> Result<Option<String>, String> {
  read_optional_string(tree, key)
}

pub fn read_optional_i32(tree: &EdnMapView, key: &str) -> Result<Option<i32>, String> {
  match tree.get(&tag(key)) {
    Some(Edn::Number(value))
      if value.is_finite() && value.fract() == 0.0 && *value >= i32::MIN as f64 && *value <= i32::MAX as f64 =>
    {
      Ok(Some(*value as i32))
    }
    Some(value) => Err(format!("{key} must be an integer, got {value}")),
    None => Ok(None),
  }
}

pub fn extract_shortcut_modifiers(tree: &EdnMapView) -> Result<Option<ShortcutModifiers>, String> {
  match tree.get(&tag("modifiers")) {
    Some(Edn::Map(modifiers)) => Ok(Some(ShortcutModifiers {
      shift: read_bool(modifiers, "shift?")?,
      control: read_bool(modifiers, "control?")?,
      alt: read_bool(modifiers, "alt?")?,
      super_key: read_bool(modifiers, "super?")?,
    })),
    Some(value) => Err(format!("key-listener :modifiers must be a map, got {value}")),
    None => Ok(None),
  }
}

fn read_optional_positive_usize(tree: &EdnMapView, key: &str) -> Result<Option<usize>, String> {
  match tree.get(&tag(key)) {
    Some(Edn::Number(value))
      if value.is_finite() && value.fract() == 0.0 && *value >= 1.0 && *value <= usize::MAX as f64 =>
    {
      Ok(Some(*value as usize))
    }
    Some(value) => Err(format!("{key} must be a positive integer, got {value}")),
    None => Ok(None),
  }
}

pub fn extract_paragraph_layout(tree: &EdnMapView) -> Result<ParagraphLayout, String> {
  let size = validate_positive("paragraph size", read_f32(tree, "size")?)?;
  let max_width = validate_positive("paragraph max-width", read_f32(tree, "max-width")?)?;
  let line_height = read_optional_f32(tree, "line-height")?
    .map(|value| validate_positive("paragraph line-height", value))
    .transpose()?;
  let max_lines = read_optional_positive_usize(tree, "max-lines")?;
  let ellipsis = read_optional_string(tree, "ellipsis")?;
  if ellipsis.is_some() && max_lines.is_none() {
    return Err("paragraph :ellipsis requires :max-lines".to_owned());
  }
  Ok(ParagraphLayout {
    text: read_string(tree, "text")?,
    max_width,
    size,
    align: read_optional_text_align(tree, "align")?.unwrap_or(TextAlign::Left),
    direction: read_text_direction(tree, "direction")?,
    style: extract_text_style(tree)?,
    line_height,
    max_lines,
    ellipsis,
  })
}

pub fn extract_text_style(tree: &EdnMapView) -> Result<TextStyle, String> {
  let family = match tree.get(&tag("font-family")) {
    Some(Edn::Str(family)) => Some(family.to_string()),
    Some(Edn::Nil) | None => None,
    Some(value) => return Err(format!("font-family must be a string, got {value}")),
  };
  let weight = match tree.get(&tag("weight")) {
    Some(Edn::Number(weight)) => read_font_weight(*weight, "weight")?,
    // The original demo used a string weight before this option was implemented.
    Some(Edn::Str(weight)) => weight
      .parse::<f64>()
      .map_err(|_| format!("weight must be a number between 100 and 900, got {weight}"))
      .and_then(|weight| read_font_weight(weight, "weight"))?,
    Some(value) => return Err(format!("weight must be a number between 100 and 900, got {value}")),
    None => 400,
  };
  let slant = match tree.get(&tag("style")) {
    Some(Edn::Tag(style)) => match style.ref_str() {
      "normal" => TextSlant::Normal,
      "italic" => TextSlant::Italic,
      _ => return Err(format!("unsupported text style: {style}")),
    },
    Some(value) => return Err(format!("text style must be a tag, got {value}")),
    None => TextSlant::Normal,
  };
  let baseline = match tree.get(&tag("baseline")) {
    Some(Edn::Tag(baseline)) => match baseline.ref_str() {
      "alphabetic" => TextBaseline::Alphabetic,
      "top" => TextBaseline::Top,
      "middle" => TextBaseline::Middle,
      "bottom" => TextBaseline::Bottom,
      _ => return Err(format!("unsupported text baseline: {baseline}")),
    },
    Some(value) => return Err(format!("text baseline must be a tag, got {value}")),
    None => TextBaseline::Alphabetic,
  };
  Ok(TextStyle {
    family,
    weight,
    slant,
    baseline,
  })
}

fn read_font_weight(weight: f64, field: &str) -> Result<i32, String> {
  if weight.is_finite() && weight.fract() == 0.0 && (100.0..=900.0).contains(&weight) {
    Ok(weight as i32)
  } else {
    Err(format!("{field} must be an integer between 100 and 900, got {weight}"))
  }
}

pub fn read_line_join(tree: &EdnMapView, key: &str) -> Result<Join, String> {
  match tree.get(&tag(key)) {
    Some(Edn::Tag(k)) => match k.ref_str() {
      "round" => Ok(Join::Round),
      "miter" => Ok(Join::Miter),
      // "miter-clip" => Ok(Join::MiterClip),
      "bevel" => Ok(Join::Bevel),
      _ => Err(format!("unknown align value: {}", k)),
    },
    Some(a) => Err(format!("invalid text align: {}", a)),
    None => Err(format!("cannot read line join from empty from: {}", key)),
  }
}

pub fn read_line_cap(tree: &EdnMapView, key: &str) -> Result<Cap, String> {
  match tree.get(&tag(key)) {
    Some(Edn::Tag(k)) => match k.ref_str() {
      "round" => Ok(Cap::Round),
      "butt" => Ok(Cap::Butt),
      "square" => Ok(Cap::Square),
      _ => Err(format!("unknown align value: {}", k)),
    },
    Some(a) => Err(format!("invalid text align: {}", a)),
    None => Err(format!("cannot read line join from empty from: {}", key)),
  }
}

pub fn read_points(tree: &EdnMapView, key: &str) -> Result<Vec<Point2D<f32, f32>>, String> {
  match tree.get(&tag(key)) {
    Some(Edn::List(EdnListView(xs))) => {
      let mut ys: Vec<Point2D<f32, f32>> = vec![];
      for x in xs {
        match x {
          Edn::List(EdnListView(pair)) if pair.len() == 2 => match (&pair[0], &pair[1]) {
            (Edn::Number(x), Edn::Number(y)) => ys.push(Point2D::new(*x as f32, *y as f32)),
            (a, b) => return Err(format!("invalid point: {} {}", a, b)),
          },
          Edn::List(EdnListView(ps)) => return Err(format!("invalid point position: {:?}", ps)),
          _ => return Err(format!("invalid position value: {}", x)),
        }
      }
      Ok(ys)
    }
    Some(a) => Err(format!("cannot be used as points positions: {}", a)),
    None => Err(format!("cannot read position from empty from: {}", key)),
  }
}

pub fn extract_touch_area_shape(m: &EdnMapView) -> Result<TouchAreaShape, String> {
  if let Some(Edn::Number(n)) = m.get(&tag("radius")) {
    Ok(TouchAreaShape::Circle(*n as f32))
  } else {
    match (m.get(&tag("dx")), m.get(&tag("dy"))) {
      (Some(Edn::Number(dx)), Some(Edn::Number(dy))) => Ok(TouchAreaShape::Rect(*dx as f32, *dy as f32)),
      (a, b) => Err(format!("invalid touch area shape: {:?} {:?}", a, b)),
    }
  }
}

pub fn read_optional_cursor_icon(tree: &EdnMapView) -> Result<Option<CursorIcon>, String> {
  match tree.get(&tag("cursor")) {
    None | Some(Edn::Nil) => Ok(None),
    Some(Edn::Tag(name)) => {
      let name = name.ref_str();
      let cursor = match name {
        "dnd-ask" => CursorIcon::DndAsk,
        "all-resize" => CursorIcon::AllResize,
        _ => name.parse().map_err(|_| {
          format!(
            "invalid cursor :{name}; expected a W3C cursor tag such as :default, :pointer, :text, :grab, or :crosshair"
          )
        })?,
      };
      Ok(Some(cursor))
    }
    Some(value) => Err(format!("cursor must be a tag or nil, got {value}")),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn map(fields: impl IntoIterator<Item = (&'static str, Edn)>) -> Edn {
    let mut values = EdnMapView::default();
    for (key, value) in fields {
      values.insert(tag(key), value);
    }
    Edn::Map(values)
  }

  fn list(values: impl IntoIterator<Item = Edn>) -> Edn {
    Edn::List(EdnListView(values.into_iter().collect()))
  }

  fn point(x: f64, y: f64) -> Edn {
    list([Edn::Number(x), Edn::Number(y)])
  }

  fn color(hue: f64) -> Edn {
    list([Edn::Number(hue), Edn::Number(80.0), Edn::Number(60.0)])
  }

  fn stop(offset: f64, hue: f64) -> Edn {
    list([Edn::Number(offset), color(hue)])
  }

  fn map_view(value: &Edn) -> &EdnMapView {
    let Edn::Map(value) = value else { panic!("expected map") };
    value
  }

  #[test]
  fn reads_strict_optional_cursor_tags() {
    assert_eq!(read_optional_cursor_icon(&EdnMapView::default()).unwrap(), None);
    assert_eq!(
      read_optional_cursor_icon(map_view(&map([("cursor", Edn::Nil)]))).unwrap(),
      None
    );
    assert_eq!(
      read_optional_cursor_icon(map_view(&map([("cursor", tag("pointer"))]))).unwrap(),
      Some(CursorIcon::Pointer)
    );
    assert_eq!(
      read_optional_cursor_icon(map_view(&map([("cursor", tag("all-resize"))]))).unwrap(),
      Some(CursorIcon::AllResize)
    );
    assert!(read_optional_cursor_icon(map_view(&map([("cursor", Edn::str("pointer"))]))).is_err());
    assert!(read_optional_cursor_icon(map_view(&map([("cursor", tag("unknown-cursor"))]))).is_err());
  }

  #[test]
  fn extracts_explicit_accessibility_metadata_and_rejects_invalid_roles() {
    let mut metadata = EdnMapView::default();
    metadata.insert(tag("id"), Edn::str("save"));
    metadata.insert(tag("role"), tag("button"));
    metadata.insert(tag("label"), Edn::str("Save document"));
    metadata.insert(tag("enabled?"), Edn::Bool(true));
    let scene = map([("accessibility", Edn::Map(metadata))]);
    assert!(matches!(
      extract_accessibility(map_view(&scene)),
      Ok(Some(AccessibilityProperties {
        id,
        role: AccessibilityRole::Button,
        enabled: true,
        ..
      })) if id == "save"
    ));

    let mut invalid = EdnMapView::default();
    invalid.insert(tag("id"), Edn::str("save"));
    invalid.insert(tag("role"), tag("slider"));
    invalid.insert(tag("label"), Edn::str("Save document"));
    let invalid = map([("accessibility", Edn::Map(invalid))]);
    assert!(extract_accessibility(map_view(&invalid))
      .unwrap_err()
      .contains("unsupported :accessibility :role :slider"));
  }

  #[test]
  fn treats_missing_and_nil_event_target_fields_as_optional() {
    assert_eq!(extract_event_target(&EdnMapView::default()), EventTarget::default());

    let explicit_nil = map([("action", Edn::Nil), ("path", Edn::Nil), ("data", Edn::Nil)]);
    assert_eq!(extract_event_target(map_view(&explicit_nil)), EventTarget::default());

    let populated = map([
      ("action", Edn::tag("drag")),
      ("path", Edn::tag("canvas")),
      ("data", Edn::Number(1.0)),
    ]);
    assert_eq!(
      extract_event_target(map_view(&populated)),
      EventTarget {
        action: Some(Edn::tag("drag")),
        path: Some(Edn::tag("canvas")),
        data: Some(Edn::Number(1.0)),
      }
    );
  }

  #[test]
  fn keeps_legacy_solid_fill_and_stroke() {
    let shape = map([
      ("fill-color", color(20.0)),
      ("line-color", color(200.0)),
      ("line-width", Edn::Number(3.0)),
    ]);
    assert!(matches!(
      extract_fill_style(map_view(&shape)),
      Ok(Some(PaintSource::Solid(_)))
    ));
    assert!(matches!(
      extract_stroke_style(map_view(&shape)),
      Ok(Some(StrokeStyle {
        width: 3.0,
        dash: None,
        ..
      }))
    ));
  }

  #[test]
  fn extracts_linear_and_radial_gradients() {
    let linear = map([
      ("type", Edn::tag("linear-gradient")),
      ("from", point(0.0, 0.0)),
      ("to", point(100.0, 0.0)),
      ("stops", list([stop(0.0, 10.0), stop(1.0, 210.0)])),
    ]);
    let shape = map([("fill", linear)]);
    assert!(matches!(
      extract_fill_style(map_view(&shape)),
      Ok(Some(PaintSource::LinearGradient { .. }))
    ));

    let radial = map([
      ("type", Edn::tag("radial-gradient")),
      ("center", point(50.0, 50.0)),
      ("radius", Edn::Number(40.0)),
      ("stops", list([stop(0.0, 50.0), stop(1.0, 280.0)])),
    ]);
    let shape = map([("fill", radial)]);
    assert!(matches!(
      extract_fill_style(map_view(&shape)),
      Ok(Some(PaintSource::RadialGradient { radius: 40.0, .. }))
    ));
  }

  #[test]
  fn extracts_dashed_stroke_options() {
    let paint = map([("type", Edn::tag("solid")), ("color", color(120.0))]);
    let stroke = map([
      ("paint", paint),
      ("width", Edn::Number(5.0)),
      ("cap", Edn::tag("square")),
      ("join", Edn::tag("miter")),
      ("miter-limit", Edn::Number(8.0)),
      ("dash", list([Edn::Number(12.0), Edn::Number(6.0)])),
      ("dash-offset", Edn::Number(2.0)),
    ]);
    let shape = map([("stroke", stroke)]);
    let style = extract_stroke_style(map_view(&shape)).unwrap().unwrap();
    assert_eq!(style.width, 5.0);
    assert_eq!(style.cap, Cap::Square);
    assert_eq!(style.join, Join::Miter);
    assert_eq!(style.miter_limit, 8.0);
    assert_eq!(
      style.dash,
      Some(DashPattern {
        intervals: vec![12.0, 6.0],
        offset: 2.0,
      })
    );
  }

  #[test]
  fn extracts_text_style_defaults_and_legacy_weight() {
    let defaults = map([]);
    assert_eq!(
      extract_text_style(map_view(&defaults)),
      Ok(TextStyle {
        family: None,
        weight: 400,
        slant: TextSlant::Normal,
        baseline: TextBaseline::Alphabetic,
      })
    );

    let explicit = map([
      ("font-family", Edn::Str("monospace".into())),
      // The pre-existing runnable demo used string weights.
      ("weight", Edn::Str("300".into())),
      ("style", Edn::tag("italic")),
      ("baseline", Edn::tag("middle")),
    ]);
    assert_eq!(
      extract_text_style(map_view(&explicit)),
      Ok(TextStyle {
        family: Some("monospace".into()),
        weight: 300,
        slant: TextSlant::Italic,
        baseline: TextBaseline::Middle,
      })
    );
  }

  #[test]
  fn rejects_invalid_text_style_options() {
    let invalid_weight = map([("weight", Edn::Number(250.5))]);
    assert!(extract_text_style(map_view(&invalid_weight))
      .unwrap_err()
      .contains("integer between 100 and 900"));

    let invalid_style = map([("style", Edn::tag("oblique"))]);
    assert!(extract_text_style(map_view(&invalid_style))
      .unwrap_err()
      .contains("unsupported text style"));

    let invalid_baseline = map([("baseline", Edn::tag("hanging"))]);
    assert!(extract_text_style(map_view(&invalid_baseline))
      .unwrap_err()
      .contains("unsupported text baseline"));
  }

  #[test]
  fn rejects_invalid_gradient_and_dash_inputs() {
    let unordered = map([
      ("type", Edn::tag("linear-gradient")),
      ("from", point(0.0, 0.0)),
      ("to", point(100.0, 0.0)),
      ("stops", list([stop(0.8, 10.0), stop(0.2, 210.0)])),
    ]);
    let shape = map([("fill", unordered)]);
    assert!(extract_fill_style(map_view(&shape))
      .unwrap_err()
      .contains("strictly increasing"));

    let degenerate = map([
      ("type", Edn::tag("linear-gradient")),
      ("from", point(10.0, 10.0)),
      ("to", point(10.0, 10.0)),
      ("stops", list([stop(0.0, 10.0), stop(1.0, 210.0)])),
    ]);
    let shape = map([("fill", degenerate)]);
    assert!(extract_fill_style(map_view(&shape))
      .unwrap_err()
      .contains("must be different"));

    let paint = map([("type", Edn::tag("solid")), ("color", color(120.0))]);
    let empty_dash = map([("paint", paint.clone()), ("dash", list([]))]);
    let shape = map([("stroke", empty_dash)]);
    assert!(extract_stroke_style(map_view(&shape))
      .unwrap_err()
      .contains("non-empty even number"));

    let negative_dash = map([("paint", paint), ("dash", list([Edn::Number(5.0), Edn::Number(-1.0)]))]);
    let shape = map([("stroke", negative_dash)]);
    assert!(extract_stroke_style(map_view(&shape))
      .unwrap_err()
      .contains("finite positive number"));
  }

  #[test]
  fn validates_blend_modes() {
    let blend = map([("mode", Edn::tag("multiply"))]);
    assert_eq!(read_blend_mode(map_view(&blend), "mode"), Ok(BlendMode::Multiply));

    let unknown = map([("mode", Edn::tag("magic"))]);
    assert!(read_blend_mode(map_view(&unknown), "mode")
      .unwrap_err()
      .contains("unsupported blend mode"));
  }
}

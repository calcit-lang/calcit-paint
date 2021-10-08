use raqote::{Color, LineCap, LineJoin};

use euclid::{Point2D, Vector2D};

use calcit_runner::{
  primes::{load_kwd, lookup_order_kwd_str},
  Calcit,
};

use crate::{
  color::extract_color,
  primes::{kwd, TextAlign, TouchAreaShape},
};

pub fn read_f32(tree: &im::HashMap<Calcit, Calcit>, key: &str) -> Result<f32, String> {
  match tree.get(&load_kwd(key)) {
    Some(Calcit::Number(n)) => Ok(*n as f32),
    Some(a) => Err(format!("cannot be used as f32: {}", a)),
    None => Err(format!(
      "cannot read f32 {} from empty from: {}",
      key,
      Calcit::Map(tree.to_owned())
    )),
  }
}

pub fn read_bool(tree: &im::HashMap<Calcit, Calcit>, key: &str) -> Result<bool, String> {
  match tree.get(&load_kwd(key)) {
    Some(Calcit::Bool(b)) => Ok(*b),
    Some(a) => Err(format!("cannot be used as bool: {}", a)),
    None => Ok(false),
  }
}

pub fn read_string(tree: &im::HashMap<Calcit, Calcit>, key: &str) -> Result<String, String> {
  match tree.get(&load_kwd(key)) {
    Some(Calcit::Str(s)) => Ok(s.to_string()),
    Some(Calcit::Keyword(s)) => Ok(s.to_string()),
    Some(a) => Err(format!(
      "cannot be used as string {} in {}",
      a,
      Calcit::Map(tree.to_owned())
    )),
    None => Err(format!("cannot read string from empty from: {}", key)),
  }
}

pub fn read_position(tree: &im::HashMap<Calcit, Calcit>, key: &str) -> Result<Vector2D<f32, f32>, String> {
  match tree.get(&load_kwd(key)) {
    Some(Calcit::List(xs)) if xs.len() == 2 => match (&xs[0], &xs[1]) {
      (Calcit::Number(x), Calcit::Number(y)) => Ok(Vector2D::new(*x as f32, *y as f32)),
      (a, b) => Err(format!("invalid positon values: {} {}", a, b)),
    },
    Some(Calcit::List(xs)) => Err(format!("invalid position length: {:?}", xs)),
    Some(Calcit::Nil) => Ok(Vector2D::new(0.0, 0.0)),
    Some(a) => Err(format!(
      "cannot be used as position: {} in {}",
      a,
      Calcit::Map(tree.to_owned())
    )),
    None => Ok(Vector2D::new(0.0, 0.0)),
  }
}

// get position from a value
pub fn extract_position(x: &Calcit) -> Result<Point2D<f32, f32>, String> {
  match x {
    Calcit::List(xs) if xs.len() == 2 => match (&xs[0], &xs[1]) {
      (Calcit::Number(x), Calcit::Number(y)) => Ok(Point2D::new(*x as f32, *y as f32)),
      (a, b) => Err(format!("invalid positon values: {} {}", a, b)),
    },
    a => Err(format!("cannot be used as position: {} in {}", a, x)),
  }
}

pub fn read_color(tree: &im::HashMap<Calcit, Calcit>, key: &str) -> Result<Color, String> {
  match tree.get(&load_kwd(key)) {
    Some(a) => extract_color(a),
    None => Err(format!("cannot read color from empty from: {}", key)),
  }
}

pub fn read_some_color(tree: &im::HashMap<Calcit, Calcit>, key: &str) -> Result<Option<Color>, String> {
  match tree.get(&load_kwd(key)) {
    Some(a) => match extract_color(a) {
      Ok(c) => Ok(Some(c)),
      Err(e) => Err(e),
    },
    None => Ok(None),
  }
}

pub fn extract_line_style(tree: &im::HashMap<Calcit, Calcit>) -> Result<Option<(Color, f32)>, String> {
  match (tree.get(&kwd("line-color")), tree.get(&kwd("line-width"))) {
    (Some(color_field), Some(width_field)) => match (extract_color(color_field), width_field) {
      (Ok(color), Calcit::Number(n)) => Ok(Some((color, *n as f32))),
      (Ok(_), _) => Err(format!("failed to extract line-width from: {}", width_field)),
      (Err(e), _) => Err(format!("failed line-color, {}", e)),
    },
    (Some(color_field), None) => match extract_color(color_field) {
      Ok(color) => Ok(Some((color, 1.0))),
      Err(e) => Err(format!("failed line-color, {}", e)),
    },
    (None, None) => Ok(None),
    (a, b) => Err(format!("invalid line-style combination: {:?} {:?}", a, b)),
  }
}

pub fn read_text_align(tree: &im::HashMap<Calcit, Calcit>, key: &str) -> Result<TextAlign, String> {
  match tree.get(&load_kwd(key)) {
    Some(Calcit::Keyword(k)) => match lookup_order_kwd_str(k).as_str() {
      "left" => Ok(TextAlign::Left),
      "center" => Ok(TextAlign::Center),
      "right" => Ok(TextAlign::Right),
      _ => Err(format!("unknown align value: {}", k)),
    },
    Some(a) => Err(format!("invalid text align: {}", a)),
    None => Err(format!("cannot read text align from empty from: {}", key)),
  }
}

pub fn read_line_join(tree: &im::HashMap<Calcit, Calcit>, key: &str) -> Result<LineJoin, String> {
  match tree.get(&load_kwd(key)) {
    Some(Calcit::Keyword(k)) => match lookup_order_kwd_str(k).as_str() {
      "round" => Ok(LineJoin::Round),
      "miter" => Ok(LineJoin::Miter),
      // "miter-clip" => Ok(LineJoin::MiterClip),
      "bevel" => Ok(LineJoin::Bevel),
      _ => Err(format!("unknown align value: {}", k)),
    },
    Some(a) => Err(format!("invalid text align: {}", a)),
    None => Err(format!("cannot read line join from empty from: {}", key)),
  }
}

pub fn read_line_cap(tree: &im::HashMap<Calcit, Calcit>, key: &str) -> Result<LineCap, String> {
  match tree.get(&load_kwd(key)) {
    Some(Calcit::Keyword(k)) => match lookup_order_kwd_str(k).as_str() {
      "round" => Ok(LineCap::Round),
      "butt" => Ok(LineCap::Butt),
      "square" => Ok(LineCap::Square),
      _ => Err(format!("unknown align value: {}", k)),
    },
    Some(a) => Err(format!("invalid text align: {}", a)),
    None => Err(format!("cannot read line join from empty from: {}", key)),
  }
}

pub fn read_points(tree: &im::HashMap<Calcit, Calcit>, key: &str) -> Result<Vec<Point2D<f32, f32>>, String> {
  match tree.get(&load_kwd(key)) {
    Some(Calcit::List(xs)) => {
      let mut ys: Vec<Point2D<f32, f32>> = vec![];
      for x in xs {
        match x {
          Calcit::List(pair) if pair.len() == 2 => match (&pair[0], &pair[1]) {
            (Calcit::Number(x), Calcit::Number(y)) => ys.push(Point2D::new(*x as f32, *y as f32)),
            (a, b) => return Err(format!("invalid point: {} {}", a, b)),
          },
          Calcit::List(ps) => return Err(format!("invalid point position: {:?}", ps)),
          _ => return Err(format!("invalid position value: {}", x)),
        }
      }
      Ok(ys)
    }
    Some(a) => Err(format!("cannot be used as points positions: {}", a)),
    None => Err(format!("cannot read position from empty from: {}", key)),
  }
}

pub fn extract_touch_area_shape(m: &im::HashMap<Calcit, Calcit>) -> Result<TouchAreaShape, String> {
  if let Some(Calcit::Number(n)) = m.get(&load_kwd("radius")) {
    Ok(TouchAreaShape::Circle(*n as f32))
  } else {
    match (m.get(&load_kwd("dx")), m.get(&load_kwd("dy"))) {
      (Some(Calcit::Number(dx)), Some(Calcit::Number(dy))) => Ok(TouchAreaShape::Rect(*dx as f32, *dy as f32)),
      (a, b) => Err(format!("invalid touch area shape: {:?} {:?}", a, b)),
    }
  }
}

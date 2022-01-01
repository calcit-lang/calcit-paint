// use raqote::{Color, Cap, LineJoin};

use cirru_edn::Edn;
use euclid::{Point2D, Vector2D};
use std::collections::HashMap;

use skia_safe::paint::{Cap, Join};
use skia_safe::Color;

use crate::{
  color::extract_color,
  primes::{kwd, TextAlign, TouchAreaShape},
};

pub fn load_kwd(s: &str) -> Edn {
  Edn::kwd(s)
}

pub fn read_f32(tree: &HashMap<Edn, Edn>, key: &str) -> Result<f32, String> {
  match tree.get(&load_kwd(key)) {
    Some(Edn::Number(n)) => Ok(*n as f32),
    Some(a) => Err(format!("cannot be used as f32: {}", a)),
    None => Err(format!(
      "cannot read f32 {} from empty from: {}",
      key,
      Edn::Map(tree.to_owned())
    )),
  }
}

pub fn read_bool(tree: &HashMap<Edn, Edn>, key: &str) -> Result<bool, String> {
  match tree.get(&load_kwd(key)) {
    Some(Edn::Bool(b)) => Ok(*b),
    Some(a) => Err(format!("cannot be used as bool: {}", a)),
    None => Ok(false),
  }
}

pub fn read_string(tree: &HashMap<Edn, Edn>, key: &str) -> Result<String, String> {
  match tree.get(&load_kwd(key)) {
    Some(Edn::Str(s)) => Ok(s.to_string()),
    Some(Edn::Keyword(s)) => Ok(s.to_string()),
    Some(a) => Err(format!(
      "cannot be used as string {} in {}",
      a,
      Edn::Map(tree.to_owned())
    )),
    None => Err(format!("cannot read string from empty from: {}", key)),
  }
}

pub fn read_position(tree: &HashMap<Edn, Edn>, key: &str) -> Result<Vector2D<f32, f32>, String> {
  match tree.get(&load_kwd(key)) {
    Some(Edn::List(xs)) if xs.len() == 2 => match (&xs[0], &xs[1]) {
      (Edn::Number(x), Edn::Number(y)) => Ok(Vector2D::new(*x as f32, *y as f32)),
      (a, b) => Err(format!("invalid positon values: {} {}", a, b)),
    },
    Some(Edn::List(xs)) => Err(format!("invalid position length: {:?}", xs)),
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
    Edn::List(xs) if xs.len() == 2 => match (&xs[0], &xs[1]) {
      (Edn::Number(x), Edn::Number(y)) => Ok(Point2D::new(*x as f32, *y as f32)),
      (a, b) => Err(format!("invalid positon values: {} {}", a, b)),
    },
    a => Err(format!("cannot be used as position: {} in {}", a, x)),
  }
}

pub fn read_color(tree: &HashMap<Edn, Edn>, key: &str) -> Result<Color, String> {
  match tree.get(&load_kwd(key)) {
    Some(a) => extract_color(a),
    None => Err(format!("cannot read color from empty from: {}", key)),
  }
}

pub fn read_some_color(tree: &HashMap<Edn, Edn>, key: &str) -> Result<Option<Color>, String> {
  match tree.get(&load_kwd(key)) {
    Some(a) => match extract_color(a) {
      Ok(c) => Ok(Some(c)),
      Err(e) => Err(e),
    },
    None => Ok(None),
  }
}

pub fn extract_line_style(tree: &HashMap<Edn, Edn>) -> Result<Option<(Color, f32)>, String> {
  match (tree.get(&kwd("line-color")), tree.get(&kwd("line-width"))) {
    (Some(color_field), Some(width_field)) => match (extract_color(color_field), width_field) {
      (Ok(color), Edn::Number(n)) => Ok(Some((color, *n as f32))),
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

pub fn read_text_align(tree: &HashMap<Edn, Edn>, key: &str) -> Result<TextAlign, String> {
  match tree.get(&load_kwd(key)) {
    Some(Edn::Keyword(k)) => match &*k.to_str() {
      "left" => Ok(TextAlign::Left),
      "center" => Ok(TextAlign::Center),
      "right" => Ok(TextAlign::Right),
      _ => Err(format!("unknown align value: {}", k)),
    },
    Some(a) => Err(format!("invalid text align: {}", a)),
    None => Err(format!("cannot read text align from empty from: {}", key)),
  }
}

pub fn read_line_join(tree: &HashMap<Edn, Edn>, key: &str) -> Result<Join, String> {
  match tree.get(&load_kwd(key)) {
    Some(Edn::Keyword(k)) => match &*k.to_str() {
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

pub fn read_line_cap(tree: &HashMap<Edn, Edn>, key: &str) -> Result<Cap, String> {
  match tree.get(&load_kwd(key)) {
    Some(Edn::Keyword(k)) => match &*k.to_str() {
      "round" => Ok(Cap::Round),
      "butt" => Ok(Cap::Butt),
      "square" => Ok(Cap::Square),
      _ => Err(format!("unknown align value: {}", k)),
    },
    Some(a) => Err(format!("invalid text align: {}", a)),
    None => Err(format!("cannot read line join from empty from: {}", key)),
  }
}

pub fn read_points(tree: &HashMap<Edn, Edn>, key: &str) -> Result<Vec<Point2D<f32, f32>>, String> {
  match tree.get(&load_kwd(key)) {
    Some(Edn::List(xs)) => {
      let mut ys: Vec<Point2D<f32, f32>> = vec![];
      for x in xs {
        match x {
          Edn::List(pair) if pair.len() == 2 => match (&pair[0], &pair[1]) {
            (Edn::Number(x), Edn::Number(y)) => ys.push(Point2D::new(*x as f32, *y as f32)),
            (a, b) => return Err(format!("invalid point: {} {}", a, b)),
          },
          Edn::List(ps) => return Err(format!("invalid point position: {:?}", ps)),
          _ => return Err(format!("invalid position value: {}", x)),
        }
      }
      Ok(ys)
    }
    Some(a) => Err(format!("cannot be used as points positions: {}", a)),
    None => Err(format!("cannot read position from empty from: {}", key)),
  }
}

pub fn extract_touch_area_shape(m: &HashMap<Edn, Edn>) -> Result<TouchAreaShape, String> {
  if let Some(Edn::Number(n)) = m.get(&load_kwd("radius")) {
    Ok(TouchAreaShape::Circle(*n as f32))
  } else {
    match (m.get(&load_kwd("dx")), m.get(&load_kwd("dy"))) {
      (Some(Edn::Number(dx)), Some(Edn::Number(dy))) => Ok(TouchAreaShape::Rect(*dx as f32, *dy as f32)),
      (a, b) => Err(format!("invalid touch area shape: {:?} {:?}", a, b)),
    }
  }
}

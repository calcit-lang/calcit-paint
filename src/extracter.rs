use ggez::graphics::Color;
use glam::Vec2;

use calcit_runner::Calcit;

use crate::{
  color::extract_color,
  primes::{LineCap, LineJoin, Shape, ShapeStyle, TextAlign, TouchAreaShape},
};

pub fn read_f32(tree: &im::HashMap<Calcit, Calcit>, key: &str) -> Result<f32, String> {
  match tree.get(&Calcit::Keyword(String::from(key))) {
    Some(Calcit::Number(n)) => Ok(*n as f32),
    Some(a) => Err(format!("cannot be used as f32: {}", a)),
    None => Err(format!("cannot read f32 from empty from: {}", key)),
  }
}

pub fn read_bool(tree: &im::HashMap<Calcit, Calcit>, key: &str) -> Result<bool, String> {
  match tree.get(&Calcit::Keyword(String::from(key))) {
    Some(Calcit::Bool(b)) => Ok(*b),
    Some(a) => Err(format!("cannot be used as bool: {}", a)),
    None => Err(format!("cannot read bool from empty from: {}", key)),
  }
}

pub fn read_string(tree: &im::HashMap<Calcit, Calcit>, key: &str) -> Result<String, String> {
  match tree.get(&Calcit::Keyword(String::from(key))) {
    Some(Calcit::Str(s)) => Ok(s.to_string()),
    Some(Calcit::Keyword(s)) => Ok(s.to_string()),
    Some(a) => Err(format!("cannot be used as string: {}", a)),
    None => Err(format!("cannot read string from empty from: {}", key)),
  }
}

pub fn read_position(tree: &im::HashMap<Calcit, Calcit>, key: &str) -> Result<Vec2, String> {
  match tree.get(&Calcit::Keyword(String::from(key))) {
    Some(Calcit::List(xs)) if xs.len() == 2 => match (&xs[0], &xs[1]) {
      (Calcit::Number(x), Calcit::Number(y)) => Ok(Vec2::new(*x as f32, *y as f32)),
      (a, b) => Err(format!("invalid positon values: {} {}", a, b)),
    },
    Some(Calcit::List(xs)) => Err(format!("invalid position length: {:?}", xs)),
    Some(a) => Err(format!("cannot be used as position: {}", a)),
    None => Err(format!("cannot read position from empty from: {}", key)),
  }
}

pub fn extract_style(m: &im::HashMap<Calcit, Calcit>) -> Result<ShapeStyle, String> {
  if let Some(raw_color) = m.get(&Calcit::Keyword(String::from("fill-color"))) {
    let color = extract_color(raw_color)?;
    Ok(ShapeStyle::Fill { color })
  } else if let Some(raw_color) = m.get(&Calcit::Keyword(String::from("line-color"))) {
    let color = extract_color(raw_color)?;
    let width = match m.get(&Calcit::Keyword(String::from("line-width"))) {
      Some(Calcit::Number(w)) => *w as f32,
      Some(a) => return Err(format!("invalid line-width value: {}", a)),
      None => 1.0,
    };
    Ok(ShapeStyle::Line { color, width })
  } else {
    Err(format!("expected fill-color or line-color"))
  }
}

pub fn read_color(tree: &im::HashMap<Calcit, Calcit>, key: &str) -> Result<Color, String> {
  match tree.get(&Calcit::Keyword(String::from(key))) {
    Some(a) => extract_color(a),
    None => Err(format!("cannot read color from empty from: {}", key)),
  }
}

pub fn read_text_align(tree: &im::HashMap<Calcit, Calcit>, key: &str) -> Result<TextAlign, String> {
  match tree.get(&Calcit::Keyword(String::from(key))) {
    Some(Calcit::Keyword(k)) => match k.as_str() {
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
  match tree.get(&Calcit::Keyword(String::from(key))) {
    Some(Calcit::Keyword(k)) => match k.as_str() {
      "round" => Ok(LineJoin::Round),
      "miter" => Ok(LineJoin::Miter),
      "miter-clip" => Ok(LineJoin::MiterClip),
      "bevel" => Ok(LineJoin::Bevel),
      _ => Err(format!("unknown align value: {}", k)),
    },
    Some(a) => Err(format!("invalid text align: {}", a)),
    None => Err(format!("cannot read line join from empty from: {}", key)),
  }
}

pub fn read_line_cap(tree: &im::HashMap<Calcit, Calcit>, key: &str) -> Result<LineCap, String> {
  match tree.get(&Calcit::Keyword(String::from(key))) {
    Some(Calcit::Keyword(k)) => match k.as_str() {
      "round" => Ok(LineCap::Round),
      "butt" => Ok(LineCap::Butt),
      "square" => Ok(LineCap::Square),
      _ => Err(format!("unknown align value: {}", k)),
    },
    Some(a) => Err(format!("invalid text align: {}", a)),
    None => Err(format!("cannot read line join from empty from: {}", key)),
  }
}

pub fn read_points(tree: &im::HashMap<Calcit, Calcit>, key: &str) -> Result<Vec<Vec2>, String> {
  match tree.get(&Calcit::Keyword(String::from(key))) {
    Some(Calcit::List(xs)) => {
      let mut ys: Vec<Vec2> = vec![];
      for x in xs {
        match x {
          Calcit::List(pair) if pair.len() == 2 => match (&pair[0], &pair[1]) {
            (Calcit::Number(x), Calcit::Number(y)) => ys.push(Vec2::new(*x as f32, *y as f32)),
            (a, b) => return Err(format!("invalid point: {} {}", a, b)),
          },
          Calcit::List(ps) => return Err(format!("invalid point position: {:?}", ps)),
          _ => return Err(format!("invalid position value: {}", x)),
        }
      }
      Ok(ys)
    }
    Some(a) => Err(format!("cannot be used as position: {}", a)),
    None => Err(format!("cannot read position from empty from: {}", key)),
  }
}

pub fn extract_touch_area_shape(m: &im::HashMap<Calcit, Calcit>) -> Result<TouchAreaShape, String> {
  if let Some(Calcit::Number(n)) = m.get(&Calcit::Keyword(String::from("radius"))) {
    Ok(TouchAreaShape::Circle(*n as f32))
  } else {
    match (
      m.get(&Calcit::Keyword(String::from("dx"))),
      m.get(&Calcit::Keyword(String::from("dy"))),
    ) {
      (Some(Calcit::Number(dx)), Some(Calcit::Number(dy))) => {
        Ok(TouchAreaShape::Rect(*dx as f32, *dy as f32))
      }
      (a, b) => Err(format!("invalid touch area shape: {:?} {:?}", a, b)),
    }
  }
}

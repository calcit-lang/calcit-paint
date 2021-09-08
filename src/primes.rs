use calcit_runner::primes::Calcit;
use glam::Vec2;

use raqote::{Color, LineCap, LineJoin};

#[derive(Debug, PartialEq, Clone)]
pub enum TextAlign {
  Left,
  Center,
  Right,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PaintPathTo {
  Move(Vec2),
  Line(Vec2),
  QuadraticBezier(Vec2, Vec2),
  CubicBezier(Vec2, Vec2, Vec2),
  // ClosePath,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Shape {
  Rectangle {
    position: Vec2,
    width: f32,
    height: f32,
    line_style: Option<(Color, f32)>,
    fill_style: Option<Color>,
  },
  Group {
    position: Vec2,
    children: Vec<Shape>,
  },
  Circle {
    position: Vec2,
    radius: f32,
    line_style: Option<(Color, f32)>,
    fill_style: Option<Color>,
  },
  Text {
    text: String,
    position: Vec2,
    size: f32,
    // weight: String, // TODO
    color: Color,
    align: TextAlign,
  },
  // Arc {
  //   position: Vec2,
  //   radius: f32,
  //   from_angle: f32,
  //   to_angle: f32,
  //   negative: bool,
  //   style: ShapeStyle,
  // },
  PaintOps {
    position: Vec2,
    path: Vec<PaintPathTo>,
    line_style: Option<(Color, f32)>,
    fill_style: Option<Color>,
  },
  Polyline {
    position: Vec2,
    stops: Vec<Vec2>,
    skip_first: bool,
    color: Color,
    width: f32,
    join: LineJoin,
    cap: LineCap,
  },
  TouchArea {
    path: Box<Calcit>,
    action: Box<Calcit>,
    data: Box<Calcit>,
    position: Vec2,
    // children: Vec<Shape>, // TODO
    area: TouchAreaShape,
    line_style: Option<(Color, f32)>,
    fill_style: Option<Color>,
  },
  KeyListener {
    key: String, // TODO modifier
    path: Box<Calcit>,
    action: Box<Calcit>,
    data: Box<Calcit>,
    // children: Vec<Shape>, // TODO
  },
  Translate {
    x: f32,
    y: f32,
    children: Vec<Shape>,
  },
  Rotate {
    radius: f32,
    children: Vec<Shape>,
  },
  Scale {
    factor: f32,
    children: Vec<Shape>,
  },
}

#[derive(Debug, PartialEq, Clone)]
pub enum TouchAreaShape {
  Circle(f32),
  Rect(f32, f32),
}

pub fn kwd(s: &str) -> Calcit {
  Calcit::Keyword(s.to_string())
}

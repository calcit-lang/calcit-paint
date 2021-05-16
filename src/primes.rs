use calcit_runner::primes::Calcit;
use ggez::graphics::Color;
use glam::Vec2;

pub use ggez::graphics::{LineCap, LineJoin};

#[derive(Debug, PartialEq, Clone)]
pub enum TextAlign {
  Left,
  Center,
  Right,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PaintOp {
  Stroke,
  Fill,
  StrokePreserve,
  FillPreserve,
  LineWidth(f64),
  Rgb(Color), // translate HSL
  MoveTo(Vec2),
  LineTo(Vec2),
  RelativeLineTo(Vec2),
  CurveTo(Vec<Vec2>),
  RelativeCurveTo(Vec<Vec2>),
  Arc(Vec2, f64, f64, f64, bool),
  NewPath,
  ClosePath,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ShapeStyle {
  Line { color: Color, width: f32 },
  Fill { color: Color },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Shape {
  Rectangle {
    position: Vec2,
    width: f32,
    height: f32,
    style: ShapeStyle,
  },
  Group {
    position: Vec2,
    children: Vec<Shape>,
  },
  Circle {
    position: Vec2,
    radius: f32,
    // children: Vec<Shape>,
    style: ShapeStyle,
  },
  Text {
    text: String,
    position: Vec2,
    size: f32,
    // weight: String, // TODO
    color: Color,
    // align: TextAlign,
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
    ops: Vec<PaintOp>,
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
    path: Calcit,
    action: Calcit,
    data: Calcit,
    position: Vec2,
    style: ShapeStyle,
    // children: Vec<Shape>, // TODO
    area: TouchAreaShape,
  },
  KeyListener {
    key: String, // TODO modifier
    action: Calcit,
    path: Calcit,
    data: Calcit,
    // children: Vec<Shape>, // TODO
  },
}

#[derive(Debug, PartialEq, Clone)]
pub enum TouchAreaShape {
  Circle(f32),
  Rect(f32, f32),
}

pub fn path_add(a: &Vec2, b: &Vec2) -> Vec2 {
  Vec2::new(a.x + b.x, a.y + b.y)
}

use calcit_runner::primes::Calcit;
use ggez::graphics::Color;
use glam::Vec2;

pub use ggez::graphics::{LineCap, LineJoin};

#[derive(Debug, PartialEq)]
pub enum TextAlign {
  Left,
  Center,
  Right,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum ShapeStyle {
  Line { color: Color, width: f32 },
  Fill { color: Color },
}

#[derive(Debug, PartialEq)]
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
  Arc {
    position: Vec2,
    radius: f32,
    from_angle: f32,
    to_angle: f32,
    negative: bool,
    style: ShapeStyle,
  },
  PaintOps {
    position: Vec2,
    ops: Vec<PaintOp>,
  },
  Polyline {
    position: Vec2,
    stops: Vec<Vec2>,
    skip_first: bool,
    color: Color,
    size: f32,
    line_join: LineJoin,
    line_cap: LineCap,
  },
  TouchArea {
    path: Calcit,
    action: Calcit,
    data: Calcit,
    position: Vec2,
    radius: f32,
    style: ShapeStyle,
    // children: Vec<Shape>, // TODO
    area: TouchAreaShape,
  },
  KeyListener {
    key: String, // TODO modifier
    path: Calcit,
    action: Calcit,
    data: Calcit,
    // children: Vec<Shape>, // TODO
  },
}

#[derive(Debug, PartialEq)]
pub enum TouchAreaShape {
  Circle(f32),
  Rect(f32, f32),
}

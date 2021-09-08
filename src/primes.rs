use calcit_runner::primes::Calcit;
use euclid::Point2D;

use raqote::{Color, LineCap, LineJoin};

#[derive(Debug, PartialEq, Clone)]
pub enum TextAlign {
  Left,
  Center,
  Right,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PaintPathTo {
  Move(Point2D<f32, f32>),
  Line(Point2D<f32, f32>),
  QuadraticBezier(Point2D<f32, f32>, Point2D<f32, f32>),
  CubicBezier(Point2D<f32, f32>, Point2D<f32, f32>, Point2D<f32, f32>),
  // ClosePath,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Shape {
  Rectangle {
    position: Point2D<f32, f32>,
    width: f32,
    height: f32,
    line_style: Option<(Color, f32)>,
    fill_style: Option<Color>,
  },
  Group {
    position: Point2D<f32, f32>,
    children: Vec<Shape>,
  },
  Circle {
    position: Point2D<f32, f32>,
    radius: f32,
    line_style: Option<(Color, f32)>,
    fill_style: Option<Color>,
  },
  Text {
    text: String,
    position: Point2D<f32, f32>,
    size: f32,
    // weight: String, // TODO
    color: Color,
    align: TextAlign,
  },
  // Arc {
  //   position: Point2D<f32,f32>,
  //   radius: f32,
  //   from_angle: f32,
  //   to_angle: f32,
  //   negative: bool,
  //   style: ShapeStyle,
  // },
  PaintOps {
    position: Point2D<f32, f32>,
    path: Vec<PaintPathTo>,
    line_style: Option<(Color, f32)>,
    fill_style: Option<Color>,
  },
  Polyline {
    position: Point2D<f32, f32>,
    stops: Vec<Point2D<f32, f32>>,
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
    position: Point2D<f32, f32>,
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

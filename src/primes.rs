use cirru_edn::Edn;
use euclid::{Point2D, Vector2D};

use skia_safe::{
  paint::{Cap, Join},
  Color, Rect,
};

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
  Close,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Shape {
  Rectangle {
    position: Vector2D<f32, f32>,
    width: f32,
    height: f32,
    line_style: Option<(Color, f32)>,
    fill_style: Option<Color>,
  },
  RoundedRectangle {
    position: Vector2D<f32, f32>,
    width: f32,
    height: f32,
    radius_x: f32,
    radius_y: f32,
    line_style: Option<(Color, f32)>,
    fill_style: Option<Color>,
  },
  Group {
    position: Vector2D<f32, f32>,
    children: Vec<Shape>,
  },
  Circle {
    position: Vector2D<f32, f32>,
    radius: f32,
    line_style: Option<(Color, f32)>,
    fill_style: Option<Color>,
  },
  Ellipse {
    position: Vector2D<f32, f32>,
    radius_x: f32,
    radius_y: f32,
    line_style: Option<(Color, f32)>,
    fill_style: Option<Color>,
  },
  Arc {
    position: Vector2D<f32, f32>,
    radius_x: f32,
    radius_y: f32,
    start_angle: f32,
    sweep_angle: f32,
    use_center: bool,
    line_style: Option<(Color, f32)>,
    fill_style: Option<Color>,
  },
  Text {
    text: String,
    position: Vector2D<f32, f32>,
    size: f32,
    // weight: String, // TODO
    color: Color,
    align: TextAlign,
  },
  // Arc {
  //   position: Vector2D<f32,f32>,
  //   radius: f32,
  //   from_angle: f32,
  //   to_angle: f32,
  //   negative: bool,
  //   style: ShapeStyle,
  // },
  PaintOps {
    position: Vector2D<f32, f32>,
    path: Vec<PaintPathTo>,
    line_style: Option<(Color, f32)>,
    fill_style: Option<Color>,
  },
  Polyline {
    position: Vector2D<f32, f32>,
    stops: Vec<Point2D<f32, f32>>,
    skip_first: bool,
    color: Color,
    width: f32,
    join: Join,
    cap: Cap,
  },
  TouchArea {
    path: Box<Edn>,
    action: Box<Edn>,
    data: Box<Edn>,
    position: Vector2D<f32, f32>,
    // children: Vec<Shape>, // TODO
    area: TouchAreaShape,
    line_style: Option<(Color, f32)>,
    fill_style: Option<Color>,
  },
  KeyListener {
    key: String, // TODO modifier
    path: Box<Edn>,
    action: Box<Edn>,
    data: Box<Edn>,
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
  ClipRect {
    position: Vector2D<f32, f32>,
    width: f32,
    height: f32,
    children: Vec<Shape>,
  },
  Opacity {
    alpha: f32,
    children: Vec<Shape>,
  },
  Image {
    file_path: String,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    crop: Option<Rect>,
  },
}

#[derive(Debug, PartialEq, Clone)]
pub enum TouchAreaShape {
  Circle(f32),
  Rect(f32, f32),
}

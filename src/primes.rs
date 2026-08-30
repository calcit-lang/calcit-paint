use cirru_edn::Edn;
use euclid::{Point2D, Vector2D};

use skia_safe::{
  paint::{Cap, Join},
  BlendMode, Color, Rect,
};
use winit::window::CursorIcon;

#[derive(Debug, PartialEq, Clone)]
pub enum TextAlign {
  Left,
  Center,
  Right,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TextDirection {
  Ltr,
  Rtl,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TextSlant {
  Normal,
  Italic,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TextBaseline {
  Alphabetic,
  Top,
  Middle,
  Bottom,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TextStyle {
  pub family: Option<String>,
  pub weight: i32,
  pub slant: TextSlant,
  pub baseline: TextBaseline,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParagraphLayout {
  pub text: String,
  pub max_width: f32,
  pub size: f32,
  pub align: TextAlign,
  pub direction: TextDirection,
  pub style: TextStyle,
  pub line_height: Option<f32>,
  pub max_lines: Option<usize>,
  pub ellipsis: Option<String>,
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
pub struct GradientStop {
  pub offset: f32,
  pub color: Color,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PaintSource {
  Solid(Color),
  LinearGradient {
    from: Point2D<f32, f32>,
    to: Point2D<f32, f32>,
    stops: Vec<GradientStop>,
  },
  RadialGradient {
    center: Point2D<f32, f32>,
    radius: f32,
    stops: Vec<GradientStop>,
  },
}

#[derive(Debug, PartialEq, Clone)]
pub struct DashPattern {
  pub intervals: Vec<f32>,
  pub offset: f32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StrokeStyle {
  pub paint: PaintSource,
  pub width: f32,
  pub cap: Cap,
  pub join: Join,
  pub miter_limit: f32,
  pub dash: Option<DashPattern>,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct EventTarget {
  pub action: Option<Edn>,
  pub path: Option<Edn>,
  pub data: Option<Edn>,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct ShortcutModifiers {
  pub shift: bool,
  pub control: bool,
  pub alt: bool,
  pub super_key: bool,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ImageFit {
  Fill,
  Contain,
  Cover,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ImageSampling {
  Nearest,
  Linear,
  Cubic,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Shape {
  Rectangle {
    position: Vector2D<f32, f32>,
    width: f32,
    height: f32,
    line_style: Option<StrokeStyle>,
    fill_style: Option<PaintSource>,
  },
  RoundedRectangle {
    position: Vector2D<f32, f32>,
    width: f32,
    height: f32,
    radius_x: f32,
    radius_y: f32,
    line_style: Option<StrokeStyle>,
    fill_style: Option<PaintSource>,
  },
  Group {
    position: Vector2D<f32, f32>,
    children: Vec<Shape>,
  },
  CachedGroup {
    cache_key: String,
    revision: i32,
    position: Vector2D<f32, f32>,
    width: i32,
    height: i32,
    children: Vec<Shape>,
  },
  Circle {
    position: Vector2D<f32, f32>,
    radius: f32,
    line_style: Option<StrokeStyle>,
    fill_style: Option<PaintSource>,
  },
  Ellipse {
    position: Vector2D<f32, f32>,
    radius_x: f32,
    radius_y: f32,
    line_style: Option<StrokeStyle>,
    fill_style: Option<PaintSource>,
  },
  Arc {
    position: Vector2D<f32, f32>,
    radius_x: f32,
    radius_y: f32,
    start_angle: f32,
    sweep_angle: f32,
    use_center: bool,
    line_style: Option<StrokeStyle>,
    fill_style: Option<PaintSource>,
  },
  Text {
    text: String,
    position: Vector2D<f32, f32>,
    size: f32,
    color: Color,
    align: TextAlign,
    style: TextStyle,
  },
  Paragraph {
    position: Vector2D<f32, f32>,
    color: Color,
    layout: ParagraphLayout,
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
    line_style: Option<StrokeStyle>,
    fill_style: Option<PaintSource>,
  },
  Polyline {
    position: Vector2D<f32, f32>,
    stops: Vec<Point2D<f32, f32>>,
    skip_first: bool,
    line_style: StrokeStyle,
  },
  TouchArea {
    id: String,
    target: EventTarget,
    position: Vector2D<f32, f32>,
    // children: Vec<Shape>, // TODO
    area: TouchAreaShape,
    cursor: Option<CursorIcon>,
    line_style: Option<StrokeStyle>,
    fill_style: Option<PaintSource>,
  },
  KeyListener {
    key: String,
    modifiers: Option<ShortcutModifiers>,
    focus_id: Option<String>,
    target: EventTarget,
    // children: Vec<Shape>, // TODO
  },
  FocusArea {
    id: String,
    target: EventTarget,
    position: Vector2D<f32, f32>,
    area: TouchAreaShape,
    tab_index: i32,
    text_input: bool,
    line_style: Option<StrokeStyle>,
    fill_style: Option<PaintSource>,
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
  ClipRoundedRect {
    position: Vector2D<f32, f32>,
    width: f32,
    height: f32,
    radius_x: f32,
    radius_y: f32,
    children: Vec<Shape>,
  },
  Opacity {
    alpha: f32,
    children: Vec<Shape>,
  },
  Blend {
    mode: BlendMode,
    children: Vec<Shape>,
  },
  Image {
    id: String,
    file_path: String,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    crop: Option<Rect>,
    fit: ImageFit,
    sampling: ImageSampling,
  },
}

#[derive(Debug, PartialEq, Clone)]
pub enum TouchAreaShape {
  Circle(f32),
  Rect(f32, f32),
}

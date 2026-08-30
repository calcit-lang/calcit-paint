use crate::{
  focus,
  hit_test::{ClipRegion, ClipShape},
  touches,
};
use std::collections::HashMap;
use std::fs;
use std::sync::{
  atomic::{AtomicU64, Ordering},
  RwLock,
};
use std::time::SystemTime;

use euclid::{Angle, Vector2D};

use cirru_edn::{Edn, EdnListView, EdnMapView};

use lazy_static::lazy_static;

type Transform = euclid::default::Transform2D<f32>;

use skia_safe::canvas::{SaveLayerRec, SrcRectConstraint};
use skia_safe::font_style::{Slant, Weight, Width};
use skia_safe::textlayout::{
  FontCollection, Paragraph, ParagraphBuilder, ParagraphStyle, TextAlign as SkParagraphAlign,
  TextDirection as SkTextDirection, TextStyle as ParagraphTextStyle,
};
use skia_safe::{
  gradient, surfaces, AlphaType, Color, Color4f, ColorSpace, ColorType, CubicResampler, Data, EncodedImageFormat,
  FilterMode, Font, FontMgr, FontStyle, Image, ImageInfo, Paint, PaintStyle, PathBuilder, PathEffect, RRect, Rect,
  SamplingOptions, Shader, Surface, TextBlob, TileMode,
};

#[derive(Clone)]
struct CachedImage {
  modified: Option<SystemTime>,
  len: u64,
  bytes: usize,
  last_used: u64,
  image: Image,
}

#[derive(Default)]
struct ImageCache {
  entries: HashMap<String, CachedImage>,
  bytes: usize,
}

#[derive(Clone)]
struct CachedSubtree {
  revision: i32,
  width: i32,
  height: i32,
  bytes: usize,
  last_used: u64,
  image: Image,
}

#[derive(Default)]
struct SubtreeCache {
  entries: HashMap<String, CachedSubtree>,
  bytes: usize,
}

lazy_static! {
  static ref PREV_MESSAGES: RwLock<Vec<(Box<str>, Edn)>> = RwLock::new(vec![]);
  static ref BG_COLOR: RwLock<Color> = RwLock::new(Color::BLACK);
  static ref IMAGE_CACHE: RwLock<ImageCache> = RwLock::new(ImageCache::default());
  static ref SHADER_CACHE: RwLock<HashMap<String, Shader>> = RwLock::new(HashMap::new());
  static ref DASH_EFFECT_CACHE: RwLock<HashMap<String, PathEffect>> = RwLock::new(HashMap::new());
  static ref SUBTREE_CACHE: RwLock<SubtreeCache> = RwLock::new(SubtreeCache::default());
}

static SUBTREE_CACHE_TICK: AtomicU64 = AtomicU64::new(1);
static IMAGE_CACHE_TICK: AtomicU64 = AtomicU64::new(1);

const MAX_OFFSCREEN_DIMENSION: i32 = 4096;
const MAX_OFFSCREEN_PIXELS: usize = 16 * 1024 * 1024;
const SUBTREE_CACHE_LIMIT_BYTES: usize = 32 * 1024 * 1024;
const SUBTREE_CACHE_MAX_ENTRIES: usize = 32;
const IMAGE_CACHE_LIMIT_BYTES: usize = 64 * 1024 * 1024;
const IMAGE_CACHE_MAX_ENTRIES: usize = 64;

#[derive(Clone, Debug, PartialEq, Eq)]
struct SceneDiagnostic {
  path: String,
  message: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct SceneDiagnostics(Vec<SceneDiagnostic>);

impl SceneDiagnostics {
  fn at(path: &str, message: impl Into<String>) -> Self {
    Self(vec![SceneDiagnostic {
      path: path.to_owned(),
      message: message.into(),
    }])
  }

  fn into_messages(self) -> Vec<String> {
    self
      .0
      .into_iter()
      .map(|diagnostic| format!("{}: {}", diagnostic.path, diagnostic.message))
      .collect()
  }

  fn with_default_path(mut self, path: &str) -> Self {
    for diagnostic in &mut self.0 {
      if diagnostic.path.is_empty() {
        diagnostic.path = path.to_owned();
      }
    }
    self
  }
}

impl From<String> for SceneDiagnostics {
  fn from(message: String) -> Self {
    Self(vec![SceneDiagnostic {
      path: String::new(),
      message,
    }])
  }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum RenderMode {
  Interactive,
  Offscreen,
}

use crate::{
  color::extract_color,
  extracter::{
    extract_accessibility, extract_event_target, extract_fill_style, extract_paragraph_layout,
    extract_polyline_stroke_style, extract_position, extract_shortcut_modifiers, extract_stroke_style,
    extract_text_style, extract_touch_area_shape, read_blend_mode, read_bool, read_color, read_f32,
    read_optional_cursor_icon, read_optional_f32, read_optional_i32, read_optional_string_field, read_points,
    read_position, read_string, read_text_align, tag,
  },
  key_listener,
  primes::{
    DashPattern, ImageFit, ImageSampling, PaintPathTo, PaintSource, ParagraphLayout, Shape, StrokeStyle, TextAlign,
    TextBaseline, TextDirection, TextSlant, TextStyle, TouchAreaShape,
  },
};

// TODO Stack

pub fn reset_page(_canvas: &skia_safe::canvas::Canvas, color: Color) -> Result<(), String> {
  touches::reset_touches_stack();
  key_listener::reset_listeners_stack();

  let mut c = BG_COLOR.write().unwrap();
  *c = color;

  Ok(())
}

pub fn get_bg_color() -> Color {
  let c = BG_COLOR.read().unwrap();
  c.to_owned()
}

fn create_text_font(style: &TextStyle, size: f32) -> Result<Font, String> {
  if !size.is_finite() || size <= 0.0 {
    return Err(format!("text size must be a finite positive number, got {size}"));
  }
  let slant = match &style.slant {
    TextSlant::Normal => Slant::Upright,
    TextSlant::Italic => Slant::Italic,
  };
  let font_style = FontStyle::new(Weight::from(style.weight), Width::NORMAL, slant);
  let font_mgr = FontMgr::new();
  // A requested family can be absent on another desktop. In that case retain
  // the requested weight/slant while asking Skia for the platform default.
  let typeface = font_mgr
    .legacy_make_typeface(style.family.as_deref(), font_style)
    .or_else(|| font_mgr.legacy_make_typeface(None, font_style))
    .ok_or_else(|| "Skia could not resolve a default typeface".to_owned())?;
  Ok(Font::new(typeface, size))
}

fn text_x_offset(align: &TextAlign, width: f32) -> f32 {
  match align {
    TextAlign::Left => 0.0,
    TextAlign::Center => -0.5 * width,
    TextAlign::Right => -width,
  }
}

fn text_baseline_y(y: f32, style: &TextStyle, font: &Font) -> f32 {
  let (_, metrics) = font.metrics();
  match &style.baseline {
    TextBaseline::Alphabetic => y,
    TextBaseline::Top => y - metrics.ascent,
    TextBaseline::Middle => y - 0.5 * (metrics.ascent + metrics.descent),
    TextBaseline::Bottom => y - metrics.descent,
  }
}

pub fn measure_text(data: &Edn) -> Result<Edn, String> {
  let Edn::Map(data) = data else {
    return Err(format!("measure-text expects one map, got {data}"));
  };
  let text = read_string(data, "text")?;
  let size = read_f32(data, "size")?;
  let style = extract_text_style(data)?;
  let font = create_text_font(&style, size)?;
  let (width, _) = font.measure_str(&text, None);
  let (line_height, metrics) = font.metrics();
  let mut result = EdnMapView::default();
  result.insert(tag("width"), Edn::Number(width as f64));
  result.insert(tag("height"), Edn::Number((metrics.descent - metrics.ascent) as f64));
  result.insert(tag("line-height"), Edn::Number(line_height as f64));
  result.insert(tag("ascent"), Edn::Number(metrics.ascent as f64));
  result.insert(tag("descent"), Edn::Number(metrics.descent as f64));
  result.insert(tag("leading"), Edn::Number(metrics.leading as f64));
  result.insert(tag("baseline"), Edn::Number((-metrics.ascent) as f64));
  Ok(Edn::Map(result))
}

fn build_paragraph(layout: &ParagraphLayout, color: Color) -> Paragraph {
  let slant = match layout.style.slant {
    TextSlant::Normal => Slant::Upright,
    TextSlant::Italic => Slant::Italic,
  };
  let font_style = FontStyle::new(Weight::from(layout.style.weight), Width::NORMAL, slant);
  let mut text_style = ParagraphTextStyle::new();
  text_style
    .set_color(color)
    .set_font_style(font_style)
    .set_font_size(layout.size);
  if let Some(family) = &layout.style.family {
    text_style.set_font_families(&[family]);
  }
  if let Some(line_height) = layout.line_height {
    text_style
      .set_height(line_height / layout.size)
      .set_height_override(true);
  }

  let mut paragraph_style = ParagraphStyle::new();
  paragraph_style
    .set_text_style(&text_style)
    .set_text_align(match layout.align {
      TextAlign::Left => SkParagraphAlign::Left,
      TextAlign::Center => SkParagraphAlign::Center,
      TextAlign::Right => SkParagraphAlign::Right,
    })
    .set_text_direction(match layout.direction {
      TextDirection::Ltr => SkTextDirection::LTR,
      TextDirection::Rtl => SkTextDirection::RTL,
    })
    .set_max_lines(layout.max_lines);
  if let Some(ellipsis) = &layout.ellipsis {
    paragraph_style.set_ellipsis(ellipsis);
  }

  let mut fonts = FontCollection::new();
  fonts.set_default_font_manager(FontMgr::new(), None);
  let mut builder = ParagraphBuilder::new(&paragraph_style, fonts);
  builder.add_text(&layout.text);
  let mut paragraph = builder.build();
  paragraph.layout(layout.max_width);
  paragraph
}

pub fn measure_paragraph(data: &Edn) -> Result<Edn, String> {
  let Edn::Map(data) = data else {
    return Err(format!("measure-paragraph expects one map, got {data}"));
  };
  let layout = extract_paragraph_layout(data)?;
  let paragraph = build_paragraph(&layout, Color::BLACK);
  let mut result = EdnMapView::default();
  result.insert(tag("width"), Edn::Number(paragraph.longest_line() as f64));
  result.insert(tag("height"), Edn::Number(paragraph.height() as f64));
  result.insert(tag("line-count"), Edn::Number(paragraph.line_number() as f64));
  result.insert(tag("max-width"), Edn::Number(paragraph.max_width() as f64));
  result.insert(
    tag("min-intrinsic-width"),
    Edn::Number(paragraph.min_intrinsic_width() as f64),
  );
  result.insert(
    tag("max-intrinsic-width"),
    Edn::Number(paragraph.max_intrinsic_width() as f64),
  );
  result.insert(
    tag("alphabetic-baseline"),
    Edn::Number(paragraph.alphabetic_baseline() as f64),
  );
  result.insert(
    tag("ideographic-baseline"),
    Edn::Number(paragraph.ideographic_baseline() as f64),
  );
  Ok(Edn::Map(result))
}

impl ImageCache {
  fn remove(&mut self, key: &str) -> Option<CachedImage> {
    let removed = self.entries.remove(key)?;
    self.bytes = self.bytes.saturating_sub(removed.bytes);
    Some(removed)
  }

  fn fresh_image(&mut self, key: &str, modified: Option<SystemTime>, len: u64, tick: u64) -> Option<Image> {
    let fresh = self
      .entries
      .get(key)
      .is_some_and(|entry| entry.modified == modified && entry.len == len);
    if !fresh {
      self.remove(key);
      return None;
    }
    let entry = self.entries.get_mut(key)?;
    entry.last_used = tick;
    Some(entry.image.clone())
  }

  fn insert_with_limits(&mut self, key: String, entry: CachedImage, max_entries: usize, max_bytes: usize) {
    self.remove(&key);
    if max_entries == 0 || entry.bytes > max_bytes {
      return;
    }
    self.bytes = self.bytes.saturating_add(entry.bytes);
    self.entries.insert(key, entry);
    while self.entries.len() > max_entries || self.bytes > max_bytes {
      let Some(victim) = self
        .entries
        .iter()
        .min_by_key(|(_, cached)| cached.last_used)
        .map(|(key, _)| key.clone())
      else {
        break;
      };
      self.remove(&victim);
    }
  }
}

fn decoded_image_bytes(image: &Image) -> usize {
  (image.width().max(0) as usize)
    .saturating_mul(image.height().max(0) as usize)
    .saturating_mul(4)
}

fn load_image(file_path: &str) -> Result<Option<Image>, String> {
  let metadata = match fs::metadata(file_path) {
    Ok(metadata) => metadata,
    Err(error) => {
      IMAGE_CACHE
        .write()
        .map_err(|_| "image cache lock is poisoned".to_owned())?
        .remove(file_path);
      eprintln!("[Paint Error] failed to load {file_path}: {error}");
      return Ok(None);
    }
  };
  let modified = metadata.modified().ok();
  let len = metadata.len();
  let tick = IMAGE_CACHE_TICK.fetch_add(1, Ordering::Relaxed);
  if let Some(image) = IMAGE_CACHE
    .write()
    .map_err(|_| "image cache lock is poisoned".to_owned())?
    .fresh_image(file_path, modified, len, tick)
  {
    return Ok(Some(image));
  }

  let file_data = fs::read(file_path).map_err(|error| format!("[Paint Error] failed to load {file_path}: {error}"))?;
  let image = Image::from_encoded(Data::new_copy(&file_data))
    .ok_or_else(|| format!("[Paint Error] failed to decode image: {file_path}"))?;
  let bytes = decoded_image_bytes(&image);
  IMAGE_CACHE
    .write()
    .map_err(|_| "image cache lock is poisoned".to_owned())?
    .insert_with_limits(
      file_path.to_owned(),
      CachedImage {
        modified,
        len,
        bytes,
        last_used: tick,
        image: image.clone(),
      },
      IMAGE_CACHE_MAX_ENTRIES,
      IMAGE_CACHE_LIMIT_BYTES,
    );
  Ok(Some(image))
}

fn image_sampling_options(sampling: ImageSampling) -> SamplingOptions {
  match sampling {
    ImageSampling::Nearest => SamplingOptions::default(),
    ImageSampling::Linear => FilterMode::Linear.into(),
    ImageSampling::Cubic => CubicResampler::mitchell().into(),
  }
}

fn resolve_image_rects(
  id: &str,
  file_path: &str,
  image_width: f32,
  image_height: f32,
  destination: Rect,
  crop: Option<Rect>,
  fit: ImageFit,
) -> Result<(Option<Rect>, Rect), String> {
  if image_width <= 0.0 || image_height <= 0.0 {
    return Err(format!(
      "{id}: decoded image has invalid dimensions {image_width}x{image_height}: {file_path}"
    ));
  }
  let full_source = Rect::from_wh(image_width, image_height);
  let source = crop.unwrap_or(full_source);
  let source_width = source.width();
  let source_height = source.height();
  if !source.left.is_finite()
    || !source.top.is_finite()
    || !source.right.is_finite()
    || !source.bottom.is_finite()
    || source.left < 0.0
    || source.top < 0.0
    || source_width <= 0.0
    || source_height <= 0.0
    || source.right > image_width
    || source.bottom > image_height
  {
    return Err(format!(
      "{id}: image :crop {source:?} exceeds decoded bounds {image_width}x{image_height}: {file_path}"
    ));
  }

  let destination_width = destination.width();
  let destination_height = destination.height();
  match fit {
    ImageFit::Fill => Ok((crop, destination)),
    ImageFit::Contain => {
      let scale = (destination_width / source_width).min(destination_height / source_height);
      let width = source_width * scale;
      let height = source_height * scale;
      Ok((
        crop,
        Rect::from_xywh(
          destination.left + (destination_width - width) * 0.5,
          destination.top + (destination_height - height) * 0.5,
          width,
          height,
        ),
      ))
    }
    ImageFit::Cover => {
      let destination_aspect = destination_width / destination_height;
      let source_aspect = source_width / source_height;
      let cover_source = if source_aspect > destination_aspect {
        let width = source_height * destination_aspect;
        Rect::from_xywh(
          source.left + (source_width - width) * 0.5,
          source.top,
          width,
          source_height,
        )
      } else {
        let height = source_width / destination_aspect;
        Rect::from_xywh(
          source.left,
          source.top + (source_height - height) * 0.5,
          source_width,
          height,
        )
      };
      Ok((Some(cover_source), destination))
    }
  }
}

const EFFECT_CACHE_LIMIT: usize = 256;

fn stroke_paint(style: &StrokeStyle) -> Result<Paint, String> {
  let mut paint = Paint::default();
  paint
    .set_anti_alias(true)
    .set_style(PaintStyle::Stroke)
    .set_stroke_width(style.width)
    .set_stroke_cap(style.cap)
    .set_stroke_join(style.join)
    .set_stroke_miter(style.miter_limit);
  apply_paint_source(&mut paint, &style.paint)?;
  if let Some(dash) = &style.dash {
    paint.set_path_effect(dash_effect(dash)?);
  }
  Ok(paint)
}

fn fill_paint(source: &PaintSource) -> Result<Paint, String> {
  let mut paint = Paint::default();
  paint.set_anti_alias(true).set_style(PaintStyle::Fill);
  apply_paint_source(&mut paint, source)?;
  Ok(paint)
}

fn apply_paint_source(paint: &mut Paint, source: &PaintSource) -> Result<(), String> {
  match source {
    PaintSource::Solid(color) => {
      paint.set_color(*color);
    }
    PaintSource::LinearGradient { .. } | PaintSource::RadialGradient { .. } => {
      paint.set_shader(gradient_shader(source)?);
    }
  }
  Ok(())
}

fn gradient_shader(source: &PaintSource) -> Result<Shader, String> {
  let key = format!("{source:?}");
  if let Some(shader) = SHADER_CACHE
    .read()
    .map_err(|_| "gradient shader cache lock is poisoned".to_owned())?
    .get(&key)
  {
    return Ok(shader.clone());
  }

  let shader = match source {
    PaintSource::LinearGradient { from, to, stops } => {
      let colors: Vec<Color4f> = stops.iter().map(|stop| Color4f::from(stop.color)).collect();
      let positions: Vec<f32> = stops.iter().map(|stop| stop.offset).collect();
      let colors = gradient::Colors::new(colors.as_slice(), Some(positions.as_slice()), TileMode::Clamp, None);
      let gradient = gradient::Gradient::new(colors, gradient::Interpolation::default());
      gradient::shaders::linear_gradient(((*from).to_tuple(), (*to).to_tuple()), &gradient, None)
        .ok_or_else(|| "Skia failed to create linear-gradient shader".to_owned())?
    }
    PaintSource::RadialGradient { center, radius, stops } => {
      let colors: Vec<Color4f> = stops.iter().map(|stop| Color4f::from(stop.color)).collect();
      let positions: Vec<f32> = stops.iter().map(|stop| stop.offset).collect();
      let colors = gradient::Colors::new(colors.as_slice(), Some(positions.as_slice()), TileMode::Clamp, None);
      let gradient = gradient::Gradient::new(colors, gradient::Interpolation::default());
      gradient::shaders::radial_gradient(((*center).to_tuple(), *radius), &gradient, None)
        .ok_or_else(|| "Skia failed to create radial-gradient shader".to_owned())?
    }
    PaintSource::Solid(_) => return Err("solid paint does not use a shader".to_owned()),
  };

  let mut cache = SHADER_CACHE
    .write()
    .map_err(|_| "gradient shader cache lock is poisoned".to_owned())?;
  if cache.len() >= EFFECT_CACHE_LIMIT {
    cache.clear();
  }
  cache.insert(key, shader.clone());
  Ok(shader)
}

fn dash_effect(dash: &DashPattern) -> Result<PathEffect, String> {
  let key = format!("{:?}:{:?}", dash.intervals, dash.offset);
  if let Some(effect) = DASH_EFFECT_CACHE
    .read()
    .map_err(|_| "dash effect cache lock is poisoned".to_owned())?
    .get(&key)
  {
    return Ok(effect.clone());
  }
  let effect = PathEffect::dash(&dash.intervals, dash.offset)
    .ok_or_else(|| "Skia failed to create dash path effect".to_owned())?;
  let mut cache = DASH_EFFECT_CACHE
    .write()
    .map_err(|_| "dash effect cache lock is poisoned".to_owned())?;
  if cache.len() >= EFFECT_CACHE_LIMIT {
    cache.clear();
  }
  cache.insert(key, effect.clone());
  Ok(effect)
}

fn validate_surface_size(width: i32, height: i32, context: &str) -> Result<usize, String> {
  if !(1..=MAX_OFFSCREEN_DIMENSION).contains(&width) || !(1..=MAX_OFFSCREEN_DIMENSION).contains(&height) {
    return Err(format!(
      "{context} dimensions must be integers between 1 and {MAX_OFFSCREEN_DIMENSION}, got {width}x{height}"
    ));
  }
  let pixels = width as usize * height as usize;
  if pixels > MAX_OFFSCREEN_PIXELS {
    return Err(format!(
      "{context} exceeds the {MAX_OFFSCREEN_PIXELS}-pixel limit: {width}x{height}"
    ));
  }
  Ok(pixels)
}

fn read_surface_dimension(tree: &EdnMapView, key: &str, context: &str) -> Result<i32, String> {
  match tree.get(&tag(key)) {
    Some(Edn::Number(value))
      if value.is_finite() && value.fract() == 0.0 && *value >= 1.0 && *value <= MAX_OFFSCREEN_DIMENSION as f64 =>
    {
      Ok(*value as i32)
    }
    Some(value) => Err(format!(
      "{context} :{key} must be an integer between 1 and {MAX_OFFSCREEN_DIMENSION}, got {value}"
    )),
    None => Err(format!("{context} requires :{key}")),
  }
}

fn make_raster_surface(width: i32, height: i32) -> Result<Surface, String> {
  validate_surface_size(width, height, "offscreen surface")?;
  let info = ImageInfo::new(
    (width, height),
    ColorType::RGBA8888,
    AlphaType::Premul,
    ColorSpace::new_srgb(),
  );
  surfaces::raster(&info, None, None)
    .ok_or_else(|| format!("Skia failed to allocate a {width}x{height} RGBA8888 sRGB raster surface"))
}

fn render_offscreen_shape(width: i32, height: i32, background: Color, shape: &Shape) -> Result<Image, String> {
  let mut surface = make_raster_surface(width, height)?;
  let canvas = surface.canvas();
  canvas.clear(background);
  canvas.reset_matrix();
  draw_shape_with_mode(canvas, shape, &Transform::identity(), RenderMode::Offscreen, &[])?;
  Ok(surface.image_snapshot())
}

pub fn render_to_png(data: &Edn) -> Result<(), String> {
  let Edn::Map(options) = data else {
    return Err(format!("render-to-png expects one options map, got {data}"));
  };
  let width = read_surface_dimension(options, "width", "render-to-png")?;
  let height = read_surface_dimension(options, "height", "render-to-png")?;
  validate_surface_size(width, height, "render-to-png")?;
  let path = read_string(options, "path")?;
  if path.is_empty() {
    return Err("render-to-png :path must not be empty".to_owned());
  }
  let scene = options
    .get(&tag("scene"))
    .or_else(|| options.get(&tag("shape")))
    .ok_or_else(|| "render-to-png requires :scene (or the :shape alias)".to_owned())?;
  let background = match options.get(&tag("background")) {
    Some(color) => extract_color(color)?,
    None => Color::from_argb(0, 0, 0, 0),
  };
  let shape = extract_shape(scene)?;
  let image = render_offscreen_shape(width, height, background, &shape)?;
  let png = image
    .encode(None, EncodedImageFormat::PNG, 100)
    .ok_or_else(|| "Skia PNG encoding is unavailable".to_owned())?;
  fs::write(&path, png.as_bytes()).map_err(|error| format!("failed writing offscreen PNG {path}: {error}"))
}

fn shape_contains_interactive(shape: &Shape) -> bool {
  match shape {
    Shape::TouchArea { .. } | Shape::KeyListener { .. } | Shape::FocusArea { .. } => true,
    Shape::Group { children, .. }
    | Shape::CachedGroup { children, .. }
    | Shape::Translate { children, .. }
    | Shape::Rotate { children, .. }
    | Shape::Scale { children, .. }
    | Shape::ClipRect { children, .. }
    | Shape::ClipRoundedRect { children, .. }
    | Shape::Opacity { children, .. }
    | Shape::Blend { children, .. } => children.iter().any(shape_contains_interactive),
    _ => false,
  }
}

fn render_cached_subtree(
  cache_key: &str,
  revision: i32,
  width: i32,
  height: i32,
  children: &[Shape],
) -> Result<Image, String> {
  if cache_key.is_empty() {
    return Err("cached-group :cache-key must not be empty".to_owned());
  }
  if children.iter().any(shape_contains_interactive) {
    return Err(format!(
      "cached-group {cache_key:?} cannot contain touch-area, key-listener, or focus-area nodes"
    ));
  }
  let pixels = validate_surface_size(width, height, "cached-group")?;
  let bytes = pixels * 4;
  if bytes > SUBTREE_CACHE_LIMIT_BYTES {
    return Err(format!(
      "cached-group {cache_key:?} requires {bytes} bytes, above the {SUBTREE_CACHE_LIMIT_BYTES}-byte cache limit"
    ));
  }
  let tick = SUBTREE_CACHE_TICK.fetch_add(1, Ordering::Relaxed);
  {
    let mut cache = SUBTREE_CACHE
      .write()
      .map_err(|_| "static-subtree cache lock is poisoned".to_owned())?;
    if let Some(entry) = cache.entries.get_mut(cache_key) {
      if entry.revision == revision && entry.width == width && entry.height == height {
        entry.last_used = tick;
        return Ok(entry.image.clone());
      }
    }
  }

  let mut surface = make_raster_surface(width, height)?;
  let canvas = surface.canvas();
  canvas.clear(Color::from_argb(0, 0, 0, 0));
  for child in children {
    draw_shape_with_mode(canvas, child, &Transform::identity(), RenderMode::Offscreen, &[])?;
  }
  let image = surface.image_snapshot();

  let mut cache = SUBTREE_CACHE
    .write()
    .map_err(|_| "static-subtree cache lock is poisoned".to_owned())?;
  if let Some(previous) = cache.entries.remove(cache_key) {
    cache.bytes = cache.bytes.saturating_sub(previous.bytes);
  }
  while !cache.entries.is_empty()
    && (cache.entries.len() >= SUBTREE_CACHE_MAX_ENTRIES || cache.bytes + bytes > SUBTREE_CACHE_LIMIT_BYTES)
  {
    let oldest_key = cache
      .entries
      .iter()
      .min_by_key(|(_, entry)| entry.last_used)
      .map(|(key, _)| key.clone())
      .expect("non-empty cache has an oldest entry");
    if let Some(removed) = cache.entries.remove(&oldest_key) {
      cache.bytes = cache.bytes.saturating_sub(removed.bytes);
    }
  }
  cache.bytes += bytes;
  cache.entries.insert(
    cache_key.to_owned(),
    CachedSubtree {
      revision,
      width,
      height,
      bytes,
      last_used: tick,
      image: image.clone(),
    },
  );
  Ok(image)
}

pub fn draw_page(
  canvas: &skia_safe::canvas::Canvas,
  base_messages: Vec<(Box<str>, Edn)>,
  eager_render: bool,
) -> Result<(), String> {
  let mut messages = base_messages;
  if eager_render {
    // render previous piece of data, during resizing
    if messages.is_empty() {
      let m = PREV_MESSAGES.read().unwrap();
      messages = m.to_owned();
    }
  }
  if !messages.is_empty() {
    // tracking
    let mut m = PREV_MESSAGES.write().unwrap();
    *m = messages.to_owned();

    let mut shown_shape = false;
    for (call_op, arg) in messages {
      match (&*call_op, arg) {
        ("render-canvas!", tree) => {
          shown_shape = true;
          let shape = extract_shape(&tree)?;
          draw_shape(canvas, &shape, &Transform::identity())?;
        }
        ("reset-canvas!", tree) => {
          reset_page(canvas, extract_color(&tree)?)?;
        }
        _ => return Err(format!("unknown paint operation: {call_op}")),
      }
    }
    if shown_shape {
      // draw_cost(canvas, cost)
    }
  }
  Ok(())
}

// fn draw_cost(canvas: &mut skia_safe::canvas::Canvas, cost: f64) -> Result<(), String> {
//   let text = format!("{}ms", cost);
//   let font = Font::new(Typeface::default(), Some(14.0));
//   let text_blob = TextBlob::new(text, &font).unwrap();

//   let mut paint = Paint::default();
//   paint.set_anti_alias(true);
//   paint.set_style(PaintStyle::Fill).set_color(Color::WHITE);

//   canvas.draw_text_blob(text_blob, (10, 190), &paint);

//   Ok(())
// }

fn draw_shape(canvas: &skia_safe::canvas::Canvas, tree: &Shape, tr: &Transform) -> Result<(), String> {
  draw_shape_with_mode(canvas, tree, tr, RenderMode::Interactive, &[])
}

fn draw_shape_with_mode(
  canvas: &skia_safe::canvas::Canvas,
  tree: &Shape,
  tr: &Transform,
  render_mode: RenderMode,
  clips: &[ClipRegion],
) -> Result<(), String> {
  match tree {
    Shape::Rectangle {
      position,
      width,
      height,
      line_style,
      fill_style,
    } => {
      let rect_path = Rect::from_xywh(position.x, position.y, *width, *height);

      // canvas.set_transform(tr);

      if let Some(style) = line_style {
        canvas.draw_rect(rect_path, &stroke_paint(style)?);
      }
      if let Some(source) = fill_style {
        canvas.draw_rect(rect_path, &fill_paint(source)?);
      }
    }
    Shape::RoundedRectangle {
      position,
      width,
      height,
      radius_x,
      radius_y,
      line_style,
      fill_style,
    } => {
      let rect = Rect::from_xywh(position.x, position.y, *width, *height);
      let rounded = RRect::new_rect_xy(rect, *radius_x, *radius_y);
      if let Some(style) = line_style {
        canvas.draw_rrect(rounded, &stroke_paint(style)?);
      }
      if let Some(source) = fill_style {
        canvas.draw_rrect(rounded, &fill_paint(source)?);
      }
    }
    Shape::Circle {
      position,
      radius,
      line_style,
      fill_style,
    } => {
      // canvas.set_transform(tr);

      if let Some(style) = line_style {
        canvas.draw_circle((position.x, position.y), *radius, &stroke_paint(style)?);
      }
      if let Some(source) = fill_style {
        canvas.draw_circle((position.x, position.y), *radius, &fill_paint(source)?);
      }
    }
    Shape::Ellipse {
      position,
      radius_x,
      radius_y,
      line_style,
      fill_style,
    } => {
      let oval = Rect::from_xywh(
        position.x - radius_x,
        position.y - radius_y,
        2.0 * radius_x,
        2.0 * radius_y,
      );
      if let Some(style) = line_style {
        canvas.draw_oval(oval, &stroke_paint(style)?);
      }
      if let Some(source) = fill_style {
        canvas.draw_oval(oval, &fill_paint(source)?);
      }
    }
    Shape::Arc {
      position,
      radius_x,
      radius_y,
      start_angle,
      sweep_angle,
      use_center,
      line_style,
      fill_style,
    } => {
      let oval = Rect::from_xywh(
        position.x - radius_x,
        position.y - radius_y,
        2.0 * radius_x,
        2.0 * radius_y,
      );
      if let Some(style) = line_style {
        canvas.draw_arc(oval, *start_angle, *sweep_angle, *use_center, &stroke_paint(style)?);
      }
      if let Some(source) = fill_style {
        canvas.draw_arc(oval, *start_angle, *sweep_angle, *use_center, &fill_paint(source)?);
      }
    }
    Shape::Group { position, children } => {
      canvas.save();
      let pos = Vector2D::new(position.x, position.y);
      canvas.translate((pos.x, pos.y));
      for child in children {
        let t1 = Transform::identity().then_translate(pos);
        draw_shape_with_mode(canvas, child, &t1.then(tr), render_mode, clips)?;
      }
      canvas.restore();
    }
    Shape::CachedGroup {
      cache_key,
      revision,
      position,
      width,
      height,
      children,
    } => {
      let image = render_cached_subtree(cache_key, *revision, *width, *height, children)?;
      canvas.draw_image(image, (position.x, position.y), None);
    }
    Shape::Text {
      text,
      position,
      size,
      color,
      align,
      style,
    } => {
      // canvas.set_transform(tr);
      // https://github.com/jrmuizel/raqote/issues/179
      // for now we have to by pass bug in text rendering
      // canvas.set_transform(&Transform::identity());

      let font = create_text_font(style, *size)?;
      let text_blob = TextBlob::new(text, &font).ok_or_else(|| "failed to create text blob".to_owned())?;

      let mut paint = Paint::default();
      paint.set_anti_alias(true);
      paint.set_style(PaintStyle::Fill).set_color(*color);

      let x_offset = text_x_offset(align, text_blob.bounds().width());
      let y = text_baseline_y(position.y, style, &font);
      canvas.draw_text_blob(text_blob, (position.x + x_offset, y), &paint);
    }
    Shape::Paragraph {
      position,
      color,
      layout,
    } => {
      let paragraph = build_paragraph(layout, *color);
      paragraph.paint(canvas, (position.x, position.y));
    }
    Shape::Polyline {
      position,
      stops,
      line_style,
      skip_first,
    } => {
      let mut path = PathBuilder::new();
      // canvas.set_transform(tr);

      if *skip_first && !stops.is_empty() {
        path.move_to((position.x + stops[0].x, position.y + stops[0].y));
      } else {
        path.move_to((position.x, position.y));
      }
      for stop in stops {
        path.line_to((position.x + stop.x, position.y + stop.y));
      }
      path.close();
      let path = path.detach();

      canvas.draw_path(&path, &stroke_paint(line_style)?);
    }
    Shape::Image {
      id,
      file_path,
      x,
      y,
      w,
      h,
      crop,
      fit,
      sampling,
    } => {
      let paint = Paint::default();
      let Some(image) = load_image(file_path)? else {
        return Ok(());
      };
      let (source, destination) = resolve_image_rects(
        id,
        file_path,
        image.width() as f32,
        image.height() as f32,
        Rect::from_xywh(*x, *y, *w, *h),
        *crop,
        *fit,
      )?;
      match source.as_ref() {
        Some(source) => {
          canvas.draw_image_rect_with_sampling_options(
            image,
            Some((source, SrcRectConstraint::Strict)),
            destination,
            image_sampling_options(*sampling),
            &paint,
          );
        }
        None => {
          canvas.draw_image_rect_with_sampling_options(
            image,
            None,
            destination,
            image_sampling_options(*sampling),
            &paint,
          );
        }
      }
    }
    Shape::TouchArea {
      id,
      position,
      target,
      cursor,
      accessibility,
      line_style,
      fill_style,
      area,
    } => {
      match area {
        TouchAreaShape::Circle(r) => {
          // canvas.set_transform(tr);

          if let Some(style) = line_style {
            canvas.draw_circle((position.x, position.y), *r, &stroke_paint(style)?);
          }
          if let Some(source) = fill_style {
            canvas.draw_circle((position.x, position.y), *r, &fill_paint(source)?);
          }
        }
        TouchAreaShape::Rect(dx, dy) => {
          let rect_path = Rect::from_xywh(
            position.x - *dx,
            position.y - *dy,
            2. * dx.to_owned(),
            2. * dy.to_owned(),
          );

          // canvas.set_transform(tr);

          if let Some(style) = line_style {
            canvas.draw_rect(rect_path, &stroke_paint(style)?);
          }
          if let Some(source) = fill_style {
            canvas.draw_rect(rect_path, &fill_paint(source)?);
          }
        }
      }
      if render_mode == RenderMode::Interactive {
        touches::add_touch_area(
          id.to_owned(),
          position.to_owned(),
          area.to_owned(),
          target.to_owned(),
          *cursor,
          tr,
          clips,
        );
        if let Some(accessibility) = accessibility {
          crate::accessibility::register(accessibility, target, *position, area.clone(), tr, None)?;
        }
      }
    }
    Shape::KeyListener {
      key,
      modifiers,
      focus_id,
      target,
    } => {
      if render_mode == RenderMode::Interactive {
        key_listener::add_key_listener(
          key.to_owned(),
          modifiers.to_owned(),
          focus_id.to_owned(),
          target.to_owned(),
        );
      }
    }
    Shape::FocusArea {
      id,
      target,
      position,
      area,
      tab_index,
      text_input,
      accessibility,
      line_style,
      fill_style,
    } => {
      match area {
        TouchAreaShape::Circle(radius) => {
          if let Some(style) = line_style {
            canvas.draw_circle((position.x, position.y), *radius, &stroke_paint(style)?);
          }
          if let Some(source) = fill_style {
            canvas.draw_circle((position.x, position.y), *radius, &fill_paint(source)?);
          }
        }
        TouchAreaShape::Rect(dx, dy) => {
          let rect = Rect::from_xywh(position.x - dx, position.y - dy, 2.0 * dx, 2.0 * dy);
          if let Some(style) = line_style {
            canvas.draw_rect(rect, &stroke_paint(style)?);
          }
          if let Some(source) = fill_style {
            canvas.draw_rect(rect, &fill_paint(source)?);
          }
        }
      }
      if render_mode == RenderMode::Interactive {
        focus::register_focus_area(focus::FocusArea {
          id: id.to_owned(),
          target: target.to_owned(),
          position: position.to_owned(),
          area: area.to_owned(),
          transform: tr.to_owned(),
          clips: clips.to_vec(),
          tab_index: *tab_index,
          text_input: *text_input,
          order: 0,
        })?;
        if let Some(accessibility) = accessibility {
          crate::accessibility::register(accessibility, target, *position, area.clone(), tr, Some(id))?;
        }
      }
    }
    Shape::PaintOps {
      path: ops_path,
      line_style,
      fill_style,
      position,
    } => {
      let mut path = PathBuilder::new();
      let x0 = position.x;
      let y0 = position.y;
      path.move_to((x0, y0));
      // canvas.set_transform(tr);

      for p in ops_path {
        match p {
          PaintPathTo::Move(a) => {
            path.move_to((x0 + a.x, y0 + a.y));
          }
          PaintPathTo::Line(a) => {
            path.line_to((x0 + a.x, y0 + a.y));
          }
          PaintPathTo::QuadraticBezier(a, b) => {
            path.quad_to((x0 + a.x, y0 + a.y), (x0 + b.x, y0 + b.y));
          }
          PaintPathTo::CubicBezier(a, b, c) => {
            path.cubic_to((x0 + a.x, y0 + a.y), (x0 + b.x, y0 + b.y), (x0 + c.x, y0 + c.y));
          }
          PaintPathTo::Close => {
            path.close();
          }
        }
      }
      if fill_style.is_some() {
        path.close();
      }
      let path = path.detach();

      if let Some(style) = line_style {
        canvas.draw_path(&path, &stroke_paint(style)?);
      }

      if let Some(source) = fill_style {
        canvas.draw_path(&path, &fill_paint(source)?);
      }
    }
    Shape::Scale { factor, children } => {
      canvas.save();
      canvas.scale((*factor, *factor));
      let t1 = Transform::identity().then_scale(factor.to_owned(), factor.to_owned());
      for child in children {
        draw_shape_with_mode(canvas, child, &t1.then(tr), render_mode, clips)?;
      }
      canvas.restore();
    }
    Shape::Rotate { radius, children } => {
      canvas.save();
      canvas.rotate(radius.to_degrees(), None);
      let t1 = Transform::identity().then_rotate(Angle {
        radians: radius.to_owned(),
      });
      for child in children {
        draw_shape_with_mode(canvas, child, &t1.then(tr), render_mode, clips)?;
      }
      canvas.restore();
    }
    Shape::Translate { x, y, children } => {
      canvas.save();
      canvas.translate((*x, *y));
      let v = Vector2D::new(x.to_owned(), y.to_owned());
      let t1 = Transform::identity().then_translate(v);
      for child in children {
        draw_shape_with_mode(canvas, child, &t1.then(tr), render_mode, clips)?;
      }
      canvas.restore();
    }
    Shape::ClipRect {
      position,
      width,
      height,
      children,
    } => {
      canvas.save();
      canvas.clip_rect(Rect::from_xywh(position.x, position.y, *width, *height), None, true);
      let mut child_clips = clips.to_vec();
      child_clips.push(ClipRegion {
        shape: ClipShape::Rect {
          position: *position,
          width: *width,
          height: *height,
        },
        transform: *tr,
      });
      for child in children {
        draw_shape_with_mode(canvas, child, tr, render_mode, &child_clips)?;
      }
      canvas.restore();
    }
    Shape::ClipRoundedRect {
      position,
      width,
      height,
      radius_x,
      radius_y,
      children,
    } => {
      canvas.save();
      let rect = Rect::from_xywh(position.x, position.y, *width, *height);
      canvas.clip_rrect(RRect::new_rect_xy(rect, *radius_x, *radius_y), None, true);
      let mut child_clips = clips.to_vec();
      child_clips.push(ClipRegion {
        shape: ClipShape::RoundedRect {
          position: *position,
          width: *width,
          height: *height,
          radius_x: *radius_x,
          radius_y: *radius_y,
        },
        transform: *tr,
      });
      for child in children {
        draw_shape_with_mode(canvas, child, tr, render_mode, &child_clips)?;
      }
      canvas.restore();
    }
    Shape::Opacity { alpha, children } => {
      canvas.save_layer_alpha_f(None, alpha.clamp(0.0, 1.0));
      for child in children {
        draw_shape_with_mode(canvas, child, tr, render_mode, clips)?;
      }
      canvas.restore();
    }
    Shape::Blend { mode, children } => {
      let mut paint = Paint::default();
      paint.set_blend_mode(*mode);
      canvas.save_layer(&SaveLayerRec::default().paint(&paint));
      for child in children {
        draw_shape_with_mode(canvas, child, tr, render_mode, clips)?;
      }
      canvas.restore();
    }
  }
  Ok(())
}

fn extract_shape(tree: &Edn) -> Result<Shape, String> {
  extract_shape_at(tree, "$").map_err(|diagnostics| diagnostics.into_messages().join("\n"))
}

pub fn validate_scene(tree: &Edn) -> Vec<String> {
  match extract_shape_at(tree, "$") {
    Ok(_) => vec![],
    Err(diagnostics) => diagnostics.into_messages(),
  }
}

fn extract_shape_at(tree: &Edn, path: &str) -> Result<Shape, SceneDiagnostics> {
  let result: Result<Shape, SceneDiagnostics> = (|| match tree {
    Edn::Map(m) => match m.get(&tag("type")) {
      Some(Edn::Tag(name)) => match name.ref_str() {
        "rectangle" | "rect" => Ok(Shape::Rectangle {
          position: read_position(m, "position")?,
          width: read_f32(m, "width")?,
          height: read_f32(m, "height")?,
          fill_style: extract_fill_style(m)?,
          line_style: extract_stroke_style(m)?,
        }),
        "rounded-rectangle" | "rounded-rect" => {
          let radius = read_optional_f32(m, "radius")?;
          let radius_x = read_optional_f32(m, "radius-x")?
            .or(radius)
            .ok_or_else(|| "rounded-rectangle requires :radius or :radius-x".to_owned())?;
          let radius_y = read_optional_f32(m, "radius-y")?.unwrap_or(radius_x);
          validate_non_negative("rounded-rectangle radius-x", radius_x)?;
          validate_non_negative("rounded-rectangle radius-y", radius_y)?;
          Ok(Shape::RoundedRectangle {
            position: read_position(m, "position")?,
            width: read_non_negative_f32(m, "width")?,
            height: read_non_negative_f32(m, "height")?,
            radius_x,
            radius_y,
            fill_style: extract_fill_style(m)?,
            line_style: extract_stroke_style(m)?,
          })
        }
        "circle" => Ok(Shape::Circle {
          position: read_position(m, "position")?,
          radius: read_f32(m, "radius")?,
          fill_style: extract_fill_style(m)?,
          line_style: extract_stroke_style(m)?,
        }),
        "ellipse" => Ok(Shape::Ellipse {
          position: read_position(m, "position")?,
          radius_x: read_non_negative_f32(m, "radius-x")?,
          radius_y: read_non_negative_f32(m, "radius-y")?,
          fill_style: extract_fill_style(m)?,
          line_style: extract_stroke_style(m)?,
        }),
        "arc" => Ok(Shape::Arc {
          position: read_position(m, "position")?,
          radius_x: read_non_negative_f32(m, "radius-x")?,
          radius_y: read_non_negative_f32(m, "radius-y")?,
          start_angle: read_f32(m, "start-angle")?,
          sweep_angle: read_f32(m, "sweep-angle")?,
          use_center: read_bool(m, "use-center?")?,
          fill_style: extract_fill_style(m)?,
          line_style: extract_stroke_style(m)?,
        }),
        "group" => {
          let c = m.get(&tag("children"));
          let children = extract_children(c, path)?;

          Ok(Shape::Group {
            position: read_position(m, "position")?,
            children,
          })
        }
        "cached-group" | "static-group" => {
          let width = read_surface_dimension(m, "width", "cached-group")?;
          let height = read_surface_dimension(m, "height", "cached-group")?;
          validate_surface_size(width, height, "cached-group")?;
          Ok(Shape::CachedGroup {
            cache_key: read_string(m, "cache-key")?,
            revision: read_optional_i32(m, "revision")?.unwrap_or(0),
            position: read_position(m, "position")?,
            width,
            height,
            children: extract_children(m.get(&tag("children")), path)?,
          })
        }
        // "arc" => Ok(Shape::Arc {
        //   position: read_position(m, "position")?,
        //   radius: read_f32(m, "radius")?,
        //   from_angle: read_f32(m, "from-angle")?,
        //   to_angle: read_f32(m, "to-angle")?,
        //   negative: read_bool(m, "negative")?,
        //   style: extract_style(m)?,
        // }),
        "ops" => Ok(Shape::PaintOps {
          position: read_position(m, "position")?,
          path: extract_paint_path(m.get(&tag("path")).unwrap_or(&Edn::Nil))?,
          fill_style: extract_fill_style(m)?,
          line_style: extract_stroke_style(m)?,
        }),
        "text" => Ok(Shape::Text {
          text: read_string(m, "text")?,
          position: read_position(m, "position")?,
          size: read_f32(m, "size")?,
          color: read_color(m, "color")?,
          align: read_text_align(m, "align")?,
          style: extract_text_style(m)?,
        }),
        "paragraph" | "text-block" => Ok(Shape::Paragraph {
          position: read_position(m, "position")?,
          color: read_color(m, "color")?,
          layout: extract_paragraph_layout(m)?,
        }),
        "polyline" => Ok(Shape::Polyline {
          position: read_position(m, "position")?,
          skip_first: read_bool(m, "skip-first?")?,
          stops: read_points(m, "stops")?,
          line_style: extract_polyline_stroke_style(m)?,
        }),
        "touch-area" => Ok(Shape::TouchArea {
          id: path.to_owned(),
          target: extract_event_target(m),
          position: read_position(m, "position")?,
          area: extract_touch_area_shape(m)?,
          cursor: read_optional_cursor_icon(m)?,
          accessibility: extract_accessibility(m)?,
          fill_style: extract_fill_style(m)?,
          line_style: extract_stroke_style(m)?,
        }),
        "key-listener" => Ok(Shape::KeyListener {
          key: read_string(m, "key")?,
          modifiers: extract_shortcut_modifiers(m)?,
          focus_id: read_optional_string_field(m, "focus-id")?,
          target: extract_event_target(m),
        }),
        "focus-area" | "focusable" => Ok(Shape::FocusArea {
          id: read_string(m, "focus-id")?,
          target: extract_event_target(m),
          position: read_position(m, "position")?,
          area: extract_touch_area_shape(m)?,
          tab_index: read_optional_i32(m, "tab-index")?.unwrap_or(0),
          text_input: read_bool(m, "text-input?")?,
          accessibility: extract_accessibility(m)?,
          fill_style: extract_fill_style(m)?,
          line_style: extract_stroke_style(m)?,
        }),
        "rotate" => {
          let c = m.get(&tag("children"));
          let children = extract_children(c, path)?;

          Ok(Shape::Rotate {
            radius: read_f32(m, "radius")?,
            children,
          })
        }
        "scale" => {
          let c = m.get(&tag("children"));
          let children = extract_children(c, path)?;

          Ok(Shape::Scale {
            factor: read_f32(m, "factor")?,
            children,
          })
        }
        "translate" => {
          let c = m.get(&tag("children"));
          let children = extract_children(c, path)?;

          Ok(Shape::Translate {
            x: read_f32(m, "x")?,
            y: read_f32(m, "y")?,
            children,
          })
        }
        "clip-rect" => Ok(Shape::ClipRect {
          position: read_position(m, "position")?,
          width: read_non_negative_f32(m, "width")?,
          height: read_non_negative_f32(m, "height")?,
          children: extract_children(m.get(&tag("children")), path)?,
        }),
        "clip-rounded-rect" | "clip-rounded-rectangle" => {
          let radius = read_optional_f32(m, "radius")?;
          let radius_x = read_optional_f32(m, "radius-x")?
            .or(radius)
            .ok_or_else(|| "clip-rounded-rect requires :radius or :radius-x".to_owned())?;
          let radius_y = read_optional_f32(m, "radius-y")?.unwrap_or(radius_x);
          validate_non_negative("clip-rounded-rect radius-x", radius_x)?;
          validate_non_negative("clip-rounded-rect radius-y", radius_y)?;
          Ok(Shape::ClipRoundedRect {
            position: read_position(m, "position")?,
            width: read_non_negative_f32(m, "width")?,
            height: read_non_negative_f32(m, "height")?,
            radius_x,
            radius_y,
            children: extract_children(m.get(&tag("children")), path)?,
          })
        }
        "opacity" => {
          let alpha = read_f32(m, "alpha")?;
          if !(0.0..=1.0).contains(&alpha) {
            return Err(format!("opacity alpha must be between 0 and 1, got {alpha}").into());
          }
          Ok(Shape::Opacity {
            alpha,
            children: extract_children(m.get(&tag("children")), path)?,
          })
        }
        "blend" => Ok(Shape::Blend {
          mode: read_blend_mode(m, "mode")?,
          children: extract_children(m.get(&tag("children")), path)?,
        }),
        "image" => {
          let crop = match m.get(&tag("crop")) {
            Some(Edn::Map(m)) => Some(Rect::from_xywh(
              read_non_negative_f32(m, "x")?,
              read_non_negative_f32(m, "y")?,
              read_positive_f32(m, "w")?,
              read_positive_f32(m, "h")?,
            )),
            None | Some(Edn::Nil) => None,
            Some(value) => return Err(format!("image :crop must be a map or nil, got {value}").into()),
          };
          Ok(Shape::Image {
            id: path.to_owned(),
            file_path: read_string(m, "file-path")?,
            x: read_finite_f32(m, "x")?,
            y: read_finite_f32(m, "y")?,
            w: read_positive_f32(m, "w")?,
            h: read_positive_f32(m, "h")?,
            crop,
            fit: read_image_fit(m)?,
            sampling: read_image_sampling(m)?,
          })
        }
        _ => Err(format!("unknown kind: {name}").into()),
      },
      Some(value) => Err(format!("unknown kind value, {value}").into()),
      None => Err(String::from("nil type").into()),
    },
    Edn::Nil => Ok(Shape::Group {
      position: Vector2D::new(0.0, 0.0),
      children: vec![],
    }),
    _ => Err(format!("expected a map, got {tree}").into()),
  })();
  result.map_err(|diagnostics| diagnostics.with_default_path(path))
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

fn validate_finite(name: &str, value: f32) -> Result<f32, String> {
  if value.is_finite() {
    Ok(value)
  } else {
    Err(format!("{name} must be a finite number, got {value}"))
  }
}

fn read_non_negative_f32(tree: &EdnMapView, key: &str) -> Result<f32, String> {
  validate_non_negative(key, read_f32(tree, key)?)
}

fn read_positive_f32(tree: &EdnMapView, key: &str) -> Result<f32, String> {
  validate_positive(key, read_f32(tree, key)?)
}

fn read_finite_f32(tree: &EdnMapView, key: &str) -> Result<f32, String> {
  validate_finite(key, read_f32(tree, key)?)
}

fn read_image_fit(tree: &EdnMapView) -> Result<ImageFit, String> {
  match tree.get(&tag("fit")) {
    None | Some(Edn::Nil) => Ok(ImageFit::Fill),
    Some(Edn::Tag(value)) => match value.ref_str() {
      "fill" => Ok(ImageFit::Fill),
      "contain" => Ok(ImageFit::Contain),
      "cover" => Ok(ImageFit::Cover),
      _ => Err(format!("unsupported image fit: {value}")),
    },
    Some(value) => Err(format!("image :fit must be a tag or nil, got {value}")),
  }
}

fn read_image_sampling(tree: &EdnMapView) -> Result<ImageSampling, String> {
  match tree.get(&tag("sampling")) {
    None | Some(Edn::Nil) => Ok(ImageSampling::Nearest),
    Some(Edn::Tag(value)) => match value.ref_str() {
      "nearest" => Ok(ImageSampling::Nearest),
      "linear" => Ok(ImageSampling::Linear),
      "cubic" => Ok(ImageSampling::Cubic),
      _ => Err(format!("unsupported image sampling: {value}")),
    },
    Some(value) => Err(format!("image :sampling must be a tag or nil, got {value}")),
  }
}

fn extract_children(children: Option<&Edn>, parent_path: &str) -> Result<Vec<Shape>, SceneDiagnostics> {
  match children {
    Some(Edn::List(EdnListView(xs))) => {
      let mut shapes = Vec::with_capacity(xs.len());
      let mut diagnostics = vec![];
      for (index, child) in xs.iter().enumerate() {
        let child_path = format!("{parent_path}.children[{index}]");
        match extract_shape_at(child, &child_path) {
          Ok(shape) => shapes.push(shape),
          Err(child_diagnostics) => diagnostics.extend(child_diagnostics.0),
        }
      }
      if diagnostics.is_empty() {
        Ok(shapes)
      } else {
        Err(SceneDiagnostics(diagnostics))
      }
    }
    Some(value) => Err(SceneDiagnostics::at(
      &format!("{parent_path}.children"),
      format!("expected a list, got {value}"),
    )),
    None => Ok(vec![]),
  }
}

fn extract_paint_path(data: &Edn) -> Result<Vec<PaintPathTo>, String> {
  if let Edn::List(EdnListView(xs)) = data {
    let mut ys = vec![];
    for x in xs {
      match x {
        Edn::List(EdnListView(zs)) => ys.push(extract_paint_op(zs)?),
        _ => return Err(format!("expected single op in list, for {}", x)),
      }
    }
    Ok(ys)
  } else {
    Err(String::from("expected ops in list"))
  }
}

fn extract_paint_op(xs: &[Edn]) -> Result<PaintPathTo, String> {
  if !xs.is_empty() {
    let op: &str = match &xs[0] {
      Edn::Tag(s) => s.ref_str(),
      Edn::Str(s) => s,
      _ => return Err(format!("unknown paint op value: {}", xs[0])),
    };
    match op {
      "move-to" => match xs.get(1) {
        Some(v) => match extract_position(v) {
          Ok(p) => Ok(PaintPathTo::Move(p)),
          Err(e) => Err(format!("failed move-to position, {}", e)),
        },
        None => Err(String::from("missing line-to position")),
      },
      "line-to" => match xs.get(1) {
        Some(v) => match extract_position(v) {
          Ok(p) => Ok(PaintPathTo::Line(p)),
          Err(e) => Err(format!("failed line-to position, {}", e)),
        },
        None => Err(String::from("missing line-to position")),
      },
      "quadratic-bezier-to" | "bezier2-to" => match (xs.get(1), xs.get(2)) {
        (Some(v1), Some(v2)) => match (extract_position(v1), extract_position(v2)) {
          (Ok(p1), Ok(p2)) => Ok(PaintPathTo::QuadraticBezier(p1, p2)),
          (a, b) => Err(format!("failed quadratic points, {:?} {:?}", a, b)),
        },
        (a, b) => Err(format!("missing quadratic points {:?} {:?}", a, b)),
      },
      "cubic-bezier-to" | "bezier3-to" => match (xs.get(1), xs.get(2), xs.get(3)) {
        (Some(v1), Some(v2), Some(v3)) => match (extract_position(v1), extract_position(v2), extract_position(v3)) {
          (Ok(p1), Ok(p2), Ok(p3)) => Ok(PaintPathTo::CubicBezier(p1, p2, p3)),
          (a, b, c) => Err(format!("failed quadratic points, {:?} {:?} {:?}", a, b, c)),
        },
        (a, b, c) => Err(format!("missing quadratic points {:?} {:?} {:?}", a, b, c)),
      },
      "close" | "close-path" => {
        if xs.len() == 1 {
          Ok(PaintPathTo::Close)
        } else {
          Err(format!("close-path does not accept arguments: {xs:?}"))
        }
      }
      _ => Err(format!("unknown paint op: {}", op)),
    }
  } else {
    Err(String::from("empty is not paint op"))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use cirru_edn::EdnMapView;

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

  fn text_style() -> TextStyle {
    TextStyle {
      family: None,
      weight: 400,
      slant: TextSlant::Normal,
      baseline: TextBaseline::Alphabetic,
    }
  }

  fn paragraph_data(text: &str, max_width: f64) -> Edn {
    map([
      ("text", Edn::Str(text.into())),
      ("max-width", Edn::Number(max_width)),
      ("size", Edn::Number(20.0)),
    ])
  }

  fn metric(data: &Edn, key: &str) -> f64 {
    let Edn::Map(values) = data else {
      panic!("expected measurement map, got {data}");
    };
    let Some(Edn::Number(value)) = values.get(&tag(key)) else {
      panic!("expected numeric :{key} in {data}");
    };
    *value
  }

  fn rgba_pixels(image: &Image, width: i32, height: i32) -> Vec<u8> {
    let info = ImageInfo::new(
      (width, height),
      ColorType::RGBA8888,
      AlphaType::Unpremul,
      ColorSpace::new_srgb(),
    );
    let mut pixels = vec![0; width as usize * height as usize * 4];
    assert!(image.read_pixels(
      &info,
      &mut pixels,
      width as usize * 4,
      (0, 0),
      skia_safe::image::CachingHint::Disallow,
    ));
    pixels
  }

  fn fnv1a64(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf29ce484222325, |hash, byte| {
      (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3)
    })
  }

  fn solid_rectangle(color: Color) -> Shape {
    Shape::Rectangle {
      position: Vector2D::new(2.0, 2.0),
      width: 4.0,
      height: 4.0,
      line_style: None,
      fill_style: Some(PaintSource::Solid(color)),
    }
  }

  fn reset_subtree_cache() {
    *SUBTREE_CACHE.write().unwrap() = SubtreeCache::default();
    SUBTREE_CACHE_TICK.store(1, Ordering::Relaxed);
  }

  fn reset_image_cache() {
    *IMAGE_CACHE.write().unwrap() = ImageCache::default();
    IMAGE_CACHE_TICK.store(1, Ordering::Relaxed);
  }

  fn write_two_color_image(path: &std::path::Path) {
    let mut surface = surfaces::raster_n32_premul((2, 1)).unwrap();
    let mut paint = Paint::default();
    paint.set_color(Color::RED);
    surface.canvas().draw_rect(Rect::from_xywh(0.0, 0.0, 1.0, 1.0), &paint);
    paint.set_color(Color::BLUE);
    surface.canvas().draw_rect(Rect::from_xywh(1.0, 0.0, 1.0, 1.0), &paint);
    let png = surface
      .image_snapshot()
      .encode(None, EncodedImageFormat::PNG, 100)
      .unwrap();
    fs::write(path, png.as_bytes()).unwrap();
  }

  #[test]
  fn extracts_new_skia_shapes() {
    let rounded = map([
      ("type", Edn::tag("rounded-rect")),
      ("position", list([Edn::Number(10.0), Edn::Number(20.0)])),
      ("width", Edn::Number(80.0)),
      ("height", Edn::Number(40.0)),
      ("radius", Edn::Number(6.0)),
    ]);
    assert!(matches!(
      extract_shape(&rounded),
      Ok(Shape::RoundedRectangle {
        radius_x: 6.0,
        radius_y: 6.0,
        ..
      })
    ));

    let ellipse = map([
      ("type", Edn::tag("ellipse")),
      ("radius-x", Edn::Number(30.0)),
      ("radius-y", Edn::Number(20.0)),
    ]);
    assert!(matches!(extract_shape(&ellipse), Ok(Shape::Ellipse { .. })));

    let touch = map([
      ("type", Edn::tag("touch-area")),
      ("radius", Edn::Number(12.0)),
      ("cursor", Edn::tag("pointer")),
    ]);
    assert!(matches!(
      extract_shape(&touch),
      Ok(Shape::TouchArea {
        ref id,
        cursor: Some(winit::window::CursorIcon::Pointer),
        ..
      }) if id.len() == 1
    ));

    let invalid_cursor = map([
      ("type", Edn::tag("touch-area")),
      ("radius", Edn::Number(12.0)),
      ("cursor", Edn::str("pointer")),
    ]);
    assert!(extract_shape(&invalid_cursor)
      .unwrap_err()
      .contains("cursor must be a tag"));

    let image = map([
      ("type", Edn::tag("image")),
      ("file-path", Edn::str("demo.png")),
      ("x", Edn::Number(10.0)),
      ("y", Edn::Number(20.0)),
      ("w", Edn::Number(120.0)),
      ("h", Edn::Number(80.0)),
      ("fit", Edn::tag("contain")),
      ("sampling", Edn::tag("linear")),
    ]);
    assert!(matches!(
      extract_shape(&image),
      Ok(Shape::Image {
        ref id,
        fit: ImageFit::Contain,
        sampling: ImageSampling::Linear,
        crop: None,
        ..
      }) if id == "$"
    ));

    let legacy_image = map([
      ("type", Edn::tag("image")),
      ("file-path", Edn::str("legacy.png")),
      ("x", Edn::Number(0.0)),
      ("y", Edn::Number(0.0)),
      ("w", Edn::Number(32.0)),
      ("h", Edn::Number(24.0)),
    ]);
    assert!(matches!(
      extract_shape(&legacy_image),
      Ok(Shape::Image {
        fit: ImageFit::Fill,
        sampling: ImageSampling::Nearest,
        crop: None,
        ..
      })
    ));
  }

  #[test]
  fn renders_deterministic_rgba_offscreen_pixels() {
    let scene = solid_rectangle(Color::from_argb(255, 255, 0, 0));
    let image = render_offscreen_shape(8, 8, Color::from_argb(0, 0, 0, 0), &scene).unwrap();
    let pixels = rgba_pixels(&image, 8, 8);
    assert_eq!(&pixels[0..4], &[0, 0, 0, 0]);
    assert_eq!(&pixels[(3 * 8 + 3) * 4..(3 * 8 + 3) * 4 + 4], &[255, 0, 0, 255]);
    assert_eq!(fnv1a64(&pixels), 0xc795_b70a_8da3_da05);
  }

  #[test]
  fn resolves_image_fit_and_sampling_options() {
    let destination = Rect::from_xywh(10.0, 20.0, 100.0, 100.0);
    let (contain_source, contain_destination) =
      resolve_image_rects("$", "demo.png", 200.0, 100.0, destination, None, ImageFit::Contain).unwrap();
    assert_eq!(contain_source, None);
    assert_eq!(contain_destination, Rect::from_xywh(10.0, 45.0, 100.0, 50.0));

    let (cover_source, cover_destination) =
      resolve_image_rects("$", "demo.png", 200.0, 100.0, destination, None, ImageFit::Cover).unwrap();
    assert_eq!(cover_source, Some(Rect::from_xywh(50.0, 0.0, 100.0, 100.0)));
    assert_eq!(cover_destination, destination);

    assert_eq!(
      image_sampling_options(ImageSampling::Nearest).filter,
      FilterMode::Nearest
    );
    assert_eq!(image_sampling_options(ImageSampling::Linear).filter, FilterMode::Linear);
    assert!(image_sampling_options(ImageSampling::Cubic).use_cubic);
  }

  #[test]
  fn image_shape_renders_contain_and_rejects_out_of_bounds_crop() {
    reset_image_cache();
    let path = std::env::temp_dir().join(format!("calcit-paint-image-fit-{}.png", std::process::id()));
    write_two_color_image(&path);
    let image_shape = Shape::Image {
      id: "$.children[0]".into(),
      file_path: path.to_string_lossy().into_owned(),
      x: 0.0,
      y: 0.0,
      w: 8.0,
      h: 8.0,
      crop: None,
      fit: ImageFit::Contain,
      sampling: ImageSampling::Nearest,
    };
    let image = render_offscreen_shape(8, 8, Color::TRANSPARENT, &image_shape).unwrap();
    let pixels = rgba_pixels(&image, 8, 8);
    assert_eq!(&pixels[0..4], &[0, 0, 0, 0]);
    assert_eq!(&pixels[(3 * 8 + 1) * 4..(3 * 8 + 1) * 4 + 4], &[255, 0, 0, 255]);
    assert_eq!(&pixels[(3 * 8 + 6) * 4..(3 * 8 + 6) * 4 + 4], &[0, 0, 255, 255]);

    let invalid_crop = Shape::Image {
      id: "$.children[0]".into(),
      file_path: path.to_string_lossy().into_owned(),
      x: 0.0,
      y: 0.0,
      w: 8.0,
      h: 8.0,
      crop: Some(Rect::from_xywh(1.0, 0.0, 2.0, 1.0)),
      fit: ImageFit::Contain,
      sampling: ImageSampling::Nearest,
    };
    let error = render_offscreen_shape(8, 8, Color::TRANSPARENT, &invalid_crop).unwrap_err();
    assert!(error.contains("$.children[0]: image :crop"));
    assert!(error.contains("decoded bounds 2x1"));

    fs::remove_file(path).unwrap();
    reset_image_cache();
  }

  #[test]
  fn decoded_image_cache_is_lru_and_byte_bounded() {
    let image = surfaces::raster_n32_premul((1, 1)).unwrap().image_snapshot();
    let entry = |bytes, last_used| CachedImage {
      modified: None,
      len: 1,
      bytes,
      last_used,
      image: image.clone(),
    };
    let mut cache = ImageCache::default();
    cache.insert_with_limits("a".into(), entry(4, 1), 2, 8);
    cache.insert_with_limits("b".into(), entry(4, 2), 2, 8);
    assert!(cache.fresh_image("a", None, 1, 3).is_some());
    cache.insert_with_limits("c".into(), entry(4, 4), 2, 8);
    assert!(cache.entries.contains_key("a"));
    assert!(!cache.entries.contains_key("b"));
    assert!(cache.entries.contains_key("c"));
    assert_eq!(cache.bytes, 8);

    assert!(cache.fresh_image("a", None, 2, 5).is_none());
    assert!(!cache.entries.contains_key("a"));
    assert_eq!(cache.bytes, 4);
    cache.insert_with_limits("too-large".into(), entry(12, 6), 2, 8);
    assert!(!cache.entries.contains_key("too-large"));
    assert_eq!(cache.bytes, 4);
  }

  #[test]
  fn rounded_clip_matches_offscreen_pixels() {
    let scene = Shape::ClipRoundedRect {
      position: Vector2D::new(0.0, 0.0),
      width: 12.0,
      height: 12.0,
      radius_x: 4.0,
      radius_y: 4.0,
      children: vec![Shape::Rectangle {
        position: Vector2D::new(0.0, 0.0),
        width: 12.0,
        height: 12.0,
        line_style: None,
        fill_style: Some(PaintSource::Solid(Color::from_argb(255, 255, 0, 0))),
      }],
    };
    let image = render_offscreen_shape(12, 12, Color::TRANSPARENT, &scene).unwrap();
    let pixels = rgba_pixels(&image, 12, 12);
    assert_eq!(&pixels[0..4], &[0, 0, 0, 0]);
    assert_eq!(&pixels[(6 * 12 + 6) * 4..(6 * 12 + 6) * 4 + 4], &[255, 0, 0, 255]);
  }

  #[test]
  fn renderer_propagates_clips_to_touch_hits() {
    touches::reset_pointer_state();
    touches::reset_touches_stack();
    let scene = Shape::ClipRect {
      position: Vector2D::new(20.0, 10.0),
      width: 40.0,
      height: 30.0,
      children: vec![Shape::TouchArea {
        id: "$.children[0]".into(),
        target: crate::primes::EventTarget::default(),
        position: Vector2D::new(50.0, 30.0),
        area: TouchAreaShape::Rect(50.0, 30.0),
        cursor: Some(winit::window::CursorIcon::Pointer),
        accessibility: None,
        line_style: None,
        fill_style: None,
      }],
    };
    let mut surface = surfaces::raster_n32_premul((100, 60)).unwrap();
    draw_shape(surface.canvas(), &scene, &Transform::identity()).unwrap();
    assert!(touches::find_touch_area(Vector2D::new(40.0, 20.0)).is_some());
    assert!(touches::find_touch_area(Vector2D::new(10.0, 20.0)).is_none());
  }

  #[test]
  fn transformed_clip_keeps_paint_and_hit_rotation_in_sync() {
    touches::reset_pointer_state();
    touches::reset_touches_stack();
    let clipped_touch = Shape::ClipRect {
      position: Vector2D::new(0.0, 0.0),
      width: 40.0,
      height: 30.0,
      children: vec![Shape::TouchArea {
        id: "$.children[0].children[0].children[0]".into(),
        target: crate::primes::EventTarget::default(),
        position: Vector2D::new(20.0, 15.0),
        area: TouchAreaShape::Rect(50.0, 40.0),
        cursor: None,
        accessibility: None,
        line_style: None,
        fill_style: Some(PaintSource::Solid(Color::RED)),
      }],
    };
    let scene = Shape::Translate {
      x: 80.0,
      y: 20.0,
      children: vec![Shape::Rotate {
        radius: std::f32::consts::FRAC_PI_2,
        children: vec![clipped_touch],
      }],
    };
    let mut surface = surfaces::raster_n32_premul((100, 80)).unwrap();
    draw_shape(surface.canvas(), &scene, &Transform::identity()).unwrap();
    let image = surface.image_snapshot();
    let pixels = rgba_pixels(&image, 100, 80);
    assert_eq!(&pixels[(40 * 100 + 60) * 4..(40 * 100 + 60) * 4 + 4], &[255, 0, 0, 255]);
    assert!(touches::find_touch_area(Vector2D::new(60.0, 40.0)).is_some());
    assert!(touches::find_touch_area(Vector2D::new(90.0, 40.0)).is_none());
  }

  #[test]
  fn exports_png_only_to_the_explicit_path() {
    let path = std::env::temp_dir().join(format!("calcit-paint-offscreen-{}.png", std::process::id()));
    let request = map([
      ("width", Edn::Number(7.0)),
      ("height", Edn::Number(5.0)),
      ("path", Edn::Str(path.to_string_lossy().into_owned().into())),
      ("scene", Edn::Nil),
    ]);
    render_to_png(&request).unwrap();
    let png = fs::read(&path).unwrap();
    assert_eq!(&png[..8], b"\x89PNG\r\n\x1a\n");
    let decoded = Image::from_encoded(Data::new_copy(&png)).unwrap();
    assert_eq!((decoded.width(), decoded.height()), (7, 5));
    fs::remove_file(path).unwrap();
  }

  #[test]
  fn cached_groups_use_revision_invalidation_and_reject_interaction() {
    reset_subtree_cache();
    let red = solid_rectangle(Color::from_argb(255, 255, 0, 0));
    let blue = solid_rectangle(Color::from_argb(255, 0, 0, 255));
    let first = render_cached_subtree("badge", 0, 8, 8, std::slice::from_ref(&red)).unwrap();
    let stale_by_contract = render_cached_subtree("badge", 0, 8, 8, std::slice::from_ref(&blue)).unwrap();
    assert_eq!(rgba_pixels(&first, 8, 8), rgba_pixels(&stale_by_contract, 8, 8));

    let refreshed = render_cached_subtree("badge", 1, 8, 8, &[blue]).unwrap();
    assert_eq!(
      &rgba_pixels(&refreshed, 8, 8)[(3 * 8 + 3) * 4..(3 * 8 + 3) * 4 + 4],
      &[0, 0, 255, 255]
    );
    let cache = SUBTREE_CACHE.read().unwrap();
    assert_eq!(cache.entries.len(), 1);
    assert_eq!(cache.bytes, 8 * 8 * 4);
    drop(cache);

    let listener = Shape::KeyListener {
      key: "K".into(),
      modifiers: None,
      focus_id: None,
      target: Default::default(),
    };
    assert!(render_cached_subtree("interactive", 0, 8, 8, &[listener])
      .unwrap_err()
      .contains("cannot contain"));

    reset_subtree_cache();
    for index in 0..=SUBTREE_CACHE_MAX_ENTRIES {
      render_cached_subtree(&format!("entry-{index}"), 0, 1, 1, &[]).unwrap();
    }
    let cache = SUBTREE_CACHE.read().unwrap();
    assert_eq!(cache.entries.len(), SUBTREE_CACHE_MAX_ENTRIES);
    assert!(!cache.entries.contains_key("entry-0"));
    drop(cache);
    reset_subtree_cache();
  }

  #[test]
  fn validates_new_shape_arguments() {
    let rounded = map([
      ("type", Edn::tag("rounded-rect")),
      ("width", Edn::Number(80.0)),
      ("height", Edn::Number(40.0)),
    ]);
    assert!(extract_shape(&rounded).unwrap_err().contains("requires :radius"));

    let rounded_clip = map([
      ("type", Edn::tag("clip-rounded-rect")),
      ("position", list([Edn::Number(4.0), Edn::Number(6.0)])),
      ("width", Edn::Number(80.0)),
      ("height", Edn::Number(40.0)),
      ("radius-x", Edn::Number(8.0)),
      ("radius-y", Edn::Number(12.0)),
      ("children", list([])),
    ]);
    assert!(matches!(
      extract_shape(&rounded_clip),
      Ok(Shape::ClipRoundedRect {
        radius_x: 8.0,
        radius_y: 12.0,
        ..
      })
    ));

    let missing_clip_radius = map([
      ("type", Edn::tag("clip-rounded-rect")),
      ("width", Edn::Number(80.0)),
      ("height", Edn::Number(40.0)),
      ("children", list([])),
    ]);
    assert!(extract_shape(&missing_clip_radius)
      .unwrap_err()
      .contains("requires :radius"));

    let negative_clip_radius = map([
      ("type", Edn::tag("clip-rounded-rect")),
      ("width", Edn::Number(80.0)),
      ("height", Edn::Number(40.0)),
      ("radius", Edn::Number(-1.0)),
      ("children", list([])),
    ]);
    assert!(extract_shape(&negative_clip_radius)
      .unwrap_err()
      .contains("radius-x must be a finite non-negative number"));

    let invalid_image_fit = map([
      ("type", Edn::tag("image")),
      ("file-path", Edn::str("demo.png")),
      ("x", Edn::Number(0.0)),
      ("y", Edn::Number(0.0)),
      ("w", Edn::Number(20.0)),
      ("h", Edn::Number(20.0)),
      ("fit", Edn::str("cover")),
    ]);
    assert!(extract_shape(&invalid_image_fit)
      .unwrap_err()
      .contains("image :fit must be a tag or nil"));

    let invalid_image_crop = map([
      ("type", Edn::tag("image")),
      ("file-path", Edn::str("demo.png")),
      ("x", Edn::Number(0.0)),
      ("y", Edn::Number(0.0)),
      ("w", Edn::Number(20.0)),
      ("h", Edn::Number(20.0)),
      ("crop", Edn::str("not-a-map")),
    ]);
    assert!(extract_shape(&invalid_image_crop)
      .unwrap_err()
      .contains("image :crop must be a map or nil"));

    let invalid_crop_dimensions = map([
      ("type", Edn::tag("image")),
      ("file-path", Edn::str("demo.png")),
      ("x", Edn::Number(0.0)),
      ("y", Edn::Number(0.0)),
      ("w", Edn::Number(20.0)),
      ("h", Edn::Number(20.0)),
      (
        "crop",
        map([
          ("x", Edn::Number(-1.0)),
          ("y", Edn::Number(0.0)),
          ("w", Edn::Number(10.0)),
          ("h", Edn::Number(10.0)),
        ]),
      ),
    ]);
    assert!(extract_shape(&invalid_crop_dimensions)
      .unwrap_err()
      .contains("x must be a finite non-negative number"));

    let zero_image_width = map([
      ("type", Edn::tag("image")),
      ("file-path", Edn::str("demo.png")),
      ("x", Edn::Number(0.0)),
      ("y", Edn::Number(0.0)),
      ("w", Edn::Number(0.0)),
      ("h", Edn::Number(20.0)),
    ]);
    assert!(extract_shape(&zero_image_width)
      .unwrap_err()
      .contains("w must be a finite positive number"));

    let nested_invalid_image = map([
      ("type", Edn::tag("group")),
      (
        "children",
        list([map([
          ("type", Edn::tag("image")),
          ("file-path", Edn::str("demo.png")),
          ("x", Edn::Number(0.0)),
          ("y", Edn::Number(0.0)),
          ("w", Edn::Number(20.0)),
          ("h", Edn::Number(-1.0)),
        ])]),
      ),
    ]);
    assert_eq!(
      validate_scene(&nested_invalid_image),
      vec!["$.children[0]: h must be a finite positive number, got -1"]
    );

    let opacity = map([
      ("type", Edn::tag("opacity")),
      ("alpha", Edn::Number(1.5)),
      ("children", list([])),
    ]);
    assert!(extract_shape(&opacity).unwrap_err().contains("between 0 and 1"));

    let ellipse = map([
      ("type", Edn::tag("ellipse")),
      ("radius-x", Edn::Number(-1.0)),
      ("radius-y", Edn::Number(20.0)),
    ]);
    assert!(extract_shape(&ellipse)
      .unwrap_err()
      .contains("radius-x must be a finite non-negative number"));

    let blend = map([
      ("type", Edn::tag("blend")),
      ("mode", Edn::tag("multiply")),
      ("children", list([])),
    ]);
    assert!(matches!(
      extract_shape(&blend),
      Ok(Shape::Blend {
        mode: skia_safe::BlendMode::Multiply,
        ..
      })
    ));

    let unknown_blend = map([
      ("type", Edn::tag("blend")),
      ("mode", Edn::tag("unknown")),
      ("children", list([])),
    ]);
    assert!(extract_shape(&unknown_blend)
      .unwrap_err()
      .contains("unsupported blend mode"));

    let fractional_cache = map([
      ("type", Edn::tag("cached-group")),
      ("cache-key", Edn::Str("badge".into())),
      ("width", Edn::Number(8.5)),
      ("height", Edn::Number(8.0)),
    ]);
    assert!(extract_shape(&fractional_cache)
      .unwrap_err()
      .contains(":width must be an integer"));

    let missing_scene = map([
      ("width", Edn::Number(8.0)),
      ("height", Edn::Number(8.0)),
      ("path", Edn::Str("unused.png".into())),
    ]);
    assert!(render_to_png(&missing_scene).unwrap_err().contains("requires :scene"));
  }

  #[test]
  fn validates_scene_with_stable_nested_paths_and_all_sibling_failures() {
    assert!(validate_scene(&map([("type", Edn::tag("group")), ("children", list([])),])).is_empty());
    assert_eq!(validate_scene(&Edn::Number(3.0)), vec!["$: expected a map, got 3"]);

    let invalid = map([
      ("type", Edn::tag("group")),
      (
        "children",
        list([
          map([("type", Edn::tag("missing-shape"))]),
          map([("type", Edn::tag("group")), ("children", list([Edn::Bool(true)]))]),
        ]),
      ),
    ]);
    let diagnostics = validate_scene(&invalid);
    assert_eq!(
      diagnostics,
      vec![
        "$.children[0]: unknown kind: missing-shape",
        "$.children[1].children[0]: expected a map, got true",
      ]
    );
    assert_eq!(extract_shape(&invalid).unwrap_err(), diagnostics.join("\n"));

    let invalid_children = map([("type", Edn::tag("group")), ("children", Edn::Str("not-a-list".into()))]);
    assert_eq!(
      validate_scene(&invalid_children),
      vec!["$.children: expected a list, got |not-a-list"]
    );

    let path = std::env::temp_dir().join(format!("calcit-paint-invalid-scene-{}.png", std::process::id()));
    let request = map([
      ("width", Edn::Number(8.0)),
      ("height", Edn::Number(8.0)),
      ("path", Edn::Str(path.to_string_lossy().into_owned().into())),
      ("scene", invalid),
    ]);
    let error = render_to_png(&request).unwrap_err();
    assert!(error.contains("$.children[0]: unknown kind: missing-shape"));
    assert!(!path.exists());
  }

  #[test]
  fn supports_explicit_path_close() {
    assert_eq!(extract_paint_op(&[Edn::tag("close-path")]), Ok(PaintPathTo::Close));
    assert!(extract_paint_op(&[Edn::tag("close"), Edn::Nil]).is_err());
  }

  #[test]
  fn text_alignment_uses_the_requested_anchor() {
    assert_eq!(text_x_offset(&TextAlign::Left, 120.0), 0.0);
    assert_eq!(text_x_offset(&TextAlign::Center, 120.0), -60.0);
    assert_eq!(text_x_offset(&TextAlign::Right, 120.0), -120.0);
  }

  #[test]
  fn extracts_legacy_and_extended_text_shapes() {
    let legacy = map([
      ("type", Edn::tag("text")),
      ("text", Edn::Str("Demo".into())),
      ("position", list([Edn::Number(10.0), Edn::Number(20.0)])),
      ("size", Edn::Number(24.0)),
      ("color", list([Edn::Number(0.0), Edn::Number(0.0), Edn::Number(100.0)])),
      ("align", Edn::tag("center")),
      ("weight", Edn::Str("300".into())),
    ]);
    assert!(matches!(
      extract_shape(&legacy),
      Ok(Shape::Text {
        style: TextStyle {
          family: None,
          weight: 300,
          slant: TextSlant::Normal,
          baseline: TextBaseline::Alphabetic,
        },
        ..
      })
    ));

    let extended = map([
      ("type", Edn::tag("text")),
      ("text", Edn::Str("Layout".into())),
      ("position", list([Edn::Number(10.0), Edn::Number(20.0)])),
      ("size", Edn::Number(24.0)),
      ("color", list([Edn::Number(0.0), Edn::Number(0.0), Edn::Number(100.0)])),
      ("align", Edn::tag("left")),
      ("font-family", Edn::Str("monospace".into())),
      ("weight", Edn::Number(700.0)),
      ("style", Edn::tag("italic")),
      ("baseline", Edn::tag("top")),
    ]);
    assert!(matches!(
      extract_shape(&extended),
      Ok(Shape::Text {
        style: TextStyle {
          family: Some(_),
          weight: 700,
          slant: TextSlant::Italic,
          baseline: TextBaseline::Top,
        },
        ..
      })
    ));
  }

  #[test]
  fn extracts_paragraph_shape_and_text_block_alias() {
    for kind in ["paragraph", "text-block"] {
      let paragraph = map([
        ("type", Edn::tag(kind)),
        ("text", Edn::Str("Calcit 段落".into())),
        ("position", list([Edn::Number(12.0), Edn::Number(24.0)])),
        ("max-width", Edn::Number(240.0)),
        ("size", Edn::Number(20.0)),
        (
          "color",
          list([Edn::Number(200.0), Edn::Number(80.0), Edn::Number(90.0)]),
        ),
        ("align", Edn::tag("center")),
        ("direction", Edn::tag("ltr")),
        ("line-height", Edn::Number(30.0)),
        ("max-lines", Edn::Number(2.0)),
        ("ellipsis", Edn::Str("…".into())),
      ]);
      assert!(matches!(
        extract_shape(&paragraph),
        Ok(Shape::Paragraph {
          layout: ParagraphLayout {
            align: TextAlign::Center,
            direction: TextDirection::Ltr,
            line_height: Some(30.0),
            max_lines: Some(2),
            ..
          },
          ..
        })
      ));
    }
  }

  #[test]
  fn extracts_focus_areas_and_compatible_shortcut_listeners() {
    let focus_area = map([
      ("type", Edn::tag("focus-area")),
      ("focus-id", Edn::Str("editor".into())),
      ("position", list([Edn::Number(40.0), Edn::Number(60.0)])),
      ("dx", Edn::Number(30.0)),
      ("dy", Edn::Number(20.0)),
      ("tab-index", Edn::Number(2.0)),
      ("text-input?", Edn::Bool(true)),
    ]);
    assert!(matches!(
      extract_shape(&focus_area),
      Ok(Shape::FocusArea {
        id,
        tab_index: 2,
        text_input: true,
        ..
      }) if id == "editor"
    ));

    let mut modifier_values = EdnMapView::default();
    modifier_values.insert(tag("control?"), Edn::Bool(true));
    let shortcut = map([
      ("type", Edn::tag("key-listener")),
      ("key", Edn::Str("K".into())),
      ("focus-id", Edn::Str("editor".into())),
      ("modifiers", Edn::Map(modifier_values)),
    ]);
    assert!(matches!(
      extract_shape(&shortcut),
      Ok(Shape::KeyListener {
        modifiers: Some(crate::primes::ShortcutModifiers { control: true, .. }),
        focus_id: Some(id),
        ..
      }) if id == "editor"
    ));

    let legacy = map([("type", Edn::tag("key-listener")), ("key", Edn::Str("D".into()))]);
    assert!(matches!(
      extract_shape(&legacy),
      Ok(Shape::KeyListener {
        modifiers: None,
        focus_id: None,
        ..
      })
    ));

    let invalid_modifiers = map([
      ("type", Edn::tag("key-listener")),
      ("key", Edn::Str("K".into())),
      ("modifiers", Edn::Number(1.0)),
    ]);
    assert!(extract_shape(&invalid_modifiers)
      .unwrap_err()
      .contains(":modifiers must be a map"));

    let invalid_tab_index = map([
      ("type", Edn::tag("focus-area")),
      ("focus-id", Edn::Str("editor".into())),
      ("radius", Edn::Number(20.0)),
      ("tab-index", Edn::Number(1.5)),
    ]);
    assert!(extract_shape(&invalid_tab_index)
      .unwrap_err()
      .contains("tab-index must be an integer"));
  }

  #[test]
  fn paragraph_layout_handles_empty_cjk_and_rtl_text() {
    for text in ["", "中文段落可以安全换行", "مرحبا بالعالم"] {
      let measured = measure_paragraph(&paragraph_data(text, 140.0)).unwrap();
      for key in [
        "width",
        "height",
        "line-count",
        "max-width",
        "min-intrinsic-width",
        "max-intrinsic-width",
        "alphabetic-baseline",
        "ideographic-baseline",
      ] {
        assert!(metric(&measured, key).is_finite(), "non-finite :{key} for {text:?}");
      }
    }

    let Edn::Map(mut rtl) = paragraph_data("مرحبا بالعالم", 140.0) else {
      unreachable!();
    };
    rtl.insert(tag("direction"), Edn::tag("rtl"));
    rtl.insert(tag("align"), Edn::tag("right"));
    assert!(measure_paragraph(&Edn::Map(rtl)).is_ok());
  }

  #[test]
  fn paragraph_respects_newlines_max_lines_and_measurement_metrics() {
    let explicit_break = paragraph_data("first line\nsecond line", 500.0);
    let measured = measure_paragraph(&explicit_break).unwrap();
    assert_eq!(metric(&measured, "line-count"), 2.0);

    let Edn::Map(mut truncated) =
      paragraph_data("one two three four five six seven eight nine ten eleven twelve", 90.0)
    else {
      unreachable!();
    };
    truncated.insert(tag("max-lines"), Edn::Number(2.0));
    truncated.insert(tag("ellipsis"), Edn::Str("…".into()));
    let layout = extract_paragraph_layout(&truncated).unwrap();
    let paragraph = build_paragraph(&layout, Color::BLACK);
    assert_eq!(paragraph.line_number(), 2);
    assert!(paragraph.did_exceed_max_lines());

    let measured = measure_paragraph(&Edn::Map(truncated)).unwrap();
    assert_eq!(metric(&measured, "line-count"), paragraph.line_number() as f64);
    assert_eq!(metric(&measured, "height"), paragraph.height() as f64);
    assert_eq!(metric(&measured, "width"), paragraph.longest_line() as f64);
  }

  #[test]
  fn paragraph_rejects_invalid_layout_constraints() {
    assert!(measure_paragraph(&paragraph_data("bad width", 0.0))
      .unwrap_err()
      .contains("max-width"));

    let Edn::Map(mut invalid_height) = paragraph_data("bad height", 120.0) else {
      unreachable!();
    };
    invalid_height.insert(tag("line-height"), Edn::Number(-1.0));
    assert!(measure_paragraph(&Edn::Map(invalid_height))
      .unwrap_err()
      .contains("line-height"));

    let Edn::Map(mut orphan_ellipsis) = paragraph_data("bad ellipsis", 120.0) else {
      unreachable!();
    };
    orphan_ellipsis.insert(tag("ellipsis"), Edn::Str("…".into()));
    assert!(measure_paragraph(&Edn::Map(orphan_ellipsis))
      .unwrap_err()
      .contains("requires :max-lines"));
  }

  #[test]
  fn text_font_falls_back_and_honors_supported_weights() {
    let missing_family = TextStyle {
      family: Some("Calcit Paint Missing Family".into()),
      weight: 700,
      slant: TextSlant::Italic,
      baseline: TextBaseline::Alphabetic,
    };
    assert!(create_text_font(&missing_family, 18.0).is_ok());
    assert!(create_text_font(
      &TextStyle {
        weight: 100,
        ..text_style()
      },
      18.0,
    )
    .is_ok());
    assert!(create_text_font(
      &TextStyle {
        weight: 900,
        ..text_style()
      },
      18.0,
    )
    .is_ok());
  }

  #[test]
  fn text_baselines_and_empty_measurement_are_stable() {
    let font = create_text_font(&text_style(), 20.0).unwrap();
    let (_, metrics) = font.metrics();
    let top = text_baseline_y(
      100.0,
      &TextStyle {
        baseline: TextBaseline::Top,
        ..text_style()
      },
      &font,
    );
    let middle = text_baseline_y(
      100.0,
      &TextStyle {
        baseline: TextBaseline::Middle,
        ..text_style()
      },
      &font,
    );
    let bottom = text_baseline_y(
      100.0,
      &TextStyle {
        baseline: TextBaseline::Bottom,
        ..text_style()
      },
      &font,
    );
    assert!((top - (100.0 - metrics.ascent)).abs() < f32::EPSILON);
    assert!((middle - (100.0 - 0.5 * (metrics.ascent + metrics.descent))).abs() < f32::EPSILON);
    assert!((bottom - (100.0 - metrics.descent)).abs() < f32::EPSILON);

    let measured = measure_text(&map([("text", Edn::Str("".into())), ("size", Edn::Number(20.0))])).unwrap();
    let Edn::Map(measured) = measured else {
      panic!("expected text metrics map")
    };
    assert_eq!(measured.get(&tag("width")), Some(&Edn::Number(0.0)));
    assert!(matches!(measured.get(&tag("height")), Some(Edn::Number(height)) if *height > 0.0));
    assert!(matches!(measured.get(&tag("baseline")), Some(Edn::Number(offset)) if *offset > 0.0));
  }
}

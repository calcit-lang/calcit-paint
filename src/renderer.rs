use crate::{focus, touches};
use std::collections::HashMap;
use std::fs;
use std::sync::RwLock;
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
  gradient, Color, Color4f, Data, Font, FontMgr, FontStyle, Image, Paint, PaintStyle, PathBuilder, PathEffect, RRect,
  Rect, Shader, TextBlob, TileMode,
};

#[derive(Clone)]
struct CachedImage {
  modified: Option<SystemTime>,
  len: u64,
  image: Image,
}

lazy_static! {
  static ref PREV_MESSAGES: RwLock<Vec<(Box<str>, Edn)>> = RwLock::new(vec![]);
  static ref BG_COLOR: RwLock<Color> = RwLock::new(Color::BLACK);
  static ref IMAGE_CACHE: RwLock<HashMap<String, CachedImage>> = RwLock::new(HashMap::new());
  static ref SHADER_CACHE: RwLock<HashMap<String, Shader>> = RwLock::new(HashMap::new());
  static ref DASH_EFFECT_CACHE: RwLock<HashMap<String, PathEffect>> = RwLock::new(HashMap::new());
}

use crate::{
  color::extract_color,
  extracter::{
    extract_event_target, extract_fill_style, extract_paragraph_layout, extract_polyline_stroke_style,
    extract_position, extract_shortcut_modifiers, extract_stroke_style, extract_text_style, extract_touch_area_shape,
    read_blend_mode, read_bool, read_color, read_f32, read_optional_f32, read_optional_i32, read_optional_string_field,
    read_points, read_position, read_string, read_text_align, tag,
  },
  key_listener,
  primes::{
    DashPattern, PaintPathTo, PaintSource, ParagraphLayout, Shape, StrokeStyle, TextAlign, TextBaseline, TextDirection,
    TextSlant, TextStyle, TouchAreaShape,
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

fn load_image(file_path: &str) -> Result<Option<Image>, String> {
  let metadata = match fs::metadata(file_path) {
    Ok(metadata) => metadata,
    Err(error) => {
      eprintln!("[Paint Error] failed to load {file_path}: {error}");
      return Ok(None);
    }
  };
  let modified = metadata.modified().ok();
  let len = metadata.len();
  if let Some(cached) = IMAGE_CACHE
    .read()
    .map_err(|_| "image cache lock is poisoned".to_owned())?
    .get(file_path)
    .filter(|cached| cached.modified == modified && cached.len == len)
  {
    return Ok(Some(cached.image.clone()));
  }

  let file_data = fs::read(file_path).map_err(|error| format!("[Paint Error] failed to load {file_path}: {error}"))?;
  let image = Image::from_encoded(Data::new_copy(&file_data))
    .ok_or_else(|| format!("[Paint Error] failed to decode image: {file_path}"))?;
  IMAGE_CACHE
    .write()
    .map_err(|_| "image cache lock is poisoned".to_owned())?
    .insert(
      file_path.to_owned(),
      CachedImage {
        modified,
        len,
        image: image.clone(),
      },
    );
  Ok(Some(image))
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
      // println!("op: {} {:?}", call_op, arg);
      match (&*call_op, arg) {
        ("render-canvas!", tree) => {
          shown_shape = true;
          match extract_shape(&tree) {
            Ok(shape) => draw_shape(canvas, &shape, &Transform::identity())?,
            Err(failure) => {
              println!("Failed to extract shape: {}", failure)
            }
          }
        }
        ("reset-canvas!", tree) => {
          reset_page(canvas, extract_color(&tree)?)?;
        }
        _ => println!("Unknown op: {}", call_op),
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
        draw_shape(canvas, child, &t1.then(tr))?;
      }
      canvas.restore();
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
      file_path,
      x,
      y,
      w,
      h,
      crop,
    } => {
      let paint = Paint::default();
      let Some(image) = load_image(file_path)? else {
        return Ok(());
      };
      let area = Rect::from_xywh(*x, *y, *w, *h);
      match crop {
        Some(crop) => {
          let c = crop.to_owned();
          canvas.draw_image_rect(image, Some((&c, SrcRectConstraint::Fast)), area, &paint);
        }
        None => {
          canvas.draw_image_rect(image, None, area, &paint);
        }
      }
    }
    Shape::TouchArea {
      position,
      target,
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
      touches::add_touch_area(position.to_owned(), area.to_owned(), target.to_owned(), tr);
    }
    Shape::KeyListener {
      key,
      modifiers,
      focus_id,
      target,
    } => {
      key_listener::add_key_listener(
        key.to_owned(),
        modifiers.to_owned(),
        focus_id.to_owned(),
        target.to_owned(),
      );
    }
    Shape::FocusArea {
      id,
      target,
      position,
      area,
      tab_index,
      text_input,
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
      focus::register_focus_area(focus::FocusArea {
        id: id.to_owned(),
        target: target.to_owned(),
        position: position.to_owned(),
        area: area.to_owned(),
        transform: tr.to_owned(),
        tab_index: *tab_index,
        text_input: *text_input,
        order: 0,
      })?;
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
        draw_shape(canvas, child, &t1.then(tr))?;
      }
      canvas.restore();
    }
    Shape::Rotate { radius, children } => {
      canvas.save();
      canvas.rotate(*radius, None);
      let t1 = Transform::identity().then_rotate(Angle {
        radians: radius.to_owned(),
      });
      for child in children {
        draw_shape(canvas, child, &t1.then(tr))?;
      }
      canvas.restore();
    }
    Shape::Translate { x, y, children } => {
      canvas.save();
      canvas.translate((*x, *y));
      let v = Vector2D::new(x.to_owned(), y.to_owned());
      let t1 = Transform::identity().then_translate(v);
      for child in children {
        draw_shape(canvas, child, &t1.then(tr))?;
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
      for child in children {
        draw_shape(canvas, child, tr)?;
      }
      canvas.restore();
    }
    Shape::Opacity { alpha, children } => {
      canvas.save_layer_alpha_f(None, alpha.clamp(0.0, 1.0));
      for child in children {
        draw_shape(canvas, child, tr)?;
      }
      canvas.restore();
    }
    Shape::Blend { mode, children } => {
      let mut paint = Paint::default();
      paint.set_blend_mode(*mode);
      canvas.save_layer(&SaveLayerRec::default().paint(&paint));
      for child in children {
        draw_shape(canvas, child, tr)?;
      }
      canvas.restore();
    }
  }
  Ok(())
}

fn extract_shape(tree: &Edn) -> Result<Shape, String> {
  // println!("extracting shape: {:?} -- {:?}", tag("type"), tree);
  match tree {
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
          let children = extract_children(c)?;

          Ok(Shape::Group {
            position: read_position(m, "position")?,
            children,
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
          target: extract_event_target(m),
          position: read_position(m, "position")?,
          area: extract_touch_area_shape(m)?,
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
          fill_style: extract_fill_style(m)?,
          line_style: extract_stroke_style(m)?,
        }),
        "rotate" => {
          let c = m.get(&tag("children"));
          let children = extract_children(c)?;

          Ok(Shape::Rotate {
            radius: read_f32(m, "radius")?,
            children,
          })
        }
        "scale" => {
          let c = m.get(&tag("children"));
          let children = extract_children(c)?;

          Ok(Shape::Scale {
            factor: read_f32(m, "factor")?,
            children,
          })
        }
        "translate" => {
          let c = m.get(&tag("children"));
          let children = extract_children(c)?;

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
          children: extract_children(m.get(&tag("children")))?,
        }),
        "opacity" => {
          let alpha = read_f32(m, "alpha")?;
          if !(0.0..=1.0).contains(&alpha) {
            return Err(format!("opacity alpha must be between 0 and 1, got {alpha}"));
          }
          Ok(Shape::Opacity {
            alpha,
            children: extract_children(m.get(&tag("children")))?,
          })
        }
        "blend" => Ok(Shape::Blend {
          mode: read_blend_mode(m, "mode")?,
          children: extract_children(m.get(&tag("children")))?,
        }),
        "image" => {
          let crop = match m.get(&tag("crop")) {
            Some(Edn::Map(m)) => Some(Rect::from_xywh(
              read_f32(m, "x")?,
              read_f32(m, "y")?,
              read_f32(m, "w")?,
              read_f32(m, "h")?,
            )),
            _ => None,
          };
          Ok(Shape::Image {
            file_path: read_string(m, "file-path")?,
            x: read_f32(m, "x")?,
            y: read_f32(m, "y")?,
            w: read_f32(m, "w")?,
            h: read_f32(m, "h")?,
            crop,
          })
        }
        _ => Err(format!("unknown kind: {}", name)),
      },
      Some(a) => Err(format!("unknown kind value, {}", a)),
      None => Err(String::from("nil type")),
    },
    Edn::Nil => Ok(Shape::Group {
      position: Vector2D::new(0.0, 0.0),
      children: vec![],
    }),
    _ => Err(format!("expected a map, got {}", tree)),
  }
}

fn validate_non_negative(name: &str, value: f32) -> Result<f32, String> {
  if value.is_finite() && value >= 0.0 {
    Ok(value)
  } else {
    Err(format!("{name} must be a finite non-negative number, got {value}"))
  }
}

fn read_non_negative_f32(tree: &EdnMapView, key: &str) -> Result<f32, String> {
  validate_non_negative(key, read_f32(tree, key)?)
}

fn extract_children(children: Option<&Edn>) -> Result<Vec<Shape>, String> {
  let empty_group = Shape::Group {
    position: Vector2D::new(0.0, 0.0),
    children: vec![],
  };
  match children {
    Some(Edn::List(EdnListView(xs))) => {
      let mut ys = vec![];
      for x in xs {
        match extract_shape(x) {
          Ok(v) => ys.push(v),
          Err(failure) => {
            println!("Failed extracting: {}\n  in {}", failure, x);
            ys.push(empty_group.to_owned());
          }
        }
      }
      Ok(ys)
    }
    Some(a) => Err(format!("invalid children: {}", a)),
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
  }

  #[test]
  fn validates_new_shape_arguments() {
    let rounded = map([
      ("type", Edn::tag("rounded-rect")),
      ("width", Edn::Number(80.0)),
      ("height", Edn::Number(40.0)),
    ]);
    assert!(extract_shape(&rounded).unwrap_err().contains("requires :radius"));

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

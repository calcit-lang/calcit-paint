use crate::touches;
use std::collections::HashMap;
use std::fs;
use std::sync::RwLock;
use std::time::SystemTime;

use euclid::{Angle, Vector2D};

use cirru_edn::{Edn, EdnListView, EdnMapView};

use lazy_static::lazy_static;

type Transform = euclid::default::Transform2D<f32>;

use skia_safe::canvas::SrcRectConstraint;
use skia_safe::paint::{Cap, Join};
use skia_safe::{Color, Data, Font, Image, Paint, PaintStyle, PathBuilder, RRect, Rect, TextBlob};

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
}

use crate::{
  color::extract_color,
  extracter::{
    extract_line_style, extract_position, extract_touch_area_shape, read_bool, read_color, read_f32, read_line_cap,
    read_line_join, read_optional_f32, read_points, read_position, read_some_color, read_string, read_text_align, tag,
  },
  key_listener,
  primes::{PaintPathTo, Shape, TouchAreaShape},
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

fn stroke_paint(color: Color, width: f32) -> Paint {
  let mut paint = Paint::default();
  paint
    .set_anti_alias(true)
    .set_style(PaintStyle::Stroke)
    .set_stroke_width(width)
    .set_stroke_cap(Cap::Round)
    .set_stroke_join(Join::Round)
    .set_color(color);
  paint
}

fn fill_paint(color: Color) -> Paint {
  let mut paint = Paint::default();
  paint.set_anti_alias(true).set_style(PaintStyle::Fill).set_color(color);
  paint
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

      if let Some((color, width)) = line_style {
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint
          .set_style(PaintStyle::Stroke)
          .set_stroke_width(*width)
          .set_stroke_cap(Cap::Round)
          .set_stroke_join(Join::Round)
          .set_color(*color);

        canvas.draw_rect(rect_path, &paint);
      }
      if let Some(color) = fill_style {
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Fill).set_color(*color);

        canvas.draw_rect(rect_path, &paint);
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
      if let Some((color, width)) = line_style {
        canvas.draw_rrect(rounded, &stroke_paint(*color, *width));
      }
      if let Some(color) = fill_style {
        canvas.draw_rrect(rounded, &fill_paint(*color));
      }
    }
    Shape::Circle {
      position,
      radius,
      line_style,
      fill_style,
    } => {
      // canvas.set_transform(tr);

      if let Some((color, width)) = line_style {
        let mut paint = Paint::default();
        paint.set_anti_alias(true);

        paint
          .set_style(PaintStyle::Stroke)
          .set_stroke_width(*width)
          .set_stroke_cap(Cap::Round)
          .set_stroke_join(Join::Round)
          .set_color(*color);

        canvas.draw_circle((position.x, position.y), *radius, &paint);
      }
      if let Some(color) = fill_style {
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Fill).set_color(*color);

        canvas.draw_circle((position.x, position.y), *radius, &paint);
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
      if let Some((color, width)) = line_style {
        canvas.draw_oval(oval, &stroke_paint(*color, *width));
      }
      if let Some(color) = fill_style {
        canvas.draw_oval(oval, &fill_paint(*color));
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
      if let Some((color, width)) = line_style {
        canvas.draw_arc(
          oval,
          *start_angle,
          *sweep_angle,
          *use_center,
          &stroke_paint(*color, *width),
        );
      }
      if let Some(color) = fill_style {
        canvas.draw_arc(oval, *start_angle, *sweep_angle, *use_center, &fill_paint(*color));
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
      // weight: _w,
      align,
    } => {
      // canvas.set_transform(tr);
      // https://github.com/jrmuizel/raqote/issues/179
      // for now we have to by pass bug in text rendering
      // canvas.set_transform(&Transform::identity());

      let mut font = Font::default();
      font.set_size(*size);
      let text_blob = TextBlob::new(text, &font).unwrap();

      let mut paint = Paint::default();
      paint.set_anti_alias(true);
      paint.set_style(PaintStyle::Fill).set_color(*color);

      let x_offset = match align {
        crate::primes::TextAlign::Left => 0.0,
        crate::primes::TextAlign::Center => -0.5 * text_blob.bounds().width(),
        crate::primes::TextAlign::Right => -text_blob.bounds().width(),
      };
      canvas.draw_text_blob(text_blob, (position.x + x_offset, position.y), &paint);
    }
    Shape::Polyline {
      position,
      stops,
      width,
      color,
      join,
      cap,
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

      let mut paint = Paint::default();
      paint.set_anti_alias(true);
      paint
        .set_style(PaintStyle::Stroke)
        .set_stroke_width(*width)
        .set_stroke_cap(*cap)
        .set_stroke_join(*join)
        .set_color(*color);

      canvas.draw_path(&path, &paint);
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
      action,
      data,
      path,
      line_style,
      fill_style,
      area,
    } => {
      match area {
        TouchAreaShape::Circle(r) => {
          // canvas.set_transform(tr);

          if let Some((color, width)) = line_style {
            let mut paint = Paint::default();
            paint.set_anti_alias(true);
            paint
              .set_style(PaintStyle::Stroke)
              .set_stroke_width(*width)
              .set_stroke_cap(Cap::Round)
              .set_stroke_join(Join::Round)
              .set_color(*color);

            canvas.draw_circle((position.x, position.y), *r, &paint);
          }
          if let Some(color) = fill_style {
            let mut paint = Paint::default();
            paint.set_anti_alias(true);
            paint.set_style(PaintStyle::Fill).set_color(*color);

            canvas.draw_circle((position.x, position.y), *r, &paint);
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

          if let Some((color, width)) = line_style {
            let mut paint = Paint::default();
            paint.set_anti_alias(true);
            paint
              .set_style(PaintStyle::Stroke)
              .set_stroke_width(*width)
              .set_stroke_cap(Cap::Round)
              .set_stroke_join(Join::Round)
              .set_color(*color);

            canvas.draw_rect(rect_path, &paint);
          }
          if let Some(color) = fill_style {
            let mut paint = Paint::default();
            paint.set_anti_alias(true);
            paint.set_style(PaintStyle::Fill).set_color(*color);

            canvas.draw_rect(rect_path, &paint);
          }
        }
      }
      touches::add_touch_area(
        position.to_owned(),
        area.to_owned(),
        (**action).to_owned(),
        (**path).to_owned(),
        (**data).to_owned(),
        tr,
      );
    }
    Shape::KeyListener {
      key,
      action,
      path,
      data,
    } => {
      key_listener::add_key_listener(
        key.to_owned(),
        (**action).to_owned(),
        (**path).to_owned(),
        (**data).to_owned(),
      );
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

      if let Some((color, width)) = line_style {
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint
          .set_style(PaintStyle::Stroke)
          .set_stroke_width(*width)
          .set_stroke_cap(Cap::Round)
          .set_stroke_join(Join::Round)
          .set_color(*color);

        canvas.draw_path(&path, &paint);
      }

      if let Some(color) = fill_style {
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Fill).set_color(*color);

        canvas.draw_path(&path, &paint);
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
          fill_style: read_some_color(m, "fill-color")?,
          line_style: extract_line_style(m)?,
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
            fill_style: read_some_color(m, "fill-color")?,
            line_style: extract_line_style(m)?,
          })
        }
        "circle" => Ok(Shape::Circle {
          position: read_position(m, "position")?,
          radius: read_f32(m, "radius")?,
          fill_style: read_some_color(m, "fill-color")?,
          line_style: extract_line_style(m)?,
        }),
        "ellipse" => Ok(Shape::Ellipse {
          position: read_position(m, "position")?,
          radius_x: read_non_negative_f32(m, "radius-x")?,
          radius_y: read_non_negative_f32(m, "radius-y")?,
          fill_style: read_some_color(m, "fill-color")?,
          line_style: extract_line_style(m)?,
        }),
        "arc" => Ok(Shape::Arc {
          position: read_position(m, "position")?,
          radius_x: read_non_negative_f32(m, "radius-x")?,
          radius_y: read_non_negative_f32(m, "radius-y")?,
          start_angle: read_f32(m, "start-angle")?,
          sweep_angle: read_f32(m, "sweep-angle")?,
          use_center: read_bool(m, "use-center?")?,
          fill_style: read_some_color(m, "fill-color")?,
          line_style: extract_line_style(m)?,
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
          fill_style: read_some_color(m, "fill-color")?,
          line_style: extract_line_style(m)?,
        }),
        "text" => {
          Ok(Shape::Text {
            text: read_string(m, "text")?,
            position: read_position(m, "position")?,
            size: read_f32(m, "size")?,
            color: read_color(m, "color")?,
            // weight: read_string(m, "weight")?, // TODO
            align: read_text_align(m, "align")?,
          })
        }
        "polyline" => Ok(Shape::Polyline {
          position: read_position(m, "position")?,
          join: read_line_join(m, "join")?,
          cap: read_line_cap(m, "cap")?,
          skip_first: read_bool(m, "skip-first?")?,
          stops: read_points(m, "stops")?,
          color: read_color(m, "color")?,
          width: read_f32(m, "width")?,
        }),
        "touch-area" => Ok(Shape::TouchArea {
          path: Box::new(m.get(&tag("path")).unwrap_or(&Edn::Nil).to_owned()),
          action: Box::new(m.get(&tag("action")).unwrap_or(&Edn::Nil).to_owned()),
          data: Box::new(m.get(&tag("data")).unwrap_or(&Edn::Nil).to_owned()),
          position: read_position(m, "position")?,
          area: extract_touch_area_shape(m)?,
          fill_style: read_some_color(m, "fill-color")?,
          line_style: extract_line_style(m)?,
        }),
        "key-listener" => Ok(Shape::KeyListener {
          key: read_string(m, "key")?,
          path: Box::new(m.get(&tag("path")).unwrap_or(&Edn::Nil).to_owned()),
          action: Box::new(m.get(&tag("action")).unwrap_or(&Edn::Nil).to_owned()),
          data: Box::new(m.get(&tag("data")).unwrap_or(&Edn::Nil).to_owned()),
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
  }

  #[test]
  fn supports_explicit_path_close() {
    assert_eq!(extract_paint_op(&[Edn::tag("close-path")]), Ok(PaintPathTo::Close));
    assert!(extract_paint_op(&[Edn::tag("close"), Edn::Nil]).is_err());
  }
}

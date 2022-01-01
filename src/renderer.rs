use crate::touches;
use std::sync::RwLock;

use euclid::{Angle, Point2D, Vector2D};

use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;

use cirru_edn::Edn;

type Transform = euclid::default::Transform2D<f32>;

use skia_safe::paint::{Cap, Join};
use skia_safe::{Color, Font, Paint, PaintStyle, Path, Point, Rect, TextBlob, Typeface};

use crate::{
  color::extract_color,
  extracter::{
    extract_line_style, extract_position, extract_touch_area_shape, load_kwd, read_bool, read_color, read_f32,
    read_line_cap, read_line_join, read_points, read_position, read_some_color, read_string, read_text_align,
  },
  key_listener,
  primes::{LineCap, LineJoin, PaintPathTo, Shape, TouchAreaShape},
};

// TODO Stack

pub fn reset_page(canvas: &mut skia_safe::canvas::Canvas, color: Color) -> Result<(), String> {
  touches::reset_touches_stack();
  key_listener::reset_listeners_stack();
  // canvas.clear(SolidSource {
  //   r: color.r(),
  //   g: color.g(),
  //   b: color.b(),
  //   a: color.a(),
  // });
  println!("TODO reset page color");
  Ok(())
}

lazy_static! {
  static ref PREV_MESSAGES: RwLock<Vec<(Box<str>, Edn)>> = RwLock::new(vec![]);
}

pub fn draw_page(
  canvas: &mut skia_safe::canvas::Canvas,
  base_messages: Vec<(Box<str>, Edn)>,
  cost: f64,
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
      draw_cost(canvas, cost)
    } else {
      Ok(())
    }
  } else {
    Ok(())
  }
}

fn draw_cost(canvas: &mut skia_safe::canvas::Canvas, cost: f64) -> Result<(), String> {
  // let font = SystemSource::new()
  //   .select_best_match(&[FamilyName::SansSerif], &Properties::new())
  //   .unwrap()
  //   .load()
  //   .unwrap();

  // canvas.draw_text(
  //   &font,
  //   14.,
  //   &format!("{}ms", cost),
  //   Point::new(10., 190.),
  //   &Source::Solid(SolidSource {
  //     r: 0xff,
  //     g: 0xff,
  //     b: 0xff,
  //     a: 0xff,
  //   }),
  //   &DrawOptions::new(),
  // );

  Ok(())
}

// fn turn_color_source(color: &Color) -> Source {
//   Source::Solid(SolidSource::from_unpremultiplied_argb(
//     color.a(),
//     color.r(),
//     color.g(),
//     color.b(),
//   ))
// }

fn draw_shape(canvas: &mut skia_safe::canvas::Canvas, tree: &Shape, tr: &Transform) -> Result<(), String> {
  match tree {
    Shape::Rectangle {
      position,
      width,
      height,
      line_style,
      fill_style,
    } => {
      let mut rect_path = Rect::from_xywh(position.x, position.y, *width, *height);

      // canvas.set_transform(tr);

      if let Some((color, width)) = line_style {
        let mut paint = Paint::default();
        paint
          .set_style(PaintStyle::Stroke)
          .set_stroke_width(*width)
          .set_stroke_cap(Cap::Round)
          .set_stroke_join(Join::Round)
          .set_color(0x_ff_ff_ff_ff);
        // .set_color(color.into());

        canvas.draw_rect(&rect_path, &paint);
      }
      if let Some(color) = fill_style {
        let mut paint = Paint::default();
        paint
          .set_style(PaintStyle::Fill)
          // .set_color(color.into())
          .set_color(0x_ff_ff_ff_ff);

        canvas.draw_rect(&rect_path, &paint);
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

        paint
          .set_style(PaintStyle::Stroke)
          .set_stroke_width(*width)
          .set_stroke_cap(Cap::Round)
          .set_stroke_join(Join::Round)
          // .set_color(color.into())
          .set_color(0x_ff_ff_ff_ff);

        canvas.draw_circle((position.x, position.y), *radius, &paint);
      }
      if let Some(color) = fill_style {
        let mut paint = Paint::default();
        paint
          .set_style(PaintStyle::Fill)
          // .set_color(color.into())
          .set_color(0x_ff_ff_ff_ff);

        canvas.draw_circle((position.x, position.y), *radius, &paint);
      }
    }
    Shape::Group { position, children } => {
      for child in children {
        let pos = Vector2D::new(position.x, position.y);
        let t1 = Transform::identity().then_translate(pos);
        draw_shape(canvas, child, &t1.then(tr))?;
      }
    }
    Shape::Text {
      text,
      position,
      size,
      color,
      // weight: _w,
      align: _a,
    } => {
      // canvas.set_transform(tr);
      // https://github.com/jrmuizel/raqote/issues/179
      // for now we have to by pass bug in text rendering
      let text_pos = tr.transform_point(Point2D::new(position.x, position.y));
      // canvas.set_transform(&Transform::identity());

      let font = Font::new(Typeface::default(), *size);
      let text_blob = TextBlob::new(text, &font).unwrap();

      let mut paint = Paint::default();
      paint
        .set_style(PaintStyle::Fill)
        // .set_color(color.into())
        .set_color(0x_ff_ff_ff_ff);

      canvas.draw_text_blob(text_blob, (text_pos.x, text_pos.y), &paint);
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
      let mut path = Path::default();
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

      let mut paint = Paint::default();
      paint
        .set_style(PaintStyle::Stroke)
        .set_stroke_width(*width)
        .set_stroke_cap(Cap::Round)
        .set_stroke_join(Join::Round)
        // .set_color(color.into())
        .set_color(0x_ff_ff_ff_ff);

      canvas.draw_path(&path, &paint);
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
            paint
              .set_style(PaintStyle::Stroke)
              .set_stroke_width(*width)
              .set_stroke_cap(Cap::Round)
              .set_stroke_join(Join::Round)
              // .set_color(color.into());
              .set_color(0x_ff_ff_ff_ff);

            canvas.draw_circle((position.x, position.y), *r, &paint);
          }
          if let Some(color) = fill_style {
            let mut paint = Paint::default();
            paint.set_style(PaintStyle::Fill).set_color(0x_ff_ff_ff_ff);
            // set_color(color.into());

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
            paint
              .set_style(PaintStyle::Stroke)
              .set_stroke_width(*width)
              .set_stroke_cap(Cap::Round)
              .set_stroke_join(Join::Round)
              // .set_color(color.into());
              .set_color(0x_ff_ff_ff_ff);

            canvas.draw_rect(&rect_path, &paint);
          }
          if let Some(color) = fill_style {
            let mut paint = Paint::default();
            paint
              .set_style(PaintStyle::Fill)
              // .set_color(color.into());
              .set_color(0x_ff_ff_ff_ff);

            canvas.draw_rect(&rect_path, &paint);
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
      let mut path = Path::default();
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
        }
      }
      if fill_style.is_some() {
        path.close();
      }

      if let Some((color, width)) = line_style {
        let mut paint = Paint::default();
        paint
          .set_style(PaintStyle::Stroke)
          .set_stroke_width(*width)
          .set_stroke_cap(Cap::Round)
          .set_stroke_join(Join::Round)
          // .set_color(color.into());
          .set_color(0x_ff_ff_ff_ff);

        canvas.draw_path(&path, &paint);
      }

      if let Some(color) = fill_style {
        let mut paint = Paint::default();
        paint
          .set_style(PaintStyle::Fill)
          // .set_color(color.into());
          .set_color(0x_ff_ff_ff_ff);
        canvas.draw_path(&path, &paint);
      }
    }
    Shape::Scale { factor, children } => {
      let t1 = Transform::identity().then_scale(factor.to_owned(), factor.to_owned());
      for child in children {
        draw_shape(canvas, child, &t1.then(tr))?;
      }
    }
    Shape::Rotate { radius, children } => {
      let t1 = Transform::identity().then_rotate(Angle {
        radians: radius.to_owned(),
      });
      for child in children {
        draw_shape(canvas, child, &t1.then(tr))?;
      }
    }
    Shape::Translate { x, y, children } => {
      let v = Vector2D::new(x.to_owned(), y.to_owned());
      let t1 = Transform::identity().then_translate(v);
      for child in children {
        draw_shape(canvas, child, &t1.then(tr))?;
      }
    }
  }
  Ok(())
}

fn extract_shape(tree: &Edn) -> Result<Shape, String> {
  // println!("extracting shape: {:?} -- {:?}", load_kwd("type"), tree);
  match tree {
    Edn::Map(m) => match m.get(&load_kwd("type")) {
      Some(Edn::Keyword(name)) => match &*name.to_str() {
        "rectangle" | "rect" => Ok(Shape::Rectangle {
          position: read_position(m, "position")?,
          width: read_f32(m, "width")?,
          height: read_f32(m, "height")?,
          fill_style: read_some_color(m, "fill-color")?,
          line_style: extract_line_style(m)?,
        }),
        "circle" => Ok(Shape::Circle {
          position: read_position(m, "position")?,
          radius: read_f32(m, "radius")?,
          fill_style: read_some_color(m, "fill-color")?,
          line_style: extract_line_style(m)?,
        }),
        "group" => {
          let c = m.get(&load_kwd("children"));
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
          path: extract_paint_path(m.get(&load_kwd("path")).unwrap_or(&Edn::Nil))?,
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
          path: Box::new(m.get(&load_kwd("path")).unwrap_or(&Edn::Nil).to_owned()),
          action: Box::new(m.get(&load_kwd("action")).unwrap_or(&Edn::Nil).to_owned()),
          data: Box::new(m.get(&load_kwd("data")).unwrap_or(&Edn::Nil).to_owned()),
          position: read_position(m, "position")?,
          area: extract_touch_area_shape(m)?,
          fill_style: read_some_color(m, "fill-color")?,
          line_style: extract_line_style(m)?,
        }),
        "key-listener" => Ok(Shape::KeyListener {
          key: read_string(m, "key")?,
          path: Box::new(m.get(&load_kwd("path")).unwrap_or(&Edn::Nil).to_owned()),
          action: Box::new(m.get(&load_kwd("action")).unwrap_or(&Edn::Nil).to_owned()),
          data: Box::new(m.get(&load_kwd("data")).unwrap_or(&Edn::Nil).to_owned()),
        }),
        "rotate" => {
          let c = m.get(&load_kwd("children"));
          let children = extract_children(c)?;

          Ok(Shape::Rotate {
            radius: read_f32(m, "radius")?,
            children,
          })
        }
        "scale" => {
          let c = m.get(&load_kwd("children"));
          let children = extract_children(c)?;

          Ok(Shape::Scale {
            factor: read_f32(m, "factor")?,
            children,
          })
        }
        "translate" => {
          let c = m.get(&load_kwd("children"));
          let children = extract_children(c)?;

          Ok(Shape::Translate {
            x: read_f32(m, "x")?,
            y: read_f32(m, "y")?,
            children,
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

fn extract_children(children: Option<&Edn>) -> Result<Vec<Shape>, String> {
  let empty_group = Shape::Group {
    position: Vector2D::new(0.0, 0.0),
    children: vec![],
  };
  match children {
    Some(Edn::List(xs)) => {
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
    Some(a) => return Err(format!("invalid children: {}", a)),
    None => Ok(vec![]),
  }
}

fn extract_paint_path(data: &Edn) -> Result<Vec<PaintPathTo>, String> {
  if let Edn::List(xs) = data {
    let mut ys = vec![];
    for x in xs {
      match x {
        Edn::List(zs) => ys.push(extract_paint_op(zs)?),
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
    let op: Box<str> = match &xs[0] {
      Edn::Keyword(s) => s.to_str(),
      Edn::Str(s) => s.to_owned(),
      _ => return Err(format!("unknown paint op value: {}", xs[0])),
    };
    match &*op {
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
      // "close-path" => Ok(PaintPathTo::ClosePath),
      _ => Err(format!("unknown paint op: {}", op)),
    }
  } else {
    Err(String::from("empty is not paint op"))
  }
}

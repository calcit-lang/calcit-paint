use glam::Vec2;

use crate::touches;
use calcit_runner::program;
use calcit_runner::Calcit;

use euclid::{Angle, Vector2D};

use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;

use raqote::{
  Color, DrawOptions, DrawTarget, LineCap, LineJoin, PathBuilder, Point, SolidSource, Source, StrokeStyle, Transform,
};

use crate::{
  color::extract_color,
  extracter::{
    extract_line_style, extract_position, extract_touch_area_shape, read_bool, read_color, read_f32, read_line_cap,
    read_line_join, read_points, read_position, read_some_color, read_string, read_text_align,
  },
  key_listener,
  primes::{PaintPathTo, Shape, TouchAreaShape},
};

// TODO Stack

pub fn reset_page(draw_target: &mut DrawTarget, color: Color) -> Result<(), String> {
  touches::reset_touches_stack();
  key_listener::reset_listeners_stack();
  draw_target.clear(SolidSource {
    r: color.r(),
    g: color.g(),
    b: color.b(),
    a: color.a(),
  });
  Ok(())
}

pub fn draw_page(draw_target: &mut DrawTarget, cost: f64) -> Result<(), String> {
  let messages = program::take_ffi_messages().unwrap();
  // clear scene and start drawing
  if !messages.is_empty() {
    let mut shown_shape = false;
    for (call_op, args) in messages {
      // println!("op: {}", call_op);
      match (call_op.as_str(), args.get(0)) {
        ("render-canvas!", Some(tree)) => {
          shown_shape = true;
          match extract_shape(tree) {
            Ok(shape) => draw_shape(draw_target, &shape, &Transform::identity())?,
            Err(failure) => {
              println!("Failed to extract shape {}", failure)
            }
          }
        }
        ("reset-canvas!", Some(tree)) => {
          reset_page(draw_target, extract_color(tree)?)?;
        }
        _ => println!("Unknown op: {}", call_op),
      }
    }
    if shown_shape {
      draw_cost(draw_target, cost)
    } else {
      Ok(())
    }
  } else {
    Ok(())
  }
}

fn draw_cost(draw_target: &mut DrawTarget, cost: f64) -> Result<(), String> {
  let font = SystemSource::new()
    .select_best_match(&[FamilyName::SansSerif], &Properties::new())
    .unwrap()
    .load()
    .unwrap();

  draw_target.draw_text(
    &font,
    14.,
    &format!("{}ms", cost),
    Point::new(10., 190.),
    &Source::Solid(SolidSource {
      r: 0xff,
      g: 0xff,
      b: 0xff,
      a: 0xff,
    }),
    &DrawOptions::new(),
  );

  Ok(())
}

fn turn_color_source(color: &Color) -> Source {
  Source::Solid(SolidSource::from_unpremultiplied_argb(
    color.a(),
    color.r(),
    color.g(),
    color.b(),
  ))
}

fn draw_shape(draw_target: &mut DrawTarget, tree: &Shape, tr: &Transform) -> Result<(), String> {
  match tree {
    Shape::Rectangle {
      position,
      width,
      height,
      line_style,
      fill_style,
    } => {
      let mut pb = PathBuilder::new();
      pb.rect(position.x, position.y, *width, *height);
      let path = pb.finish();

      draw_target.set_transform(tr);

      if let Some((color, width)) = line_style {
        draw_target.stroke(
          &path,
          &turn_color_source(color),
          &StrokeStyle {
            cap: LineCap::Round,
            join: LineJoin::Miter,
            width: width.to_owned(),
            miter_limit: 4.,
            dash_array: Vec::new(),
            dash_offset: 0.,
          },
          &DrawOptions::new(),
        )
      }
      if let Some(color) = fill_style {
        draw_target.fill(&path, &turn_color_source(color), &DrawOptions::new());
      }
    }
    Shape::Circle {
      position,
      radius,
      line_style,
      fill_style,
    } => {
      let mut pb = PathBuilder::new();
      pb.arc(
        position.x,
        position.y,
        radius.to_owned(),
        0.0,
        2.0 * std::f64::consts::PI as f32,
      );
      let path = pb.finish();

      draw_target.set_transform(tr);

      if let Some((color, width)) = line_style {
        draw_target.stroke(
          &path,
          &turn_color_source(color),
          &StrokeStyle {
            cap: LineCap::Round,
            join: LineJoin::Miter,
            width: width.to_owned(),
            miter_limit: 4.,
            dash_array: Vec::new(),
            dash_offset: 0.,
          },
          &DrawOptions::new(),
        )
      }
      if let Some(color) = fill_style {
        draw_target.fill(&path, &turn_color_source(color), &DrawOptions::new());
      }
    }
    Shape::Group { position, children } => {
      for child in children {
        let pos = Vector2D::new(position.x, position.y);
        let t1 = Transform::identity().then_translate(pos);
        draw_shape(draw_target, child, &t1.then(tr))?;
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
      draw_target.set_transform(tr);

      let font = SystemSource::new()
        .select_best_match(&[FamilyName::SansSerif], &Properties::new())
        .unwrap()
        .load()
        .unwrap();

      draw_target.draw_text(
        &font,
        size.to_owned(),
        &text.to_owned(),
        Point::new(position.x, position.y),
        &turn_color_source(color),
        &DrawOptions::new(),
      );
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
      let mut pb = PathBuilder::new();
      draw_target.set_transform(tr);

      if *skip_first && !stops.is_empty() {
        pb.move_to(position.x + stops[0][0], position.y + stops[0][1]);
      } else {
        pb.move_to(position.x, position.y);
      }
      for stop in stops {
        pb.line_to(position.x + stop[0], position.y + stop[1]);
      }
      let path = pb.finish();

      draw_target.stroke(
        &path,
        &turn_color_source(color),
        &StrokeStyle {
          cap: cap.to_owned(),
          join: join.to_owned(),
          width: width.to_owned(),
          miter_limit: 4.,
          dash_array: Vec::new(),
          dash_offset: 0.,
        },
        &DrawOptions::new(),
      );
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
          let mut pb = PathBuilder::new();
          pb.arc(
            position.x,
            position.y,
            r.to_owned(),
            0.0,
            2.0 * std::f64::consts::PI as f32,
          );
          let path = pb.finish();
          draw_target.set_transform(tr);

          if let Some((color, width)) = line_style {
            draw_target.stroke(
              &path,
              &turn_color_source(color),
              &StrokeStyle {
                cap: LineCap::Round,
                join: LineJoin::Miter,
                width: width.to_owned(),
                miter_limit: 4.,
                dash_array: Vec::new(),
                dash_offset: 0.,
              },
              &DrawOptions::new(),
            );
          }
          if let Some(color) = fill_style {
            draw_target.fill(&path, &turn_color_source(color), &DrawOptions::new());
          }
        }
        TouchAreaShape::Rect(dx, dy) => {
          let mut pb = PathBuilder::new();
          pb.rect(
            position.x - *dx,
            position.y - *dy,
            2. * dx.to_owned(),
            2. * dy.to_owned(),
          );
          let path = pb.finish();
          draw_target.set_transform(tr);

          if let Some((color, width)) = line_style {
            draw_target.stroke(
              &path,
              &turn_color_source(color),
              &StrokeStyle {
                cap: LineCap::Round,
                join: LineJoin::Miter,
                width: width.to_owned(),
                miter_limit: 4.,
                dash_array: Vec::new(),
                dash_offset: 0.,
              },
              &DrawOptions::new(),
            );
          }
          if let Some(color) = fill_style {
            draw_target.fill(&path, &turn_color_source(color), &DrawOptions::new());
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
      let mut pb = PathBuilder::new();
      let x0 = position.x;
      let y0 = position.y;
      pb.move_to(x0, y0);
      draw_target.set_transform(tr);

      for p in ops_path {
        match p {
          PaintPathTo::Move(a) => {
            pb.move_to(x0 + a.x, y0 + a.y);
          }
          PaintPathTo::Line(a) => {
            pb.line_to(x0 + a.x, y0 + a.y);
          }
          PaintPathTo::QuadraticBezier(a, b) => {
            pb.quad_to(x0 + a.x, y0 + a.y, x0 + b.x, y0 + b.y);
          }
          PaintPathTo::CubicBezier(a, b, c) => pb.cubic_to(x0 + a.x, y0 + a.y, x0 + b.x, y0 + b.y, x0 + c.x, y0 + c.y),
        }
      }
      if fill_style.is_some() {
        pb.close();
      }
      let path = pb.finish();

      if let Some((color, width)) = line_style {
        draw_target.stroke(
          &path,
          &turn_color_source(color),
          &StrokeStyle {
            cap: LineCap::Round,
            join: LineJoin::Miter,
            width: width.to_owned(),
            miter_limit: 4.,
            dash_array: Vec::new(),
            dash_offset: 0.,
          },
          &DrawOptions::new(),
        );
      }

      if let Some(color) = fill_style {
        draw_target.fill(&path, &turn_color_source(color), &DrawOptions::new());
      }
    }
    Shape::Scale { factor, children } => {
      let t1 = Transform::identity().then_scale(factor.to_owned(), factor.to_owned());
      for child in children {
        draw_shape(draw_target, child, &t1.then(tr))?;
      }
    }
    Shape::Rotate { radius, children } => {
      let t1 = Transform::identity().then_rotate(Angle {
        radians: radius.to_owned(),
      });
      for child in children {
        draw_shape(draw_target, child, &t1.then(tr))?;
      }
    }
    Shape::Translate { x, y, children } => {
      let point = Vector2D::new(x.to_owned(), y.to_owned());
      let t1 = Transform::identity().then_translate(point);
      for child in children {
        draw_shape(draw_target, child, &t1.then(tr))?;
      }
    }
  }
  Ok(())
}

fn extract_shape(tree: &Calcit) -> Result<Shape, String> {
  match tree {
    Calcit::Map(m) => match m.get(&Calcit::Keyword(String::from("type"))) {
      Some(Calcit::Keyword(name)) => match name.as_str() {
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
          let c = m.get(&Calcit::Keyword(String::from("children")));
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
          path: extract_paint_path(m.get(&Calcit::Keyword(String::from("path"))).unwrap_or(&Calcit::Nil))?,
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
          path: Box::new(
            m.get(&Calcit::Keyword(String::from("path")))
              .unwrap_or(&Calcit::Nil)
              .to_owned(),
          ),
          action: Box::new(
            m.get(&Calcit::Keyword(String::from("action")))
              .unwrap_or(&Calcit::Nil)
              .to_owned(),
          ),
          data: Box::new(
            m.get(&Calcit::Keyword(String::from("data")))
              .unwrap_or(&Calcit::Nil)
              .to_owned(),
          ),
          position: read_position(m, "position")?,
          area: extract_touch_area_shape(m)?,
          fill_style: read_some_color(m, "fill-color")?,
          line_style: extract_line_style(m)?,
        }),
        "key-listener" => Ok(Shape::KeyListener {
          key: read_string(m, "key")?,
          path: Box::new(
            m.get(&Calcit::Keyword(String::from("path")))
              .unwrap_or(&Calcit::Nil)
              .to_owned(),
          ),
          action: Box::new(
            m.get(&Calcit::Keyword(String::from("action")))
              .unwrap_or(&Calcit::Nil)
              .to_owned(),
          ),
          data: Box::new(
            m.get(&Calcit::Keyword(String::from("data")))
              .unwrap_or(&Calcit::Nil)
              .to_owned(),
          ),
        }),
        "rotate" => {
          let c = m.get(&Calcit::Keyword(String::from("children")));
          let children = extract_children(c)?;

          Ok(Shape::Rotate {
            radius: read_f32(m, "radius")?,
            children,
          })
        }
        "scale" => {
          let c = m.get(&Calcit::Keyword(String::from("children")));
          let children = extract_children(c)?;

          Ok(Shape::Scale {
            factor: read_f32(m, "factor")?,
            children,
          })
        }
        "translate" => {
          let c = m.get(&Calcit::Keyword(String::from("children")));
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
    Calcit::Nil => Ok(Shape::Group {
      position: Vec2::new(0.0, 0.0),
      children: vec![],
    }),
    _ => Err(format!("expected a map, got {}", tree)),
  }
}

fn extract_children(children: Option<&Calcit>) -> Result<Vec<Shape>, String> {
  let empty_group = Shape::Group {
    position: Vec2::new(0.0, 0.0),
    children: vec![],
  };
  match children {
    Some(Calcit::List(xs)) => {
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

fn extract_paint_path(data: &Calcit) -> Result<Vec<PaintPathTo>, String> {
  if let Calcit::List(xs) = data {
    let mut ys = vec![];
    for x in xs {
      match x {
        Calcit::List(zs) => ys.push(extract_paint_op(zs)?),
        _ => return Err(format!("expected single op in list, for {}", x)),
      }
    }
    Ok(ys)
  } else {
    Err(String::from("expected ops in list"))
  }
}

fn extract_paint_op(xs: &im::Vector<Calcit>) -> Result<PaintPathTo, String> {
  if !xs.is_empty() {
    match &xs[0] {
      Calcit::Keyword(s) | Calcit::Str(s) => match s.as_str() {
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
        _ => Err(format!("unknown paint op: {}", s)),
      },
      _ => Err(format!("unknown paint op value: {}", xs[0])),
    }
  } else {
    Err(String::from("empty is not paint op"))
  }
}

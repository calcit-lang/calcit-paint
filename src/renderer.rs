use ggez;
use glam::Vec2;

use ggez::graphics;
use ggez::graphics::{Color, DrawMode, DrawParam};
use ggez::graphics::{FillOptions, LineCap, LineJoin, StrokeOptions};
use ggez::{Context, GameError, GameResult};

use crate::{primes::path_add, touches};
use calcit_runner::program;
use calcit_runner::Calcit;

use crate::{
  color::extract_color,
  extracter::{
    extract_line_style, extract_touch_area_shape, read_bool, read_color, read_f32, read_line_cap, read_line_join,
    read_points, read_position, read_some_color, read_string, read_text_align,
  },
  key_listener,
  primes::{PaintOp, Shape, TouchAreaShape},
};

// TODO Stack

pub fn to_game_err(e: String) -> GameError {
  GameError::CustomError(e)
}

pub fn reset_page(ctx: &mut Context, color: Color) -> GameResult {
  println!("reset with color: {:?}", color);
  touches::reset_touches_stack();
  key_listener::reset_listeners_stack();
  graphics::clear(ctx, color);
  Ok(())
}

pub fn draw_page(ctx: &mut Context) -> GameResult {
  let messages = program::take_ffi_messages().unwrap();
  // clear scene and start drawing
  if !messages.is_empty() {
    println!("Calling draw_page");
    let mut shown_shape = false;
    for (call_op, args) in messages {
      println!("op: {}", call_op);
      match (call_op.as_str(), args.get(0)) {
        ("render-canvas!", Some(tree)) => {
          shown_shape = true;
          match extract_shape(&tree) {
            Ok(shape) => draw_shape(ctx, &shape, &Vec2::new(0.0, 0.0))?,
            Err(failure) => {
              println!("Failed to extract shape {}", failure)
            }
          }
        }
        ("reset-canvas!", Some(tree)) => {
          reset_page(ctx, extract_color(tree).map_err(to_game_err)?)?;
        }
        _ => println!("Unknown op: {}", call_op),
      }
    }
    if shown_shape {
      graphics::present(ctx)
    } else {
      Ok(())
    }
  } else {
    Ok(())
  }
}

fn draw_shape(ctx: &mut Context, tree: &Shape, base: &Vec2) -> GameResult {
  match tree {
    Shape::Rectangle {
      position,
      width,
      height,
      line_style,
      fill_style,
    } => {
      let rect = graphics::Rect::new(base.x + position.x, base.y + position.y, *width, *height);
      if let Some((color, width)) = line_style {
        let r1 = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::stroke(*width), rect, *color)?;
        graphics::draw(ctx, &r1, DrawParam::default())?;
      }
      if let Some(color) = fill_style {
        let r1 = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, *color)?;
        graphics::draw(ctx, &r1, DrawParam::default())?;
      }
    }
    Shape::Circle {
      position,
      radius,
      line_style,
      fill_style,
    } => {
      if let Some((color, width)) = line_style {
        let circle = graphics::Mesh::new_circle(
          ctx,
          DrawMode::stroke(*width),
          Vec2::new(0.0, 0.0),
          *radius,
          2.0,
          color.to_owned(),
        )?;
        graphics::draw(ctx, &circle, (path_add(position, base),))?;
      }
      if let Some(color) = fill_style {
        let circle = graphics::Mesh::new_circle(
          ctx,
          DrawMode::fill(),
          Vec2::new(0.0, 0.0),
          *radius,
          2.0,
          color.to_owned(),
        )?;
        graphics::draw(ctx, &circle, (path_add(position, base),))?;
      }
    }
    Shape::Group { position, children } => {
      for child in children {
        draw_shape(ctx, child, &path_add(position, base))?;
      }
    }
    Shape::Text {
      text,
      position,
      size,
      color,
      // weight: _w,
      // align: _a,
    } => {
      let mono_font = graphics::Font::new(ctx, "/SourceCodePro-Medium.ttf")?;
      let text_mesh = graphics::Text::new((text.as_str(), mono_font, *size));
      graphics::draw(
        ctx,
        &text_mesh,
        graphics::DrawParam::new()
          .dest(path_add(position, base))
          .color(color.to_owned()),
      )?;
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
      let mut points = stops.to_owned();
      if *skip_first && points.len() >= 1 {
        points.remove(0);
      }
      let points_mesh = graphics::Mesh::new_polyline(
        ctx,
        DrawMode::Stroke(
          StrokeOptions::default()
            .with_line_join(*join)
            .with_line_cap(*cap)
            .with_line_width(*width),
        ),
        stops,
        *color,
      )?;
      graphics::draw(
        ctx,
        &points_mesh,
        graphics::DrawParam::new()
          .dest(path_add(position, base))
          .color(color.to_owned()),
      )?;
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
          if let Some((color, width)) = line_style {
            let circle = graphics::Mesh::new_circle(
              ctx,
              DrawMode::stroke(*width),
              Vec2::new(0.0, 0.0),
              *r,
              2.0,
              color.to_owned(),
            )?;
            graphics::draw(ctx, &circle, (path_add(position, base),))?;
          }
          if let Some(color) = fill_style {
            let circle =
              graphics::Mesh::new_circle(ctx, DrawMode::fill(), Vec2::new(0.0, 0.0), *r, 2.0, color.to_owned())?;
            graphics::draw(ctx, &circle, (path_add(position, base),))?;
          }
        }
        TouchAreaShape::Rect(dx, dy) => {
          let rect = graphics::Rect::new(base.x + position.x - dx, base.y + position.y - dy, 2.0 * dx, 2.0 * dy);
          if let Some((color, width)) = line_style {
            let r1 = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::stroke(*width), rect, *color)?;
            graphics::draw(ctx, &r1, DrawParam::default())?;
          }
          if let Some(color) = fill_style {
            let r1 = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, *color)?;
            graphics::draw(ctx, &r1, DrawParam::default())?;
          }
        }
      }
      touches::add_touch_area(
        path_add(position, base),
        area.to_owned(),
        action.to_owned(),
        path.to_owned(),
        data.to_owned(),
      );
    }
    Shape::KeyListener {
      key,
      action,
      path,
      data,
    } => {
      key_listener::add_key_listener(key.to_owned(), action.to_owned(), path.to_owned(), data.to_owned());
    }
    Shape::PaintOps { .. } => {
      println!("TODO ops {:?}", tree)
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
          let children = match m.get(&Calcit::Keyword(String::from("children"))) {
            Some(Calcit::List(xs)) => {
              let mut ys = vec![];
              for x in xs {
                match extract_shape(&x) {
                  Ok(v) => ys.push(v),
                  Err(failure) => {
                    println!("Failed extracting: {}\n  in {}", failure, x);
                    ys.push(Shape::Group {
                      position: read_position(m, "position")?,
                      children: vec![],
                    })
                  }
                }
              }
              ys
            }
            Some(a) => return Err(format!("invalid children: {}", a)),
            None => vec![],
          };
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
          ops: extract_ops(m.get(&Calcit::Keyword(String::from("ops"))).unwrap_or(&Calcit::Nil))?,
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
            // align: read_text_align(m, "align")?,
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
          path: m
            .get(&Calcit::Keyword(String::from("path")))
            .unwrap_or(&Calcit::Nil)
            .to_owned(),
          action: m
            .get(&Calcit::Keyword(String::from("action")))
            .unwrap_or(&Calcit::Nil)
            .to_owned(),
          data: m
            .get(&Calcit::Keyword(String::from("data")))
            .unwrap_or(&Calcit::Nil)
            .to_owned(),
          position: read_position(m, "position")?,
          area: extract_touch_area_shape(m)?,
          fill_style: read_some_color(m, "fill-color")?,
          line_style: extract_line_style(m)?,
        }),
        "key-listener" => Ok(Shape::KeyListener {
          key: read_string(m, "key")?,
          path: m
            .get(&Calcit::Keyword(String::from("path")))
            .unwrap_or(&Calcit::Nil)
            .to_owned(),
          action: m
            .get(&Calcit::Keyword(String::from("action")))
            .unwrap_or(&Calcit::Nil)
            .to_owned(),
          data: m
            .get(&Calcit::Keyword(String::from("data")))
            .unwrap_or(&Calcit::Nil)
            .to_owned(),
        }),
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

fn extract_ops(data: &Calcit) -> Result<Vec<PaintOp>, String> {
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
    Err(format!("expected ops in list"))
  }
}

fn extract_paint_op(xs: &im::Vector<Calcit>) -> Result<PaintOp, String> {
  if xs.len() >= 1 {
    match &xs[0] {
      Calcit::Keyword(s) | Calcit::Str(s) => match s.as_str() {
        // TODO refactor to fit ggez
        // "stroke" => ,
        // "fill" => ,
        // "stroke-preserve" => ,
        // "fill-preserve" => ,
        // "line-width" => ,
        // "hsl" => ,
        // "move-to" => ,
        // "line-to" => ,
        // "relative-line-to" => ,
        _ => Err(format!("TODO paint op: {}", s)),
      },
      _ => Err(format!("unknown paint op value: {}", xs[0])),
    }
  } else {
    Err(format!("empty is not paint op"))
  }
}

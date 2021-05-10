use ggez;
use glam::Vec2;

use ggez::graphics;
use ggez::graphics::{Color, DrawMode, DrawParam};
use ggez::{Context, GameError, GameResult};

use calcit_runner::program;
use calcit_runner::Calcit;

use crate::{
  color::extract_color,
  extracter::{
    extract_style, extract_touch_area_shape, read_bool, read_color, read_f32, read_line_join,
    read_points, read_position, read_string, read_text_align,
  },
  primes::{PaintOp, Shape, ShapeStyle},
};

// TODO Stack

pub fn to_game_err(e: String) -> GameError {
  GameError::CustomError(e)
}

pub fn reset_page(ctx: &mut Context, color: Color) -> GameResult {
  println!("reset with color: {:?}", color);
  graphics::clear(ctx, color);
  Ok(())
}

pub fn draw_page(ctx: &mut Context) -> GameResult {
  let messages = program::take_ffi_messages().unwrap();
  // clear scene and start drawing

  for (call_op, args) in messages {
    match (call_op.as_str(), args.get(0)) {
      ("render-canvas!", Some(tree)) => match extract_shape(&tree) {
        Ok(shape) => draw_shape(ctx, &shape, Vec2::new(0.0, 0.0))?,
        Err(failure) => {
          println!("Failed to extract shape {}, {}", tree, failure)
        }
      },
      ("reset-canvas!", Some(tree)) => {
        reset_page(ctx, extract_color(tree).map_err(to_game_err)?)?;
      }
      _ => println!("Unknown op: {}", call_op),
    }
  }
  println!("present");
  graphics::present(ctx)
}

fn draw_shape(ctx: &mut Context, tree: &Shape, base: Vec2) -> GameResult {
  match tree {
    Shape::Rectangle {
      position,
      width,
      height,
      style,
    } => {
      let rect = graphics::Rect::new(base.x + position.x, base.y + position.y, *width, *height);
      match style {
        ShapeStyle::Line { color, width: _w } => {
          let r1 = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, *color)?;
          graphics::draw(ctx, &r1, DrawParam::default())?;
        }
        ShapeStyle::Fill { color } => {
          let r1 = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, *color)?;
          graphics::draw(ctx, &r1, DrawParam::default())?;
        }
      }
    }
    Shape::Circle {
      position,
      radius,
      style,
    } => {
      let (mode, color) = match style {
        ShapeStyle::Line { color, width } => (DrawMode::stroke(*width), *color),
        ShapeStyle::Fill { color } => (DrawMode::fill(), *color),
      };

      let circle = graphics::Mesh::new_circle(ctx, mode, Vec2::new(0.0, 0.0), *radius, 2.0, color)?;
      graphics::draw(ctx, &circle, (*position,))?;
    }
    Shape::Group { position, children } => {
      for child in children {
        draw_shape(ctx, child, position.to_owned())?;
      }
    }
    _ => println!("TODO {:?}", tree),
  }
  Ok(())
}

fn extract_shape(tree: &Calcit) -> Result<Shape, String> {
  match tree {
    Calcit::Map(m) => match m.get(&Calcit::Keyword(String::from("kind"))) {
      Some(Calcit::Keyword(name)) => match name.as_str() {
        "rectangle" => Ok(Shape::Rectangle {
          position: read_position(m, "position")?,
          width: read_f32(m, "width")?,
          height: read_f32(m, "height")?,
          style: extract_style(m)?,
        }),
        "circle" => Ok(Shape::Circle {
          position: read_position(m, "position")?,
          radius: read_f32(m, "radius")?,
          style: extract_style(m)?,
        }),
        "group" => {
          let children = match m.get(&Calcit::Keyword(String::from("children"))) {
            Some(Calcit::List(xs)) => {
              let mut ys = vec![];
              for x in xs {
                ys.push(extract_shape(&x)?)
              }
              ys
            }
            Some(a) => return Err(format!("invalid children: {}", a)),
            None => vec![],
          };
          Ok(Shape::Group {
            position: Vec2::new(10.0, 10.0),
            children,
          })
        }
        "arc" => Ok(Shape::Arc {
          position: read_position(m, "position")?,
          radius: read_f32(m, "radius")?,
          from_angle: read_f32(m, "from-angle")?,
          to_angle: read_f32(m, "to-angle")?,
          negative: read_bool(m, "negative")?,
          style: extract_style(m)?,
        }),
        "ops" => Ok(Shape::PaintOps {
          position: read_position(m, "position")?,
          ops: extract_ops(
            m.get(&Calcit::Keyword(String::from("ops")))
              .unwrap_or(&Calcit::Nil),
          )?,
        }),
        "text" => {
          Ok(Shape::Text {
            text: read_string(m, "text")?,
            position: read_position(m, "position")?,
            font_size: read_f32(m, "font-size")?,
            font_weight: String::from("TODO"), // TODO
            color: read_color(m, "color")?,
            align: read_text_align(m, "align")?,
          })
        }
        "polyline" => Ok(Shape::Polyline {
          position: read_position(m, "position")?,
          line_join: read_line_join(m, "line-join")?,
          skip_first: read_bool(m, "skip-first?")?,
          stops: read_points(m, "points")?,
          style: extract_style(m)?,
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
          style: extract_style(m)?,
          radius: read_f32(m, "radius")?,
          area: extract_touch_area_shape(m)?,
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
      None => Err(String::from("nil kind")),
    },
    _ => Err(String::from("expected a map")),
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

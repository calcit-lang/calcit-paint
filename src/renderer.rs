use ggez;
use glam::Vec2;

use ggez::graphics;
use ggez::graphics::{Color, DrawMode, DrawParam};
use ggez::graphics::{FillOptions, LineCap, LineJoin, StrokeOptions};
use ggez::{Context, GameError, GameResult};

use crate::{primes::path_add, touches};
use calcit_runner::program;
use calcit_runner::Calcit;
use lyon::tessellation;
use lyon::tessellation::math::{point, Point};
use lyon::tessellation::VertexBuffers;

use crate::{
  color::extract_color,
  extracter::{
    extract_line_style, extract_position, extract_touch_area_shape, read_bool, read_color, read_f32, read_line_cap,
    read_line_join, read_points, read_position, read_some_color, read_string, read_text_align,
  },
  key_listener,
  primes::{PaintPath, Shape, TouchAreaShape},
};

// TODO Stack

pub fn to_game_err(e: String) -> GameError {
  GameError::CustomError(e)
}

pub fn reset_page(ctx: &mut Context, color: Color) -> GameResult {
  // println!("reset with color: {:?}", color);
  touches::reset_touches_stack();
  key_listener::reset_listeners_stack();
  graphics::clear(ctx, color);
  Ok(())
}

pub fn draw_page(ctx: &mut Context, cost: f64) -> GameResult {
  let messages = program::take_ffi_messages().unwrap();
  // clear scene and start drawing
  if !messages.is_empty() {
    // println!("Calling draw_page");
    let mut shown_shape = false;
    for (call_op, args) in messages {
      // println!("op: {}", call_op);
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
      draw_cost(ctx, cost)?;
      graphics::present(ctx)
    } else {
      Ok(())
    }
  } else {
    Ok(())
  }
}

fn draw_cost(ctx: &mut Context, cost: f64) -> GameResult {
  let mono_font = graphics::Font::new(ctx, "/SourceCodePro-Medium.ttf")?;
  let text_mesh = graphics::Text::new((format!("{}ms", cost), mono_font, 14.0));
  graphics::draw(
    ctx,
    &text_mesh,
    graphics::DrawParam::new()
      .dest(Vec2::new(10.0, 190.0))
      .color(Color::new(1.0, 1.0, 1.0, 0.3)),
  )
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
          0.1,
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
          0.1,
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
              0.1,
              color.to_owned(),
            )?;
            graphics::draw(ctx, &circle, (path_add(position, base),))?;
          }
          if let Some(color) = fill_style {
            let circle =
              graphics::Mesh::new_circle(ctx, DrawMode::fill(), Vec2::new(0.0, 0.0), *r, 0.1, color.to_owned())?;
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
    Shape::PaintOps {
      path,
      line_style,
      fill_style,
      position,
    } => {
      let mut buffers: VertexBuffers<Point, u16> = VertexBuffers::new();
      let mut path_builder = lyon::path::Path::builder();
      let from = path_add(&base, &position);
      path_builder.begin(as_point(&from));
      for p in path {
        match p {
          PaintPath::LineTo(a) => {
            path_builder.line_to(add_to_point(&from, a));
          }
          PaintPath::QuadraticBezierTo(a, b) => {
            path_builder.quadratic_bezier_to(add_to_point(&from, a), add_to_point(&from, b));
          }
          PaintPath::CubicBezierTo(a, b, c) => {
            path_builder.cubic_bezier_to(add_to_point(&from, a), add_to_point(&from, b), add_to_point(&from, c));
          }
        }
      }
      path_builder.end(fill_style.is_some());

      let g_path = path_builder.build();
      // save another copy, may be used in filling
      let mut cloned_buffers = buffers.clone();

      if let Some((color, width)) = line_style {
        // https://docs.rs/lyon_tessellation/0.17.6/lyon_tessellation/struct.StrokeTessellator.html
        let mut vertex_builder = tessellation::geometry_builder::simple_builder(&mut buffers);
        let mut tessellator = tessellation::StrokeTessellator::new();
        let _ = tessellator.tessellate(
          &g_path,
          &lyon::tessellation::StrokeOptions::default().with_line_width(*width),
          &mut vertex_builder,
        );
        let mut g_vertices: Vec<graphics::Vertex> = vec![];
        for v in buffers.vertices {
          g_vertices.push(point_to_vertex(v, color.to_owned()))
        }
        let mut g_indices: Vec<u32> = vec![];
        for i in buffers.indices {
          g_indices.push(i as u32);
        }
        if g_vertices.len() < 3 {
          println!("[Warn] ops needs at least 3 vertices");
        } else {
          let mesh = graphics::Mesh::from_raw(ctx, &g_vertices, &g_indices, None)?;
          graphics::draw(ctx, &mesh, (position.to_owned(),))?;
        }
      }

      if let Some(color) = fill_style {
        // https://docs.rs/lyon_tessellation/0.17.6/lyon_tessellation/struct.StrokeTessellator.html
        let mut vertex_builder = tessellation::geometry_builder::simple_builder(&mut cloned_buffers);
        let mut tessellator = tessellation::FillTessellator::new();
        let _ = tessellator.tessellate(
          &g_path,
          &lyon::tessellation::FillOptions::default(),
          &mut vertex_builder,
        );
        let mut g_vertices: Vec<graphics::Vertex> = vec![];
        for v in cloned_buffers.vertices {
          g_vertices.push(point_to_vertex(v, color.to_owned()))
        }
        let mut g_indices: Vec<u32> = vec![];
        for i in cloned_buffers.indices {
          g_indices.push(i as u32);
        }

        if g_vertices.len() < 3 {
          println!("[Warn] ops needs at least 3 vertices");
        } else {
          let mesh = graphics::Mesh::from_raw(ctx, &g_vertices, &g_indices, None)?;
          graphics::draw(ctx, &mesh, (position.to_owned(),))?;
        }
      }
    }
  }
  Ok(())
}

// dirty but decoupled functions...
fn point_to_vertex(v: Point, color: Color) -> graphics::Vertex {
  graphics::Vertex {
    pos: [v.x, v.y],
    uv: [v.x, v.y],
    color: [color.r, color.g, color.b, color.a],
  }
}

fn as_point(a: &Vec2) -> Point {
  point(a.x, a.y)
}

fn add_to_point(a: &Vec2, b: &Vec2) -> Point {
  point(a.x + b.x, a.y + b.y)
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

fn extract_paint_path(data: &Calcit) -> Result<Vec<PaintPath>, String> {
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

fn extract_paint_op(xs: &im::Vector<Calcit>) -> Result<PaintPath, String> {
  if xs.len() >= 1 {
    match &xs[0] {
      Calcit::Keyword(s) | Calcit::Str(s) => match s.as_str() {
        "line-to" => match xs.get(1) {
          Some(v) => match extract_position(&v) {
            Ok(p) => Ok(PaintPath::LineTo(p)),
            Err(e) => Err(format!("failed position, {}", e)),
          },
          None => Err(format!("missing line-to position")),
        },
        "quadratic-bezier-to" => match (xs.get(1), xs.get(2)) {
          (Some(v1), Some(v2)) => match (extract_position(&v1), extract_position(&v2)) {
            (Ok(p1), Ok(p2)) => Ok(PaintPath::QuadraticBezierTo(p1, p2)),
            (a, b) => Err(format!("failed quadratic points, {:?} {:?}", a, b)),
          },
          (a, b) => Err(format!("missing quadratic points {:?} {:?}", a, b)),
        },
        "cubic-bezier-to" => match (xs.get(1), xs.get(2), xs.get(3)) {
          (Some(v1), Some(v2), Some(v3)) => match (extract_position(&v1), extract_position(&v2), extract_position(&v3))
          {
            (Ok(p1), Ok(p2), Ok(p3)) => Ok(PaintPath::CubicBezierTo(p1, p2, p3)),
            (a, b, c) => Err(format!("failed quadratic points, {:?} {:?} {:?}", a, b, c)),
          },
          (a, b, c) => Err(format!("missing quadratic points {:?} {:?} {:?}", a, b, c)),
        },
        // "close-path" => Ok(PaintPath::ClosePath),
        _ => Err(format!("unknown paint op: {}", s)),
      },
      _ => Err(format!("unknown paint op value: {}", xs[0])),
    }
  } else {
    Err(format!("empty is not paint op"))
  }
}

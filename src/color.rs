use calcit_runner::Calcit;
use ggez::graphics::Color;

fn hsl_helper(p: f32, q: f32, t0: f32) -> f32 {
  let mut t = t0;
  if t < 0.0 {
    t += 1.0
  } else if t > 1.0 {
    t -= 1.0
  }

  if t < 1.0 / 6.0 {
    p + (q - p) * 6.0 * t
  } else if t < 1.0 / 2.0 {
    q
  } else if t < 2.0 / 3.0 {
    p + (q - p) * (2.0 / 3.0 - t) * 6.0
  } else {
    p
  }
}

fn hsl_to_rgb(h0: f32, s0: f32, l0: f32, a: f32) -> Color {
  let h = h0 / 360.0;
  let s = s0 * 0.01;
  let l = l0 * 0.01;
  if s == 0.0 {
    Color {
      r: l,
      g: l,
      b: l,
      a,
    }
  } else {
    let q = if l < 0.5 {
      l * (1.0 + s)
    } else {
      l + s - l * s
    };
    let p = 2.0 * l - q;
    let r = hsl_helper(p, q, h + 1.0 / 3.0);
    let g = hsl_helper(p, q, h);
    let b = hsl_helper(p, q, h - 1.0 / 3.0);

    return Color { r, g, b, a };
  }
}

pub fn extract_color(x: &Calcit) -> Result<Color, String> {
  match x {
    Calcit::List(xs) if xs.len() == 3 || xs.len() == 4 => match (&xs[0], &xs[1], &xs[2]) {
      (Calcit::Number(h), Calcit::Number(s), Calcit::Number(l)) => match xs.get(3) {
        Some(Calcit::Number(a)) => Ok(hsl_to_rgb(*h as f32, *s as f32, *l as f32, *a as f32)),
        Some(a) => return Err(format!("invalid alpha: {}", a)),
        None => Ok(hsl_to_rgb(*h as f32, *s as f32, *l as f32, 1.0)),
      },
      (a, b, c) => Err(format!("unknown color values: {} {} {}", a, b, c)),
    },
    Calcit::List(xs) => Err(format!("unknown length of color: {}", xs.len())),
    _ => Err(format!("unknown type for color")),
  }
}

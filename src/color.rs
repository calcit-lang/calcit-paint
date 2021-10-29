use cirru_edn::Edn;
use raqote::Color;

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

fn hsl_to_rgb(h0: f32, s0: f32, l0: f32, alpha: f32) -> Color {
  let hue = h0 / 360.0;
  let s = s0 * 0.01;
  let l = l0 * 0.01;
  if s == 0.0 {
    Color::new(
      (alpha * 256.0).round() as u8,
      (l * 256.0).round() as u8,
      (l * 256.0).round() as u8,
      (l * 256.0).round() as u8,
    )
  } else {
    let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
    let p = 2.0 * l - q;
    let red = hsl_helper(p, q, hue + 1.0 / 3.0);
    let green = hsl_helper(p, q, hue);
    let blue = hsl_helper(p, q, hue - 1.0 / 3.0);

    Color::new(
      (alpha * 256.0).round() as u8,
      (red * 256.0).round() as u8,
      (green * 256.0).round() as u8,
      (blue * 256.0).round() as u8,
    )
  }
}

pub fn extract_color(x: &Edn) -> Result<Color, String> {
  match x {
    Edn::List(xs) if xs.len() == 3 || xs.len() == 4 => match (&xs[0], &xs[1], &xs[2]) {
      (Edn::Number(hue), Edn::Number(s), Edn::Number(light)) => match xs.get(3) {
        Some(Edn::Number(alpha)) => Ok(hsl_to_rgb(*hue as f32, *s as f32, *light as f32, *alpha as f32)),
        Some(a) => return Err(format!("invalid alpha: {}", a)),
        None => Ok(hsl_to_rgb(*hue as f32, *s as f32, *light as f32, 1.0)),
      },
      (a, b, c) => Err(format!("unknown color values: {} {} {}", a, b, c)),
    },
    Edn::List(xs) => Err(format!("unknown length of color: {}", xs.len())),
    _ => Err(String::from("unknown type for color")),
  }
}

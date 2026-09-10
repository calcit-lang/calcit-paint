use skia_safe::{svg::Dom, Canvas, FontMgr, Size};

const FIXTURE: &str = include_str!("fixtures/svg-research.svg");

/// Parses SVG without a network-capable resource provider.
fn parse(source: &str) -> Result<Dom, String> {
  Dom::from_str(source, FontMgr::empty()).map_err(|error| error.to_string())
}

/// Renders the checked fixture into a contained destination for research probes.
pub(crate) fn render_fixture(canvas: &Canvas, x: f32, y: f32, width: f32, height: f32) -> Result<(), String> {
  if ![x, y, width, height].iter().all(|value| value.is_finite()) || width <= 0.0 || height <= 0.0 {
    return Err("SVG research destination must use finite coordinates and positive dimensions".to_owned());
  }
  let mut dom = parse(FIXTURE)?;
  let intrinsic = dom.root().intrinsic_size();
  if intrinsic.width <= 0.0 || intrinsic.height <= 0.0 {
    return Err("SVG research fixture must expose a positive intrinsic size".to_owned());
  }
  let scale = (width / intrinsic.width).min(height / intrinsic.height);
  let dx = x + (width - intrinsic.width * scale) * 0.5;
  let dy = y + (height - intrinsic.height * scale) * 0.5;

  dom.set_container_size(Size::new(width, height));
  canvas.save();
  canvas.translate((dx, dy));
  canvas.scale((scale, scale));
  dom.render(canvas);
  canvas.restore();
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use skia_safe::{surfaces, Color, EncodedImageFormat, Rect};

  #[test]
  fn parses_sizes_rejects_invalid_xml_and_renders_pixels() {
    let dom = parse(FIXTURE).expect("fixture should parse");
    assert_eq!(dom.root().intrinsic_size(), Size::new(48.0, 32.0));
    assert_eq!(dom.root().view_box(), Some(&Rect::from_xywh(0.0, 0.0, 24.0, 16.0)));
    assert_eq!(parse("<svg><path").unwrap_err(), "Failed to load svg (reason unknown)");

    let mut surface = surfaces::raster_n32_premul((216, 152)).expect("raster surface");
    surface.canvas().clear(Color::WHITE);
    render_fixture(surface.canvas(), 12.0, 12.0, 192.0, 128.0).expect("fixture should render");
    let image = surface.image_snapshot();
    let pixels = image.peek_pixels().expect("raster pixels");
    assert_ne!(pixels.get_color((108, 76)), Color::WHITE);
    assert_eq!(pixels.get_color((215, 151)), Color::WHITE);
    let png = image.encode(None, EncodedImageFormat::PNG, 100).expect("PNG encoding");
    assert_eq!(&png.as_bytes()[..8], b"\x89PNG\r\n\x1a\n");
  }
}

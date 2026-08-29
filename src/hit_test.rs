use euclid::Vector2D;

pub type Transform = euclid::default::Transform2D<f32>;

#[derive(Debug, PartialEq, Clone)]
pub enum ClipShape {
  Rect {
    position: Vector2D<f32, f32>,
    width: f32,
    height: f32,
  },
  RoundedRect {
    position: Vector2D<f32, f32>,
    width: f32,
    height: f32,
    radius_x: f32,
    radius_y: f32,
  },
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClipRegion {
  pub shape: ClipShape,
  pub transform: Transform,
}

impl ClipRegion {
  pub fn contains(&self, position: Vector2D<f32, f32>) -> bool {
    let Some(transform) = self.transform.inverse() else {
      return false;
    };
    let point = transform.transform_point(euclid::default::Point2D::new(position.x, position.y));
    match self.shape {
      ClipShape::Rect {
        position,
        width,
        height,
      } => rect_contains(point, position, width, height),
      ClipShape::RoundedRect {
        position,
        width,
        height,
        radius_x,
        radius_y,
      } => rounded_rect_contains(point, position, width, height, radius_x, radius_y),
    }
  }
}

pub fn clips_contain(clips: &[ClipRegion], position: Vector2D<f32, f32>) -> bool {
  clips.iter().all(|clip| clip.contains(position))
}

fn rect_contains(point: euclid::default::Point2D<f32>, position: Vector2D<f32, f32>, width: f32, height: f32) -> bool {
  if width <= 0.0 || height <= 0.0 {
    return false;
  }
  point.x >= position.x && point.x <= position.x + width && point.y >= position.y && point.y <= position.y + height
}

fn rounded_rect_contains(
  point: euclid::default::Point2D<f32>,
  position: Vector2D<f32, f32>,
  width: f32,
  height: f32,
  radius_x: f32,
  radius_y: f32,
) -> bool {
  if !rect_contains(point, position, width, height) {
    return false;
  }

  let radius_x = radius_x.min(width * 0.5);
  let radius_y = radius_y.min(height * 0.5);
  if radius_x <= 0.0 || radius_y <= 0.0 {
    return true;
  }

  let left = position.x;
  let top = position.y;
  let right = left + width;
  let bottom = top + height;
  let center_x = if point.x < left + radius_x {
    left + radius_x
  } else if point.x > right - radius_x {
    right - radius_x
  } else {
    return true;
  };
  let center_y = if point.y < top + radius_y {
    top + radius_y
  } else if point.y > bottom - radius_y {
    bottom - radius_y
  } else {
    return true;
  };
  let x = (point.x - center_x) / radius_x;
  let y = (point.y - center_y) / radius_y;
  x * x + y * y <= 1.0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn applies_transformed_and_nested_rect_clips() {
    let clips = vec![
      ClipRegion {
        shape: ClipShape::Rect {
          position: Vector2D::new(0.0, 0.0),
          width: 50.0,
          height: 40.0,
        },
        transform: Transform::translation(20.0, 10.0),
      },
      ClipRegion {
        shape: ClipShape::Rect {
          position: Vector2D::new(10.0, 5.0),
          width: 20.0,
          height: 20.0,
        },
        transform: Transform::translation(20.0, 10.0),
      },
    ];
    assert!(clips_contain(&clips, Vector2D::new(40.0, 25.0)));
    assert!(!clips_contain(&clips, Vector2D::new(25.0, 15.0)));
    assert!(!clips_contain(&clips, Vector2D::new(75.0, 25.0)));
  }

  #[test]
  fn excludes_rounded_corners_and_matches_clamped_radii() {
    let clip = ClipRegion {
      shape: ClipShape::RoundedRect {
        position: Vector2D::new(0.0, 0.0),
        width: 100.0,
        height: 40.0,
        radius_x: 80.0,
        radius_y: 80.0,
      },
      transform: Transform::identity(),
    };
    assert!(!clip.contains(Vector2D::new(1.0, 1.0)));
    assert!(clip.contains(Vector2D::new(50.0, 1.0)));
    assert!(clip.contains(Vector2D::new(20.0, 20.0)));
  }

  #[test]
  fn rejects_degenerate_clip_transforms() {
    let clip = ClipRegion {
      shape: ClipShape::Rect {
        position: Vector2D::new(0.0, 0.0),
        width: 100.0,
        height: 100.0,
      },
      transform: Transform::scale(0.0, 1.0),
    };
    assert!(!clip.contains(Vector2D::new(0.0, 0.0)));
  }

  #[test]
  fn zero_area_clips_never_hit() {
    let clip = ClipRegion {
      shape: ClipShape::Rect {
        position: Vector2D::new(10.0, 10.0),
        width: 0.0,
        height: 20.0,
      },
      transform: Transform::identity(),
    };
    assert!(!clip.contains(Vector2D::new(10.0, 15.0)));
  }
}

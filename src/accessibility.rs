use std::sync::RwLock;

use accesskit::{Action, Node, NodeId, Rect, Role, TreeId, TreeInfo, TreeUpdate};
use euclid::{Point2D, Vector2D};

use crate::{
  focus,
  primes::{AccessibilityProperties, AccessibilityRole, EventTarget, TouchAreaShape},
};

pub const ROOT_NODE_ID: NodeId = NodeId(0);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Bounds {
  pub x0: f32,
  pub y0: f32,
  pub x1: f32,
  pub y1: f32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SemanticNode {
  pub properties: AccessibilityProperties,
  pub target: EventTarget,
  pub bounds: Bounds,
  pub focus_id: Option<String>,
}

#[derive(Default)]
struct AccessibilityState {
  nodes: Vec<SemanticNode>,
}

lazy_static! {
  static ref ACCESSIBILITY_STATE: RwLock<AccessibilityState> = RwLock::new(AccessibilityState::default());
}

#[cfg(test)]
pub static ACCESSIBILITY_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

pub fn begin_frame() {
  ACCESSIBILITY_STATE.write().unwrap().nodes.clear();
}

pub fn register(
  properties: &AccessibilityProperties,
  target: &EventTarget,
  position: Vector2D<f32, f32>,
  area: TouchAreaShape,
  transform: &focus::Transform,
  focus_id: Option<&str>,
) -> Result<(), String> {
  if properties.focusable && focus_id.is_none() {
    return Err(format!(
      "accessibility node {} is :focusable? but is not attached to a focus-area",
      properties.id
    ));
  }
  let mut state = ACCESSIBILITY_STATE.write().unwrap();
  if state.nodes.iter().any(|node| node.properties.id == properties.id) {
    return Err(format!(
      "duplicate accessibility :id in rendered scene: {}",
      properties.id
    ));
  }
  state.nodes.push(SemanticNode {
    properties: properties.clone(),
    target: target.clone(),
    bounds: transformed_bounds(position, area, transform),
    focus_id: focus_id.map(str::to_owned),
  });
  Ok(())
}

pub fn node_for_id(id: NodeId) -> Option<SemanticNode> {
  ACCESSIBILITY_STATE
    .read()
    .unwrap()
    .nodes
    .iter()
    .find(|node| node_id(&node.properties.id) == id)
    .cloned()
}

pub fn tree_update() -> TreeUpdate {
  let nodes = ACCESSIBILITY_STATE.read().unwrap().nodes.clone();
  let focused_id = focus::current().map(|area| area.id);
  let child_ids: Vec<NodeId> = nodes.iter().map(|node| node_id(&node.properties.id)).collect();
  let focus = focused_id
    .as_deref()
    .and_then(|focus_id| {
      nodes
        .iter()
        .find(|node| node.focus_id.as_deref() == Some(focus_id))
        .map(|node| node_id(&node.properties.id))
    })
    .unwrap_or(ROOT_NODE_ID);
  let mut root = Node::new(Role::Window);
  root.set_children(child_ids);
  let mut update_nodes = vec![(ROOT_NODE_ID, root)];
  update_nodes.extend(
    nodes
      .iter()
      .map(|node| (node_id(&node.properties.id), build_node(node))),
  );
  TreeUpdate {
    nodes: update_nodes,
    tree: Some(TreeInfo::new(ROOT_NODE_ID)),
    tree_id: TreeId::ROOT,
    focus,
  }
}

fn build_node(node: &SemanticNode) -> Node {
  let mut result = Node::new(match node.properties.role {
    AccessibilityRole::Button => Role::Button,
    AccessibilityRole::TextInput => Role::TextInput,
    AccessibilityRole::Image => Role::Image,
  });
  result.set_label(node.properties.label.clone());
  if let Some(value) = &node.properties.value {
    result.set_value(value.clone());
  }
  result.set_bounds(Rect::new(
    node.bounds.x0 as f64,
    node.bounds.y0 as f64,
    node.bounds.x1 as f64,
    node.bounds.y1 as f64,
  ));
  if !node.properties.enabled {
    result.set_disabled();
  } else {
    result.add_action(Action::Click);
    if node.properties.focusable {
      result.add_action(Action::Focus);
    }
  }
  result
}

fn node_id(id: &str) -> NodeId {
  let hash = id.bytes().fold(0xcbf2_9ce4_8422_2325_u64, |hash, byte| {
    (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3)
  });
  NodeId(if hash == 0 { 1 } else { hash })
}

fn transformed_bounds(position: Vector2D<f32, f32>, area: TouchAreaShape, transform: &focus::Transform) -> Bounds {
  let (dx, dy) = match area {
    TouchAreaShape::Rect(dx, dy) => (dx, dy),
    TouchAreaShape::Circle(radius) => (radius, radius),
  };
  let corners = [
    Point2D::new(position.x - dx, position.y - dy),
    Point2D::new(position.x - dx, position.y + dy),
    Point2D::new(position.x + dx, position.y - dy),
    Point2D::new(position.x + dx, position.y + dy),
  ]
  .map(|point| transform.transform_point(point));
  Bounds {
    x0: corners.iter().map(|point| point.x).fold(f32::INFINITY, f32::min),
    y0: corners.iter().map(|point| point.y).fold(f32::INFINITY, f32::min),
    x1: corners.iter().map(|point| point.x).fold(f32::NEG_INFINITY, f32::max),
    y1: corners.iter().map(|point| point.y).fold(f32::NEG_INFINITY, f32::max),
  }
}

#[cfg(test)]
pub fn reset_for_test() {
  *ACCESSIBILITY_STATE.write().unwrap() = AccessibilityState::default();
}

#[cfg(test)]
mod tests {
  use super::*;

  fn properties(id: &str) -> AccessibilityProperties {
    AccessibilityProperties {
      id: id.into(),
      role: AccessibilityRole::Button,
      label: "Confirm".into(),
      value: None,
      enabled: true,
      focusable: false,
    }
  }

  #[test]
  fn rejects_duplicate_stable_ids() {
    let _guard = ACCESSIBILITY_TEST_LOCK.lock().unwrap();
    reset_for_test();
    let metadata = properties("confirm");
    register(
      &metadata,
      &EventTarget::default(),
      Vector2D::new(10.0, 20.0),
      TouchAreaShape::Rect(5.0, 3.0),
      &focus::Transform::identity(),
      None,
    )
    .unwrap();
    assert!(register(
      &metadata,
      &EventTarget::default(),
      Vector2D::new(10.0, 20.0),
      TouchAreaShape::Rect(5.0, 3.0),
      &focus::Transform::identity(),
      None,
    )
    .unwrap_err()
    .contains("duplicate accessibility :id"));
  }

  #[test]
  fn exposes_bounds_and_semantic_actions() {
    let _guard = ACCESSIBILITY_TEST_LOCK.lock().unwrap();
    reset_for_test();
    let metadata = properties("confirm");
    register(
      &metadata,
      &EventTarget::default(),
      Vector2D::new(10.0, 20.0),
      TouchAreaShape::Rect(5.0, 3.0),
      &focus::Transform::identity(),
      None,
    )
    .unwrap();
    let node = node_for_id(node_id("confirm")).unwrap();
    assert_eq!(
      node.bounds,
      Bounds {
        x0: 5.0,
        y0: 17.0,
        x1: 15.0,
        y1: 23.0
      }
    );
    let update = tree_update();
    assert_eq!(update.tree.unwrap().root, ROOT_NODE_ID);
    assert_eq!(update.nodes.len(), 2);
  }
}

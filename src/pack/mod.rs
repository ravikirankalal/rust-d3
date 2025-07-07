// D3 pack layout module for Rust D3
// Provides a simple pack layout for hierarchical data (bubble packing).

use crate::hierarchy::Node;

#[derive(Debug, Clone)]
pub struct PackNode<T> {
    pub value: T,
    pub x: f64,
    pub y: f64,
    pub r: f64,
}

pub fn pack<T: Clone + Into<f64>>(root: &mut Node<T>, radius: f64) -> Vec<PackNode<T>> {
    // Calculate radii based on value
    assign_radii(root);

    // Position nodes recursively
    position_nodes(root, 0.0, 0.0, radius);

    let mut result = Vec::new();
    collect_pack_nodes(root, &mut result);
    result
}

fn assign_radii<T: Clone + Into<f64>>(node: &mut Node<T>) {
    if node.children.is_empty() {
        // Leaf node: radius based on its value
        node.r = node.value.clone().into().sqrt(); // Simple sqrt scaling
    } else {
        // Internal node: recursively assign radii to children
        for child in &mut node.children {
            assign_radii(child);
        }
        // Parent radius is sum of children radii (simplified, not accurate packing)
        node.r = node.children.iter().map(|c| c.r).sum();
    }
}

fn position_nodes<T: Clone>(node: &mut Node<T>, x: f64, y: f64, _parent_r: f64) {
    node.x = x;
    node.y = y;

    if node.children.is_empty() { return; }

    // Simple circular arrangement for children
    let num_children = node.children.len();
    let angle_step = std::f64::consts::TAU / num_children as f64;
    let current_r = node.r; // Use the calculated radius for positioning

    for (i, child) in node.children.iter_mut().enumerate() {
        let angle = i as f64 * angle_step;
        let child_x = x + (current_r - child.r) * angle.cos(); // Position based on parent's radius and child's radius
        let child_y = y + (current_r - child.r) * angle.sin();
        position_nodes(child, child_x, child_y, child.r);
    }
}

fn collect_pack_nodes<T: Clone>(node: &Node<T>, result: &mut Vec<PackNode<T>>) {
    result.push(PackNode {
        value: node.value.clone(),
        x: node.x,
        y: node.y,
        r: node.r,
    });
    for child in &node.children {
        collect_pack_nodes(child, result);
    }
}

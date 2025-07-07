// D3 pack layout module for Rust D3
// Provides a simple pack layout for hierarchical data (bubble packing, placeholder).

use crate::hierarchy::Node;

#[derive(Debug, Clone)]
pub struct PackNode<T> {
    pub value: T,
    pub x: f64,
    pub y: f64,
    pub r: f64,
}

pub fn pack<T: Clone>(root: &Node<T>, radius: f64) -> Vec<PackNode<T>> {
    let mut result = Vec::new();
    fn walk<T: Clone>(node: &Node<T>, x: f64, y: f64, r: f64, result: &mut Vec<PackNode<T>>) {
        result.push(PackNode {
            value: node.value.clone(),
            x,
            y,
            r,
        });
        let n = node.children.len();
        for (i, child) in node.children.iter().enumerate() {
            let angle = (i as f64) * std::f64::consts::TAU / n.max(1) as f64;
            let cx = x + r * angle.cos();
            let cy = y + r * angle.sin();
            walk(child, cx, cy, r / 2.0, result);
        }
    }
    walk(root, 0.0, 0.0, radius, &mut result);
    result
}

// (Unit tests moved to tests/integration.rs)

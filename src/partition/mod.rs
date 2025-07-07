// D3 partition layout module for Rust D3
// Provides a simple partition layout for hierarchical data (placeholder).

use crate::hierarchy::Node;

#[derive(Debug, Clone)]
pub struct PartitionNode<T> {
    pub value: T,
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
}

pub fn partition<T: Clone>(root: &Node<T>, width: f64, height: f64) -> Vec<PartitionNode<T>> {
    let mut result = Vec::new();
    fn walk<T: Clone>(node: &Node<T>, x0: f64, x1: f64, y0: f64, y1: f64, result: &mut Vec<PartitionNode<T>>) {
        result.push(PartitionNode {
            value: node.value.clone(),
            x0,
            x1,
            y0,
            y1,
        });
        let n = node.children.len();
        for (i, child) in node.children.iter().enumerate() {
            let child_x0 = x0 + (x1 - x0) * i as f64 / n.max(1) as f64;
            let child_x1 = x0 + (x1 - x0) * (i as f64 + 1.0) / n.max(1) as f64;
            walk(child, child_x0, child_x1, y0 + (y1 - y0) / 2.0, y1, result);
        }
    }
    walk(root, 0.0, width, 0.0, height, &mut result);
    result
}

// (Unit tests moved to tests/integration.rs)

// D3 tree layout module for Rust D3
// Provides a simple tree layout for hierarchical data.

use crate::hierarchy::Node;

pub fn tree<T: Clone>(root: &Node<T>, depth: usize) -> Vec<(usize, usize, T)> {
    let mut result = Vec::new();
    fn walk<T: Clone>(node: &Node<T>, depth: usize, pos: &mut usize, result: &mut Vec<(usize, usize, T)>) {
        let this_pos = *pos;
        result.push((this_pos, depth, node.value.clone()));
        *pos += 1;
        for child in &node.children {
            walk(child, depth + 1, pos, result);
        }
    }
    let mut pos = 0;
    walk(root, depth, &mut pos, &mut result);
    result
}



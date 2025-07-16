//! d3-hierarchy cluster layout

use super::node::Node;

pub struct ClusterLayout {
    pub node_size: Option<(f64, f64)>,
}

impl ClusterLayout {
    pub fn new() -> Self {
        ClusterLayout { node_size: None }
    }
    pub fn node_size(mut self, size: (f64, f64)) -> Self {
        self.node_size = Some(size);
        self
    }
    pub fn layout<T>(&self, root: &mut Node<T>) {
        root.compute_depths(0);
        root.compute_heights();
        let mut next_x = 0.0;
        Self::assign_x(root, &mut next_x);
        Self::assign_y(root, 0.0, self.node_size.map(|(_, y)| y).unwrap_or(100.0));
    }
    // fn count_leaves<T>(node: &Node<T>) -> usize {
    //     if node.children.is_empty() {
    //         1
    //     } else {
    //         node.children.iter().map(Self::count_leaves).sum()
    //     }
    // }
    fn assign_x<T>(node: &mut Node<T>, next_x: &mut f64) {
        if node.children.is_empty() {
            node.x = Some(*next_x);
            *next_x += 1.0;
        } else {
            for child in &mut node.children {
                Self::assign_x(child, next_x);
            }
            let first = node.children.first().unwrap().x.unwrap();
            let last = node.children.last().unwrap().x.unwrap();
            node.x = Some((first + last) / 2.0);
        }
    }
    fn assign_y<T>(node: &mut Node<T>, y: f64, y_step: f64) {
        node.y = Some(y + node.depth as f64 * y_step);
        for child in &mut node.children {
            Self::assign_y(child, y, y_step);
        }
    }
}

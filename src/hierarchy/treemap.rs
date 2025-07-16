//! d3-hierarchy treemap layout

use super::node::Node;

pub struct TreemapLayout {
    pub size: (f64, f64),
}

impl TreemapLayout {
    pub fn new() -> Self {
        TreemapLayout { size: (1.0, 1.0) }
    }
    pub fn size(mut self, size: (f64, f64)) -> Self {
        self.size = size;
        self
    }
    pub fn layout<T>(&self, root: &mut Node<T>) {
        let n = root.children.len();
        if n == 0 {
            root.x = Some(0.0);
            root.y = Some(0.0);
            return;
        }
        let w = self.size.0;
        let h = self.size.1;
        let child_w = w / n as f64;
        for (i, child) in root.children.iter_mut().enumerate() {
            child.x = Some(i as f64 * child_w);
            child.y = Some(0.0);
            // Recursively layout children (vertical split)
            TreemapLayout { size: (child_w, h) }.layout(child);
        }
        root.x = Some(0.0);
        root.y = Some(0.0);
    }
}

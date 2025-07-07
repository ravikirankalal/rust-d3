// Node implementation for hierarchy

#[derive(Debug, Clone, PartialEq)]
pub struct Node<T> {
    pub value: T,
    pub children: Vec<Node<T>>,
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            children: Vec::new(),
            x0: 0.0,
            y0: 0.0,
            x1: 0.0,
            y1: 0.0,
        }
    }

    pub fn with_children(value: T, children: Vec<Node<T>>) -> Self {
        Self { value, children, x0: 0.0, y0: 0.0, x1: 0.0, y1: 0.0 }
    }

    pub fn add_child(&mut self, child: Node<T>) {
        self.children.push(child);
    }

    pub fn traverse_preorder<F: FnMut(&T)>(&self, f: &mut F) {
        f(&self.value);
        for child in &self.children {
            child.traverse_preorder(f);
        }
    }
}

/// Placeholder for d3-hierarchy API parity.
/// See: https://github.com/d3/d3-hierarchy#api-reference
/// TODO: Implement full API parity with d3-hierarchy (hierarchy, stratify, tree, cluster, pack, partition, treemap, etc.)
pub fn hierarchy<T>(_data: T) {
    // TODO: Implement hierarchy logic
}

pub fn stratify<T>(_data: T) {
    // TODO: Implement stratify logic
}

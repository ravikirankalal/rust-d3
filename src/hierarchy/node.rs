//! Hierarchy Node structure (d3-hierarchy)
///
/// A node in a tree or hierarchy.
///
///
//// # Fields
//// - `data`: The user data for this node.
//// - `children`: Child nodes.
//// - `parent`: Optional pointer to parent node.
//// - `depth`: Depth from root (0 for root).
//// - `height`: Height to furthest leaf.
//// - `value`: Aggregated value (e.g., for sum).
//// - `x`, `y`: Layout coordinates (if computed).

#[derive(Debug, Clone)]
pub struct Node<T> {
    pub data: T,
    pub children: Vec<Node<T>>,
    pub parent: Option<*const Node<T>>,
    pub depth: usize,
    pub height: usize,
    pub value: Option<f64>,
    pub x: Option<f64>,
    pub y: Option<f64>,
}

impl<T> Node<T> {
    /// Create a new node with the given data.
    pub fn new(data: T) -> Self {
        Node {
            data,
            children: Vec::new(),
            parent: None,
            depth: 0,
            height: 0,
            value: None,
            x: None,
            y: None,
        }
    }
    /// Add a child node.
    pub fn add_child(&mut self, mut child: Node<T>) {
        child.parent = Some(self as *const _);
        self.children.push(child);
    }
    /// Returns true if this node is a leaf.
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
    /// Recursively compute depths for this node and its children.
    pub fn compute_depths(&mut self, depth: usize) {
        self.depth = depth;
        for child in &mut self.children {
            child.compute_depths(depth + 1);
        }
    }
    /// Recursively compute heights for this node and its children.
    pub fn compute_heights(&mut self) -> usize {
        if self.children.is_empty() {
            self.height = 0;
        } else {
            self.height = 1 + self.children.iter_mut().map(|c| c.compute_heights()).max().unwrap_or(0);
        }
        self.height
    }
    /// Traverse the tree, calling `f` on each node (preorder).
    pub fn each<F: FnMut(&mut Node<T>)>(&mut self, f: &mut F) {
        f(self);
        for child in &mut self.children {
            child.each(f);
        }
    }
    /// Aggregate values for this node and its children using `value_fn`.
    pub fn sum<F: Fn(&T) -> f64>(&mut self, value_fn: &F) -> f64 {
        let mut sum = value_fn(&self.data);
        for child in &mut self.children {
            sum += child.sum(value_fn);
        }
        self.value = Some(sum);
        sum
    }
}

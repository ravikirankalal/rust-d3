mod cluster;
pub use cluster::cluster;

// --- D3 Cluster Advanced (from cluster_adv) ---
#[derive(Debug, Clone)]
pub struct TreeNode<T> {
    pub value: T,
    pub children: Vec<TreeNode<T>>,
}

impl<T> TreeNode<T> {
    pub fn new(value: T) -> Self {
        Self { value, children: Vec::new() }
    }
    pub fn with_children(value: T, children: Vec<TreeNode<T>>) -> Self {
        Self { value, children }
    }
}

/// Computes the depth of a tree structure.
pub fn cluster_depth<T>(root: &TreeNode<T>) -> usize {
    fn depth<T>(node: &TreeNode<T>, d: usize) -> usize {
        if node.children.is_empty() {
            d
        } else {
            node.children.iter().map(|c| depth(c, d + 1)).max().unwrap_or(d)
        }
    }
    depth(root, 1)
}

/// Computes the number of leaves in the tree.
pub fn cluster_leaf_count<T>(root: &TreeNode<T>) -> usize {
    if root.children.is_empty() {
        1
    } else {
        root.children.iter().map(cluster_leaf_count).sum()
    }
}

/// Computes the height of the tree (max distance from root to any leaf).
pub fn cluster_height<T>(root: &TreeNode<T>) -> usize {
    if root.children.is_empty() {
        0
    } else {
        1 + root.children.iter().map(cluster_height).max().unwrap_or(0)
    }
}

/// Collects all leaf values in the tree.
pub fn cluster_leaves<T: Clone>(root: &TreeNode<T>) -> Vec<T> {
    if root.children.is_empty() {
        vec![root.value.clone()]
    } else {
        root.children.iter().flat_map(cluster_leaves).collect()
    }
}

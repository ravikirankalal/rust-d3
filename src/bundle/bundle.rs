use crate::hierarchy::Node;

/// Returns all parent-child edges in the tree for edge bundling.
pub fn bundle<T: Clone>(root: &Node<T>) -> Vec<(T, T)> {
    let mut edges = Vec::new();
    fn walk<T: Clone>(node: &Node<T>, edges: &mut Vec<(T, T)>) {
        for child in &node.children {
            edges.push((node.value.clone(), child.value.clone()));
            walk(child, edges);
        }
    }
    walk(root, &mut edges);
    edges
}

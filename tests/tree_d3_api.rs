//! Unit tests for d3-tree D3.js API methods
use rust_d3::hierarchy::Node;
use rust_d3::tree::tree_layout;

#[test]
fn test_tree_layout_api() {
    let root = Node::new("root");
    // Should be callable and not panic
    tree_layout(&root);
}

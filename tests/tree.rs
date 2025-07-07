//! Unit tests for d3 tree layout
use rust_d3::hierarchy::Node;
use rust_d3::tree::tree;

#[test]
fn test_tree() {
    let mut root = Node::new("root");
    root.add_child(Node::new("a"));
    root.add_child(Node::new("b"));
    tree(&mut root, (100.0, 100.0));
    assert_eq!(root.x, 0.0);
    assert_eq!(root.y, 0.0);
}

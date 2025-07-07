//! Unit tests for d3 tree layout
use rust_d3::hierarchy::Node;
use rust_d3::tree::{tree};

#[test]
fn test_tree() {
    let mut root = Node::new("root");
    root.add_child(Node::new("a"));
    root.add_child(Node::new("b"));
    let result = tree(&root, 0);
    assert_eq!(result.len(), 3);
    assert_eq!(result[0].2, "root");
    assert_eq!(result[1].2, "a");
    assert_eq!(result[2].2, "b");
}
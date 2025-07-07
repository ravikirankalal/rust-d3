//! Unit tests for d3 cluster layout
use rust_d3::hierarchy::Node;
use rust_d3::cluster::cluster;

#[test]
fn test_cluster() {
    let mut root = Node::new("root");
    root.add_child(Node::new("a"));
    root.add_child(Node::new("b"));
    cluster(&mut root, (100.0, 100.0));
    assert_eq!(root.x, 0.0);
    assert_eq!(root.y, 0.0);
}
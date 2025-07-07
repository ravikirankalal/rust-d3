//! Unit test for d3 bundle
use rust_d3::hierarchy::Node;
use rust_d3::bundle::bundle;

#[test]
fn test_bundle_edges() {
    let mut root = Node::new(1);
    let mut child1 = Node::new(2);
    let child2 = Node::new(3);
    child1.children.push(Node::new(4));
    root.children.push(child1);
    root.children.push(child2);
    let edges = bundle(&root);
    assert!(edges.contains(&(1, 2)));
    assert!(edges.contains(&(1, 3)));
    assert!(edges.contains(&(2, 4)));
    assert_eq!(edges.len(), 3);
}

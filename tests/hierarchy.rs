//! Unit tests for d3 hierarchy (Node)
use rust_d3::hierarchy::Node;

#[test]
fn test_node_tree() {
    let mut root = Node::new(1);
    let child1 = Node::new(2);
    let mut child2 = Node::new(3);
    child2.add_child(Node::new(4));
    root.add_child(child1);
    root.add_child(child2.clone());
    let mut values = vec![];
    root.traverse_preorder(&mut |v| values.push(*v));
    assert_eq!(values, vec![1, 2, 3, 4]);
    assert_eq!(root.children[1], child2);
}

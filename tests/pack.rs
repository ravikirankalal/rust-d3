//! Unit tests for d3 pack layout
use rust_d3::hierarchy::Node;
use rust_d3::pack::{pack, PackNode};

#[test]
fn test_pack() {
    let mut root = Node::new("root");
    root.add_child(Node::new("a"));
    root.add_child(Node::new("b"));
    let packed = pack(&root, 10.0);
    assert_eq!(packed.len(), 3);
    assert_eq!(packed[0].value, "root");
    assert_eq!(packed[1].value, "a");
    assert_eq!(packed[2].value, "b");
}

#[test]
fn test_pack_empty_and_edge_cases() {
    // Empty tree (single node)
    let root = Node::new("root");
    let packed = pack(&root, 10.0);
    assert_eq!(packed.len(), 1);
    assert_eq!(packed[0].value, "root");
    // Zero radius
    let packed = pack(&root, 0.0);
    assert_eq!(packed.len(), 1);
    // Negative radius (should not panic, but may behave as zero)
    let packed = pack(&root, -5.0);
    assert_eq!(packed.len(), 1);
}

#[test]
fn test_pack_deep_tree() {
    let mut root = Node::new("root");
    let mut child = Node::new("child");
    child.add_child(Node::new("grandchild"));
    root.add_child(child);
    let packed = pack(&root, 10.0);
    assert_eq!(packed.iter().any(|n| n.value == "grandchild"), true);
}

#[test]
fn test_pack_duplicate_values() {
    let mut root = Node::new("x");
    root.add_child(Node::new("x"));
    let packed = pack(&root, 10.0);
    assert_eq!(packed.iter().filter(|n| n.value == "x").count(), 2);
}

#[test]
fn test_pack_nodes_with_no_children_at_various_depths() {
    let mut root = Node::new("root");
    let mut a = Node::new("a");
    let b = Node::new("b"); // no children
    let c = Node::new("c"); // no children
    a.add_child(Node::new("a1")); // a1 has no children
    root.add_child(a);
    root.add_child(b);
    root.add_child(c);
    let packed = pack(&root, 5.0);
    assert_eq!(packed.iter().filter(|n| n.value == "a1").count(), 1);
    assert_eq!(packed.iter().filter(|n| n.value == "b").count(), 1);
    assert_eq!(packed.iter().filter(|n| n.value == "c").count(), 1);
}

#[test]
fn test_pack_large_and_small_radii() {
    let root = Node::new("root");
    let packed_large = pack(&root, 1e6);
    assert_eq!(packed_large[0].r, 1e6);
    let packed_small = pack(&root, 1e-6);
    assert_eq!(packed_small[0].r, 1e-6);
}

#[test]
fn test_pack_many_children() {
    let mut root = Node::new("root".to_string());
    for i in 0..10 {
        let label = format!("child{}", i);
        root.add_child(Node::new(label.clone()));
    }
    let packed = pack(&root, 10.0);
    assert_eq!(packed.len(), 11);
    for i in 0..10 {
        let label = format!("child{}", i);
        assert!(packed.iter().any(|n| n.value == label));
    }
}

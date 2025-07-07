//! Unit tests for d3 partition layout
use rust_d3::hierarchy::Node;
use rust_d3::partition::partition;

#[test]
fn test_partition() {
    let mut root = Node::new("root");
    root.add_child(Node::new("a"));
    root.add_child(Node::new("b"));
    let parts = partition(&root, 100.0, 50.0);
    assert_eq!(parts.len(), 3);
    assert_eq!(parts[0].value, "root");
    assert_eq!(parts[1].value, "a");
    assert_eq!(parts[2].value, "b");
}

#[test]
fn test_partition_empty_and_edge_cases() {
    // Empty tree (single node)
    let root = Node::new("root");
    let parts = partition(&root, 100.0, 50.0);
    assert_eq!(parts.len(), 1);
    assert_eq!(parts[0].value, "root");
    // Zero width/height
    let parts = partition(&root, 0.0, 0.0);
    assert_eq!(parts.len(), 1);
    // Negative width/height (should not panic, but may behave as zero)
    let parts = partition(&root, -10.0, -5.0);
    assert_eq!(parts.len(), 1);
}

#[test]
fn test_partition_deep_tree() {
    let mut root = Node::new("root");
    let mut child = Node::new("child");
    child.add_child(Node::new("grandchild"));
    root.add_child(child);
    let parts = partition(&root, 100.0, 50.0);
    assert_eq!(parts.iter().any(|n| n.value == "grandchild"), true);
}

#[test]
fn test_partition_duplicate_values() {
    let mut root = Node::new("x");
    root.add_child(Node::new("x"));
    let parts = partition(&root, 100.0, 50.0);
    assert_eq!(parts.iter().filter(|n| n.value == "x").count(), 2);
}

#[test]
fn test_partition_nodes_with_no_children_at_various_depths() {
    let mut root = Node::new("root");
    let mut a = Node::new("a");
    let b = Node::new("b"); // no children
    let c = Node::new("c"); // no children
    a.add_child(Node::new("a1")); // a1 has no children
    root.add_child(a);
    root.add_child(b);
    root.add_child(c);
    let parts = partition(&root, 5.0, 5.0);
    assert_eq!(parts.iter().filter(|n| n.value == "a1").count(), 1);
    assert_eq!(parts.iter().filter(|n| n.value == "b").count(), 1);
    assert_eq!(parts.iter().filter(|n| n.value == "c").count(), 1);
}

#[test]
fn test_partition_large_and_small_sizes() {
    let root = Node::new("root");
    let parts_large = partition(&root, 1e6, 1e6);
    assert_eq!(parts_large[0].x1, 1e6);
    assert_eq!(parts_large[0].y1, 1e6);
    let parts_small = partition(&root, 1e-6, 1e-6);
    assert_eq!(parts_small[0].x1, 1e-6);
    assert_eq!(parts_small[0].y1, 1e-6);
}

#[test]
fn test_partition_many_children() {
    let mut root = Node::new("root".to_string());
    for i in 0..10 {
        let label = format!("child{}", i);
        root.add_child(Node::new(label.clone()));
    }
    let parts = partition(&root, 10.0, 10.0);
    assert_eq!(parts.len(), 11);
    for i in 0..10 {
        let label = format!("child{}", i);
        assert!(parts.iter().any(|n| n.value == label));
    }
}

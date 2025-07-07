//! Tests for d3 partitioning (sunburst/treemap placeholder)

use rust_d3::partitioning::{sunburst, PartitionedNode};
use rust_d3::hierarchy::Node;

#[test]
fn test_sunburst_basic() {
    #[derive(Clone, Debug, PartialEq)]
    struct DummyNode {
        value: f64,
    }
    let root = Node::with_children(
        DummyNode { value: 1.0 },
        vec![
            Node::new(DummyNode { value: 2.0 }),
            Node::new(DummyNode { value: 3.0 }),
        ],
    );
    let result = sunburst(&root, 100.0);
    assert_eq!(result.len(), 3); // root + 2 children
    assert_eq!(result[0].x1, 100.0);
    assert_eq!(result[0].y1, 100.0);
}

#[test]
fn test_sunburst_no_children() {
    #[derive(Clone, Debug, PartialEq)]
    struct DummyNode {
        value: i32,
    }
    let root = Node::new(DummyNode { value: 42 });
    let result = sunburst(&root, 50.0);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].x0, 0.0);
    assert_eq!(result[0].x1, 50.0);
    assert_eq!(result[0].y0, 0.0);
    assert_eq!(result[0].y1, 50.0);
}

#[test]
fn test_sunburst_deep_nesting() {
    #[derive(Clone, Debug, PartialEq)]
    struct DummyNode {
        value: &'static str,
    }
    let root = Node::with_children(
        DummyNode { value: "root" },
        vec![Node::with_children(
            DummyNode { value: "child" },
            vec![Node::new(DummyNode { value: "leaf" })],
        )],
    );
    let result = sunburst(&root, 10.0);
    assert_eq!(result.len(), 3);
    assert_eq!(result[0].value.value, "root");
    assert_eq!(result[1].value.value, "child");
    assert_eq!(result[2].value.value, "leaf");
}

#[test]
fn test_sunburst_negative_and_zero_radius() {
    #[derive(Clone, Debug, PartialEq)]
    struct DummyNode {
        value: i32,
    }
    let root = Node::new(DummyNode { value: 1 });
    let result_neg = sunburst(&root, -10.0);
    assert_eq!(result_neg[0].x1, -10.0);
    let result_zero = sunburst(&root, 0.0);
    assert_eq!(result_zero[0].x1, 0.0);
}

#[test]
fn test_sunburst_many_children() {
    #[derive(Clone, Debug, PartialEq)]
    struct DummyNode {
        value: usize,
    }
    let children: Vec<_> = (0..10).map(|i| Node::new(DummyNode { value: i })).collect();
    let root = Node::with_children(DummyNode { value: 100 }, children);
    let result = sunburst(&root, 20.0);
    assert_eq!(result.len(), 11); // root + 10 children
    assert_eq!(result[0].x1, 20.0);
}

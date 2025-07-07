//! Tests for d3 treemap

use rust_d3::hierarchy::Node;
use rust_d3::treemap::{Treemap, TreemapTiling};

#[test]
fn test_treemap_slice() {
    let mut root = Node::with_children(0.0, vec![
        Node::new(0.0),
        Node::new(0.0),
        Node::with_children(0.0, vec![Node::new(0.0), Node::new(0.0)])
    ]);
    let treemap = Treemap::new().tiling(TreemapTiling::Slice).size(6.0, 2.0);
    treemap.layout(&mut root);
    // Check that bounds are assigned
    assert_eq!(root.x0, 0.0);
    assert_eq!(root.y0, 0.0);
    assert_eq!(root.x1, 6.0);
    assert_eq!(root.y1, 2.0);
}

#[test]
fn test_treemap_dice() {
    let mut root = Node::with_children(0.0, vec![Node::new(0.0), Node::new(0.0)]);
    let treemap = Treemap::new().tiling(TreemapTiling::Dice).size(4.0, 2.0);
    treemap.layout(&mut root);
    assert_eq!(root.x0, 0.0);
    assert_eq!(root.x1, 4.0);
}

#[test]
fn test_treemap_slicedice_padding() {
    let mut root = Node::with_children(0.0, vec![Node::new(0.0), Node::new(0.0)]);
    let treemap = Treemap::new().tiling(TreemapTiling::SliceDice).size(4.0, 2.0).padding(0.5);
    treemap.layout(&mut root);
    assert_eq!(root.x0, 0.0);
    assert_eq!(root.x1, 4.0);
    // Children should be separated by padding
    let c0 = &root.children[0];
    let c1 = &root.children[1];
    assert!((c1.x0 - c0.x1 - 0.5).abs() < 1e-6);
}

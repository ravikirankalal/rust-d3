//! Tests for d3 treemap advanced tiling
use rust_d3::{TreemapTiler, TreemapTiling};

fn sum_rect_area(rects: &[(f64, f64, f64, f64)]) -> f64 {
    rects.iter().map(|&(_, _, w, h)| w * h).sum()
}

#[test]
fn test_treemap_squarify() {
    let tiler = TreemapTiler::new(TreemapTiling::Squarify);
    let weights = vec![3.0, 2.0, 1.0];
    let rects = tiler.tile(&weights, 0.0, 0.0, 6.0, 2.0);
    assert_eq!(rects.len(), 3);
    let total_area = sum_rect_area(&rects);
    assert!((total_area - 12.0).abs() < 1e-5);
}

#[test]
fn test_treemap_binary() {
    let tiler = TreemapTiler::new(TreemapTiling::Binary);
    let weights = vec![1.0, 1.0];
    let rects = tiler.tile(&weights, 0.0, 0.0, 4.0, 2.0);
    assert_eq!(rects.len(), 2);
    let total_area = sum_rect_area(&rects);
    assert!((total_area - 8.0).abs() < 1e-8);
}

#[test]
fn test_treemap_dice() {
    let tiler = TreemapTiler::new(TreemapTiling::Dice);
    let weights = vec![2.0, 2.0];
    let rects = tiler.tile(&weights, 0.0, 0.0, 4.0, 2.0);
    assert_eq!(rects.len(), 2);
    assert!((rects[0].2 + rects[1].2 - 4.0).abs() < 1e-8);
}

#[test]
fn test_treemap_slice() {
    let tiler = TreemapTiler::new(TreemapTiling::Slice);
    let weights = vec![1.0, 3.0];
    let rects = tiler.tile(&weights, 0.0, 0.0, 2.0, 4.0);
    assert_eq!(rects.len(), 2);
    assert!((rects[0].3 + rects[1].3 - 4.0).abs() < 1e-8);
}

#[test]
fn test_treemap_slicedice() {
    let tiler = TreemapTiler::new(TreemapTiling::SliceDice);
    let weights = vec![1.0, 1.0, 1.0];
    let rects = tiler.tile(&weights, 0.0, 0.0, 3.0, 3.0);
    assert_eq!(rects.len(), 3);
    let total_area = sum_rect_area(&rects);
    assert!((total_area - 9.0).abs() < 1e-8);
}

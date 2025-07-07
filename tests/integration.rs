// Integration tests for Rust D3

use rust_d3::*;
use rust_d3::array;

#[test]
fn test_linear_scale() {
    let scale = scale::LinearScale::new((0.0, 10.0), (0.0, 100.0));
    assert_eq!(scale.scale(0.0), 0.0);
    assert_eq!(scale.scale(5.0), 50.0);
    assert_eq!(scale.scale(10.0), 100.0);
}

#[test]
fn test_selection_map_filter() {
    let sel = selection::Selection::new(vec![1, 2, 3, 4]);
    let mapped = sel.map(|x| x * 2);
    assert_eq!(mapped.data(), &vec![2, 4, 6, 8]);
    let filtered = sel.filter(|x| *x % 2 == 0);
    assert_eq!(filtered.data(), &vec![2, 4]);
}

#[test]
fn test_force_simulation() {
    let mut sim = force::ForceSimulation::new(vec![force::ForceNode::new(0.0, 0.0)]);
    sim.nodes[0].vx = 1.0;
    sim.nodes[0].vy = 2.0;
    sim.tick();
    assert_eq!(sim.nodes[0].x, 1.0);
    assert_eq!(sim.nodes[0].y, 2.0);
}

#[test]
fn test_statistics() {
    let data = [1, 2, 3, 4, 5];
    assert_eq!(statistics::mean(&data), Some(3.0));
    assert_eq!(statistics::sum(&data), Some(15.0));
    assert_eq!(statistics::median(&data), Some(3.0));
}

#[test]
fn test_polygon_hull() {
    let points = vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0), (0.5, 0.5)];
    let hull = polygon_hull::convex_hull(points);
    assert_eq!(hull.len(), 4);
}

#[test]
fn test_axis_generate() {
    let scale = scale::LinearScale::new((0.0, 10.0), (0.0, 100.0));
    let axis = axis::Axis::new(3);
    let ticks = axis.generate(&scale);
    assert_eq!(ticks, vec![(0.0, 0.0), (5.0, 50.0), (10.0, 100.0)]);
}

#[test]
fn test_color_scale() {
    let scale = color::ColorScale::new((0.0, 10.0), vec!["red".into(), "green".into(), "blue".into()]);
    assert_eq!(scale.scale(0.0), "red");
    assert_eq!(scale.scale(5.0), "green");
    assert_eq!(scale.scale(10.0), "blue");
}

#[test]
fn test_binning() {
    let data = [1.0, 2.0, 2.5, 3.0, 4.0, 5.0];
    let bins = binning::histogram(&data, 2);
    assert_eq!(bins.len(), 2);
}

#[test]
fn test_stack() {
    let a = vec![1.0, 2.0, 3.0];
    let b = vec![4.0, 5.0, 6.0];
    let stacked = stack::stack(&[a.clone(), b.clone()]);
    assert_eq!(stacked[0], vec![(0.0, 1.0), (0.0, 2.0), (0.0, 3.0)]);
    assert_eq!(stacked[1], vec![(1.0, 5.0), (2.0, 7.0), (3.0, 9.0)]);
}

#[test]
fn test_tree_layout() {
    let mut root = hierarchy::Node::new("root");
    root.add_child(hierarchy::Node::new("a"));
    let layout = tree::tree(&root, 0);
    assert_eq!(layout.len(), 2);
}

#[test]
fn test_quadtree() {
    let mut qt = quadtree::Quadtree::new((0.0, 0.0, 10.0, 10.0));
    qt.insert(quadtree::Point { x: 1.0, y: 1.0 });
    qt.insert(quadtree::Point { x: 5.0, y: 5.0 });
    let found = qt.query((0.0, 0.0, 5.0, 5.0));
    assert_eq!(found.len(), 2);
}

#[test]
fn test_interpolate() {
    assert_eq!(interpolate::interpolate(0.0, 10.0, 0.5), 5.0);
}

#[test]
fn test_transition() {
    let mut values = Vec::new();
    let trans = transition::Transition::new(100);
    trans.interpolate(0.0, 1.0, |v| values.push(v));
    assert!((values[0] - 0.0).abs() < 1e-8);
    assert!((values.last().unwrap() - 1.0).abs() < 1e-8);
}

#[test]
fn test_polygon_area_centroid_contains() {
    let poly = vec![(0.0, 0.0), (4.0, 0.0), (4.0, 3.0), (0.0, 3.0)];
    assert!((polygon::area(&poly) - 12.0).abs() < 1e-8);
    let (cx, cy) = polygon::centroid(&poly);
    assert!((cx - 2.0).abs() < 1e-8);
    assert!((cy - 1.5).abs() < 1e-8);
    assert!(polygon::contains(&poly, (2.0, 1.0)));
    assert!(!polygon::contains(&poly, (5.0, 1.0)));
}

#[test]
fn test_bisector() {
    let arr = [1, 2, 4, 4, 5];
    assert_eq!(bisector::bisect_left(&arr, &4), 2);
    assert_eq!(bisector::bisect_right(&arr, &4), 4);
}

#[test]
fn test_extent_nice() {
    let data = [3, 1, 4, 1, 5, 9];
    assert_eq!(extent::extent(&data), Some((1, 9)));
    assert_eq!(nice::nice((1.3, 9.7), 2.0), (0.0, 10.0));
}

#[test]
fn test_quantile() {
    let data = [1.0, 2.0, 3.0, 4.0, 5.0];
    assert_eq!(array::quantile(&data, 0.0), Some(1.0));
    assert_eq!(array::quantile(&data, 0.5), Some(3.0));
    assert_eq!(array::quantile(&data, 1.0), Some(5.0));
}
#[test]
fn test_cumsum() {
    let data = [1, 2, 3];
    assert_eq!(array::cumsum(&data), vec![1.0, 3.0, 6.0]);
}
#[test]
fn test_range() {
    assert_eq!(array::range(0.0, 1.0, 0.25), vec![0.0, 0.25, 0.5, 0.75]);
}
#[test]
fn test_ticks() {
    assert_eq!(array::ticks((0.0, 1.0), 5), vec![0.0, 0.25, 0.5, 0.75, 1.0]);
}

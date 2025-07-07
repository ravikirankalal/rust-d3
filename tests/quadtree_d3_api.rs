//! Unit tests for d3-quadtree D3.js API methods
use rust_d3::quadtree::{Point, Quadtree};

#[test]
fn test_add_and_remove() {
    let mut qt = Quadtree::new((0.0, 0.0, 10.0, 10.0));
    qt.add(Point { x: 2.0, y: 2.0 });
    qt.add(Point { x: 4.0, y: 4.0 });
    assert_eq!(qt.points.len(), 2);
    qt.remove(&Point { x: 2.0, y: 2.0 });
    assert_eq!(qt.points.len(), 1);
    assert_eq!(qt.points[0].x, 4.0);
}

#[test]
fn test_find() {
    let mut qt = Quadtree::new((0.0, 0.0, 10.0, 10.0));
    qt.add(Point { x: 3.0, y: 3.0 });
    qt.add(Point { x: 7.0, y: 7.0 });
    let found = qt.find(3.0, 3.0);
    assert!(found.is_some());
    assert_eq!(found.unwrap().x, 3.0);
    // D3.js returns the closest point even if far away, so this should return Some(3.0)
    let found_far = qt.find(1.0, 1.0);
    assert!(found_far.is_some());
    assert_eq!(found_far.unwrap().x, 3.0);
}

#[test]
fn test_visit() {
    let mut qt = Quadtree::new((0.0, 0.0, 10.0, 10.0));
    qt.add(Point { x: 1.0, y: 1.0 });
    qt.add(Point { x: 2.0, y: 2.0 });
    let mut sum = 0.0;
    qt.visit(|p| sum += p.x + p.y);
    assert_eq!(sum, 1.0 + 1.0 + 2.0 + 2.0);
}

#[test]
fn test_cover() {
    let mut qt = Quadtree::new((0.0, 0.0, 1.0, 1.0));
    qt.cover(5.0, 5.0); // Should expand bounds to include (5.0, 5.0)
    assert_eq!(qt.bounds, (0.0, 0.0, 5.0, 5.0));
}

//! Unit tests for d3 quadtree
use rust_d3::quadtree::{Point, Quadtree};

#[test]
fn test_quadtree_insert_query() {
    let mut qt = Quadtree::new((0.0, 0.0, 10.0, 10.0));
    qt.insert(Point { x: 1.0, y: 1.0 });
    qt.insert(Point { x: 5.0, y: 5.0 });
    qt.insert(Point { x: 9.0, y: 9.0 });
    let found = qt.query((0.0, 0.0, 5.0, 5.0));
    assert_eq!(found.len(), 2);
    assert!(found.iter().any(|p| p.x == 1.0 && p.y == 1.0));
    assert!(found.iter().any(|p| p.x == 5.0 && p.y == 5.0));
}

#[test]
fn test_quadtree_add_remove_find() {
    let mut qt = Quadtree::new((0.0, 0.0, 10.0, 10.0));
    let p = Point { x: 2.0, y: 3.0 };
    qt.add(p.clone());
    assert!(qt.find(2.0, 3.0).is_some());
    qt.remove(&p);
    assert!(qt.find(2.0, 3.0).is_none());
}

#[test]
fn test_quadtree_visit() {
    let mut qt = Quadtree::new((0.0, 0.0, 10.0, 10.0));
    qt.add(Point { x: 1.0, y: 1.0 });
    qt.add(Point { x: 2.0, y: 2.0 });
    let mut sum = 0.0;
    qt.visit(|p| { sum += p.x + p.y; });
    assert_eq!(sum, 6.0);
}

#[test]
fn test_quadtree_cover() {
    let mut qt = Quadtree::new((0.0, 0.0, 1.0, 1.0));
    qt.cover(-2.0, 3.0);
    assert_eq!(qt.bounds, (-2.0, 0.0, 1.0, 3.0));
}

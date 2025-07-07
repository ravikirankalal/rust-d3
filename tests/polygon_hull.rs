//! Tests for d3 polygon_hull (convex hull)

use rust_d3::polygon_hull::convex_hull;

#[test]
fn test_convex_hull_empty_and_small() {
    assert_eq!(convex_hull(vec![]), vec![]);
    assert_eq!(convex_hull(vec![(1.0, 2.0)]), vec![(1.0, 2.0)]);
    assert_eq!(convex_hull(vec![(1.0, 2.0), (3.0, 4.0)]), vec![(1.0, 2.0), (3.0, 4.0)]);
}

#[test]
fn test_convex_hull_triangle() {
    let tri = vec![(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)];
    let hull = convex_hull(tri.clone());
    assert_eq!(hull.len(), 3);
    for p in tri {
        assert!(hull.contains(&p));
    }
}

#[test]
fn test_convex_hull_square_and_collinear() {
    let square = vec![(0.0,0.0),(1.0,0.0),(1.0,1.0),(0.0,1.0)];
    let hull = convex_hull(square.clone());
    assert_eq!(hull.len(), 4);
    for p in square {
        assert!(hull.contains(&p));
    }
    let collinear = vec![(0.0,0.0),(1.0,1.0),(2.0,2.0)];
    let hull = convex_hull(collinear.clone());
    assert_eq!(hull.len(), 2); // Only endpoints
    assert!(hull.contains(&(0.0,0.0)) && hull.contains(&(2.0,2.0)));
}

#[test]
fn test_convex_hull_concave_and_duplicates() {
    let concave = vec![(0.0,0.0),(2.0,0.0),(2.0,2.0),(1.0,1.0),(0.0,2.0)];
    let hull = convex_hull(concave.clone());
    assert_eq!(hull.len(), 4);
    for p in &[(0.0,0.0),(2.0,0.0),(2.0,2.0),(0.0,2.0)] {
        assert!(hull.contains(p));
    }
    let with_dupes = vec![(0.0,0.0),(1.0,0.0),(1.0,1.0),(0.0,1.0),(0.0,0.0)];
    let hull = convex_hull(with_dupes);
    assert_eq!(hull.len(), 4);
}

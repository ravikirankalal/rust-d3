//! Tests for Delaunay triangulation implementation
use rust_d3::delaunay::Delaunay;

#[test]
fn test_delaunay_basic() {
    let points = vec![(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)];
    let delaunay = Delaunay::new(&points);
    assert_eq!(delaunay.points.len(), 3);
    assert_eq!(delaunay.triangles.len(), 1);
    let tri = &delaunay.triangles[0];
    let verts: Vec<_> = tri.vertices.iter().map(|&i| delaunay.points[i]).collect();
    assert!(verts.contains(&(0.0, 0.0)));
    assert!(verts.contains(&(1.0, 0.0)));
    assert!(verts.contains(&(0.0, 1.0)));
}

#[test]
fn test_delaunay_find() {
    let points = vec![(0.0, 0.0), (10.0, 0.0)];
    let delaunay = Delaunay::new(&points);
    let idx = delaunay.find(0.1, 0.1).unwrap();
    assert_eq!(delaunay.points[idx], (0.0, 0.0));
    let idx = delaunay.find(9.9, 0.1).unwrap();
    assert_eq!(delaunay.points[idx], (10.0, 0.0));
}

#[test]
fn test_delaunay_empty() {
    let points: Vec<(f64, f64)> = vec![];
    let delaunay = Delaunay::new(&points);
    assert_eq!(delaunay.points.len(), 0);
    assert_eq!(delaunay.triangles.len(), 0);
}

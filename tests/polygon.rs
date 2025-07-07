//! Unit tests for d3 polygon
use rust_d3::polygon::{area, centroid, contains};

#[test]
fn test_polygon_area_centroid_contains() {
    let triangle = vec![(0.0, 0.0), (4.0, 0.0), (0.0, 3.0)];
    assert!((area(&triangle) - 6.0).abs() < 1e-6);
    let c = centroid(&triangle);
    assert!((c.0 - 4.0/3.0).abs() < 1e-6);
    assert!((c.1 - 1.0).abs() < 1e-6);
    assert!(contains(&triangle, (1.0, 1.0)));
    assert!(!contains(&triangle, (5.0, 5.0)));
}

#[test]
fn test_polygon_degenerate_and_empty() {
    // Colinear points (area should be 0)
    let line = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 2.0)];
    assert!((area(&line)).abs() < 1e-6);
    let c = centroid(&line);
    // Centroid for degenerate polygon may be arbitrary, just check it returns a value
    assert!(c.0.is_finite() && c.1.is_finite());
    // Empty polygon
    let empty: Vec<(f64, f64)> = vec![];
    assert_eq!(area(&empty), 0.0);
    let c = centroid(&empty);
    assert!(c.0.is_finite() && c.1.is_finite());
}

#[test]
fn test_polygon_repeated_points() {
    let square = vec![(0.0,0.0), (1.0,0.0), (1.0,1.0), (0.0,1.0), (0.0,0.0)];
    assert!((area(&square) - 1.0).abs() < 1e-6);
    let c = centroid(&square);
    assert!((c.0 - 0.5).abs() < 1e-6);
    assert!((c.1 - 0.5).abs() < 1e-6);
    // Point on edge
    assert!(contains(&square, (0.5, 0.0)));
    // Point on vertex
    assert!(contains(&square, (0.0, 0.0)));
}

#[test]
fn test_polygon_self_intersecting() {
    // Bowtie (self-intersecting)
    let bowtie = vec![(0.0,0.0), (2.0,2.0), (0.0,2.0), (2.0,0.0)];
    // Area may be 0 for self-intersecting (shoelace formula)
    assert!((area(&bowtie)).abs() < 1e-6);
    let c = centroid(&bowtie);
    assert!(c.0.is_finite() && c.1.is_finite());
    // Contains is undefined, but should not panic
    contains(&bowtie, (1.0, 1.0));
}

#[test]
fn test_polygon_with_hole() {
    // Outer square, inner square (hole) -- as one ring
    let poly = vec![
        (0.0,0.0), (4.0,0.0), (4.0,4.0), (0.0,4.0), (0.0,0.0),
        (1.0,1.0), (1.0,3.0), (3.0,3.0), (3.0,1.0), (1.0,1.0)
    ];
    // Area is not correct for holes in this simple implementation, but should not panic
    area(&poly);
    centroid(&poly);
    contains(&poly, (2.0, 2.0));
}

#[test]
fn test_polygon_contains_edge_cases() {
    let poly = vec![(0.0,0.0), (2.0,0.0), (2.0,2.0), (0.0,2.0)];
    // On edge
    assert!(contains(&poly, (1.0, 0.0)));
    // On vertex
    assert!(contains(&poly, (0.0, 0.0)));
    // Outside
    assert!(!contains(&poly, (3.0, 1.0)));
    // Inside
    assert!(contains(&poly, (1.0, 1.0)));
}

#[test]
fn test_polygon_large() {
    let n = 1000;
    let mut poly = Vec::with_capacity(n);
    for i in 0..n {
        let theta = 2.0 * std::f64::consts::PI * (i as f64) / (n as f64);
        poly.push((theta.cos(), theta.sin()));
    }
    // Should not panic
    let a = area(&poly);
    let c = centroid(&poly);
    assert!(a.is_finite() && c.0.is_finite() && c.1.is_finite());
}

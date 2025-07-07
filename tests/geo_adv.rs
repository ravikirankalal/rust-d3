//! Tests for geo_adv module
use rust_d3::geo_adv::{geo_mercator, geo_orthographic, geo_path_advanced};

#[test]
fn test_geo_mercator() {
    let (x, y) = geo_mercator(0.0, 0.0);
    assert!((x.abs() < 1e-6) && (y.abs() < 1e-6));
}

#[test]
fn test_geo_orthographic_visible() {
    // Centered at (0,0), point at (0,0) should be visible
    let result = geo_orthographic(0.0, 0.0, 0.0, 0.0, 1.0);
    assert!(result.is_some());
}

#[test]
fn test_geo_orthographic_invisible() {
    // Point on far side of globe should be None
    let result = geo_orthographic(180.0, 0.0, 0.0, 0.0, 1.0);
    assert!(result.is_none());
}

#[test]
fn test_geo_path_advanced() {
    let coords = vec![(0.0, 0.0), (1.0, 2.0), (2.0, 3.0)];
    let path = geo_path_advanced(&coords);
    assert!(path.starts_with("M "));
    assert!(path.contains("L"));
}

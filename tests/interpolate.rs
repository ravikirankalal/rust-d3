//! Unit tests for d3 interpolate (re-exported)
use rust_d3::interpolate::*;
use serde::{Serialize, Deserialize};

#[test]
fn test_interpolate() {
    assert_eq!(interpolate(0.0, 10.0, 0.0), 0.0);
    assert_eq!(interpolate(0.0, 10.0, 1.0), 10.0);
    assert_eq!(interpolate(0.0, 10.0, 0.5), 5.0);
}

#[test]
fn test_interpolate_array() {
    let a = [1.0, 2.0, 3.0];
    let b = [4.0, 5.0, 6.0];
    let result = interpolate_array(&a, &b, 0.5);
    assert_eq!(result, vec![2.5, 3.5, 4.5]);
}

#[test]
fn test_interpolate_round() {
    assert_eq!(interpolate_round(1.0, 5.0, 0.6), 3.0);
    assert_eq!(interpolate_round(2.0, 4.0, 0.1), 2.0);
}

#[test]
fn test_interpolate_rgb() {
    let c = interpolate_rgb("#000000", "#ffffff", 0.5);
    assert!(c.starts_with("#"));
}

#[test]
fn test_interpolate_hsl() {
    let c = interpolate_hsl("hsl(0,100%,50%)", "hsl(120,100%,50%)", 0.5);
    assert!(c.starts_with('#') && c.len() == 7, "Expected hex color, got: {}", c);
}

#[test]
fn test_interpolate_object() {
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct Foo { x: f64, y: f64 }
    let a = Foo { x: 1.0, y: 2.0 };
    let b = Foo { x: 3.0, y: 4.0 };
    let c = interpolate_object(&a, &b, 0.5);
    assert_eq!(c.x, 2.0);
    assert_eq!(c.y, 3.0);
}

#[test]
fn test_interpolate_number() {
    assert_eq!(interpolate_number(1.0, 3.0, 0.5), 2.0);
}

#[test]
fn test_interpolate_string() {
    let s = interpolate_string("foo", "bar", 0.5);
    assert_eq!(s.len(), 3);
}

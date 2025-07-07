//! Tests for advanced curve generators: Catmull-Rom and Cardinal
use rust_d3::curve_adv2::{curve_catmull_rom, curve_cardinal};

#[test]
fn test_curve_catmull_rom_basic() {
    let points = vec![(0.0, 0.0), (1.0, 2.0), (2.0, 0.0)];
    let out = curve_catmull_rom(&points, 0.5, 10);
    assert!((out.first().unwrap().0 - 0.0).abs() < 1e-8);
    assert!((out.first().unwrap().1 - 0.0).abs() < 1e-8);
    assert_eq!(out.len(), 20);
}

#[test]
fn test_curve_cardinal_basic() {
    let points = vec![(0.0, 0.0), (1.0, 2.0), (2.0, 0.0)];
    let out = curve_cardinal(&points, 0.0, 10);
    assert!((out.first().unwrap().0 - 0.0).abs() < 1e-8);
    assert!((out.first().unwrap().1 - 0.0).abs() < 1e-8);
    assert_eq!(out.len(), 20);
}

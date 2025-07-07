//! Unit tests for d3 ease
use rust_d3::ease::{linear, quad_in, quad_out, quad_in_out, cubic_in, cubic_out, cubic_in_out};

#[test]
fn test_linear() {
    assert_eq!(linear(0.5), 0.5);
}

#[test]
fn test_quad_in() {
    assert_eq!(quad_in(0.5), 0.25);
}

#[test]
fn test_quad_out() {
    assert_eq!(quad_out(0.5), 0.75);
}

#[test]
fn test_quad_in_out() {
    assert!((quad_in_out(0.25) - 0.125).abs() < 1e-6);
    assert!((quad_in_out(0.75) - 0.875).abs() < 1e-6);
}

#[test]
fn test_cubic_in() {
    assert!((cubic_in(0.0) - 0.0).abs() < 1e-6);
    assert!((cubic_in(0.5) - 0.125).abs() < 1e-6);
    assert!((cubic_in(1.0) - 1.0).abs() < 1e-6);
}

#[test]
fn test_cubic_out() {
    assert!((cubic_out(0.0) - 0.0).abs() < 1e-6);
    assert!((cubic_out(0.5) - 0.875).abs() < 1e-6);
    assert!((cubic_out(1.0) - 1.0).abs() < 1e-6);
}

#[test]
fn test_cubic_in_out() {
    assert!((cubic_in_out(0.0) - 0.0).abs() < 1e-6);
    assert!((cubic_in_out(0.25) - 0.0625).abs() < 1e-6);
    assert!((cubic_in_out(0.5) - 0.5).abs() < 1e-6);
    assert!((cubic_in_out(0.75) - 0.9375).abs() < 1e-6);
    assert!((cubic_in_out(1.0) - 1.0).abs() < 1e-6);
}

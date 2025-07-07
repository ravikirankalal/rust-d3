//! Unit tests for d3 nice module
use rust_d3::nice::nice;

#[test]
fn test_nice_basic() {
    assert_eq!(nice((1.2, 9.8), 1.0), (1.0, 10.0));
    assert_eq!(nice((2.5, 7.1), 2.0), (2.0, 8.0));
}

#[test]
fn test_nice_negative_and_zero() {
    assert_eq!(nice((-3.7, 3.7), 1.0), (-4.0, 4.0));
    assert_eq!(nice((0.0, 0.0), 1.0), (0.0, 0.0));
}

#[test]
fn test_nice_small_step() {
    assert_eq!(nice((0.1, 0.9), 0.1), (0.1, 0.9));
    assert_eq!(nice((0.11, 0.89), 0.1), (0.1, 0.9));
}

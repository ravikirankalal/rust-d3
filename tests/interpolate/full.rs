//! Unit tests for d3 interpolate (re-exported)
use rust_d3::interpolate::interpolate;

#[test]
fn test_interpolate() {
    assert_eq!(interpolate(0.0, 10.0, 0.0), 0.0);
    assert_eq!(interpolate(0.0, 10.0, 1.0), 10.0);
    assert_eq!(interpolate(0.0, 10.0, 0.5), 5.0);
}

//! Unit tests for d3 PowScale
use rust_d3::scale::PowScale;

#[test]
fn test_pow_scale() {
    let scale = PowScale::new((0.0, 2.0), (0.0, 8.0), 3.0);
    assert!((scale.scale(0.0) - 0.0).abs() < 1e-6);
    assert!((scale.scale(1.0) - 1.0).abs() < 1e-6);
    assert!((scale.scale(2.0) - 8.0).abs() < 1e-6);
}

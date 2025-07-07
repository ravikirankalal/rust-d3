//! Unit tests for d3 SqrtScale
use rust_d3::scale::SqrtScale;

#[test]
fn test_sqrt_scale() {
    let scale = SqrtScale::new((0.0, 4.0), (0.0, 2.0));
    assert!((scale.scale(0.0) - 0.0).abs() < 1e-6);
    assert!((scale.scale(1.0) - 1.0).abs() < 1e-6);
    assert!((scale.scale(4.0) - 2.0).abs() < 1e-6);
}

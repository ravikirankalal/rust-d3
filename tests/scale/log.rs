//! Unit tests for d3 LogScale
use rust_d3::scale::LogScale;

#[test]
fn test_log_scale() {
    let scale = LogScale::new((1.0, 100.0), (0.0, 2.0), 10.0);
    assert!((scale.scale(1.0) - 0.0).abs() < 1e-6);
    assert!((scale.scale(10.0) - 1.0).abs() < 1e-6);
    assert!((scale.scale(100.0) - 2.0).abs() < 1e-6);
}

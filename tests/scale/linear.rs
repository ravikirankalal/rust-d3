//! Unit tests for d3 LinearScale
use rust_d3::scale::LinearScale;

#[test]
fn test_linear_scale() {
    let scale = LinearScale::new((0.0, 10.0), (0.0, 100.0));
    assert_eq!(scale.scale(0.0), 0.0);
    assert_eq!(scale.scale(5.0), 50.0);
    assert_eq!(scale.scale(10.0), 100.0);
}

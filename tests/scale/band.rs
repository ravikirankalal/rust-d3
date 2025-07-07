//! Unit tests for d3 BandScale
use rust_d3::scale::BandScale;

#[test]
fn test_band_scale() {
    let scale = BandScale::new(vec!["a", "b", "c"], (0.0, 30.0), 0.1);
    assert_eq!(scale.bandwidth(), 9.0);
    assert_eq!(scale.scale(&"a"), Some(0.0));
    assert_eq!(scale.scale(&"b"), Some(9.9));
    assert_eq!(scale.scale(&"c"), Some(19.8));
}

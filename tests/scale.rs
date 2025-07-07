//! Integration test for scale module

use rust_d3::scale::{LinearScale, BandScale, OrdinalScale, LogScale, PowScale, SqrtScale};

#[test]
fn test_linear_scale() {
    let scale = LinearScale::new((0.0, 10.0), (0.0, 100.0));
    assert_eq!(scale.scale(0.0), 0.0);
    assert_eq!(scale.scale(5.0), 50.0);
    assert_eq!(scale.scale(10.0), 100.0);
}

#[test]
fn test_band_scale() {
    let scale = BandScale::new(vec!["a", "b", "c"], (0.0, 30.0), 0.1);
    assert_eq!(scale.bandwidth(), 10.0);
    assert_eq!(scale.scale(&"a"), Some(0.0));
    assert_eq!(scale.scale(&"b"), Some(10.0));
    assert_eq!(scale.scale(&"c"), Some(20.0));
}

#[test]
fn test_ordinal_scale() {
    let scale = OrdinalScale::new(vec!["a", "b", "c"], vec![1, 2, 3]);
    assert_eq!(scale.scale(&"a"), Some(1));
    assert_eq!(scale.scale(&"b"), Some(2));
    assert_eq!(scale.scale(&"c"), Some(3));
    assert_eq!(scale.scale(&"d"), None);
}

#[test]
fn test_log_scale() {
    let scale = LogScale::new((1.0, 100.0), (0.0, 2.0), 10.0);
    assert!((scale.scale(1.0) - 0.0).abs() < 1e-6);
    assert!((scale.scale(10.0) - 1.0).abs() < 1e-6);
    assert!((scale.scale(100.0) - 2.0).abs() < 1e-6);
}

#[test]
fn test_pow_scale() {
    let scale = PowScale::new((0.0, 2.0), (0.0, 8.0), 3.0);
    assert!((scale.scale(0.0) - 0.0).abs() < 1e-6);
    assert!((scale.scale(1.0) - 1.0).abs() < 1e-6);
    assert!((scale.scale(2.0) - 8.0).abs() < 1e-6);
}

#[test]
fn test_sqrt_scale() {
    let scale = SqrtScale::new((0.0, 4.0), (0.0, 2.0));
    assert!((scale.scale(0.0) - 0.0).abs() < 1e-6);
    assert!((scale.scale(1.0) - 1.0).abs() < 1e-6);
    assert!((scale.scale(4.0) - 2.0).abs() < 1e-6);
}

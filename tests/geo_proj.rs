//! Tests for geo_proj module
use rust_d3::geo_proj::geo_equirectangular;

#[test]
fn test_geo_equirectangular() {
    let (x, y) = geo_equirectangular(180.0, 90.0);
    assert!((x - std::f64::consts::PI).abs() < 1e-6);
    assert!((y - std::f64::consts::FRAC_PI_2).abs() < 1e-6);
}

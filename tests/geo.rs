//! Unit tests for d3 geo
use rust_d3::geo::{equirectangular, Equirectangular};

#[test]
fn test_geo_equirectangular_default() {
    let (x, y) = equirectangular(180.0, 0.0);
    assert!((x - std::f64::consts::PI).abs() < 1e-10);
    assert!((y - 0.0).abs() < 1e-10);
}

#[test]
fn test_geo_equirectangular_custom() {
    let proj = Equirectangular::new(100.0, (400.0, 300.0));
    let (x, y) = proj.project(90.0, 45.0);
    assert!((x - (100.0 * std::f64::consts::FRAC_PI_2 + 400.0)).abs() < 1e-10);
    assert!((y - (300.0 - 100.0 * std::f64::consts::FRAC_PI_4)).abs() < 1e-10);
}

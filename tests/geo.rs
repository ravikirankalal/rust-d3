//! Unit tests for d3 geo
use rust_d3::geo::{equirectangular, Equirectangular, GeoPathGenerator, mercator, Projection};

fn identity_projection(pt: (f64, f64)) -> (f64, f64) { pt }

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

#[test]
fn test_geo_mercator() {
    let proj = mercator(); // Use the default mercator projection
    let (x, y) = proj.project(0.0, 0.0);
    assert!((x - 480.0).abs() < 1e-10);
    assert!((y - 250.0).abs() < 1e-10);

    let (x, y) = proj.project(180.0, 85.0);
    assert!((x - (480.0 + 156.96 * std::f64::consts::PI)).abs() < 1e-7);
    assert!((y - (250.0 + 156.96 * ((std::f64::consts::PI / 4.0) + (85.0_f64.to_radians() / 2.0)).tan().ln())).abs() < 1e-7);
}

#[test]
fn test_geo_path_generator_polygon() {
    // Simple square polygon: [(0,0), (1,0), (1,1), (0,1), (0,0)]
    let polygon = vec![
        vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0), (0.0, 0.0)]
    ];
    let path_gen = GeoPathGenerator::new(identity_projection);
    let d = path_gen.path(&polygon);
    assert_eq!(d, "M0,0L1,0L1,1L0,1L0,0Z");
}

#[test]
fn test_geo_path_generator_multi_ring() {
    // Polygon with two rings (outer and hole)
    let polygon = vec![
        vec![(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0), (0.0, 0.0)],
        vec![(0.5, 0.5), (1.5, 0.5), (1.5, 1.5), (0.5, 1.5), (0.5, 0.5)]
    ];
    let path_gen = GeoPathGenerator::new(identity_projection);
    let d = path_gen.path(&polygon);
    assert_eq!(d, "M0,0L2,0L2,2L0,2L0,0ZM0.5,0.5L1.5,0.5L1.5,1.5L0.5,1.5L0.5,0.5Z");
}

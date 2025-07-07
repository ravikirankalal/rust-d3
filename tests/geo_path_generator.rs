//! Tests for GeoPathGenerator (d3-geo parity)
use rust_d3::geo::GeoPathGenerator;

fn identity_projection(pt: (f64, f64)) -> (f64, f64) {
    pt
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

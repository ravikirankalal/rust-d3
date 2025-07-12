use rust_d3::contour::contour_density;

#[test]
fn test_contour_density_basic() {
    let data = vec![vec![10.0, 10.0], vec![20.0, 20.0], vec![30.0, 30.0]];
    let density = contour_density().size([40, 40]).bandwidth(5.0).thresholds(vec![0.01, 0.05, 0.1]);
    let features = density.compute(&data);
    // Should return a Vec<GeoJsonFeature> with length = thresholds
    assert_eq!(features.len(), 3);
    for f in features.iter() {
        assert_eq!(f._type, "Feature");
        assert_eq!(f.geometry._type, "MultiPolygon");
    }
}

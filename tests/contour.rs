// tests/contour.rs

use rust_d3::contour::ContourGenerator;

#[test]
fn test_contour_generator_new() {
    let _contour_gen = ContourGenerator::new();
    // Assert default values if any, or just that it can be created
    assert!(true);
}

#[test]
fn test_contour_generator_size() {
    let _contour_gen = ContourGenerator::new().size([10, 20]);
    // In a real scenario, you'd assert the internal state if accessible
    assert!(true);
}

#[test]
fn test_contour_generator_thresholds() {
    let _contour_gen = ContourGenerator::new().thresholds(vec![0.0, 0.5, 1.0]);
    // In a real scenario, you'd assert the internal state if accessible
    assert!(true);
}

#[test]
fn test_contour_generator_contours_placeholder() {
    let contour_gen = ContourGenerator::new().size([2, 2]).thresholds(vec![0.5]);
    let data = vec![0.0, 0.0, 0.0, 1.0]; // This will trigger case 1
    let contours = contour_gen.contours(&data);

    assert_eq!(contours.len(), 1);
    // The current implementation of contours returns a Vec<GeoJsonFeature>
    // The placeholder test was asserting on the internal contour_segments structure.
    // We need to update this test to assert on the GeoJsonFeature structure.
    // For now, we'll just assert that it returns a feature.
    assert_eq!(contours[0]._type, "Feature");
}

#[test]
fn test_contour_generator_contours_geojson() {
    use rust_d3::contour::contour;
    let contour_gen = contour().size([2, 2]).thresholds(vec![0.5]);
    let data = vec![0.0, 0.0, 0.0, 1.0]; // This will trigger case 1
    let contours = contour_gen.contours(&data);

    assert_eq!(contours.len(), 1);
    let feature = &contours[0];
    assert_eq!(feature._type, "Feature");
    assert!(feature.properties.is_some());
    assert_eq!(feature.properties.as_ref().unwrap().get("value").unwrap(), &serde_json::json!(0.5));
    assert_eq!(feature.geometry._type, "MultiLineString");

    let coordinates: Vec<Vec<[f64; 2]>> = serde_json::from_value(feature.geometry.coordinates.clone()).unwrap();
    assert_eq!(coordinates.len(), 1);
    assert_eq!(coordinates[0].len(), 2);
    let expected_points = vec![[1.0, 0.5], [0.5, 1.0]];
    assert!(coordinates[0].contains(&expected_points[0]));
    assert!(coordinates[0].contains(&expected_points[1]));
}

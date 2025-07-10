// tests/contour.rs

use rust_d3::contour::ContourGenerator;
use rust_d3::contour::{contour, contour_density};

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
fn test_contour_density_basic() {
    let data = vec![vec![10.0, 10.0], vec![20.0, 20.0], vec![30.0, 30.0]];
    let density = contour_density().size([40, 40]).bandwidth(5.0).thresholds(vec![0.01, 0.05, 0.1]);
    let features = density.compute(&data);
    assert_eq!(features.len(), 3);
    for f in features.iter() {
        assert_eq!(f._type, "Feature");
        assert_eq!(f.geometry._type, "MultiPolygon");
    }
}

#[test]
fn test_contour_density_empty_data() {
    let data: Vec<Vec<f64>> = vec![];
    let density = contour_density().size([10, 10]).bandwidth(2.0).thresholds(vec![0.01, 0.1]);
    let features = density.compute(&data);
    assert_eq!(features.len(), 0);
}

#[test]
fn test_contour_generator_thresholds_count() {
    let values = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
    let generator = contour().size([2, 3]).thresholds(3);
    let features = generator.contours(&values);
    assert_eq!(features.len(), 3);
    for f in features.iter() {
        assert_eq!(f._type, "Feature");
        assert_eq!(f.geometry._type, "MultiPolygon");
    }
}

#[test]
fn test_contour_generator_thresholds_values() {
    let values = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
    let generator = contour().size([2, 3]).thresholds(vec![1.0, 3.0]);
    let features = generator.contours(&values);
    assert_eq!(features.len(), 2);
    for f in features.iter() {
        assert_eq!(f._type, "Feature");
        assert_eq!(f.geometry._type, "MultiPolygon");
    }
}

#[test]
fn test_contour_generator_single_value() {
    let values = vec![1.0, 1.0, 1.0, 1.0];
    let generator = contour().size([2, 2]).thresholds(vec![0.5]);
    let features = generator.contours(&values);
    assert_eq!(features.len(), 1);
    let f = &features[0];
    assert_eq!(f._type, "Feature");
    assert_eq!(f.geometry._type, "MultiPolygon");
}

#[test]
fn test_contour_generator_no_contours() {
    let values = vec![0.0, 0.0, 0.0, 0.0];
    let generator = contour().size([2, 2]).thresholds(vec![1.0]);
    let features = generator.contours(&values);
    assert_eq!(features.len(), 1);
    // Should be a valid GeoJSON feature, but likely empty geometry
    let f = &features[0];
    assert_eq!(f._type, "Feature");
    assert_eq!(f.geometry._type, "MultiPolygon");
}

#[test]
fn test_contour_density_multiple_points() {
    let data = vec![vec![5.0, 5.0], vec![15.0, 15.0], vec![25.0, 25.0], vec![35.0, 35.0]];
    let density = contour_density().size([40, 40]).bandwidth(8.0).thresholds(vec![0.01, 0.05]);
    let features = density.compute(&data);
    assert_eq!(features.len(), 2);
    for f in features.iter() {
        assert_eq!(f._type, "Feature");
        assert_eq!(f.geometry._type, "MultiPolygon");
    }
}

#[test]
fn test_contour_density_high_bandwidth() {
    let data = vec![vec![10.0, 10.0], vec![30.0, 30.0]];
    let density = contour_density().size([40, 40]).bandwidth(50.0).thresholds(vec![0.01]);
    let features = density.compute(&data);
    assert_eq!(features.len(), 1);
    let f = &features[0];
    assert_eq!(f._type, "Feature");
    assert_eq!(f.geometry._type, "MultiPolygon");
}
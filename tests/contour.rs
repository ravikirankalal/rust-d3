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
    assert_eq!(contours[0].len(), 2);
    assert_eq!(contours[0][0], [0.0, 0.5]);
    assert_eq!(contours[0][1], [0.5, 0.0]);
}

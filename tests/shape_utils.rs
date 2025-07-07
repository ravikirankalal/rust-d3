//! Tests for d3 shape_utils (pie layout)

use rust_d3::shape_utils::Pie;

#[test]
fn test_pie_layout_basic() {
    let values = vec![1.0, 2.0, 3.0];
    let slices = Pie::layout(&values);
    assert_eq!(slices.len(), 3);
    let total_angle: f64 = slices.iter().map(|s| s.end_angle - s.start_angle).sum();
    assert!((total_angle - std::f64::consts::TAU).abs() < 1e-8);
    assert!((slices[0].start_angle - 0.0).abs() < 1e-8);
    assert!(slices[0].end_angle < slices[1].start_angle + 1e-8);
    assert_eq!(slices[2].index, 2);
}

#[test]
fn test_pie_layout_empty() {
    let values: Vec<f64> = vec![];
    let slices = Pie::layout(&values);
    assert!(slices.is_empty());
}

#[test]
fn test_arc_path_pie() {
    use rust_d3::shape_utils::Arc;
    // 90-degree pie sector
    let d = Arc::path(0.0, 10.0, 0.0, std::f64::consts::FRAC_PI_2);
    assert!(d.starts_with("M0,0L"));
    assert!(d.contains("A10.000000,10.000000"));
    assert!(d.ends_with("Z"));
}

#[test]
fn test_arc_path_annular() {
    use rust_d3::shape_utils::Arc;
    // 180-degree annular sector
    let d = Arc::path(5.0, 10.0, 0.0, std::f64::consts::PI);
    assert!(d.starts_with("M10.000000,0.000000A10.000000,10.000000"));
    assert!(d.contains("L-5.000000,0.000000A5.000000,5.000000"));
    assert!(d.ends_with("Z"));
}

#[test]
fn test_line_radial_basic() {
    use rust_d3::shape_utils::line_radial;
    // 3 points: 0°, 90°, 180° at radius 10
    let data = vec![
        (0.0, 10.0),
        (std::f64::consts::FRAC_PI_2, 10.0),
        (std::f64::consts::PI, 10.0),
    ];
    let path = line_radial().path(&data);
    // Should start at (10,0), then (0,10), then (-10,0)
    assert!(path.starts_with("M10.000000,0.000000"));
    assert!(path.contains("L0.000000,10.000000"));
    assert!(path.ends_with("L-10.000000,0.000000"));
}

#[test]
fn test_line_radial_with_undefined() {
    use rust_d3::shape_utils::line_radial;
    // Middle point is undefined
    let data = vec![
        (0.0, 10.0),
        (std::f64::consts::FRAC_PI_2, 10.0),
        (std::f64::consts::PI, 10.0),
    ];
    let path = line_radial()
        .defined(|i| i != 1)
        .path(&data);
    // Should move to first, skip second, move to third
    assert!(path.starts_with("M10.000000,0.000000"));
    assert!(path.ends_with("M-10.000000,0.000000"));
}

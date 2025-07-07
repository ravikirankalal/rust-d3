//! Unit tests for d3-shape API methods
use rust_d3::shape::LineGenerator;

#[test]
fn test_area_generator() {
    let data = vec![(0.0, 1.0), (1.0, 3.0), (2.0, 2.0)];
    let area = LineGenerator::area(&data, |&(x, y)| (x, y));
    // Area should outline the upper curve, then the baseline in reverse
    let expected = vec![
        (0.0, 1.0), (1.0, 3.0), (2.0, 2.0), // upper
        (2.0, 0.0), (1.0, 0.0), (0.0, 0.0)  // baseline
    ];
    assert_eq!(area, expected);
}

#[test]
fn test_arc_generator() {
    let arc = LineGenerator::arc(10.0, 20.0, 0.0, std::f64::consts::PI);
    // Should be a valid SVG path string (not empty)
    assert!(!arc.is_empty());
    assert!(arc.starts_with("M"));
    assert!(arc.contains("A20"));
}

#[test]
fn test_pie_generator() {
    let data = vec![1.0, 2.0, 3.0];
    let pie = LineGenerator::pie(&data);
    // Should return 3 slices covering 2π
    assert_eq!(pie.len(), 3);
    let total_angle: f64 = pie.iter().map(|(start, end)| end - start).sum();
    assert!((total_angle - std::f64::consts::TAU).abs() < 1e-10);
}

#[test]
fn test_symbol_generator() {
    let circle = LineGenerator::symbol("circle", 64.0);
    assert!(circle.contains("A"));
    let square = LineGenerator::symbol("square", 64.0);
    assert!(square.contains("h"));
    let triangle = LineGenerator::symbol("triangle", 64.0);
    assert!(triangle.contains("L"));
    let unknown = LineGenerator::symbol("foo", 64.0);
    assert_eq!(unknown, "");
}

#[test]
fn test_radial_area_generator() {
    let data = vec![(1.0, 0.0), (2.0, 1.0), (3.0, 2.0)];
    let area = LineGenerator::radial_area(&data, |&(r, theta)| (r, theta));
    // Should outline the upper curve, then the baseline in reverse
    let expected = vec![
        (1.0, 0.0), (2.0, 1.0), (3.0, 2.0),
        (3.0, 0.0), (2.0, 0.0), (1.0, 0.0)
    ];
    assert_eq!(area, expected);
}

#[test]
fn test_radial_line_generator() {
    let data = vec![(1.0, 0.0), (2.0, 1.0), (3.0, 2.0)];
    let line = LineGenerator::radial_line(&data, |&(r, theta)| (r, theta));
    assert_eq!(line, data);
}

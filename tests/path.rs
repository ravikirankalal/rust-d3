//! Unit tests for d3 PathBuilder
use rust_d3::path::PathBuilder;

#[test]
fn test_path_builder() {
    let mut pb = PathBuilder::new();
    pb.move_to(0.0, 0.0);
    pb.line_to(10.0, 0.0);
    pb.line_to(10.0, 10.0);
    pb.close();
    assert_eq!(pb.to_string(), "M0 0 L10 0 L10 10 Z");
}

#[test]
fn test_path_arc() {
    let mut pb = PathBuilder::new();
    pb.arc(0.0, 0.0, 10.0, 0.0, std::f64::consts::PI);
    let d = pb.to_string();
    println!("arc d: {}", d); // debug output
    // Accept either large_arc=0 or 1 due to floating-point quirks
    assert!(d.contains("A10 10 0 0 1") || d.contains("A10 10 0 1 1"));
    assert!(d.contains("M10 0"));
    assert!(d.contains("-10 0"));
}

#[test]
fn test_path_quadratic_curve() {
    let mut pb = PathBuilder::new();
    pb.move_to(0.0, 0.0);
    pb.quadratic_curve_to(5.0, 10.0, 10.0, 0.0);
    let d = pb.to_string();
    assert_eq!(d, "M0 0 Q5 10 10 0");
}

#[test]
fn test_path_bezier_curve() {
    let mut pb = PathBuilder::new();
    pb.move_to(0.0, 0.0);
    pb.bezier_curve_to(5.0, 10.0, 15.0, 10.0, 20.0, 0.0);
    let d = pb.to_string();
    assert_eq!(d, "M0 0 C5 10 15 10 20 0");
}

#[test]
fn test_path_rect() {
    let mut pb = PathBuilder::new();
    pb.rect(1.0, 2.0, 3.0, 4.0);
    let d = pb.to_string();
    assert_eq!(d, "M1 2 L4 2 L4 6 L1 6 Z");
}

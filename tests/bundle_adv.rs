//! Unit test for d3 bundle_adv (curveBundle advanced)
use rust_d3::bundle_adv::*;

fn is_straight_line(points: &[(f64, f64)]) -> bool {
    if points.len() < 2 { return true; }
    let (x0, y0) = points.first().unwrap();
    let (x1, y1) = points.last().unwrap();
    let dx = x1 - x0;
    let dy = y1 - y0;
    for (i, &(x, y)) in points.iter().enumerate() {
        let t = i as f64 / (points.len() as f64 - 1.0);
        let lx = x0 + dx * t;
        let ly = y0 + dy * t;
        if (x - lx).abs() > 1e-6 || (y - ly).abs() > 1e-6 {
            return false;
        }
    }
    true
}

#[test]
fn test_curve_bundle_beta_0() {
    let cb = CurveBundle::new(0.0);
    let points = vec![(0.0, 0.0), (1.0, 2.0), (2.0, 0.0)];
    let out = cb.generate(&points, 10);
    // Should be a straight line from first to last
    assert!((out.first().unwrap().0 - 0.0).abs() < 1e-8);
    assert!((out.first().unwrap().1 - 0.0).abs() < 1e-8);
    assert!((out.last().unwrap().0 - 2.0).abs() < 1e-8);
    assert!((out.last().unwrap().1 - 0.0).abs() < 1e-8);
    assert!(is_straight_line(&out));
}

#[test]
fn test_curve_bundle_beta_1() {
    let cb = CurveBundle::new(1.0);
    let points = vec![(0.0, 0.0), (1.0, 2.0), (2.0, 0.0)];
    let out = cb.generate(&points, 10);
    // First output point matches first input
    assert!((out.first().unwrap().0 - 0.0).abs() < 1e-8);
    assert!((out.first().unwrap().1 - 0.0).abs() < 1e-8);
    // Should NOT be a straight line
    assert!(!is_straight_line(&out));
    // Should not pass exactly through the middle control point
    assert!(!out.iter().any(|&(x, y)| (x - 1.0).abs() < 1e-8 && (y - 2.0).abs() < 1e-8));
}

#[test]
fn test_curve_bundle_beta_half() {
    let cb = CurveBundle::new(0.5);
    let points = vec![(0.0, 0.0), (1.0, 2.0), (2.0, 0.0)];
    let out = cb.generate(&points, 10);
    // First output point matches first input
    assert!((out.first().unwrap().0 - 0.0).abs() < 1e-8);
    assert!((out.first().unwrap().1 - 0.0).abs() < 1e-8);
    // Should not be a straight line
    assert!(!is_straight_line(&out));
    // Should not pass exactly through the middle control point
    assert!(!out.iter().any(|&(x, y)| (x - 1.0).abs() < 1e-8 && (y - 2.0).abs() < 1e-8));
}

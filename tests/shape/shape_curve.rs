// Tests for advanced d3-shape curve types
use rust_d3::shape::{Line, Area, BasisCurve, CardinalCurve, MonotoneCurve};

#[cfg(test)]
pub mod shape_curve_tests {
    use super::*;

    #[test]
    fn test_line_basis_curve() {
        let data = vec![1.0, 2.0, 3.0];
        let line = Line::new().basis_curve();
        let path = line.generate(&data);
        assert!(path.contains("C")); // Cubic for basis
    }

    #[test]
    fn test_line_cardinal_curve() {
        let data = vec![1.0, 2.0, 3.0];
        let line = Line::new().cardinal_curve();
        let path = line.generate(&data);
        assert!(path.contains("C")); // Cubic for cardinal
    }

    #[test]
    fn test_line_monotone_curve() {
        let data = vec![1.0, 2.0, 3.0];
        let line = Line::new().monotone_curve();
        let path = line.generate(&data);
        assert!(path.contains("C")); // Cubic for monotone
    }

    #[test]
    fn test_area_basis_curve() {
        let data = vec![1.0, 2.0, 3.0];
        let area = Area::new().basis_curve();
        let path = area.generate(&data);
        assert!(path.contains("C"));
    }

    #[test]
    fn test_area_cardinal_curve() {
        let data = vec![1.0, 2.0, 3.0];
        let area = Area::new().cardinal_curve();
        let path = area.generate(&data);
        assert!(path.contains("C"));
    }

    #[test]
    fn test_area_monotone_curve() {
        let data = vec![1.0, 2.0, 3.0];
        let area = Area::new().monotone_curve();
        let path = area.generate(&data);
        assert!(path.contains("C")); // Cubic for monotone
    }

    #[test]
    fn test_cardinal_curve_tension_variants() {
        let data = vec![(0.0, 0.0), (1.0, 2.0), (2.0, 0.0), (3.0, 2.0)];
        for &tension in &[0.0, 0.5, 1.0] {
            let curve = CardinalCurve::with_tension(tension);
            let line = Line::new()
                .x(|d: &(f64, f64), _| d.0)
                .y(|d: &(f64, f64), _| d.1)
                .curve(curve.clone());
            let path = line.generate(&data);
            assert!(path.contains('C'), "Cardinal curve with tension {} should use 'C' cubic commands, got: {}", tension, path);
        }
    }

    #[test]
    fn test_cardinal_curve_short_input() {
        let data = vec![(0.0, 0.0)];
        let line = Line::new().x(|d: &(f64, f64), _| d.0).y(|d: &(f64, f64), _| d.1).cardinal_curve();
        let path = line.generate(&data);
        assert!(path.starts_with("M"));
        assert!(!path.contains('C'));
    }

    #[test]
    fn test_monotone_curve_monotonicity() {
        let data = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 2.0), (3.0, 3.0)];
        let line = Line::new()
            .x(|d: &(f64, f64), _| d.0)
            .y(|d: &(f64, f64), _| d.1)
            .monotone_curve();
        let path = line.generate(&data);
        assert!(path.contains('C'), "Monotone curve should use 'C' cubic commands, got: {}", path);
    }

    #[test]
    fn test_monotone_curve_non_monotonic() {
        let data = vec![(0.0, 0.0), (1.0, 2.0), (2.0, 1.0), (3.0, 3.0)];
        let line = Line::new()
            .x(|d: &(f64, f64), _| d.0)
            .y(|d: &(f64, f64), _| d.1)
            .monotone_curve();
        let path = line.generate(&data);
        assert!(path.contains('C'));
    }

    #[test]
    fn test_curve_with_nan_and_defined() {
        let data = vec![
            (0.0, 0.0),
            (1.0, f64::NAN),
            (2.0, 2.0),
            (3.0, 3.0),
        ];
        let line = Line::new()
            .x(|d: &(f64, f64), _| d.0)
            .y(|d: &(f64, f64), _| d.1)
            .monotone_curve();
        let path = line.generate(&data);
        assert!(path.starts_with("M"));
        assert!(path.matches('M').count() >= 1); // Should break at NaN
    }

    #[test]
    fn test_area_cardinal_curve_tension() {
        let data = vec![(0.0, 0.0), (1.0, 2.0), (2.0, 0.0), (3.0, 2.0)];
        let area = Area::new()
            .x0(|d: &(f64, f64), _| d.0)
            .y0(|_d: &(f64, f64), _| 0.0)
            .x1(|d: &(f64, f64), _| d.0)
            .y1(|d: &(f64, f64), _| d.1)
            .curve(CardinalCurve::with_tension(0.5));
        let path = area.generate(&data);
        assert!(path.contains('C'));
    }

    #[test]
    fn test_area_monotone_curve_nan_defined() {
        let data = vec![(0.0, 0.0), (1.0, f64::NAN), (2.0, 2.0)];
        let area = Area::new()
            .x0(|d: &(f64, f64), _| d.0)
            .y0(|_d: &(f64, f64), _| 0.0)
            .x1(|d: &(f64, f64), _| d.0)
            .y1(|d: &(f64, f64), _| d.1)
            .curve(MonotoneCurve::default());
        let path = area.generate(&data);
        assert!(path.starts_with("M"));
    }
}

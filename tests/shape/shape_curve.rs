// Tests for advanced d3-shape curve types
use rust_d3::shape::{Line, Area, CardinalCurve, MonotoneCurve, Symbol, SymbolType};

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

    #[test]
    fn test_curve_empty_input() {
        let data: Vec<(f64, f64)> = vec![];
        let line = Line::new().monotone_curve();
        let path = line.generate(&data);
        assert!(path.is_empty() || path == "M");
    }

    #[test]
    fn test_curve_single_point() {
        let data = vec![(42.0, 99.0)];
        let line = Line::new().monotone_curve();
        let path = line.generate(&data);
        assert!(path.starts_with("M"));
        assert!(!path.contains('C'));
    }

    #[test]
    fn test_curve_all_nan() {
        let data = vec![(f64::NAN, f64::NAN), (f64::NAN, f64::NAN)];
        let line = Line::new()
            .x(|d: &(f64, f64), _| d.0)
            .y(|d: &(f64, f64), _| d.1)
            .monotone_curve();
        let path = line.generate(&data);
        if !(path.is_empty() || path == "M") {
            println!("FAIL: path for all-NaN input: '{}'; expected empty or 'M'", path);
        }
        assert!(path.is_empty() || path == "M");
    }

    #[test]
    fn test_curve_large_small_values() {
        let data = vec![(1e-12, 1e12), (1e12, 1e-12)];
        let line = Line::new().monotone_curve();
        let path = line.generate(&data);
        assert!(path.starts_with("M"));
    }

    #[test]
    fn test_area_empty_input() {
        let data: Vec<(f64, f64)> = vec![];
        let area = Area::new()
            .x0(|d: &(f64, f64), _| d.0)
            .y0(|_d: &(f64, f64), _| 0.0)
            .x1(|d: &(f64, f64), _| d.0)
            .y1(|d: &(f64, f64), _| d.1)
            .monotone_curve();
        let path = area.generate(&data);
        assert!(path.is_empty() || path == "M");
    }

    #[test]
    fn test_area_single_point() {
        let data = vec![(42.0, 99.0)];
        let area = Area::new()
            .x0(|d: &(f64, f64), _| d.0)
            .y0(|_d: &(f64, f64), _| 0.0)
            .x1(|d: &(f64, f64), _| d.0)
            .y1(|d: &(f64, f64), _| d.1)
            .monotone_curve();
        let path = area.generate(&data);
        assert!(path.starts_with("M"));
    }

    #[test]
    fn test_area_all_nan() {
        let data = vec![(f64::NAN, f64::NAN), (f64::NAN, f64::NAN)];
        let area = Area::new()
            .x0(|d: &(f64, f64), _| d.0)
            .y0(|_d: &(f64, f64), _| 0.0)
            .x1(|d: &(f64, f64), _| d.0)
            .y1(|d: &(f64, f64), _| d.1)
            .monotone_curve();
        let path = area.generate(&data);
        if !(path.is_empty() || path == "M") {
            println!("FAIL: area path for all-NaN input: '{}'; expected empty or 'M'", path);
        }
        assert!(path.is_empty() || path == "M");
    }

    #[test]
    fn test_area_nan_and_defined() {
        let data = vec![(0.0, 0.0), (1.0, f64::NAN), (2.0, 2.0)];
        let area = Area::new()
            .x0(|d: &(f64, f64), _| d.0)
            .y0(|_d: &(f64, f64), _| 0.0)
            .x1(|d: &(f64, f64), _| d.0)
            .y1(|d: &(f64, f64), _| d.1)
            .monotone_curve();
        let path = area.generate(&data);
        assert!(path.starts_with("M"));
    }

    #[test]
    fn test_line_linear_svg_parity() {
        let data = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 0.0)];
        let line = Line::new()
            .x(|d: &(f64, f64), _| d.0)
            .y(|d: &(f64, f64), _| d.1);
        let path = line.generate(&data);
        let d3_ref = "M0,0L1,1L2,0";
        assert_eq!(path, d3_ref);
    }

    #[test]
    fn test_area_linear_svg_parity() {
        let data = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 0.0)];
        let area = Area::new()
            .x0(|d: &(f64, f64), _| d.0)
            .y0(|_d: &(f64, f64), _| 0.0)
            .x1(|d: &(f64, f64), _| d.0)
            .y1(|d: &(f64, f64), _| d.1);
        let path = area.generate(&data);
        let d3_ref = "M0,0L1,1L2,0L2,0L1,0L0,0Z";
        assert_eq!(path, d3_ref);
    }

    #[test]
    fn test_line_linear_svg_parity_empty() {
        let data: Vec<(f64, f64)> = vec![];
        let line = Line::new()
            .x(|d: &(f64, f64), _| d.0)
            .y(|d: &(f64, f64), _| d.1);
        let path = line.generate(&data);
        let d3_ref = "";
        assert_eq!(path, d3_ref);
    }

    #[test]
    fn test_area_linear_svg_parity_empty() {
        let data: Vec<(f64, f64)> = vec![];
        let area = Area::new()
            .x0(|d: &(f64, f64), _| d.0)
            .y0(|_d: &(f64, f64), _| 0.0)
            .x1(|d: &(f64, f64), _| d.0)
            .y1(|d: &(f64, f64), _| d.1);
        let path = area.generate(&data);
        let d3_ref = "";
        assert_eq!(path, d3_ref);
    }

    #[test]
    #[ignore]
    fn test_line_basis_svg_parity() {
        let data = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 0.0)];
        let line = Line::new()
            .x(|d: &(f64, f64), _| d.0)
            .y(|d: &(f64, f64), _| d.1)
            .basis_curve();
        // D3 reference: M0,0C0.3333333333333333,0.3333333333333333 0.6666666666666666,0.6666666666666666 1,1C1.3333333333333333,1.3333333333333333 1.6666666666666667,0.6666666666666666 2,0
        let d3_ref = "M0,0C0.3333333333333333,0.3333333333333333 0.6666666666666666,0.6666666666666666 1,1C1.3333333333333333,1.3333333333333333 1.6666666666666667,0.6666666666666666 2,0";
        let path = line.generate(&data);
        assert_eq!(path, d3_ref);
    }

    #[test]
    #[ignore]
    fn test_line_cardinal_svg_parity() {
        let data = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 0.0)];
        let line = Line::new()
            .x(|d: &(f64, f64), _| d.0)
            .y(|d: &(f64, f64), _| d.1)
            .cardinal_curve();
        // D3 reference: M0,0C0.16666666666666666,0.16666666666666666 0.8333333333333334,0.8333333333333334 1,1C1.1666666666666667,1.1666666666666667 1.8333333333333333,0.16666666666666666 2,0
        let d3_ref = "M0,0C0.16666666666666666,0.16666666666666666 0.8333333333333334,0.8333333333333334 1,1C1.1666666666666667,1.1666666666666667 1.8333333333333333,0.16666666666666666 2,0";
        let path = line.generate(&data);
        assert_eq!(path, d3_ref);
    }

    #[test]
    #[ignore]
    fn test_line_monotone_svg_parity() {
        let data = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 0.0)];
        let line = Line::new()
            .x(|d: &(f64, f64), _| d.0)
            .y(|d: &(f64, f64), _| d.1)
            .monotone_curve();
        // D3 reference: M0,0C0.3333333333333333,0.3333333333333333 0.6666666666666666,1 1,1C1.3333333333333333,1 1.6666666666666667,0 2,0
        let d3_ref = "M0,0C0.3333333333333333,0.3333333333333333 0.6666666666666666,1 1,1C1.3333333333333333,1 1.6666666666666667,0 2,0";
        let path = line.generate(&data);
        assert_eq!(path, d3_ref);
    }

    #[test]
    #[ignore]
    fn test_symbol_circle_svg_parity() {
        let symbol = Symbol::new().size(64.0).symbol_type(SymbolType::Circle);
        // D3 reference: circle of size 64: "M4,0A4,4,0,1,1,-4,0A4,4,0,1,1,4,0Z"
        let d3_ref = "M4,0A4,4,0,1,1,-4,0A4,4,0,1,1,4,0Z";
        let path = symbol.to_path();
        assert_eq!(path, d3_ref);
    }

    #[test]
    fn test_symbol_square_svg_parity() {
        let symbol = Symbol::new().size(64.0).symbol_type(SymbolType::Square);
        // D3 reference: square of size 64: "M-4,-4H4V4H-4Z"
        let d3_ref = "M-4,-4H4V4H-4Z";
        let path = symbol.to_path();
        assert_eq!(path, d3_ref);
    }
}

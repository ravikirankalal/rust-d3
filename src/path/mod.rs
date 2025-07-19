//! d3-path: Path serialization and construction utilities (Rust port)
//
// This module aims to provide a builder-style API for SVG path generation, similar to d3-path.
// See: https://github.com/d3/d3-path

use crate::px;

pub struct Path {
    // Internal SVG path string
    data: String,
}

impl Path {
    pub fn new() -> Self {
        Path {
            data: String::new(),
        }
    }
    pub fn move_to(&mut self, x: f64, y: f64) {
        if !self.data.is_empty() {
            self.data.push(' ');
        }
        self.data.push_str(&format!("M{} {}", px(x), px(y)));
    }
    pub fn line_to(&mut self, x: f64, y: f64) {
        if !self.data.is_empty() {
            self.data.push(' ');
        }
        self.data.push_str(&format!("L{} {}", px(x), px(y)));
    }
    pub fn close_path(&mut self) {
        self.data.push_str("Z");
    }
    pub fn quadratic_curve_to(&mut self, cpx: f64, cpy: f64, x: f64, y: f64) {
        if !self.data.is_empty() {
            self.data.push(' ');
        }
        self.data.push_str(&format!("Q{} {} {} {}", px(cpx), px(cpy), px(x), px(y)));
    }
    pub fn bezier_curve_to(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        if !self.data.is_empty() {
            self.data.push(' ');
        }
        self.data
            .push_str(&format!("C{} {} {} {} {} {}", px(cp1x), px(cp1y), px(cp2x), px(cp2y), px(x), px(y)));
    }
    pub fn arc(
        &mut self,
        rx: f64,
        ry: f64,
        x_axis_rotation: f64,
        large_arc: bool,
        sweep: bool,
        x: f64,
        y: f64,
    ) {
        if !self.data.is_empty() {
            self.data.push(' ');
        }
        self.data.push_str(&format!(
            "A{} {} {} {} {} {} {}",
            px(rx),
            px(ry),
            px(x_axis_rotation),
            if large_arc { 1 } else { 0 },
            if sweep { 1 } else { 0 },
            px(x),
            px(y)
        ));
    }
    // TODO: Implement arc, etc.
    pub fn to_string(&self) -> &str {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_path_new() {
        let p = Path::new();
        assert_eq!(p.to_string(), "");
    }
    #[test]
    fn test_move_to() {
        let mut p = Path::new();
        p.move_to(10.0, 20.0);
        assert_eq!(p.to_string(), "M10 20");
    }
    #[test]
    fn test_line_to() {
        let mut p = Path::new();
        p.move_to(0.0, 0.0);
        p.line_to(10.0, 10.0);
        assert_eq!(p.to_string(), "M0 0 L10 10");
    }
    #[test]
    fn test_close_path() {
        let mut p = Path::new();
        p.move_to(0.0, 0.0);
        p.line_to(10.0, 0.0);
        p.close_path();
        assert_eq!(p.to_string(), "M0 0 L10 0Z");
    }
    #[test]
    fn test_quadratic_curve_to() {
        let mut p = Path::new();
        p.move_to(0.0, 0.0);
        p.quadratic_curve_to(5.0, 10.0, 10.0, 0.0);
        assert_eq!(p.to_string(), "M0 0 Q5 10 10 0");
    }
    #[test]
    fn test_bezier_curve_to() {
        let mut p = Path::new();
        p.move_to(0.0, 0.0);
        p.bezier_curve_to(2.0, 5.0, 8.0, 5.0, 10.0, 0.0);
        assert_eq!(p.to_string(), "M0 0 C2 5 8 5 10 0");
    }
    #[test]
    fn test_arc() {
        let mut p = Path::new();
        p.move_to(0.0, 0.0);
        p.arc(10.0, 10.0, 0.0, false, true, 10.0, 10.0);
        assert_eq!(p.to_string(), "M0 0 A10 10 0 0 1 10 10");
    }

    #[test]
    fn test_precision_formatting() {
        let mut p = Path::new();
        // Test with high-precision floating point values
        p.move_to(1.0/3.0, 3.1415926535);
        p.line_to(1.50000, 2.100000);
        // Should format with px function: rounded to 6 decimals and trimmed
        assert_eq!(p.to_string(), "M0.333333 3.141593 L1.5 2.1");
    }

    #[test] 
    fn test_zero_trimming() {
        let mut p = Path::new();
        p.move_to(5.000000, 0.0);
        p.quadratic_curve_to(2.500000, 1.000000, 7.000000, 0.000000);
        // Should trim trailing zeros
        assert_eq!(p.to_string(), "M5 0 Q2.5 1 7 0");
    }
}

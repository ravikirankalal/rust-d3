//! d3-path: Path serialization and construction utilities (Rust port)
//
// This module aims to provide a builder-style API for SVG path generation, similar to d3-path.
// See: https://github.com/d3/d3-path

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
        self.data.push_str(&format!("M{} {}", x, y));
    }
    pub fn line_to(&mut self, x: f64, y: f64) {
        if !self.data.is_empty() {
            self.data.push(' ');
        }
        self.data.push_str(&format!("L{} {}", x, y));
    }
    pub fn close_path(&mut self) {
        self.data.push_str("Z");
    }
    pub fn quadratic_curve_to(&mut self, cpx: f64, cpy: f64, x: f64, y: f64) {
        if !self.data.is_empty() {
            self.data.push(' ');
        }
        self.data.push_str(&format!("Q{} {} {} {}", cpx, cpy, x, y));
    }
    pub fn bezier_curve_to(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        if !self.data.is_empty() {
            self.data.push(' ');
        }
        self.data
            .push_str(&format!("C{} {} {} {} {} {}", cp1x, cp1y, cp2x, cp2y, x, y));
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
            rx,
            ry,
            x_axis_rotation,
            if large_arc { 1 } else { 0 },
            if sweep { 1 } else { 0 },
            x,
            y
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
}

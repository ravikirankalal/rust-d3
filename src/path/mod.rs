// D3 path module for Rust D3
// Provides a minimal SVG path builder utility similar to d3-path.

#[derive(Default, Debug)]
pub struct PathBuilder {
    pub d: String,
}

impl PathBuilder {
    pub fn new() -> Self {
        Self { d: String::new() }
    }
    pub fn move_to(&mut self, x: f64, y: f64) {
        self.d += &format!("M{} {} ", x, y);
    }
    pub fn line_to(&mut self, x: f64, y: f64) {
        self.d += &format!("L{} {} ", x, y);
    }
    pub fn close(&mut self) {
        self.d += "Z ";
    }
    pub fn to_string(&self) -> String {
        self.d.trim().to_string()
    }
    
    pub fn arc(&mut self, x: f64, y: f64, r: f64, a0: f64, a1: f64) {
        // SVG arc: A rx ry x-axis-rotation large-arc-flag sweep-flag x y
        let large_arc = if (a1 - a0).abs() > std::f64::consts::PI { 1 } else { 0 };
        let sweep = if a1 > a0 { 1 } else { 0 };
        let (x0, y0) = (x + r * a0.cos(), y + r * a0.sin());
        let (x1, y1) = (x + r * a1.cos(), y + r * a1.sin());
        self.d += &format!("M{} {} A{} {} 0 {} {} {} {} ", x0, y0, r, r, large_arc, sweep, x1, y1);
    }
    pub fn quadratic_curve_to(&mut self, cpx: f64, cpy: f64, x: f64, y: f64) {
        self.d += &format!("Q{} {} {} {} ", cpx, cpy, x, y);
    }
    pub fn bezier_curve_to(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        self.d += &format!("C{} {} {} {} {} {} ", cp1x, cp1y, cp2x, cp2y, x, y);
    }
    pub fn rect(&mut self, x: f64, y: f64, w: f64, h: f64) {
        self.move_to(x, y);
        self.line_to(x + w, y);
        self.line_to(x + w, y + h);
        self.line_to(x, y + h);
        self.close();
    }
}

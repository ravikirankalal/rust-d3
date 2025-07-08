// d3-shape: arc generator (SVG path string)
// Supports inner_radius, outer_radius, start_angle, end_angle, corner_radius, pad_angle, pad_radius

pub struct Arc<IR, OR, SA, EA, CR, PA, PR, T>
where
    IR: Fn(&T) -> f64,
    OR: Fn(&T) -> f64,
    SA: Fn(&T) -> f64,
    EA: Fn(&T) -> f64,
    CR: Fn(&T) -> f64,
    PA: Fn(&T) -> f64,
    PR: Fn(&T) -> f64,
{
    inner_radius: IR,
    outer_radius: OR,
    start_angle: SA,
    end_angle: EA,
    corner_radius: CR,
    pad_angle: PA,
    pad_radius: PR,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Arc<fn(&T) -> f64, fn(&T) -> f64, fn(&T) -> f64, fn(&T) -> f64, fn(&T) -> f64, fn(&T) -> f64, fn(&T) -> f64, T> {
    pub fn new() -> Self {
        Self {
            inner_radius: |_d| 0.0,
            outer_radius: |_d| 1.0,
            start_angle: |_d| 0.0,
            end_angle: |_d| std::f64::consts::PI,
            corner_radius: |_d| 0.0,
            pad_angle: |_d| 0.0,
            pad_radius: |_d| 0.0,
            _phantom: std::marker::PhantomData,
        }
    }
}

pub trait ArcOutput {
    fn move_to(&mut self, x: f64, y: f64);
    fn arc_to(&mut self, rx: f64, ry: f64, x: f64, y: f64, large_arc: bool, sweep: bool);
    fn line_to(&mut self, x: f64, y: f64);
    fn close(&mut self) {}
}

impl ArcOutput for String {
    fn move_to(&mut self, x: f64, y: f64) {
        self.push_str(&format!("M{},{}", x, y));
    }
    fn arc_to(&mut self, rx: f64, ry: f64, x: f64, y: f64, large_arc: bool, sweep: bool) {
        self.push_str(&format!("A{},{} 0 {},{} {},{}", rx, ry, if large_arc {1} else {0}, if sweep {1} else {0}, x, y));
    }
    fn line_to(&mut self, x: f64, y: f64) {
        self.push_str(&format!("L{},{}", x, y));
    }
}

impl<IR, OR, SA, EA, CR, PA, PR, T> Arc<IR, OR, SA, EA, CR, PA, PR, T>
where
    IR: Fn(&T) -> f64,
    OR: Fn(&T) -> f64,
    SA: Fn(&T) -> f64,
    EA: Fn(&T) -> f64,
    CR: Fn(&T) -> f64,
    PA: Fn(&T) -> f64,
    PR: Fn(&T) -> f64,
{
    pub fn inner_radius<IR2>(self, inner_radius: IR2) -> Arc<IR2, OR, SA, EA, CR, PA, PR, T>
    where IR2: Fn(&T) -> f64 {
        Arc { inner_radius, outer_radius: self.outer_radius, start_angle: self.start_angle, end_angle: self.end_angle, corner_radius: self.corner_radius, pad_angle: self.pad_angle, pad_radius: self.pad_radius, _phantom: std::marker::PhantomData }
    }
    pub fn outer_radius<OR2>(self, outer_radius: OR2) -> Arc<IR, OR2, SA, EA, CR, PA, PR, T>
    where OR2: Fn(&T) -> f64 {
        Arc { inner_radius: self.inner_radius, outer_radius, start_angle: self.start_angle, end_angle: self.end_angle, corner_radius: self.corner_radius, pad_angle: self.pad_angle, pad_radius: self.pad_radius, _phantom: std::marker::PhantomData }
    }
    pub fn start_angle<SA2>(self, start_angle: SA2) -> Arc<IR, OR, SA2, EA, CR, PA, PR, T>
    where SA2: Fn(&T) -> f64 {
        Arc { inner_radius: self.inner_radius, outer_radius: self.outer_radius, start_angle, end_angle: self.end_angle, corner_radius: self.corner_radius, pad_angle: self.pad_angle, pad_radius: self.pad_radius, _phantom: std::marker::PhantomData }
    }
    pub fn end_angle<EA2>(self, end_angle: EA2) -> Arc<IR, OR, SA, EA2, CR, PA, PR, T>
    where EA2: Fn(&T) -> f64 {
        Arc { inner_radius: self.inner_radius, outer_radius: self.outer_radius, start_angle: self.start_angle, end_angle, corner_radius: self.corner_radius, pad_angle: self.pad_angle, pad_radius: self.pad_radius, _phantom: std::marker::PhantomData }
    }
    pub fn corner_radius<CR2>(self, corner_radius: CR2) -> Arc<IR, OR, SA, EA, CR2, PA, PR, T>
    where CR2: Fn(&T) -> f64 {
        Arc { inner_radius: self.inner_radius, outer_radius: self.outer_radius, start_angle: self.start_angle, end_angle: self.end_angle, corner_radius, pad_angle: self.pad_angle, pad_radius: self.pad_radius, _phantom: std::marker::PhantomData }
    }
    pub fn pad_angle<PA2>(self, pad_angle: PA2) -> Arc<IR, OR, SA, EA, CR, PA2, PR, T>
    where PA2: Fn(&T) -> f64 {
        Arc { inner_radius: self.inner_radius, outer_radius: self.outer_radius, start_angle: self.start_angle, end_angle: self.end_angle, corner_radius: self.corner_radius, pad_angle, pad_radius: self.pad_radius, _phantom: std::marker::PhantomData }
    }
    pub fn pad_radius<PR2>(self, pad_radius: PR2) -> Arc<IR, OR, SA, EA, CR, PA, PR2, T>
    where PR2: Fn(&T) -> f64 {
        Arc { inner_radius: self.inner_radius, outer_radius: self.outer_radius, start_angle: self.start_angle, end_angle: self.end_angle, corner_radius: self.corner_radius, pad_angle: self.pad_angle, pad_radius, _phantom: std::marker::PhantomData }
    }
    pub fn generate_to<O: ArcOutput>(&self, datum: &T, out: &mut O) {
        let ir = (self.inner_radius)(datum);
        let or = (self.outer_radius)(datum);
        let sa = (self.start_angle)(datum);
        let ea = (self.end_angle)(datum);
        // Only basic arc for now
        let (x0, y0) = (or * sa.cos(), or * sa.sin());
        let (x1, y1) = (or * ea.cos(), or * ea.sin());
        out.move_to(x0, y0);
        out.arc_to(or, or, x1, y1, false, true);
        if ir > 0.0 {
            let (x2, y2) = (ir * ea.cos(), ir * ea.sin());
            let (x3, y3) = (ir * sa.cos(), ir * sa.sin());
            out.line_to(x2, y2);
            out.arc_to(ir, ir, x3, y3, false, false);
        }
        out.close();
    }
    pub fn generate(&self, datum: &T) -> String {
        let mut out = String::new();
        self.generate_to(datum, &mut out);
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_arc_basic() {
        let arc = Arc::new()
            .inner_radius(|_d: &f64| 0.0)
            .outer_radius(|_d: &f64| 10.0)
            .start_angle(|_d: &f64| 0.0)
            .end_angle(|_d: &f64| std::f64::consts::PI);
        let path = arc.generate(&0.0);
        assert!(path.starts_with('M'));
        assert!(path.contains('A'));
    }
}

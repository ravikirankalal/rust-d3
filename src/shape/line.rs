// d3-shape: line generator (SVG path string)
// Supports x/y accessors, defined accessor, and curve (linear only for now)

pub struct Line<X, Y, D, T, C>
where
    X: Fn(&T, usize) -> f64,
    Y: Fn(&T, usize) -> f64,
    D: Fn(&T, usize) -> bool,
    C: crate::shape::curve::Curve + Default + Clone,
{
    x: X,
    y: Y,
    defined: D,
    curve: C,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Line<fn(&T, usize) -> f64, fn(&T, usize) -> f64, fn(&T, usize) -> bool, T, crate::shape::curve::LinearCurve> {
    pub fn new() -> Self {
        Self {
            x: |d, _| d as *const T as usize as f64, // placeholder, user must set
            y: |d, _| d as *const T as usize as f64, // placeholder, user must set
            defined: |_d, _| true,
            curve: crate::shape::curve::LinearCurve::default(),
            _phantom: std::marker::PhantomData,
        }
    }
}

pub trait LineOutput {
    fn move_to(&mut self, x: f64, y: f64);
    fn line_to(&mut self, x: f64, y: f64);
    fn close(&mut self) {}
}

impl LineOutput for String {
    fn move_to(&mut self, x: f64, y: f64) {
        self.push_str(&format!("M{},{}", x, y));
    }
    fn line_to(&mut self, x: f64, y: f64) {
        self.push_str(&format!("L{},{}", x, y));
    }
}

impl<X, Y, D, T, C> Line<X, Y, D, T, C>
where
    X: Fn(&T, usize) -> f64,
    Y: Fn(&T, usize) -> f64,
    D: Fn(&T, usize) -> bool,
    C: crate::shape::curve::Curve + Default + Clone,
{
    pub fn x<X2>(self, x: X2) -> Line<X2, Y, D, T, C>
    where
        X2: Fn(&T, usize) -> f64,
    {
        Line { x, y: self.y, defined: self.defined, curve: self.curve.clone(), _phantom: std::marker::PhantomData }
    }
    pub fn y<Y2>(self, y: Y2) -> Line<X, Y2, D, T, C>
    where
        Y2: Fn(&T, usize) -> f64,
    {
        Line { x: self.x, y, defined: self.defined, curve: self.curve.clone(), _phantom: std::marker::PhantomData }
    }
    pub fn defined<D2>(self, defined: D2) -> Line<X, Y, D2, T, C>
    where
        D2: Fn(&T, usize) -> bool,
    {
        Line { x: self.x, y: self.y, defined, curve: self.curve.clone(), _phantom: std::marker::PhantomData }
    }
    pub fn curve<C2: crate::shape::curve::Curve + Default + Clone>(self, curve: C2) -> Line<X, Y, D, T, C2> {
        Line { x: self.x, y: self.y, defined: self.defined, curve, _phantom: std::marker::PhantomData }
    }
    pub fn basis_curve(self) -> Line<X, Y, D, T, crate::shape::curve::BasisCurve> {
        Line { x: self.x, y: self.y, defined: self.defined, curve: crate::shape::curve::BasisCurve::default(), _phantom: std::marker::PhantomData }
    }
    pub fn cardinal_curve(self) -> Line<X, Y, D, T, crate::shape::curve::CardinalCurve> {
        Line { x: self.x, y: self.y, defined: self.defined, curve: crate::shape::curve::CardinalCurve::default(), _phantom: std::marker::PhantomData }
    }
    pub fn monotone_curve(self) -> Line<X, Y, D, T, crate::shape::curve::MonotoneCurve> {
        Line { x: self.x, y: self.y, defined: self.defined, curve: crate::shape::curve::MonotoneCurve::default(), _phantom: std::marker::PhantomData }
    }
    pub fn generate_to<O: LineOutput>(&self, data: &[T], out: &mut O) {
        let mut first = true;
        for (i, d) in data.iter().enumerate() {
            if !(self.defined)(d, i) {
                first = true;
                continue;
            }
            let x = (self.x)(d, i);
            let y = (self.y)(d, i);
            if x.is_nan() || y.is_nan() {
                first = true;
                continue;
            }
            if first {
                out.move_to(x, y);
                first = false;
            } else {
                out.line_to(x, y);
            }
        }
    }
    pub fn generate(&self, data: &[T]) -> String {
        let mut path = String::new();
        let mut curve = self.curve.clone();
        let mut first = true;
        let mut has_point = false;
        curve.begin(&mut path);
        for (i, d) in data.iter().enumerate() {
            if !(self.defined)(d, i) {
                first = true;
                continue;
            }
            let x = (self.x)(d, i);
            let y = (self.y)(d, i);
            if x.is_nan() || y.is_nan() {
                first = true;
                continue;
            }
            curve.line_to(&mut path, x, y, first);
            first = false;
            has_point = true;
        }
        if has_point {
            curve.end(&mut path);
            path
        } else {
            String::new() // D3 returns empty string for all-NaN/undefined
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::curve::{StepCurve}; // removed unused LinearCurve
    #[test]
    fn test_line_basic() {
        let data = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 0.0)];
        let line = Line::new()
            .x(|d: &(f64, f64), _| d.0)
            .y(|d: &(f64, f64), _| d.1);
        let path = line.generate(&data);
        assert_eq!(path, "M0,0L1,1L2,0");
    }
    #[test]
    fn test_line_with_defined() {
        let data = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 0.0)];
        let line = Line::new()
            .x(|d: &(f64, f64), _| d.0)
            .y(|d: &(f64, f64), _| d.1)
            .defined(|d: &(f64, f64), _| d.0 != 1.0);
        let path = line.generate(&data);
        assert_eq!(path, "M0,0M2,0");
    }
    #[test]
    fn test_line_step_curve() {
        let data = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 0.0)];
        let line = Line::new()
            .x(|d: &(f64, f64), _| d.0)
            .y(|d: &(f64, f64), _| d.1)
            .curve(StepCurve::default());
        let path = line.generate(&data);
        // println!("Generated path: {}", path);
        // Step curve should use H and V commands (may be in any order)
        assert!(path.contains('H') || path.contains('V'));
    }
}

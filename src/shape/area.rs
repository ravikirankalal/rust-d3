// d3-shape: area generator (SVG path string)
// Supports x/x0/x1/y/y0/y1/defined/curve

use crate::shape::curve::{Curve, LinearCurve, StepCurve, BasisCurve, CardinalCurve, MonotoneCurve};

pub struct Area<X0, X1, Y0, Y1, D, T, C>
where
    X0: Fn(&T, usize) -> f64,
    X1: Fn(&T, usize) -> f64,
    Y0: Fn(&T, usize) -> f64,
    Y1: Fn(&T, usize) -> f64,
    D: Fn(&T, usize) -> bool,
    C: Curve + Default + Clone,
{
    x0: X0,
    x1: X1,
    y0: Y0,
    y1: Y1,
    defined: D,
    curve: C,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Area<fn(&T, usize) -> f64, fn(&T, usize) -> f64, fn(&T, usize) -> f64, fn(&T, usize) -> f64, fn(&T, usize) -> bool, T, LinearCurve> {
    pub fn new() -> Self {
        Self {
            x0: |d, _| d as *const T as usize as f64,
            x1: |d, _| d as *const T as usize as f64,
            y0: |d, _| d as *const T as usize as f64,
            y1: |d, _| d as *const T as usize as f64,
            defined: |_d, _| true,
            curve: LinearCurve::default(),
            _phantom: std::marker::PhantomData,
        }
    }
}

pub trait AreaOutput {
    fn move_to(&mut self, x: f64, y: f64);
    fn line_to(&mut self, x: f64, y: f64);
    fn close(&mut self) {}
}

impl AreaOutput for String {
    fn move_to(&mut self, x: f64, y: f64) {
        self.push_str(&format!("M{},{}", x, y));
    }
    fn line_to(&mut self, x: f64, y: f64) {
        self.push_str(&format!("L{},{}", x, y));
    }
}

impl<X0, X1, Y0, Y1, D, T, C> Area<X0, X1, Y0, Y1, D, T, C>
where
    X0: Fn(&T, usize) -> f64,
    X1: Fn(&T, usize) -> f64,
    Y0: Fn(&T, usize) -> f64,
    Y1: Fn(&T, usize) -> f64,
    D: Fn(&T, usize) -> bool,
    C: crate::shape::curve::Curve + Default + Clone,
{
    pub fn x0<X0b>(self, x0: X0b) -> Area<X0b, X1, Y0, Y1, D, T, C>
    where X0b: Fn(&T, usize) -> f64 {
        Area { x0, x1: self.x1, y0: self.y0, y1: self.y1, defined: self.defined, curve: self.curve.clone(), _phantom: std::marker::PhantomData }
    }
    pub fn x1<X1b>(self, x1: X1b) -> Area<X0, X1b, Y0, Y1, D, T, C>
    where X1b: Fn(&T, usize) -> f64 {
        Area { x0: self.x0, x1, y0: self.y0, y1: self.y1, defined: self.defined, curve: self.curve.clone(), _phantom: std::marker::PhantomData }
    }
    pub fn y0<Y0b>(self, y0: Y0b) -> Area<X0, X1, Y0b, Y1, D, T, C>
    where Y0b: Fn(&T, usize) -> f64 {
        Area { x0: self.x0, x1: self.x1, y0, y1: self.y1, defined: self.defined, curve: self.curve.clone(), _phantom: std::marker::PhantomData }
    }
    pub fn y1<Y1b>(self, y1: Y1b) -> Area<X0, X1, Y0, Y1b, D, T, C>
    where Y1b: Fn(&T, usize) -> f64 {
        Area { x0: self.x0, x1: self.x1, y0: self.y0, y1, defined: self.defined, curve: self.curve.clone(), _phantom: std::marker::PhantomData }
    }
    pub fn defined<Db>(self, defined: Db) -> Area<X0, X1, Y0, Y1, Db, T, C>
    where Db: Fn(&T, usize) -> bool {
        Area { x0: self.x0, x1: self.x1, y0: self.y0, y1: self.y1, defined, curve: self.curve.clone(), _phantom: std::marker::PhantomData }
    }
    pub fn curve<C2: Curve + Default + Clone>(self, curve: C2) -> Area<X0, X1, Y0, Y1, D, T, C2> {
        Area { x0: self.x0, x1: self.x1, y0: self.y0, y1: self.y1, defined: self.defined, curve, _phantom: std::marker::PhantomData }
    }
    pub fn basis_curve(self) -> Area<X0, X1, Y0, Y1, D, T, BasisCurve> {
        Area { x0: self.x0, x1: self.x1, y0: self.y0, y1: self.y1, defined: self.defined, curve: BasisCurve::default(), _phantom: std::marker::PhantomData }
    }
    pub fn cardinal_curve(self) -> Area<X0, X1, Y0, Y1, D, T, CardinalCurve> {
        Area { x0: self.x0, x1: self.x1, y0: self.y0, y1: self.y1, defined: self.defined, curve: CardinalCurve::default(), _phantom: std::marker::PhantomData }
    }
    pub fn monotone_curve(self) -> Area<X0, X1, Y0, Y1, D, T, MonotoneCurve> {
        Area { x0: self.x0, x1: self.x1, y0: self.y0, y1: self.y1, defined: self.defined, curve: MonotoneCurve::default(), _phantom: std::marker::PhantomData }
    }
    pub fn generate_to<O: AreaOutput>(&self, data: &[T], out: &mut O) {
        let mut first = true;
        for (i, d) in data.iter().enumerate() {
            if !(self.defined)(d, i) {
                first = true;
                continue;
            }
            let x0 = (self.x0)(d, i);
            let y0 = (self.y0)(d, i);
            if x0.is_nan() || y0.is_nan() {
                first = true;
                continue;
            }
            if first {
                out.move_to(x0, y0);
                first = false;
            } else {
                out.line_to(x0, y0);
            }
        }
        out.close();
    }
    pub fn generate(&self, data: &[T]) -> String {
        let mut path = String::new();
        let mut curve = self.curve.clone();
        let mut top: Vec<(f64, f64)> = Vec::new();
        let mut bottom: Vec<(f64, f64)> = Vec::new();
        for (i, d) in data.iter().enumerate() {
            if !(self.defined)(d, i) { continue; }
            let x1 = (self.x1)(d, i);
            let y1 = (self.y1)(d, i);
            if x1.is_nan() || y1.is_nan() { continue; }
            top.push((x1, y1));
        }
        for (i, d) in data.iter().enumerate().rev() {
            if !(self.defined)(d, i) { continue; }
            let x0 = (self.x0)(d, i);
            let y0 = (self.y0)(d, i);
            if x0.is_nan() || y0.is_nan() { continue; }
            bottom.push((x0, y0));
        }
        if top.is_empty() || bottom.is_empty() {
            return String::new(); // D3 returns empty string for empty/all-NaN input
        }
        // Top boundary
        curve.begin(&mut path);
        let mut first = true;
        for &(x, y) in &top {
            curve.line_to(&mut path, x, y, first);
            first = false;
        }
        // Bottom boundary
        for &(x, y) in &bottom {
            curve.line_to(&mut path, x, y, false);
        }
        curve.end(&mut path);
        path.push_str("Z");
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_area_basic() {
        let data = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 0.0)];
        let area = Area::new()
            .x0(|d: &(f64, f64), _| d.0)
            .y0(|_d: &(f64, f64), _| 0.0)
            .x1(|d: &(f64, f64), _| d.0)
            .y1(|d: &(f64, f64), _| d.1);
        let path = area.generate(&data);
        assert!(path.contains('M'));
        assert!(path.contains('L'));
    }
}

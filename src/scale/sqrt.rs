// Sqrt scale (special case of PowScale)
use super::PowScale;

pub struct SqrtScale {
    inner: PowScale,
}

impl SqrtScale {
    pub fn new(domain: (f64, f64), range: (f64, f64)) -> Self {
        Self { inner: PowScale::new(domain, range, 0.5) }
    }
    pub fn scale(&self, value: f64) -> f64 {
        self.inner.scale(value)
    }
    pub fn domain(&self) -> (f64, f64) {
        self.inner.domain()
    }
    pub fn range(&self) -> (f64, f64) {
        self.inner.range()
    }
    pub fn invert(&self, value: f64) -> f64 {
        self.inner.invert(value)
    }
    pub fn clamp(&mut self) {
        self.inner.clamp();
    }
    pub fn nice(&mut self) {
        self.inner.nice();
    }
    pub fn ticks(&self, count: usize) -> Vec<f64> {
        self.inner.ticks(count)
    }
    pub fn tick_format(&self, precision: usize) -> impl Fn(f64) -> String {
        self.inner.tick_format(precision)
    }
}

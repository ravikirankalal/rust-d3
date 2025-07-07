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
}

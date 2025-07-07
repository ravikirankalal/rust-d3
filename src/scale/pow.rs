// Power scale
pub struct PowScale {
    domain: (f64, f64),
    range: (f64, f64),
    exponent: f64,
}

impl PowScale {
    pub fn new(domain: (f64, f64), range: (f64, f64), exponent: f64) -> Self {
        Self { domain, range, exponent }
    }
    pub fn scale(&self, value: f64) -> f64 {
        let (d0, d1) = self.domain;
        let (r0, r1) = self.range;
        let pow = |x: f64| x.powf(self.exponent);
        let t = (pow(value) - pow(d0)) / (pow(d1) - pow(d0));
        r0 + t * (r1 - r0)
    }
}

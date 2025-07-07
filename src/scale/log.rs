// Logarithmic scale
pub struct LogScale {
    domain: (f64, f64),
    range: (f64, f64),
    base: f64,
}

impl LogScale {
    pub fn new(domain: (f64, f64), range: (f64, f64), base: f64) -> Self {
        Self { domain, range, base }
    }
    pub fn scale(&self, value: f64) -> f64 {
        let (d0, d1) = self.domain;
        let (r0, r1) = self.range;
        let log = |x: f64| x.log(self.base);
        let t = (log(value) - log(d0)) / (log(d1) - log(d0));
        r0 + t * (r1 - r0)
    }
}

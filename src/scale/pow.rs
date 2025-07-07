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
    pub fn domain(&self) -> (f64, f64) {
        self.domain
    }
    pub fn range(&self) -> (f64, f64) {
        self.range
    }
    pub fn exponent(&self) -> f64 {
        self.exponent
    }
    pub fn invert(&self, value: f64) -> f64 {
        let (d0, d1) = self.domain;
        let (r0, r1) = self.range;
        let pow = |x: f64| x.powf(self.exponent);
        let root = |x: f64| x.powf(1.0 / self.exponent);
        if r1 == r0 {
            return d0;
        }
        let t = (value - r0) / (r1 - r0);
        root(pow(d0) + t * (pow(d1) - pow(d0)))
    }
    pub fn clamp(&mut self) {
        let (d0, d1) = self.domain;
        let (r0, r1) = self.range;
        self.domain = (d0.min(d1), d0.max(d1));
        self.range = (r0.min(r1), r0.max(r1));
    }
    pub fn nice(&mut self) {
        // Simple nice: round domain to nearest integer
        let (d0, d1) = self.domain;
        self.domain = (d0.floor(), d1.ceil());
    }
    pub fn ticks(&self, count: usize) -> Vec<f64> {
        let (d0, d1) = self.domain;
        let step = (d1 - d0) / (count as f64 - 1.0);
        (0..count).map(|i| d0 + i as f64 * step).collect()
    }
    pub fn tick_format(&self, precision: usize) -> impl Fn(f64) -> String {
        move |v| format!("{:.1$}", v, precision)
    }
}

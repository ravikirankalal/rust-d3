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
    pub fn domain(&self) -> (f64, f64) {
        self.domain
    }
    pub fn range(&self) -> (f64, f64) {
        self.range
    }
    pub fn base(&self) -> f64 {
        self.base
    }
    pub fn invert(&self, value: f64) -> f64 {
        let (d0, d1) = self.domain;
        let (r0, r1) = self.range;
        let log = |x: f64| x.log(self.base);
        let exp = |x: f64| self.base.powf(x);
        if r1 == r0 {
            return d0;
        }
        let t = (value - r0) / (r1 - r0);
        exp(log(d0) + t * (log(d1) - log(d0)))
    }
    pub fn clamp(&mut self) {
        let (d0, d1) = self.domain;
        let (r0, r1) = self.range;
        self.domain = (d0.min(d1), d0.max(d1));
        self.range = (r0.min(r1), r0.max(r1));
    }
    pub fn nice(&mut self) {
        // Simple nice: round domain to nearest integer powers of base
        let (d0, d1) = self.domain;
        self.domain = (self.base.powf(d0.log(self.base).floor()), self.base.powf(d1.log(self.base).ceil()));
    }
    pub fn ticks(&self, _count: usize) -> Vec<f64> {
        let (d0, d1) = self.domain;
        let log = |x: f64| x.log(self.base);
        let exp = |x: f64| self.base.powf(x);
        let start = log(d0).ceil();
        let end = log(d1).floor();
        let mut ticks = Vec::new();
        for i in (start as i32)..=(end as i32) {
            ticks.push(exp(i as f64));
        }
        ticks
    }
    pub fn tick_format(&self, precision: usize) -> impl Fn(f64) -> String {
        move |v| format!("{:.1$}", v, precision)
    }
}

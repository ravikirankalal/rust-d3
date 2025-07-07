// Linear scale module for Rust D3

pub struct LinearScale {
    domain: (f64, f64),
    range: (f64, f64),
}

impl LinearScale {
    pub fn new(domain: (f64, f64), range: (f64, f64)) -> Self {
        Self { domain, range }
    }

    pub fn scale(&self, value: f64) -> f64 {
        let (d0, d1) = self.domain;
        let (r0, r1) = self.range;
        if d1 == d0 {
            return r0;
        }
        r0 + (value - d0) * (r1 - r0) / (d1 - d0)
    }

    pub fn set_domain(&mut self, domain: (f64, f64)) {
        self.domain = domain;
    }

    pub fn set_range(&mut self, range: (f64, f64)) {
        self.range = range;
    }

    pub fn domain(&self) -> (f64, f64) {
        self.domain
    }

    pub fn range(&self) -> (f64, f64) {
        self.range
    }

    pub fn invert(&self, value: f64) -> f64 {
        let (d0, d1) = self.domain;
        let (r0, r1) = self.range;
        if r1 == r0 {
            return d0;
        }
        d0 + (value - r0) * (d1 - d0) / (r1 - r0)
    }

    pub fn clamp(&mut self) {
        // Clamp domain and range to min/max
        let (d0, d1) = self.domain;
        let (r0, r1) = self.range;
        self.domain = (d0.min(d1), d0.max(d1));
        self.range = (r0.min(r1), r0.max(r1));
    }

    pub fn nice(&mut self) {
        // Simple nice implementation: round domain to nearest integer
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

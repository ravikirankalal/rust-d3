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
}

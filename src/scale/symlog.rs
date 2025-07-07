// d3-scale: ScaleSymlog
#[derive(Debug, Clone)]
pub struct ScaleSymlog {
    pub domain: [f64; 2],
    pub range: [f64; 2],
    pub constant: f64, // typically 1.0 in D3
}

impl ScaleSymlog {
    pub fn new(domain: [f64; 2], range: [f64; 2], constant: f64) -> Self {
        Self { domain, range, constant }
    }
    fn symlog(&self, x: f64) -> f64 {
        let c = self.constant;
        x.signum() * (x.abs() + c).ln() - c.ln()
    }
    fn symexp(&self, y: f64) -> f64 {
        let c = self.constant;
        (y + c.ln()).exp() - c
    }
    pub fn scale(&self, x: f64) -> f64 {
        let d0 = self.symlog(self.domain[0]);
        let d1 = self.symlog(self.domain[1]);
        let t = (self.symlog(x) - d0) / (d1 - d0);
        self.range[0] + t * (self.range[1] - self.range[0])
    }
    pub fn invert(&self, y: f64) -> f64 {
        let d0 = self.symlog(self.domain[0]);
        let d1 = self.symlog(self.domain[1]);
        let t = (y - self.range[0]) / (self.range[1] - self.range[0]);
        self.symexp(d0 + t * (d1 - d0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_symlog() {
        let s = ScaleSymlog::new([-10.0, 10.0], [0.0, 100.0], 1.0);
        let mid = s.scale(0.0);
        assert!((mid - 50.0).abs() < 1e-6);
        let inv = s.invert(mid);
        assert!((inv - 0.0).abs() < 1e-6);
    }
}

// d3-scale: ScalePow
#[derive(Debug, Clone)]
pub struct ScalePow {
    pub domain: [f64; 2],
    pub range: [f64; 2],
    pub exponent: f64,
}

impl ScalePow {
    pub fn new(domain: [f64; 2], range: [f64; 2], exponent: f64) -> Self {
        Self {
            domain,
            range,
            exponent,
        }
    }
    pub fn scale(&self, x: f64) -> f64 {
        let t = ((x - self.domain[0]) / (self.domain[1] - self.domain[0])).powf(self.exponent);
        self.range[0] + t * (self.range[1] - self.range[0])
    }
    pub fn invert(&self, y: f64) -> f64 {
        let t = (y - self.range[0]) / (self.range[1] - self.range[0]);
        self.domain[0] + t.powf(1.0 / self.exponent) * (self.domain[1] - self.domain[0])
    }
}

// d3-scale: ScaleLinear
#[derive(Debug, Clone)]
pub struct ScaleLinear {
    pub domain: [f64; 2],
    pub range: [f64; 2],
}

impl ScaleLinear {
    pub fn new(domain: [f64; 2], range: [f64; 2]) -> Self {
        Self { domain, range }
    }
    pub fn scale(&self, x: f64) -> f64 {
        let t = (x - self.domain[0]) / (self.domain[1] - self.domain[0]);
        self.range[0] + t * (self.range[1] - self.range[0])
    }
    pub fn invert(&self, y: f64) -> f64 {
        let t = (y - self.range[0]) / (self.range[1] - self.range[0]);
        self.domain[0] + t * (self.domain[1] - self.domain[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_linear() {
        let s = ScaleLinear::new([0.0, 10.0], [0.0, 100.0]);
        assert_eq!(s.scale(5.0), 50.0);
        assert_eq!(s.invert(50.0), 5.0);
    }
}

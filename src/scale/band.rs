// d3-scale: ScaleBand (D3-like, with padding/align)
#[derive(Debug, Clone)]
pub struct ScaleBand<T: Clone> {
    pub domain: Vec<T>,
    pub range: [f64; 2],
    pub padding_inner: f64,
    pub padding_outer: f64,
    pub align: f64, // 0.0 (left) to 1.0 (right), D3 default 0.5
}

impl<T: Clone + PartialEq> ScaleBand<T> {
    pub fn new(
        domain: Vec<T>,
        range: [f64; 2],
        padding_inner: f64,
        padding_outer: f64,
        align: f64,
    ) -> Self {
        Self {
            domain,
            range,
            padding_inner,
            padding_outer,
            align,
        }
    }
    pub fn step(&self) -> f64 {
        let n = self.domain.len() as f64;
        if n < 1.0 {
            return 0.0;
        }
        (self.range[1] - self.range[0]) / (n + self.padding_outer * 2.0 - self.padding_inner)
    }
    pub fn bandwidth(&self) -> f64 {
        let n = self.domain.len() as f64;
        if n < 1.0 {
            return 0.0;
        }
        self.step() * (1.0 - self.padding_inner)
    }
    pub fn scale(&self, x: &T) -> Option<f64> {
        let n = self.domain.len() as f64;
        if n < 1.0 {
            return None;
        }
        let step = self.step();
        let start = self.range[0] + (self.range[1] - self.range[0] - step * n) * self.align;
        self.domain
            .iter()
            .position(|v| v == x)
            .map(|i| start + i as f64 * step)
    }
    
    pub fn range(&self) -> [f64; 2] {
        self.range
    }
}

// Implementation for axis rendering
impl<T: Clone + PartialEq> crate::axis::axis_common::ScaleWithRange for ScaleBand<T> {
    fn range(&self) -> [f64; 2] {
        self.range()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_band() {
        let s = ScaleBand::new(vec!["a", "b", "c"], [0.0, 120.0], 0.1, 0.1, 0.5);
        let bw = s.bandwidth();
        assert!((bw - 34.8387).abs() < 1e-3);
        let a = s.scale(&"a").unwrap();
        assert!((a - 1.9355).abs() < 1e-3);
    }
}

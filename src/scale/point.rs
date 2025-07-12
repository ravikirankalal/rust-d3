// d3-scale: ScalePoint (D3-like, with align)
#[derive(Debug, Clone)]
pub struct ScalePoint<T: Clone> {
    pub domain: Vec<T>,
    pub range: [f64; 2],
    pub align: f64, // 0.0 (left) to 1.0 (right), D3 default 0.5
}

impl<T: Clone + PartialEq> ScalePoint<T> {
    pub fn new(domain: Vec<T>, range: [f64; 2], align: f64) -> Self {
        Self { domain, range, align }
    }
    pub fn scale(&self, x: &T) -> Option<f64> {
        let n = self.domain.len();
        if n == 0 { return None; }
        let step = if n > 1 {
            (self.range[1] - self.range[0]) / (n as f64 - 1.0)
        } else { 0.0 };
        let start = self.range[0] + (self.range[1] - self.range[0] - step * (n as f64 - 1.0)) * self.align;
        self.domain.iter().position(|v| v == x).map(|i| {
            start + i as f64 * step
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_point() {
        let s = ScalePoint::new(vec!["a", "b", "c"], [0.0, 100.0], 0.5);
        assert!((s.scale(&"a").unwrap() - 0.0).abs() < 1e-6);
        assert!((s.scale(&"b").unwrap() - 50.0).abs() < 1e-6);
        assert!((s.scale(&"c").unwrap() - 100.0).abs() < 1e-6);
    }
}

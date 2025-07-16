// d3-scale: ScalePoint (D3-like, with align)
#[derive(Debug, Clone)]
pub struct ScalePoint<T: Clone> {
    pub domain: Vec<T>,
    pub range: [f64; 2],
    pub align: f64, // 0.0 (left) to 1.0 (right), D3 default 0.5
}

impl<T: Clone + PartialEq> ScalePoint<T> {
    pub fn new(domain: Vec<T>, range: [f64; 2], align: f64) -> Self {
        Self {
            domain,
            range,
            align,
        }
    }
    pub fn scale(&self, x: &T) -> Option<f64> {
        let n = self.domain.len();
        if n == 0 {
            return None;
        }
        
        // Calculate the step size with padding
        let range_size = self.range[1] - self.range[0];
        let padding = self.align; // Align used directly as padding factor
        let denominator = ((n - 1) as f64 + 2.0 * padding).max(1.0);
        let step = range_size / denominator;
        let start = self.range[0] + padding * step;
        
        self.domain
            .iter()
            .position(|v| v == x)
            .map(|i| start + i as f64 * step)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_point() {
        let s = ScalePoint::new(vec!["a", "b", "c"], [0.0, 100.0], 0.5);
        // With padding = 0.5, domain length = 3:
        // step = 100 / max(1, 3 - 1 + 2 * 0.5) = 100 / 3 = 33.333...
        // start = 0 + 0.5 * 33.333... = 16.666...
        // a: 16.666... + 0 * 33.333... = 16.666...
        // b: 16.666... + 1 * 33.333... = 50.0
        // c: 16.666... + 2 * 33.333... = 83.333...
        assert!((s.scale(&"a").unwrap() - 16.666666666666668).abs() < 1e-6);
        assert!((s.scale(&"b").unwrap() - 50.0).abs() < 1e-6);
        assert!((s.scale(&"c").unwrap() - 83.33333333333334).abs() < 1e-6);
    }
    
    #[test]
    fn test_point_with_padding_quarter() {
        // Test case: 3 items, padding 0.25 (common case)
        let s = ScalePoint::new(vec!["First", "Second", "Third"], [0.0, 300.0], 0.25);
        // With padding = 0.25, domain length = 3:
        // step = 300 / max(1, 3 - 1 + 2 * 0.25) = 300 / 2.5 = 120
        // start = 0 + 0.25 * 120 = 30
        // First: 30 + 0 * 120 = 30
        // Second: 30 + 1 * 120 = 150
        // Third: 30 + 2 * 120 = 270
        assert!((s.scale(&"First").unwrap() - 30.0).abs() < 1e-6);
        assert!((s.scale(&"Second").unwrap() - 150.0).abs() < 1e-6);
        assert!((s.scale(&"Third").unwrap() - 270.0).abs() < 1e-6);
    }
    
    #[test]
    fn test_point_edge_cases() {
        // Test with single item
        let s1 = ScalePoint::new(vec!["only"], [0.0, 100.0], 0.25);
        // With padding = 0.25, domain length = 1:
        // step = 100 / max(1, 1 - 1 + 2 * 0.25) = 100 / 1 = 100
        // start = 0 + 0.25 * 100 = 25
        // only: 25 + 0 * 100 = 25
        assert!((s1.scale(&"only").unwrap() - 25.0).abs() < 1e-6);
        
        // Test with no padding (align = 0.0)
        let s2 = ScalePoint::new(vec!["a", "b", "c"], [0.0, 100.0], 0.0);
        // With padding = 0.0, domain length = 3:
        // step = 100 / max(1, 3 - 1 + 2 * 0.0) = 100 / 2 = 50
        // start = 0 + 0.0 * 50 = 0
        // a: 0 + 0 * 50 = 0
        // b: 0 + 1 * 50 = 50
        // c: 0 + 2 * 50 = 100
        assert!((s2.scale(&"a").unwrap() - 0.0).abs() < 1e-6);
        assert!((s2.scale(&"b").unwrap() - 50.0).abs() < 1e-6);
        assert!((s2.scale(&"c").unwrap() - 100.0).abs() < 1e-6);
    }
}

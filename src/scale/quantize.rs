// d3-scale: ScaleQuantize
// Maps a continuous domain to a discrete range with uniform intervals

#[derive(Debug, Clone)]
pub struct ScaleQuantize<R: Clone> {
    pub domain: [f64; 2],
    pub range: Vec<R>,
    pub thresholds: Vec<f64>,
}

impl<R: Clone> ScaleQuantize<R> {
    pub fn new(domain: [f64; 2], range: Vec<R>) -> Self {
        let mut scale = Self {
            domain,
            range,
            thresholds: vec![],
        };
        
        scale.rescale();
        scale
    }
    
    fn rescale(&mut self) {
        let n = self.range.len();
        if n == 0 {
            self.thresholds.clear();
            return;
        }
        
        self.thresholds.clear();
        
        // Calculate uniform thresholds
        let start = self.domain[0];
        let end = self.domain[1];
        let step = (end - start) / n as f64;
        
        for i in 1..n {
            self.thresholds.push(start + step * i as f64);
        }
    }
    
    pub fn scale(&self, x: f64) -> Option<R> {
        if x.is_nan() {
            return None;
        }
        
        // Clamp to domain
        let x = x.max(self.domain[0]).min(self.domain[1]);
        
        // Find the appropriate range bucket
        let mut i = 0;
        while i < self.thresholds.len() && x >= self.thresholds[i] {
            i += 1;
        }
        
        self.range.get(i).cloned()
    }
    
    pub fn invertExtent(&self, y: &R) -> Option<[f64; 2]> 
    where 
        R: PartialEq,
    {
        if let Some(i) = self.range.iter().position(|r| r == y) {
            let min = if i == 0 { 
                self.domain[0] 
            } else { 
                self.thresholds.get(i - 1).copied().unwrap_or(self.domain[0]) 
            };
            
            let max = if i == self.range.len() - 1 {
                self.domain[1]
            } else {
                self.thresholds.get(i).copied().unwrap_or(self.domain[1])
            };
            
            Some([min, max])
        } else {
            None
        }
    }
    
    pub fn domain(&self) -> [f64; 2] {
        self.domain
    }
    
    pub fn range(&self) -> &Vec<R> {
        &self.range
    }
    
    pub fn thresholds(&self) -> Vec<f64> {
        let mut result = vec![self.domain[0]];
        result.extend(self.thresholds.iter().cloned());
        result.push(self.domain[1]);
        result
    }
    
    pub fn copy(&self) -> Self {
        Self {
            domain: self.domain,
            range: self.range.clone(),
            thresholds: self.thresholds.clone(),
        }
    }
    
    pub fn nice(&mut self, count: Option<usize>) {
        let count = count.unwrap_or(10);
        let range_val = self.domain[1] - self.domain[0];
        let step = 10f64.powf((range_val / count as f64).log10().floor());
        
        self.domain[0] = (self.domain[0] / step).floor() * step;
        self.domain[1] = (self.domain[1] / step).ceil() * step;
        
        self.rescale();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_quantize_scale() {
        let scale = ScaleQuantize::new([0.0, 1.0], vec!["a", "b", "c"]);
        
        assert_eq!(scale.scale(0.0), Some("a"));
        assert_eq!(scale.scale(0.33), Some("a"));  // 0.33 < 0.333...
        assert_eq!(scale.scale(0.34), Some("b"));  // 0.34 > 0.333...
        assert_eq!(scale.scale(0.67), Some("c"));  // 0.67 > 0.666...
        assert_eq!(scale.scale(0.66), Some("b"));  // 0.66 < 0.666...
        assert_eq!(scale.scale(1.0), Some("c"));
    }
    
    #[test]
    fn test_quantize_scale_clamp() {
        let scale = ScaleQuantize::new([0.0, 1.0], vec!["a", "b"]);
        
        assert_eq!(scale.scale(-0.5), Some("a")); // Clamped to domain[0]
        assert_eq!(scale.scale(1.5), Some("b"));  // Clamped to domain[1]
    }
    
    #[test]
    fn test_quantize_scale_nan() {
        let scale = ScaleQuantize::new([0.0, 1.0], vec!["a", "b"]);
        
        assert_eq!(scale.scale(f64::NAN), None);
    }
    
    #[test]
    fn test_quantize_invert_extent() {
        let scale = ScaleQuantize::new([0.0, 1.0], vec!["a", "b", "c"]);
        
        assert_eq!(scale.invertExtent(&"a"), Some([0.0, 1.0/3.0]));
        assert_eq!(scale.invertExtent(&"b"), Some([1.0/3.0, 2.0/3.0]));
        assert_eq!(scale.invertExtent(&"c"), Some([2.0/3.0, 1.0]));
        assert_eq!(scale.invertExtent(&"d"), None);
    }
    
    #[test]
    fn test_quantize_thresholds() {
        let scale = ScaleQuantize::new([0.0, 1.0], vec!["a", "b", "c"]);
        let thresholds = scale.thresholds();
        
        assert_eq!(thresholds.len(), 4); // domain[0] + 2 internal + domain[1] = 4
        assert_eq!(thresholds[0], 0.0);
        assert_eq!(thresholds[3], 1.0);
    }
    
    #[test]
    fn test_quantize_nice() {
        let mut scale = ScaleQuantize::new([0.12, 0.87], vec!["a", "b"]);
        let original_domain = scale.domain;
        scale.nice(Some(10));
        
        // The nice function should round to nice boundaries
        assert!(scale.domain[0] <= original_domain[0]);
        assert!(scale.domain[1] >= original_domain[1]);
    }
    
    #[test]
    fn test_quantize_empty_range() {
        let scale = ScaleQuantize::new([0.0, 1.0], Vec::<&str>::new());
        
        assert_eq!(scale.scale(0.5), None);
    }
}

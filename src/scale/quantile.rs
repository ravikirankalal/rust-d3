// d3-scale: ScaleQuantile
// Maps a continuous domain to a discrete range based on quantiles
use crate::array::quantile::quantile;

#[derive(Debug, Clone)]
pub struct ScaleQuantile<R: Clone> {
    pub domain: Vec<f64>,
    pub range: Vec<R>,
    thresholds: Vec<f64>,
}

impl<R: Clone> ScaleQuantile<R> {
    pub fn new(domain: Vec<f64>, range: Vec<R>) -> Self {
        let mut sorted_domain = domain.clone();
        sorted_domain.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        
        let mut scale = Self {
            domain: sorted_domain,
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
        
        // Calculate quantile thresholds
        for i in 1..n {
            let p = i as f64 / n as f64;
            if let Some(threshold) = quantile(&self.domain, p) {
                self.thresholds.push(threshold);
            }
        }
    }
    
    pub fn scale(&self, x: f64) -> Option<R> {
        if x.is_nan() {
            return None;
        }
        
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
                self.domain.first().copied().unwrap_or(f64::NEG_INFINITY) 
            } else { 
                self.thresholds.get(i - 1).copied().unwrap_or(f64::NEG_INFINITY) 
            };
            
            let max = if i == self.range.len() - 1 {
                self.domain.last().copied().unwrap_or(f64::INFINITY)
            } else {
                self.thresholds.get(i).copied().unwrap_or(f64::INFINITY)
            };
            
            Some([min, max])
        } else {
            None
        }
    }
    
    pub fn domain(&self) -> &Vec<f64> {
        &self.domain
    }
    
    pub fn range(&self) -> &Vec<R> {
        &self.range
    }
    
    pub fn quantiles(&self) -> &Vec<f64> {
        &self.thresholds
    }
    
    pub fn copy(&self) -> Self {
        Self {
            domain: self.domain.clone(),
            range: self.range.clone(),
            thresholds: self.thresholds.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_quantile_scale() {
        let domain = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let range = vec!["low", "medium", "high"];
        let scale = ScaleQuantile::new(domain, range);
        
        assert_eq!(scale.scale(0.0), Some("low"));
        assert_eq!(scale.scale(3.0), Some("medium"));
        assert_eq!(scale.scale(7.0), Some("high"));
        assert_eq!(scale.scale(9.0), Some("high"));
    }
    
    #[test]
    fn test_quantile_scale_nan() {
        let domain = vec![0.0, 1.0, 2.0, 3.0, 4.0];
        let range = vec!["a", "b"];
        let scale = ScaleQuantile::new(domain, range);
        
        assert_eq!(scale.scale(f64::NAN), None);
    }
    
    #[test]
    fn test_quantile_invert_extent() {
        let domain = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let range = vec!["low", "medium", "high"];
        let scale = ScaleQuantile::new(domain, range);
        
        if let Some(extent) = scale.invertExtent(&"low") {
            assert_eq!(extent[0], 0.0);
            assert!(extent[1] > 0.0 && extent[1] < 9.0);
        }
        
        if let Some(extent) = scale.invertExtent(&"high") {
            assert!(extent[0] > 0.0 && extent[0] < 9.0);
            assert_eq!(extent[1], 9.0);
        }
    }
    
    #[test]
    fn test_quantile_empty_domain() {
        let domain = vec![];
        let range = vec!["a", "b"];
        let scale = ScaleQuantile::new(domain, range);
        
        assert_eq!(scale.scale(0.0), Some("a"));
    }
    
    #[test]
    fn test_quantile_empty_range() {
        let domain = vec![0.0, 1.0, 2.0];
        let range: Vec<&str> = vec![];
        let scale = ScaleQuantile::new(domain, range);
        
        assert_eq!(scale.scale(1.0), None);
    }
}

// d3-scale: ScaleThreshold
// Maps a continuous domain to a discrete range using custom thresholds
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct ScaleThreshold<R: Clone> {
    pub domain: Vec<f64>,
    pub range: Vec<R>,
}

impl<R: Clone> ScaleThreshold<R> {
    pub fn new(domain: Vec<f64>, range: Vec<R>) -> Self {
        // Ensure domain is sorted
        let mut sorted_domain = domain;
        sorted_domain.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
        
        Self {
            domain: sorted_domain,
            range,
        }
    }
    
    pub fn scale(&self, x: f64) -> Option<R> {
        if x.is_nan() {
            return None;
        }
        
        // Find the appropriate range bucket
        let mut i = 0;
        while i < self.domain.len() && x >= self.domain[i] {
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
                f64::NEG_INFINITY 
            } else { 
                self.domain.get(i - 1).copied().unwrap_or(f64::NEG_INFINITY) 
            };
            
            let max = if i >= self.domain.len() {
                f64::INFINITY
            } else {
                self.domain.get(i).copied().unwrap_or(f64::INFINITY)
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
    
    pub fn copy(&self) -> Self {
        Self {
            domain: self.domain.clone(),
            range: self.range.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_threshold_scale() {
        let scale = ScaleThreshold::new(vec![0.0, 1.0], vec!["red", "white", "blue"]);
        
        assert_eq!(scale.scale(-1.0), Some("red"));
        assert_eq!(scale.scale(0.0), Some("white"));
        assert_eq!(scale.scale(0.5), Some("white"));
        assert_eq!(scale.scale(1.0), Some("blue"));
        assert_eq!(scale.scale(2.0), Some("blue"));
    }
    
    #[test]
    fn test_threshold_scale_nan() {
        let scale = ScaleThreshold::new(vec![0.0, 1.0], vec!["a", "b", "c"]);
        
        assert_eq!(scale.scale(f64::NAN), None);
    }
    
    #[test]
    fn test_threshold_invert_extent() {
        let scale = ScaleThreshold::new(vec![0.0, 1.0], vec!["red", "white", "blue"]);
        
        assert_eq!(scale.invertExtent(&"red"), Some([f64::NEG_INFINITY, 0.0]));
        assert_eq!(scale.invertExtent(&"white"), Some([0.0, 1.0]));
        assert_eq!(scale.invertExtent(&"blue"), Some([1.0, f64::INFINITY]));
        assert_eq!(scale.invertExtent(&"green"), None);
    }
    
    #[test]
    fn test_threshold_empty_domain() {
        let scale = ScaleThreshold::new(vec![], vec!["only"]);
        
        assert_eq!(scale.scale(0.0), Some("only"));
        assert_eq!(scale.scale(1.0), Some("only"));
    }
    
    #[test]
    fn test_threshold_single_threshold() {
        let scale = ScaleThreshold::new(vec![0.5], vec!["low", "high"]);
        
        assert_eq!(scale.scale(0.0), Some("low"));
        assert_eq!(scale.scale(0.5), Some("high"));
        assert_eq!(scale.scale(1.0), Some("high"));
    }
    
    #[test]
    fn test_threshold_sorting() {
        // Domain should be sorted internally
        let scale = ScaleThreshold::new(vec![1.0, 0.0, 2.0], vec!["a", "b", "c", "d"]);
        
        assert_eq!(scale.domain(), &vec![0.0, 1.0, 2.0]);
        assert_eq!(scale.scale(-1.0), Some("a")); // Below 0.0
        assert_eq!(scale.scale(0.0), Some("b"));   // At 0.0
        assert_eq!(scale.scale(1.0), Some("c"));   // At 1.0
        assert_eq!(scale.scale(2.0), Some("d"));   // At 2.0
    }
}

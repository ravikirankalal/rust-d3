// d3-scale: ScaleIdentity
// An identity scale returns the same value for input and output
#[derive(Debug, Clone)]
pub struct ScaleIdentity {
    pub domain: [f64; 2],
    pub range: [f64; 2],
}

impl ScaleIdentity {
    pub fn new(domain: [f64; 2], range: [f64; 2]) -> Self {
        Self { domain, range }
    }
    
    pub fn scale(&self, x: f64) -> f64 {
        // Identity scale ignores domain/range and returns input
        x
    }
    
    pub fn invert(&self, y: f64) -> f64 {
        // Identity scale invert also returns input
        y
    }
    
    pub fn domain(&self) -> [f64; 2] {
        self.domain
    }
    
    pub fn range(&self) -> [f64; 2] {
        self.range
    }
    
    pub fn ticks(&self, count: usize) -> Vec<f64> {
        // Generate ticks between domain bounds
        if count == 0 {
            return vec![];
        }
        
        let start = self.domain[0];
        let end = self.domain[1];
        
        if start == end {
            return vec![start];
        }
        
        let step = (end - start) / (count - 1) as f64;
        (0..count).map(|i| start + step * i as f64).collect()
    }
    
    pub fn tick_format(&self, _count: usize) -> impl Fn(f64) -> String {
        |x| x.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_identity_scale() {
        let scale = ScaleIdentity::new([0.0, 100.0], [0.0, 100.0]);
        
        assert_eq!(scale.scale(42.0), 42.0);
        assert_eq!(scale.scale(-10.0), -10.0);
        assert_eq!(scale.scale(0.0), 0.0);
    }
    
    #[test]
    fn test_identity_invert() {
        let scale = ScaleIdentity::new([0.0, 100.0], [0.0, 100.0]);
        
        assert_eq!(scale.invert(42.0), 42.0);
        assert_eq!(scale.invert(-10.0), -10.0);
        assert_eq!(scale.invert(0.0), 0.0);
    }
    
    #[test]
    fn test_identity_ticks() {
        let scale = ScaleIdentity::new([0.0, 100.0], [0.0, 100.0]);
        let ticks = scale.ticks(5);
        
        assert_eq!(ticks.len(), 5);
        assert_eq!(ticks[0], 0.0);
        assert_eq!(ticks[4], 100.0);
    }
}

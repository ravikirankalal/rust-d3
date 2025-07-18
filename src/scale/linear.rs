// d3-scale: ScaleLinear
use crate::array::{tick_step::tick_step, ticks::ticks};
use crate::format::format::format;

#[derive(Debug, Clone)]
pub struct ScaleLinear {
    pub domain: [f64; 2],
    pub range: [f64; 2],
    pub clamp: bool,
    pub interpolate: fn(f64, f64, f64) -> f64,
    pub unknown: Option<f64>,
}

impl ScaleLinear {
    pub fn new(domain: [f64; 2], range: [f64; 2]) -> Self {
        Self {
            domain,
            range,
            clamp: false,
            interpolate: linear_interpolate,
            unknown: None,
        }
    }

    pub fn scale(&self, x: f64) -> f64 {
        if x.is_nan() {
            return self.unknown.unwrap_or(f64::NAN);
        }

        let mut x = x;
        if self.clamp {
            x = x.max(self.domain[0]).min(self.domain[1]);
        }

        let t = (x - self.domain[0]) / (self.domain[1] - self.domain[0]);
        (self.interpolate)(self.range[0], self.range[1], t)
    }

    pub fn invert(&self, y: f64) -> f64 {
        if y.is_nan() {
            return f64::NAN;
        }

        let t = (y - self.range[0]) / (self.range[1] - self.range[0]);
        let result = self.domain[0] + t * (self.domain[1] - self.domain[0]);

        if self.clamp {
            result.max(self.domain[0]).min(self.domain[1])
        } else {
            result
        }
    }

    pub fn domain(&self) -> [f64; 2] {
        self.domain
    }

    pub fn range(&self) -> [f64; 2] {
        self.range
    }

    pub fn clamp(mut self, clamp: bool) -> Self {
        self.clamp = clamp;
        self
    }

    pub fn interpolate(mut self, interpolate: fn(f64, f64, f64) -> f64) -> Self {
        self.interpolate = interpolate;
        self
    }

    pub fn unknown(mut self, value: f64) -> Self {
        self.unknown = Some(value);
        self
    }

    pub fn ticks(&self, count: usize) -> Vec<f64> {
        let domain = self.domain;
        
        // If domain[0] == domain[1], return [domain[0]]
        if domain[0] == domain[1] {
            return vec![domain[0]];
        }
        
        // Handle NaN values in domain
        if domain[0].is_nan() || domain[1].is_nan() {
            return vec![];
        }
        
        let result = ticks(domain[0], domain[1], count);
        
        // Guarantee at least one value if count > 0
        if result.is_empty() && count > 0 {
            vec![domain[0]]
        } else {
            result
        }
    }

    pub fn tick_format(&self, count: usize, specifier: Option<&str>) -> impl Fn(f64) -> String {
        let spec = specifier.unwrap_or("");
        let step = tick_step(self.domain[0], self.domain[1], count);
        let precision = if spec.is_empty() {
            Self::default_precision(step)
        } else {
            0
        };

        let spec = if spec.is_empty() {
            if precision > 0 {
                format!(".{}f", precision)
            } else {
                "g".to_string()
            }
        } else {
            spec.to_string()
        };

        move |x| format(&spec, x)
    }

    fn default_precision(step: f64) -> usize {
        let abs_step = step.abs();
        if abs_step >= 1.0 {
            0
        } else {
            (-abs_step.log10().floor() as i32).max(0) as usize
        }
    }

    pub fn nice(&mut self, count: Option<usize>) {
        let count = count.unwrap_or(10);
        let step = tick_step(self.domain[0], self.domain[1], count);

        if step.is_finite() && step > 0.0 {
            self.domain[0] = (self.domain[0] / step).floor() * step;
            self.domain[1] = (self.domain[1] / step).ceil() * step;
        }
    }

    pub fn copy(&self) -> Self {
        Self {
            domain: self.domain,
            range: self.range,
            clamp: self.clamp,
            interpolate: self.interpolate,
            unknown: self.unknown,
        }
    }

    pub fn range_round(mut self, range: [f64; 2]) -> Self {
        self.range = range;
        self.interpolate = round_interpolate;
        self
    }
}

// Default linear interpolation
fn linear_interpolate(a: f64, b: f64, t: f64) -> f64 {
    a + t * (b - a)
}

// Rounding interpolation
fn round_interpolate(a: f64, b: f64, t: f64) -> f64 {
    (a + t * (b - a)).round()
}

impl Default for ScaleLinear {
    fn default() -> Self {
        Self::new([0.0, 1.0], [0.0, 1.0])
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
    
    #[test]
    fn test_ticks_regular_domain() {
        let s = ScaleLinear::new([0.0, 10.0], [0.0, 100.0]);
        let ticks = s.ticks(5);
        
        // Should have nice, round numbers
        assert!(ticks.len() > 0);
        assert!(ticks[0] >= 0.0);
        assert!(ticks[ticks.len() - 1] <= 10.0);
        
        // Should be deterministic
        let ticks2 = s.ticks(5);
        assert_eq!(ticks, ticks2);
    }
    
    #[test]
    fn test_ticks_reversed_domain() {
        let s = ScaleLinear::new([10.0, 0.0], [0.0, 100.0]);
        let ticks = s.ticks(5);
        
        // Should handle reversed domain
        assert!(ticks.len() > 0);
        assert!(ticks[0] >= 0.0);
        assert!(ticks[ticks.len() - 1] <= 10.0);
        
        // Should be deterministic
        let ticks2 = s.ticks(5);
        assert_eq!(ticks, ticks2);
    }
    
    #[test]
    fn test_ticks_zero_span_domain() {
        let s = ScaleLinear::new([5.0, 5.0], [0.0, 100.0]);
        let ticks = s.ticks(10);
        
        // Should return single value for zero-span domain
        assert_eq!(ticks, vec![5.0]);
    }
    
    #[test]
    fn test_ticks_zero_span_domain_at_zero() {
        let s = ScaleLinear::new([0.0, 0.0], [0.0, 100.0]);
        let ticks = s.ticks(10);
        
        // Should return single value for zero-span domain
        assert_eq!(ticks, vec![0.0]);
    }
    
    #[test]
    fn test_ticks_zero_span_domain_negative() {
        let s = ScaleLinear::new([-3.0, -3.0], [0.0, 100.0]);
        let ticks = s.ticks(10);
        
        // Should return single value for zero-span domain
        assert_eq!(ticks, vec![-3.0]);
    }
    
    #[test]
    fn test_ticks_guarantee_at_least_one_value() {
        let s = ScaleLinear::new([0.0, 1.0], [0.0, 100.0]);
        let ticks = s.ticks(1);
        
        // Should guarantee at least one value
        assert!(ticks.len() >= 1);
    }
    
    #[test]
    fn test_ticks_count_zero() {
        let s = ScaleLinear::new([0.0, 10.0], [0.0, 100.0]);
        let ticks = s.ticks(0);
        
        // Should return empty vector for count = 0
        assert_eq!(ticks, Vec::<f64>::new());
    }
    
    #[test]
    fn test_ticks_nan_domain() {
        let s = ScaleLinear::new([f64::NAN, 10.0], [0.0, 100.0]);
        let ticks = s.ticks(10);
        
        // Should return empty vector for NaN in domain
        assert_eq!(ticks, Vec::<f64>::new());
        
        let s2 = ScaleLinear::new([0.0, f64::NAN], [0.0, 100.0]);
        let ticks2 = s2.ticks(10);
        
        // Should return empty vector for NaN in domain
        assert_eq!(ticks2, Vec::<f64>::new());
    }
    
    #[test]
    fn test_ticks_small_domain() {
        let s = ScaleLinear::new([0.0, 0.1], [0.0, 100.0]);
        let ticks = s.ticks(5);
        
        // Should handle small domains
        assert!(ticks.len() > 0);
        assert!(ticks[0] >= 0.0);
        assert!(ticks[ticks.len() - 1] <= 0.1);
    }
    
    #[test]
    fn test_ticks_large_domain() {
        let s = ScaleLinear::new([0.0, 1000.0], [0.0, 100.0]);
        let ticks = s.ticks(5);
        
        // Should handle large domains
        assert!(ticks.len() > 0);
        assert!(ticks[0] >= 0.0);
        assert!(ticks[ticks.len() - 1] <= 1000.0);
    }
    
    #[test]
    fn test_ticks_negative_domain() {
        let s = ScaleLinear::new([-10.0, -1.0], [0.0, 100.0]);
        let ticks = s.ticks(5);
        
        // Should handle negative domains
        assert!(ticks.len() > 0);
        assert!(ticks[0] >= -10.0);
        assert!(ticks[ticks.len() - 1] <= -1.0);
    }
    
    #[test]
    fn test_ticks_mixed_domain() {
        let s = ScaleLinear::new([-5.0, 5.0], [0.0, 100.0]);
        let ticks = s.ticks(10);
        
        // Should handle mixed positive/negative domains
        assert!(ticks.len() > 0);
        assert!(ticks[0] >= -5.0);
        assert!(ticks[ticks.len() - 1] <= 5.0);
    }
    
    #[test]
    fn test_ticks_deterministic_behavior() {
        let s = ScaleLinear::new([0.0, 100.0], [0.0, 1000.0]);
        
        // Multiple calls should return identical results
        let ticks1 = s.ticks(10);
        let ticks2 = s.ticks(10);
        let ticks3 = s.ticks(10);
        
        assert_eq!(ticks1, ticks2);
        assert_eq!(ticks2, ticks3);
    }
}

// d3-scale: ScaleLog
#[derive(Debug, Clone)]
pub struct ScaleLog {
    pub domain: [f64; 2],
    pub range: [f64; 2],
    pub base: f64,
}

impl ScaleLog {
    pub fn new(domain: [f64; 2], range: [f64; 2], base: f64) -> Self {
        Self {
            domain,
            range,
            base,
        }
    }
    pub fn scale(&self, x: f64) -> f64 {
        let t = (x.ln() - self.domain[0].ln()) / (self.domain[1].ln() - self.domain[0].ln());
        self.range[0] + t * (self.range[1] - self.range[0])
    }
    pub fn invert(&self, y: f64) -> f64 {
        let t = (y - self.range[0]) / (self.range[1] - self.range[0]);
        (self.domain[0].ln() + t * (self.domain[1].ln() - self.domain[0].ln())).exp()
    }
    pub fn ticks(&self, count: usize) -> Vec<f64> {
        // Generate log-spaced ticks between domain[0] and domain[1]
        let mut ticks = Vec::new();
        
        // Use the specified base for logarithm calculations
        let start = self.domain[0].ln() / self.base.ln();
        let end = self.domain[1].ln() / self.base.ln();
        
        // For logarithmic scales, we want ticks at nice powers of the base
        let start_power = start.floor() as i32;
        let end_power = end.ceil() as i32;
        
        // Generate ticks at powers of the base
        for power in start_power..=end_power {
            let tick_value = self.base.powi(power);
            if tick_value >= self.domain[0] && tick_value <= self.domain[1] {
                ticks.push(tick_value);
            }
        }
        
        // If we don't have enough ticks, add intermediate values
        if ticks.len() < count {
            let mut additional_ticks = Vec::new();
            
            for power in start_power..end_power {
                let base_tick = self.base.powi(power);
                
                // Add intermediate ticks (2x, 3x, 4x, etc. of base powers)
                for multiplier in 2..=9 {
                    let tick_value = base_tick * multiplier as f64;
                    if tick_value >= self.domain[0] && tick_value <= self.domain[1] {
                        additional_ticks.push(tick_value);
                    }
                }
            }
            
            ticks.extend(additional_ticks);
            ticks.sort_by(|a, b| a.partial_cmp(b).unwrap());
        }
        
        // Always include domain boundaries
        if !ticks.is_empty() {
            if ticks[0] > self.domain[0] {
                ticks.insert(0, self.domain[0]);
            }
            if ticks.last().unwrap() < &self.domain[1] {
                ticks.push(self.domain[1]);
            }
        }
        
        ticks
    }
}

//! Scale utilities for mapping data values to visual coordinates

/// Linear scale for mapping continuous domains to ranges
#[derive(Debug, Clone)]
pub struct LinearScale {
    domain_min: f64,
    domain_max: f64,
    range_min: f64,
    range_max: f64,
}

impl LinearScale {
    /// Create a new linear scale
    pub fn new() -> Self {
        Self {
            domain_min: 0.0,
            domain_max: 1.0,
            range_min: 0.0,
            range_max: 1.0,
        }
    }

    /// Set the domain (input range)
    pub fn domain(mut self, min: f64, max: f64) -> Self {
        self.domain_min = min;
        self.domain_max = max;
        self
    }

    /// Set the range (output range)
    pub fn range(mut self, min: f64, max: f64) -> Self {
        self.range_min = min;
        self.range_max = max;
        self
    }

    /// Scale a value from domain to range
    pub fn scale(&self, value: f64) -> f64 {
        if self.domain_max == self.domain_min {
            return self.range_min;
        }

        let normalized = (value - self.domain_min) / (self.domain_max - self.domain_min);
        self.range_min + normalized * (self.range_max - self.range_min)
    }

    /// Get nice tick values for the domain
    pub fn ticks(&self, count: usize) -> Vec<f64> {
        if count == 0 {
            return vec![];
        }

        let step = (self.domain_max - self.domain_min) / (count - 1) as f64;
        (0..count)
            .map(|i| self.domain_min + i as f64 * step)
            .collect()
    }
}

impl Default for LinearScale {
    fn default() -> Self {
        Self::new()
    }
}

/// Ordinal scale for mapping discrete domains to ranges
#[derive(Debug, Clone)]
pub struct OrdinalScale {
    domain: Vec<String>,
    range: Vec<String>,
}

impl OrdinalScale {
    /// Create a new ordinal scale
    pub fn new() -> Self {
        Self {
            domain: Vec::new(),
            range: Vec::new(),
        }
    }

    /// Set the domain (input values)
    pub fn domain(mut self, domain: Vec<String>) -> Self {
        self.domain = domain;
        self
    }

    /// Set the range (output values)
    pub fn range(mut self, range: Vec<String>) -> Self {
        self.range = range;
        self
    }

    /// Scale a value from domain to range
    pub fn scale(&self, value: &str) -> Option<&String> {
        if let Some(index) = self.domain.iter().position(|x| x == value) {
            self.range.get(index % self.range.len())
        } else {
            None
        }
    }
}

impl Default for OrdinalScale {
    fn default() -> Self {
        Self::new()
    }
}

/// Band scale for positioning bars in bar charts
#[derive(Debug, Clone)]
pub struct BandScale {
    domain: Vec<String>,
    range_start: f64,
    range_end: f64,
    padding: f64,
}

impl BandScale {
    /// Create a new band scale
    pub fn new() -> Self {
        Self {
            domain: Vec::new(),
            range_start: 0.0,
            range_end: 1.0,
            padding: 0.1,
        }
    }

    /// Set the domain
    pub fn domain(mut self, domain: Vec<String>) -> Self {
        self.domain = domain;
        self
    }

    /// Set the range
    pub fn range(mut self, start: f64, end: f64) -> Self {
        self.range_start = start;
        self.range_end = end;
        self
    }

    /// Set padding between bands
    pub fn padding(mut self, padding: f64) -> Self {
        self.padding = padding.clamp(0.0, 1.0);
        self
    }

    /// Get the position of a band
    pub fn scale(&self, value: &str) -> Option<f64> {
        if let Some(index) = self.domain.iter().position(|x| x == value) {
            let _bandwidth = self.bandwidth();
            let step = (self.range_end - self.range_start) / self.domain.len() as f64;
            Some(self.range_start + index as f64 * step + step * self.padding / 2.0)
        } else {
            None
        }
    }

    /// Get the bandwidth of each band
    pub fn bandwidth(&self) -> f64 {
        if self.domain.is_empty() {
            return 0.0;
        }
        let step = (self.range_end - self.range_start) / self.domain.len() as f64;
        step * (1.0 - self.padding)
    }
}

impl Default for BandScale {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_scale() {
        let scale = LinearScale::new()
            .domain(0.0, 100.0)
            .range(0.0, 500.0);

        assert_eq!(scale.scale(0.0), 0.0);
        assert_eq!(scale.scale(50.0), 250.0);
        assert_eq!(scale.scale(100.0), 500.0);
    }

    #[test]
    fn test_linear_scale_ticks() {
        let scale = LinearScale::new().domain(0.0, 10.0);
        let ticks = scale.ticks(5);
        assert_eq!(ticks, vec![0.0, 2.5, 5.0, 7.5, 10.0]);
    }

    #[test]
    fn test_ordinal_scale() {
        let scale = OrdinalScale::new()
            .domain(vec!["A".to_string(), "B".to_string(), "C".to_string()])
            .range(vec!["red".to_string(), "green".to_string(), "blue".to_string()]);

        assert_eq!(scale.scale("A"), Some(&"red".to_string()));
        assert_eq!(scale.scale("B"), Some(&"green".to_string()));
        assert_eq!(scale.scale("C"), Some(&"blue".to_string()));
        assert_eq!(scale.scale("D"), None);
    }

    #[test]
    fn test_band_scale() {
        let scale = BandScale::new()
            .domain(vec!["A".to_string(), "B".to_string(), "C".to_string()])
            .range(0.0, 300.0)
            .padding(0.2);

        assert!(scale.scale("A").is_some());
        assert!(scale.scale("B").is_some());
        assert!(scale.scale("C").is_some());
        assert!(scale.bandwidth() > 0.0);
    }
}
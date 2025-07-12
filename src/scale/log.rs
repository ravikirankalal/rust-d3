// d3-scale: ScaleLog
#[derive(Debug, Clone)]
pub struct ScaleLog {
    pub domain: [f64; 2],
    pub range: [f64; 2],
    pub base: f64,
}

impl ScaleLog {
    pub fn new(domain: [f64; 2], range: [f64; 2], base: f64) -> Self {
        Self { domain, range, base }
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
        let start = self.domain[0].log10();
        let end = self.domain[1].log10();
        let step = (end - start) / (count as f64 - 1.0);
        for i in 0..count {
            let v = 10f64.powf(start + step * i as f64);
            ticks.push(v);
        }
        ticks
    }
}

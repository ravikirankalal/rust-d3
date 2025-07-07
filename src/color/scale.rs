// ColorScale implementation

pub struct ColorScale {
    domain: (f64, f64),
    colors: Vec<String>,
}

impl ColorScale {
    pub fn new(domain: (f64, f64), colors: Vec<String>) -> Self {
        Self { domain, colors }
    }

    pub fn scale(&self, value: f64) -> &str {
        let (d0, d1) = self.domain;
        let n = self.colors.len();
        if n == 0 || d1 == d0 {
            return "";
        }
        let t = ((value - d0) / (d1 - d0)).clamp(0.0, 1.0);
        let idx = (t * (n as f64 - 1.0)).floor() as usize;
        &self.colors[idx]
    }
}

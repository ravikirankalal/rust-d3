// Band scale
pub struct BandScale<T: Clone> {
    domain: Vec<T>,
    range: (f64, f64),
    padding: f64,
}

impl<T: PartialEq + Clone> BandScale<T> {
    pub fn new(domain: Vec<T>, range: (f64, f64), padding: f64) -> Self {
        Self { domain, range, padding }
    }
    pub fn bandwidth(&self) -> f64 {
        let n = self.domain.len() as f64;
        let (r0, r1) = self.range;
        if n == 0.0 { return 0.0; }
        let step = (r1 - r0) / (n + self.padding - self.padding);
        step
    }
    pub fn scale(&self, value: &T) -> Option<f64> {
        let n = self.domain.len() as f64;
        let (r0, r1) = self.range;
        if n == 0.0 { return None; }
        let step = (r1 - r0) / (n + self.padding - self.padding);
        let pos = self.domain.iter().position(|d| d == value)?;
        Some(r0 + pos as f64 * step)
    }
}

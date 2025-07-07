// Axis implementation

use crate::scale::LinearScale;

pub struct Axis {
    pub ticks: usize,
}

impl Axis {
    pub fn new(ticks: usize) -> Self {
        Self { ticks }
    }

    pub fn generate(&self, scale: &LinearScale) -> Vec<(f64, f64)> {
        let (d0, d1) = scale.domain();
        let step = (d1 - d0) / (self.ticks as f64 - 1.0);
        (0..self.ticks)
            .map(|i| {
                let v = d0 + i as f64 * step;
                (v, scale.scale(v))
            })
            .collect()
    }
}

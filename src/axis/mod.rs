// d3-axis parity root
// Implements Axis, AxisOrientation, and tick generation for D3-like axes

pub mod orientation;
pub mod ticks;

pub use orientation::AxisOrientation;
pub use ticks::{Tick, TickFormat};

pub struct Axis<S> {
    pub scale: S,
    pub orientation: AxisOrientation,
    pub tick_count: usize,
    pub tick_format: Option<TickFormat>,
}

impl<S> Axis<S> {
    pub fn new(scale: S, orientation: AxisOrientation) -> Self {
        Self {
            scale,
            orientation,
            tick_count: 10,
            tick_format: None,
        }
    }
    pub fn tick_count(mut self, count: usize) -> Self {
        self.tick_count = count;
        self
    }
    pub fn tick_format(mut self, format: TickFormat) -> Self {
        self.tick_format = Some(format);
        self
    }
}

impl Axis<crate::scale::ScaleLinear> {
    pub fn ticks(&self) -> Vec<Tick> {
        let domain = self.scale.domain;
        let tick_count = self.tick_count;
        let step = (domain[1] - domain[0]) / (tick_count as f64 - 1.0);
        (0..tick_count)
            .map(|i| {
                let value = domain[0] + i as f64 * step;
                let position = self.scale.scale(value);
                let label = if let Some(fmt) = self.tick_format {
                    (fmt)(value)
                } else {
                    format!("{:.6}", value)
                };
                Tick::new(value, label, position)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scale::ScaleLinear;
    #[test]
    fn test_linear_axis_ticks() {
        let scale = ScaleLinear::new([0.0, 10.0], [0.0, 100.0]);
        let axis = Axis::new(scale, AxisOrientation::Bottom).tick_count(5);
        let ticks = axis.ticks();
        assert_eq!(ticks.len(), 5);
        assert!((ticks[0].value - 0.0).abs() < 1e-6);
        assert!((ticks[4].value - 10.0).abs() < 1e-6);
        assert!((ticks[2].position - 50.0).abs() < 1e-6);
        assert_eq!(ticks[0].label, "0.000000");
    }
}

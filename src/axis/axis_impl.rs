// Axis methods implementation
// Implements Axis<S> methods for D3-like axes

use super::axis_structs::{Axis, AxisLayout};
use super::orientation::AxisOrientation;
use super::ticks::{Tick, TickFormat};

impl<S> Axis<S> {
    pub fn new(scale: S, orientation: AxisOrientation) -> Self {
        Self {
            scale,
            orientation,
            tick_count: 10,
            tick_format: None,
            tick_values: None,
            tick_size_inner: 6.0,
            tick_size_outer: 6.0,
            tick_padding: 3.0,
            tick_arguments: None,
            offset: 0.0,
            locale: None,
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
    pub fn tick_values(mut self, values: Vec<f64>) -> Self {
        self.tick_values = Some(values);
        self
    }
    pub fn tick_size_inner(mut self, size: f64) -> Self {
        self.tick_size_inner = size;
        self
    }
    pub fn tick_size_outer(mut self, size: f64) -> Self {
        self.tick_size_outer = size;
        self
    }
    pub fn tick_padding(mut self, padding: f64) -> Self {
        self.tick_padding = padding;
        self
    }
    pub fn tick_arguments(mut self, args: Vec<f64>) -> Self {
        self.tick_arguments = Some(args);
        self
    }
    pub fn offset(mut self, offset: f64) -> Self {
        self.offset = offset;
        self
    }
    pub fn locale(mut self, locale: &str) -> Self {
        self.locale = Some(locale.to_string());
        self
    }
    pub fn layout(&self, axis_start: f64, axis_end: f64, ticks: Vec<Tick>) -> AxisLayout {
        AxisLayout {
            orientation: self.orientation,
            ticks,
            tick_size_inner: self.tick_size_inner,
            tick_size_outer: self.tick_size_outer,
            tick_padding: self.tick_padding,
            axis_start,
            axis_end,
            offset: self.offset,
        }
    }
}

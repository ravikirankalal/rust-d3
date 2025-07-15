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
            offset: 0.5,
            locale: None,
            grid: false,
            grid_style: None,
            title: None,
            title_style: None,
            minor_ticks: None,
            minor_tick_size: None,
            tick_label_angle: None,
            tick_label_style: None,
            axis_line_style: None,
            on_render: None,
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
    pub fn tick_size(mut self, size: f64) -> Self {
        self.tick_size_inner = size;
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
    
    /// Convenience method for setting tick count in a D3-like way
    /// This is equivalent to tick_count but matches D3's axis.ticks() signature for configuration
    pub fn with_ticks(mut self, count: usize) -> Self {
        self.tick_count = count;
        self
    }
    
    /// Convenience method for setting tick arguments with count and format specifier
    /// This matches D3's axis.ticks(count, specifier) signature
    pub fn ticks_with_format(mut self, count: usize, specifier: f64) -> Self {
        self.tick_count = count;
        self.tick_arguments = Some(vec![count as f64, specifier]);
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
    
    // Getter methods for D3 API parity
    
    /// Returns the current tick size inner value
    pub fn tick_size_inner_value(&self) -> f64 {
        self.tick_size_inner
    }
    
    /// Returns the current tick size outer value
    pub fn tick_size_outer_value(&self) -> f64 {
        self.tick_size_outer
    }
    
    /// Returns the current tick padding value
    pub fn tick_padding_value(&self) -> f64 {
        self.tick_padding
    }
    
    /// Returns the current offset value
    pub fn offset_value(&self) -> f64 {
        self.offset
    }
    
    /// Returns the current tick count
    pub fn tick_count_value(&self) -> usize {
        self.tick_count
    }
    
    /// Returns the current tick arguments
    pub fn tick_arguments_value(&self) -> Option<&Vec<f64>> {
        self.tick_arguments.as_ref()
    }
    
    /// Returns the current tick values
    pub fn tick_values_value(&self) -> Option<&Vec<f64>> {
        self.tick_values.as_ref()
    }
    
    /// Returns the current locale
    pub fn locale_value(&self) -> Option<&str> {
        self.locale.as_deref()
    }
}

// D3-style ticks method for different scale types
// This matches the D3 API where axis.ticks() returns the generated ticks

impl<T: Clone + PartialEq + ToString> Axis<crate::scale::ScaleBand<T>> {
    /// Returns the tick values that would be generated for this axis
    pub fn ticks(&self) -> Vec<Tick> {
        self.generate_ticks()
    }
}

impl<T: Clone + PartialEq + ToString> Axis<crate::scale::ScalePoint<T>> {
    /// Returns the tick values that would be generated for this axis
    pub fn ticks(&self) -> Vec<Tick> {
        self.generate_ticks()
    }
}

impl Axis<crate::scale::ScaleLinear> {
    /// Returns the tick values that would be generated for this axis
    pub fn ticks(&self) -> Vec<Tick> {
        self.generate_ticks()
    }
    
    /// Returns the tick values using the specified values instead of the scale's automatic tick generator
    pub fn ticks_with(&self, tick_values: Option<&[f64]>) -> Vec<Tick> {
        self.generate_ticks_with(tick_values)
    }
}

impl Axis<crate::scale::ScaleLog> {
    /// Returns the tick values that would be generated for this axis
    pub fn ticks(&self) -> Vec<Tick> {
        self.generate_ticks()
    }
    
    /// Returns the tick values using the specified values instead of the scale's automatic tick generator
    pub fn ticks_with(&self, tick_values: Option<Vec<f64>>) -> Vec<Tick> {
        self.generate_ticks_with(tick_values)
    }
}

impl Axis<crate::scale::ScaleTime> {
    /// Returns the tick values that would be generated for this axis
    pub fn ticks(&self) -> Vec<Tick> {
        self.generate_ticks()
    }
    
    /// Returns the tick values using the specified values instead of the scale's automatic tick generator
    pub fn ticks_with(&self, tick_values: Option<Vec<chrono::NaiveDateTime>>) -> Vec<Tick> {
        self.generate_ticks_with(tick_values)
    }
}

// Axis struct and AxisLayout struct
// Implements Axis<S> and AxisLayout for D3-like axes

use super::orientation::AxisOrientation;
use super::ticks::{Tick, TickFormat};

pub struct Axis<S> {
    pub scale: S,
    pub orientation: AxisOrientation,
    pub tick_count: usize,
    pub tick_format: Option<TickFormat>,
    pub tick_values: Option<Vec<f64>>, // For custom ticks (numeric/time)
    pub tick_size_inner: f64,
    pub tick_size_outer: f64,
    pub tick_padding: f64,
    pub tick_arguments: Option<Vec<f64>>, // For D3-like tickArguments
    pub offset: f64, // For pixel offset (D3 parity)
    pub locale: Option<String>, // For locale-aware formatting
}

pub struct AxisLayout {
    pub orientation: AxisOrientation,
    pub ticks: Vec<Tick>,
    pub tick_size_inner: f64,
    pub tick_size_outer: f64,
    pub tick_padding: f64,
    pub axis_start: f64,
    pub axis_end: f64,
    pub offset: f64,
}

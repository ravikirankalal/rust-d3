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
    pub offset: f64,                      // For pixel offset (D3 parity)
    pub locale: Option<String>,           // For locale-aware formatting
    pub grid: bool,                       // Show grid lines
    pub grid_style: Option<GridStyle>,    // Grid line styling
    pub title: Option<String>,            // Axis title
    pub title_style: Option<TitleStyle>,  // Axis title styling
    pub minor_ticks: Option<Vec<f64>>,    // Minor tick values
    pub minor_tick_size: Option<f64>,     // Minor tick size
    pub tick_label_angle: Option<f64>,    // Tick label rotation angle
    pub tick_label_style: Option<TickLabelStyle>, // Tick label styling
    pub axis_line_style: Option<AxisLineStyle>, // Axis line styling
    pub on_render: Option<Box<dyn Fn()>>, // Event hook: before/after render
}

// Styling structs
#[derive(Clone, Debug)]
pub struct GridStyle {
    pub color: String,
    pub width: f64,
    pub dasharray: Option<String>,
}
#[derive(Clone, Debug)]
pub struct TitleStyle {
    pub font: String,
    pub color: String,
    pub position: Option<(f64, f64)>,
}
#[derive(Clone, Debug)]
pub struct TickLabelStyle {
    pub font: String,
    pub color: String,
    pub padding: Option<f64>,
}
#[derive(Clone, Debug)]
pub struct AxisLineStyle {
    pub color: String,
    pub width: f64,
    pub dasharray: Option<String>,
}

impl Default for AxisLineStyle {
    fn default() -> Self {
        Self {
            color: "currentColor".to_string(),
            width: 1.0,
            dasharray: None,
        }
    }
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

impl<S> Axis<S> {
    // ...existing methods...

    pub fn grid(mut self, show: bool) -> Self {
        self.grid = show;
        self
    }
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }
    pub fn grid_style(mut self, style: GridStyle) -> Self {
        self.grid_style = Some(style);
        self
    }
    pub fn title_style(mut self, style: TitleStyle) -> Self {
        self.title_style = Some(style);
        self
    }
    pub fn minor_ticks(mut self, ticks: Vec<f64>) -> Self {
        self.minor_ticks = Some(ticks);
        self
    }
    pub fn minor_tick_size(mut self, size: f64) -> Self {
        self.minor_tick_size = Some(size);
        self
    }
    pub fn tick_label_angle(mut self, angle: f64) -> Self {
        self.tick_label_angle = Some(angle);
        self
    }
    pub fn tick_label_style(mut self, style: TickLabelStyle) -> Self {
        self.tick_label_style = Some(style);
        self
    }
    pub fn axis_line_style(mut self, style: AxisLineStyle) -> Self {
        self.axis_line_style = Some(style);
        self
    }
    pub fn on_render<F: Fn() + 'static>(mut self, hook: F) -> Self {
        self.on_render = Some(Box::new(hook));
        self
    }
    // ...existing methods...
}

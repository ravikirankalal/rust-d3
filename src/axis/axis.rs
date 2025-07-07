// Axis implementation

use crate::scale::{LinearScale, LogScale, PowScale, SqrtScale, OrdinalScale, BandScale};

pub enum AxisOrientation {
    Top,
    Bottom,
    Left,
    Right,
}

pub enum AxisScale<'a> {
    Linear(&'a LinearScale),
    Log(&'a LogScale),
    Pow(&'a PowScale),
    Sqrt(&'a SqrtScale),
    Ordinal(&'a OrdinalScale<String, String>),
    Band(&'a BandScale<String>),
}

pub struct Axis {
    pub ticks: usize,
    pub orientation: AxisOrientation,
    pub tick_format: Option<Box<dyn Fn(f64) -> String + Send + Sync>>,
    pub custom_ticks: Option<Vec<f64>>,
    pub label: Option<String>,
}

impl Axis {
    pub fn new(ticks: usize, orientation: AxisOrientation) -> Self {
        Self {
            ticks,
            orientation,
            tick_format: None,
            custom_ticks: None,
            label: None,
        }
    }

    pub fn with_tick_format<F>(mut self, fmt: F) -> Self
    where
        F: Fn(f64) -> String + Send + Sync + 'static,
    {
        self.tick_format = Some(Box::new(fmt));
        self
    }

    pub fn with_custom_ticks(mut self, ticks: Vec<f64>) -> Self {
        self.custom_ticks = Some(ticks);
        self
    }

    pub fn with_label<S: Into<String>>(mut self, label: S) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn generate(&self, scale: AxisScale) -> Vec<(f64, String)> {
        let (ticks, formatter): (Vec<f64>, &dyn Fn(f64) -> String) = match scale {
            AxisScale::Linear(s) => {
                let ticks = if let Some(ref custom) = self.custom_ticks {
                    custom.clone()
                } else {
                    s.ticks(self.ticks)
                };
                let fmt: &dyn Fn(f64) -> String = if let Some(ref f) = self.tick_format {
                    &**f
                } else {
                    &|v| v.to_string()
                };
                (ticks, fmt)
            }
            AxisScale::Log(s) => {
                let ticks = if let Some(ref custom) = self.custom_ticks {
                    custom.clone()
                } else {
                    s.ticks(self.ticks)
                };
                let fmt: &dyn Fn(f64) -> String = if let Some(ref f) = self.tick_format {
                    &**f
                } else {
                    &|v| v.to_string()
                };
                (ticks, fmt)
            }
            AxisScale::Pow(s) => {
                let ticks = if let Some(ref custom) = self.custom_ticks {
                    custom.clone()
                } else {
                    s.ticks(self.ticks)
                };
                let fmt: &dyn Fn(f64) -> String = if let Some(ref f) = self.tick_format {
                    &**f
                } else {
                    &|v| v.to_string()
                };
                (ticks, fmt)
            }
            AxisScale::Sqrt(s) => {
                let ticks = if let Some(ref custom) = self.custom_ticks {
                    custom.clone()
                } else {
                    s.ticks(self.ticks)
                };
                let fmt: &dyn Fn(f64) -> String = if let Some(ref f) = self.tick_format {
                    &**f
                } else {
                    &|v| v.to_string()
                };
                (ticks, fmt)
            }
            AxisScale::Ordinal(s) => {
                let ticks: Vec<f64> = (0..s.domain().len()).map(|i| i as f64).collect();
                let fmt: &dyn Fn(f64) -> String = &|v| v.to_string();
                (ticks, fmt)
            }
            AxisScale::Band(s) => {
                let ticks: Vec<f64> = (0..s.domain().len()).map(|i| i as f64).collect();
                let fmt: &dyn Fn(f64) -> String = &|v| v.to_string();
                (ticks, fmt)
            }
        };
        ticks
            .into_iter()
            .map(|v| {
                let label = formatter(v);
                (v, label)
            })
            .collect()
    }

    /// Render the axis as a simple SVG group (stub implementation)
    pub fn to_svg(&self, scale: AxisScale) -> String {
        let ticks = self.generate(scale);
        let orientation = match self.orientation {
            AxisOrientation::Top => "top",
            AxisOrientation::Bottom => "bottom",
            AxisOrientation::Left => "left",
            AxisOrientation::Right => "right",
        };
        let mut svg = format!("<g class='axis axis-{}'>\n", orientation);
        for (_pos, label) in ticks {
            svg.push_str(&format!("  <g class='tick'><text>{}</text></g>\n", label));
        }
        if let Some(ref label) = self.label {
            svg.push_str(&format!("  <text class='axis-label'>{}</text>\n", label));
        }
        svg.push_str("</g>");
        svg
    }
    
}

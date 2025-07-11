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

pub trait AxisRenderable {
    fn render(&self, selection: &mut crate::selection::Selection);
}

impl AxisRenderable for Axis<crate::scale::ScaleLinear> {
    fn render(&self, selection: &mut crate::selection::Selection) {
        match self.orientation {
            AxisOrientation::Bottom => {
                let ticks = self.ticks();
                println!("[AxisRenderable::ScaleLinear] Bottom axis ticks:");
                for tick in &ticks {
                    println!("  label: '{}' at position: {}", tick.label, tick.position);
                }
                if let (Some(first), Some(last)) = (ticks.first(), ticks.last()) {
                    selection.append("line")
                        .attr("x1", &first.position.to_string())
                        .attr("x2", &last.position.to_string())
                        .attr("y1", "0")
                        .attr("y2", "0")
                        .attr("stroke", "black")
                        .attr("stroke-width", "1")
                        .attr("class", "domain");
                }
                for tick in &ticks {
                    selection.append("line")
                        .attr("x1", &tick.position.to_string())
                        .attr("x2", &tick.position.to_string())
                        .attr("y1", "0")
                        .attr("y2", &self.tick_size_inner.to_string())
                        .attr("stroke", "black");
                    selection.append("text")
                        .attr("x", &tick.position.to_string())
                        .attr("y", &format!("{}", self.tick_size_inner + self.tick_padding + 12.0))
                        .attr("text-anchor", "middle")
                        .attr("font-size", "12px")
                        .attr("fill", "black")
                        .attr("font-family", "Arial, sans-serif")
                        .text(&tick.label);
                }
            }
            AxisOrientation::Left => {
                let ticks = self.ticks();
                println!("[AxisRenderable::ScaleLinear] Left axis ticks:");
                for tick in &ticks {
                    println!("  label: '{}' at position: {}", tick.label, tick.position);
                }
                if let (Some(first), Some(last)) = (ticks.first(), ticks.last()) {
                    selection.append("line")
                        .attr("x1", "0")
                        .attr("x2", "0")
                        .attr("y1", &first.position.to_string())
                        .attr("y2", &last.position.to_string())
                        .attr("stroke", "black")
                        .attr("stroke-width", "1")
                        .attr("class", "domain");
                }
                for tick in &ticks {
                    selection.append("line")
                        .attr("x1", "0")
                        .attr("x2", &self.tick_size_inner.to_string())
                        .attr("y1", &tick.position.to_string())
                        .attr("y2", &tick.position.to_string())
                        .attr("stroke", "black");
                    selection.append("text")
                        .attr("x", &format!("{}", self.tick_size_inner + self.tick_padding + 2.0))
                        .attr("y", &tick.position.to_string())
                        .attr("text-anchor", "start")
                        .attr("font-size", "12px")
                        .attr("fill", "black")
                        .attr("font-family", "Arial, sans-serif")
                        .text(&tick.label);
                }
            }
            _ => {}
        }
    }
}

impl AxisRenderable for Axis<crate::scale::ScaleLog> {
    fn render(&self, selection: &mut crate::selection::Selection) {
        match self.orientation {
            AxisOrientation::Bottom => {
                let ticks = self.ticks();
                println!("[AxisRenderable::ScaleLog] Bottom axis ticks:");
                for tick in &ticks {
                    println!("  label: '{}' at position: {}", tick.label, tick.position);
                }
                if let (Some(first), Some(last)) = (ticks.first(), ticks.last()) {
                    selection.append("line")
                        .attr("x1", &first.position.to_string())
                        .attr("x2", &last.position.to_string())
                        .attr("y1", "0")
                        .attr("y2", "0")
                        .attr("stroke", "black")
                        .attr("stroke-width", "1")
                        .attr("class", "domain");
                }
                for tick in &ticks {
                    selection.append("line")
                        .attr("x1", &tick.position.to_string())
                        .attr("x2", &tick.position.to_string())
                        .attr("y1", "0")
                        .attr("y2", &self.tick_size_inner.to_string())
                        .attr("stroke", "black");
                    selection.append("text")
                        .attr("x", &tick.position.to_string())
                        .attr("y", &format!("{}", self.tick_size_inner + self.tick_padding + 12.0))
                        .attr("text-anchor", "middle")
                        .attr("font-size", "12px")
                        .attr("fill", "black")
                        .attr("font-family", "Arial, sans-serif")
                        .text(&tick.label);
                }
            }
            AxisOrientation::Left => {
                let ticks = self.ticks();
                println!("[AxisRenderable::ScaleLog] Left axis ticks:");
                for tick in &ticks {
                    println!("  label: '{}' at position: {}", tick.label, tick.position);
                }
                if let (Some(first), Some(last)) = (ticks.first(), ticks.last()) {
                    selection.append("line")
                        .attr("x1", "0")
                        .attr("x2", "0")
                        .attr("y1", &first.position.to_string())
                        .attr("y2", &last.position.to_string())
                        .attr("stroke", "black")
                        .attr("stroke-width", "1")
                        .attr("class", "domain");
                }
                for tick in &ticks {
                    selection.append("line")
                        .attr("x1", "0")
                        .attr("x2", &self.tick_size_inner.to_string())
                        .attr("y1", &tick.position.to_string())
                        .attr("y2", &tick.position.to_string())
                        .attr("stroke", "black");
                    selection.append("text")
                        .attr("x", &format!("{}", self.tick_size_inner + self.tick_padding + 2.0))
                        .attr("y", &tick.position.to_string())
                        .attr("text-anchor", "start")
                        .attr("font-size", "12px")
                        .attr("fill", "black")
                        .attr("font-family", "Arial, sans-serif")
                        .text(&tick.label);
                }
            }
            _ => {}
        }
    }
}

impl AxisRenderable for Axis<crate::scale::ScaleTime> {
    fn render(&self, selection: &mut crate::selection::Selection) {
        match self.orientation {
            AxisOrientation::Bottom => {
                let ticks = self.ticks();
                println!("[AxisRenderable::ScaleTime] Bottom axis ticks:");
                for tick in &ticks {
                    println!("  label: '{}' at position: {}", tick.label, tick.position);
                }
                if let (Some(first), Some(last)) = (ticks.first(), ticks.last()) {
                    selection.append("line")
                        .attr("x1", &first.position.to_string())
                        .attr("x2", &last.position.to_string())
                        .attr("y1", "0")
                        .attr("y2", "0")
                        .attr("stroke", "black")
                        .attr("stroke-width", "1")
                        .attr("class", "domain");
                }
                for tick in &ticks {
                    selection.append("line")
                        .attr("x1", &tick.position.to_string())
                        .attr("x2", &tick.position.to_string())
                        .attr("y1", "0")
                        .attr("y2", &self.tick_size_inner.to_string())
                        .attr("stroke", "black");
                    selection.append("text")
                        .attr("x", &tick.position.to_string())
                        .attr("y", &format!("{}", self.tick_size_inner + self.tick_padding + 12.0))
                        .attr("text-anchor", "middle")
                        .attr("font-size", "12px")
                        .attr("fill", "black")
                        .attr("font-family", "Arial, sans-serif")
                        .text(&tick.label);
                }
            }
            AxisOrientation::Left => {
                let ticks = self.ticks();
                println!("[AxisRenderable::ScaleTime] Left axis ticks:");
                for tick in &ticks {
                    println!("  label: '{}' at position: {}", tick.label, tick.position);
                }
                if let (Some(first), Some(last)) = (ticks.first(), ticks.last()) {
                    selection.append("line")
                        .attr("x1", "0")
                        .attr("x2", "0")
                        .attr("y1", &first.position.to_string())
                        .attr("y2", &last.position.to_string())
                        .attr("stroke", "black")
                        .attr("stroke-width", "1")
                        .attr("class", "domain");
                }
                for tick in &ticks {
                    selection.append("line")
                        .attr("x1", "0")
                        .attr("x2", &self.tick_size_inner.to_string())
                        .attr("y1", &tick.position.to_string())
                        .attr("y2", &tick.position.to_string())
                        .attr("stroke", "black");
                    selection.append("text")
                        .attr("x", &format!("{}", self.tick_size_inner + self.tick_padding + 2.0))
                        .attr("y", &tick.position.to_string())
                        .attr("text-anchor", "start")
                        .attr("font-size", "12px")
                        .attr("fill", "black")
                        .attr("font-family", "Arial, sans-serif")
                        .text(&tick.label);
                }
            }
            _ => {}
        }
    }
}

impl<T: Clone + PartialEq + ToString> Axis<crate::scale::ScaleBand<T>> {
    pub fn ticks(&self) -> Vec<Tick> {
        let domain = &self.scale.domain;
        domain.iter().filter_map(|v| {
            self.scale.scale(v).map(|pos| {
                let label = v.to_string();
                Tick::new(0.0, label, pos)
            })
        }).collect()
    }
}

impl<T: Clone + PartialEq + ToString> Axis<crate::scale::ScalePoint<T>> {
    pub fn ticks(&self) -> Vec<Tick> {
        let domain = &self.scale.domain;
        domain.iter().filter_map(|v| {
            self.scale.scale(v).map(|pos| {
                let label = v.to_string();
                Tick::new(0.0, label, pos)
            })
        }).collect()
    }
}

impl Axis<crate::scale::ScaleLinear> {
    pub fn ticks(&self) -> Vec<Tick> {
        self.ticks_with(None)
    }

    pub fn ticks_with(&self, tick_input: Option<&[f64]>) -> Vec<Tick> {
        let values: Vec<f64> = if let Some(input) = tick_input {
            input.to_vec()
        } else if let Some(ref values) = self.tick_values {
            values.clone()
        } else {
            let domain = self.scale.domain;
            let count = self.tick_count.max(2);
            let step = (domain[1] - domain[0]) / (count as f64 - 1.0);
            (0..count).map(|i| domain[0] + i as f64 * step).collect()
        };
        let ticks: Vec<Tick> = values.into_iter().map(|value| {
            let position = self.scale.scale(value);
            let label = if let Some(fmt) = self.tick_format {
                (fmt)(value)
            } else if let Some(ref locale) = self.locale {
                crate::format::format_locale(value, locale, true)
            } else {
                format!("{:.6}", value)
            };
            println!("[Axis<ScaleLinear>::ticks_with] value: {}, position: {}, label: '{}'", value, position, label);
            Tick::new(value, label, position)
        }).collect();
        ticks
    }
}

impl Axis<crate::scale::ScaleLog> {
    pub fn ticks_with(&self, tick_values: Option<Vec<f64>>) -> Vec<Tick> {
        let values = tick_values.unwrap_or_else(|| {
            self.scale.ticks(self.tick_count)
        });
        values.iter().map(|&v| {
            Tick {
                position: self.scale.scale(v),
                value: v,
                label: format!("{:.2}", v),
            }
        }).collect()
    }
    pub fn ticks(&self) -> Vec<Tick> {
        self.ticks_with(None)
    }
}

impl Axis<crate::scale::ScaleTime> {
    pub fn ticks_with(&self, tick_values: Option<Vec<chrono::NaiveDateTime>>) -> Vec<Tick> {
        let values = tick_values.unwrap_or_else(|| {
            self.scale.ticks(self.tick_count)
        });
        values.iter().map(|&dt| {
            Tick {
                position: self.scale.scale(dt),
                value: dt.and_utc().timestamp_millis() as f64,
                label: dt.format("%Y-%m-%d").to_string(),
            }
        }).collect()
    }
    pub fn ticks(&self) -> Vec<Tick> {
        self.ticks_with(None)
    }
}

pub fn axis_bottom<S>(scale: S) -> Axis<S> {
    Axis::new(scale, AxisOrientation::Bottom)
}

pub fn axis_top<S>(scale: S) -> Axis<S> {
    Axis::new(scale, AxisOrientation::Top)
}
pub fn axis_right<S>(scale: S) -> Axis<S> {
    Axis::new(scale, AxisOrientation::Right)
}
pub fn axis_left<S>(scale: S) -> Axis<S> {
    Axis::new(scale, AxisOrientation::Left)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scale::{ScaleLinear, ScaleLog, ScaleTime, ScaleBand, ScalePoint};
    use chrono::NaiveDate;

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

    #[test]
    fn test_log_axis_ticks() {
        let scale = ScaleLog::new([1.0, 1000.0], [0.0, 100.0], 10.0);
        let axis = Axis::new(scale, AxisOrientation::Left).tick_count(4);
        let ticks = axis.ticks();
        assert!(ticks.iter().any(|t| (t.value - 1.0).abs() < 1e-6));
        assert!(ticks.iter().any(|t| (t.value - 10.0).abs() < 1e-6));
        assert!(ticks.iter().any(|t| (t.value - 100.0).abs() < 1e-6));
        assert!(ticks.iter().any(|t| (t.value - 1000.0).abs() < 1e-6));
    }

    #[test]
    fn test_time_axis_ticks() {
        let start = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap();
        let end = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap().and_hms_opt(0, 0, 4).unwrap();
        let scale = ScaleTime::new([start, end], [0.0, 100.0]);
        let axis = Axis::new(scale, AxisOrientation::Bottom).tick_count(5);
        let ticks = axis.ticks();
        assert_eq!(ticks.len(), 5);
        assert_eq!(ticks[0].label, "2020-01-01");
        assert_eq!(ticks[4].label, "2020-01-01");
    }

    #[test]
    fn test_band_axis_ticks() {
        let scale = ScaleBand::new(vec!["a", "b", "c"], [0.0, 120.0], 0.1, 0.1, 0.5);
        let axis = Axis::new(scale, AxisOrientation::Bottom);
        let ticks = axis.ticks();
        assert_eq!(ticks.len(), 3);
        assert_eq!(ticks[0].label, "a");
        assert_eq!(ticks[1].label, "b");
        assert_eq!(ticks[2].label, "c");
    }

    #[test]
    fn test_point_axis_ticks() {
        let scale = ScalePoint::new(vec!["x", "y", "z"], [0.0, 100.0], 0.5);
        let axis = Axis::new(scale, AxisOrientation::Left);
        let ticks = axis.ticks();
        assert_eq!(ticks.len(), 3);
        assert_eq!(ticks[0].label, "x");
        assert_eq!(ticks[1].label, "y");
        assert_eq!(ticks[2].label, "z");
    }

    #[test]
    fn test_linear_axis_custom_ticks() {
        let scale = ScaleLinear::new([0.0, 10.0], [0.0, 100.0]);
        let axis = Axis::new(scale, AxisOrientation::Bottom)
            .tick_values(vec![2.0, 5.0, 8.0]);
        let ticks = axis.ticks();
        assert_eq!(ticks.len(), 3);
        assert!((ticks[0].value - 2.0).abs() < 1e-6);
        assert!((ticks[1].value - 5.0).abs() < 1e-6);
        assert!((ticks[2].value - 8.0).abs() < 1e-6);
    }

    #[test]
    fn test_axis_layout_linear() {
        let scale = ScaleLinear::new([0.0, 10.0], [0.0, 100.0]);
        let axis = Axis::new(scale, AxisOrientation::Bottom)
            .tick_count(3)
            .tick_size_inner(8.0)
            .tick_size_outer(12.0)
            .tick_padding(5.0);
        let ticks = axis.ticks();
        let layout = axis.layout(0.0, 100.0, ticks.clone());
        assert_eq!(layout.orientation, AxisOrientation::Bottom);
        assert_eq!(layout.ticks.len(), 3);
        assert_eq!(layout.tick_size_inner, 8.0);
        assert_eq!(layout.tick_size_outer, 12.0);
        assert_eq!(layout.tick_padding, 5.0);
        assert_eq!(layout.axis_start, 0.0);
        assert_eq!(layout.axis_end, 100.0);
        assert_eq!(layout.ticks[0].label, ticks[0].label);
    }

    #[test]
    fn test_axis_layout_with_offset_and_locale() {
        let scale = ScaleLinear::new([0.0, 1.0], [0.0, 100.0]);
        let axis = Axis::new(scale, AxisOrientation::Top)
            .tick_count(2)
            .tick_size_inner(5.0)
            .tick_size_outer(7.0)
            .tick_padding(2.0)
            .offset(0.5)
            .locale("fr-FR");
        let ticks = axis.ticks();
        let layout = axis.layout(0.0, 100.0, ticks.clone());
        assert_eq!(layout.orientation, AxisOrientation::Top);
        assert_eq!(layout.offset, 0.5);
        assert_eq!(axis.locale.as_deref(), Some("fr-FR"));
    }
}

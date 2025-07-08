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

impl Axis<crate::scale::ScaleLinear> {
    pub fn ticks(&self) -> Vec<Tick> {
        if let Some(ref values) = self.tick_values {
            return values.iter().map(|&value| {
                let position = self.scale.scale(value);
                let label = if let Some(fmt) = self.tick_format {
                    (fmt)(value)
                } else if let Some(ref locale) = self.locale {
                    crate::format::format_locale(value, locale)
                } else {
                    format!("{:.6}", value)
                };
                Tick::new(value, label, position)
            }).collect();
        }
        let domain = self.scale.domain;
        let step = (domain[1] - domain[0]) / (self.tick_count as f64 - 1.0);
        (0..self.tick_count)
            .map(|i| {
                let value = domain[0] + i as f64 * step;
                let position = self.scale.scale(value);
                let label = if let Some(fmt) = self.tick_format {
                    (fmt)(value)
                } else if let Some(ref locale) = self.locale {
                    crate::format::format_locale(value, locale)
                } else {
                    format!("{:.6}", value)
                };
                Tick::new(value, label, position)
            })
            .collect()
    }
}

impl Axis<crate::scale::ScaleLog> {
    pub fn ticks(&self) -> Vec<Tick> {
        let domain = self.scale.domain;
        let base = self.scale.base;
        let (d0, d1) = (domain[0].min(domain[1]), domain[0].max(domain[1]));
        let log_base = |x: f64| x.log(base);
        let start = log_base(d0).ceil();
        let end = log_base(d1).floor();
        let mut ticks = vec![];
        // Always include lower bound if it's a power of base
        if (log_base(d0) - log_base(d0).round()).abs() < 1e-6 {
            ticks.push(d0);
        }
        for i in (start as i32)..=(end as i32) {
            let value = base.powi(i);
            if value > d0 && value < d1 {
                ticks.push(value);
            }
        }
        // Always include upper bound if it's a power of base
        if (log_base(d1) - log_base(d1).round()).abs() < 1e-6 && !ticks.iter().any(|&v| (v - d1).abs() < 1e-6) {
            ticks.push(d1);
        }
        ticks.sort_by(|a, b| a.partial_cmp(b).unwrap());
        ticks.into_iter().map(|value| {
            let position = self.scale.scale(value);
            let label = if let Some(fmt) = self.tick_format {
                (fmt)(value)
            } else {
                format!("{:.6}", value)
            };
            Tick::new(value, label, position)
        }).collect()
    }
}

impl Axis<crate::scale::ScaleTime> {
    pub fn ticks(&self) -> Vec<Tick> {
        let domain = self.scale.domain;
        let tick_count = self.tick_count as i64;
        let start = domain[0];
        let end = domain[1];
        let total_ms = end.and_utc().timestamp_millis() - start.and_utc().timestamp_millis();
        let step_ms = total_ms / (tick_count - 1).max(1);
        (0..tick_count)
            .map(|i| {
                let ms = start.and_utc().timestamp_millis() + i * step_ms;
                let value = chrono::DateTime::<chrono::Utc>::from_timestamp((ms / 1000) as i64, ((ms % 1000) * 1_000_000) as u32)
                    .map(|dt| dt.naive_utc())
                    .unwrap_or_else(|| chrono::DateTime::<chrono::Utc>::from_timestamp(0, 0).unwrap().naive_utc());
                let position = self.scale.scale(value);
                let label = if let Some(fmt) = self.tick_format {
                    (fmt)(ms as f64)
                } else {
                    value.format("%Y-%m-%d %H:%M:%S").to_string()
                };
                Tick::new(ms as f64, label, position)
            })
            .collect()
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
        assert_eq!(ticks[0].label, "2020-01-01 00:00:00");
        assert_eq!(ticks[4].label, "2020-01-01 00:00:04");
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

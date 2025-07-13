// Axis ticks logic for different scale types

use super::axis_structs::Axis;
use super::ticks::Tick;

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
                d3_si_format(value)
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

// D3-like SI formatter for numbers
fn d3_si_format(value: f64) -> String {
    if value == 0.0 {
        return "0".to_string();
    }
    let abs = value.abs();
    let prefixes = ["y", "z", "a", "f", "p", "n", "µ", "m", "", "k", "M", "G", "T", "P", "E", "Z", "Y"];
    let i = (abs.log10() / 3.0).floor() as isize + 8; // +8 to index into prefixes array
    let i = i.max(0).min(prefixes.len() as isize - 1) as usize; // Clamp index to bounds
    let value_with_prefix = value / 10f64.powf((i as f64 - 8.0) * 3.0);
    let prefix = prefixes[i];
    if abs >= 1.0 {
        format!("{:.2}{}", value_with_prefix, prefix)
    } else if abs >= 1e-2 {
        format!("{:.3}{}", value_with_prefix, prefix)
    } else {
        format!("{:.1e}{}", value_with_prefix, prefix)
    }
}

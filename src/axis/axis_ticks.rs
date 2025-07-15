// Axis ticks logic for different scale types

use super::axis_structs::Axis;
use super::ticks::Tick;

impl<T: Clone + PartialEq + ToString> Axis<crate::scale::ScaleBand<T>> {
    pub fn generate_ticks(&self) -> Vec<Tick> {
        let domain = &self.scale.domain;
        domain
            .iter()
            .filter_map(|v| {
                self.scale.scale(v).map(|pos| {
                    let label = v.to_string();
                    Tick::new(0.0, label, pos)
                })
            })
            .collect()
    }
}

impl<T: Clone + PartialEq + ToString> Axis<crate::scale::ScalePoint<T>> {
    pub fn generate_ticks(&self) -> Vec<Tick> {
        let domain = &self.scale.domain;
        domain
            .iter()
            .filter_map(|v| {
                self.scale.scale(v).map(|pos| {
                    let label = v.to_string();
                    Tick::new(0.0, label, pos)
                })
            })
            .collect()
    }
}

impl Axis<crate::scale::ScaleLinear> {
    pub fn generate_ticks(&self) -> Vec<Tick> {
        self.generate_ticks_with(None)
    }

    pub fn generate_ticks_with(&self, tick_input: Option<&[f64]>) -> Vec<Tick> {
        let values: Vec<f64> = if let Some(input) = tick_input {
            input.to_vec()
        } else if let Some(ref values) = self.tick_values {
            values.clone()
        } else if let Some(ref args) = self.tick_arguments {
            // Use tick_arguments to call scale.ticks() with custom parameters
            // args[0] is the count, following D3's axis.ticks(count, specifier) signature
            let count = args.get(0).map(|&c| c as usize).unwrap_or(self.tick_count);
            self.scale.ticks(count)
        } else {
            // Use the D3-compatible ticks method from ScaleLinear
            self.scale.ticks(self.tick_count)
        };
        
        let ticks: Vec<Tick> = values
            .into_iter()
            .map(|value| {
                let position = self.scale.scale(value);
                let label = if let Some(fmt) = self.tick_format {
                    (fmt)(value)
                } else if let Some(ref args) = self.tick_arguments {
                    // args[1] might contain a specifier for formatting
                    // Use scale.tick_format() with the specifier if available
                    if args.len() > 1 {
                        // For now, treating args[1] as a specifier parameter
                        // This would need proper string conversion in real implementation
                        let count = args.get(0).map(|&c| c as usize).unwrap_or(self.tick_count);
                        let format_fn = self.scale.tick_format(count, None); // Would need specifier parsing
                        format_fn(value)
                    } else {
                        d3_format_default_locale_6g(value)
                    }
                } else if let Some(ref locale) = self.locale {
                    crate::format::format_locale(value, locale, true)
                } else {
                    d3_format_default_locale_6g(value)
                };
                Tick::new(value, label, position)
            })
            .collect();
        ticks
    }
}

impl Axis<crate::scale::ScaleLog> {
    pub fn generate_ticks_with(&self, tick_values: Option<Vec<f64>>) -> Vec<Tick> {
        let values = tick_values.unwrap_or_else(|| self.scale.ticks(self.tick_count));
        values
            .iter()
            .map(|&v| Tick {
                position: self.scale.scale(v),
                value: v,
                label: format!("{:.2}", v),
            })
            .collect()
    }
    pub fn generate_ticks(&self) -> Vec<Tick> {
        self.generate_ticks_with(None)
    }
}

impl Axis<crate::scale::ScaleTime> {
    pub fn generate_ticks_with(&self, tick_values: Option<Vec<chrono::NaiveDateTime>>) -> Vec<Tick> {
        let values = if let Some(values) = tick_values {
            values
        } else if let Some(ref args) = self.tick_arguments {
            // Use tick_arguments to call scale.ticks() with custom parameters
            // args[0] is the count, following D3's axis.ticks(count, specifier) signature
            let count = args.get(0).map(|&c| c as usize).unwrap_or(self.tick_count);
            self.scale.ticks(count)
        } else {
            self.scale.ticks(self.tick_count)
        };
        
        values
            .iter()
            .map(|&dt| {
                let position = self.scale.scale(dt);
                let label = if let Some(fmt) = self.tick_format {
                    (fmt)(dt.and_utc().timestamp_millis() as f64)
                } else if let Some(ref args) = self.tick_arguments {
                    // args[1] might contain a specifier for time formatting
                    // For now, use a reasonable default format based on the tick count
                    if args.len() > 1 {
                        // Could implement specifier parsing here in the future
                        dt.format("%Y-%m-%d %H:%M:%S").to_string()
                    } else {
                        dt.format("%Y-%m-%d").to_string()
                    }
                } else {
                    dt.format("%Y-%m-%d").to_string()
                };
                Tick {
                    position,
                    value: dt.and_utc().timestamp_millis() as f64,
                    label,
                }
            })
            .collect()
    }
    pub fn generate_ticks(&self) -> Vec<Tick> {
        self.generate_ticks_with(None)
    }
}


// D3-format compatible formatter matching formatDefaultLocale(".6g")
fn d3_format_default_locale_6g(value: f64) -> String {
    // Handle special cases
    if value.is_nan() {
        return "NaN".to_string();
    }
    if value.is_infinite() {
        return if value > 0.0 { "∞" } else { "-∞" }.to_string();
    }
    if value == 0.0 {
        return "0".to_string();
    }
    
    let abs_value = value.abs();
    
    // For very small non-integer numbers, return "0.000000" format
    if abs_value < 1.0 && (value - value.round()).abs() > 1e-10 {
        if abs_value < 1e-6 {
            return "0.000000".to_string();
        }
    }
    
    // For integers or values that are very close to integers, format as integer
    // But only if they are not too large (should use SI prefix for large numbers)
    if (value - value.round()).abs() < 1e-10 {
        let rounded = value.round();
        if rounded.abs() < 1e6 { // Use SI prefix for integers >= 1e6
            return format!("{}", rounded as i64);
        }
    }
    
    // Use toPrecision(6) equivalent behavior for the "g" format
    let precision = 6;
    let formatted = if abs_value >= 1e-4 && abs_value < 1e6 {
        // Use fixed-point notation for values in the range [1e-4, 1e6)
        let mut result = format!("{:.prec$}", value, prec = precision);
        
        // Remove trailing zeros after decimal point (trim behavior)
        if result.contains('.') {
            result = result.trim_end_matches('0').trim_end_matches('.').to_string();
        }
        result
    } else if abs_value >= 1e6 {
        // Use SI prefix notation for large numbers
        format_with_si_prefix(value, precision)
    } else {
        // Use exponential notation for very small or very large numbers
        let mut result = format!("{:.prec$e}", value, prec = precision - 1);
        
        // Remove trailing zeros from the coefficient
        if let Some(e_pos) = result.find('e') {
            let (coeff, exp) = result.split_at(e_pos);
            let mut coeff = coeff.trim_end_matches('0').trim_end_matches('.').to_string();
            if coeff.is_empty() || coeff == "-" {
                coeff = "0".to_string();
            }
            result = format!("{}{}", coeff, exp);
        }
        result
    };
    
    formatted
}

// Helper function to format with SI prefix for large numbers
fn format_with_si_prefix(value: f64, precision: usize) -> String {
    let prefixes = [
        "y", "z", "a", "f", "p", "n", "µ", "m", "", "k", "M", "G", "T", "P", "E", "Z", "Y",
    ];
    
    let abs_value = value.abs();
    let exponent = (abs_value.log10() / 3.0).floor() as isize;
    let clamped_exp = exponent.max(-8).min(8);
    let prefix_index = (clamped_exp + 8) as usize;
    
    if prefix_index < prefixes.len() {
        let scale = 10f64.powf((clamped_exp * 3) as f64);
        let scaled_value = value / scale;
        let prefix = prefixes[prefix_index];
        
        // Format with appropriate precision and trim trailing zeros
        let mut result = format!("{:.prec$}", scaled_value, prec = precision - 1);
        if result.contains('.') {
            result = result.trim_end_matches('0').trim_end_matches('.').to_string();
        }
        
        format!("{}{}", result, prefix)
    } else {
        // Fallback to exponential notation
        format!("{:.prec$e}", value, prec = precision - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d3_format_default_locale_6g() {
        // Test special cases
        assert_eq!(d3_format_default_locale_6g(0.0), "0");
        assert_eq!(d3_format_default_locale_6g(f64::NAN), "NaN");
        assert_eq!(d3_format_default_locale_6g(f64::INFINITY), "∞");
        assert_eq!(d3_format_default_locale_6g(f64::NEG_INFINITY), "-∞");
        
        // Test integers
        assert_eq!(d3_format_default_locale_6g(1.0), "1");
        assert_eq!(d3_format_default_locale_6g(42.0), "42");
        assert_eq!(d3_format_default_locale_6g(-5.0), "-5");
        
        // Test small non-integer numbers that should return "0.000000" format
        assert_eq!(d3_format_default_locale_6g(1e-7), "0.000000");
        assert_eq!(d3_format_default_locale_6g(1e-8), "0.000000");
        assert_eq!(d3_format_default_locale_6g(-1e-7), "0.000000");
        
        // Test regular decimal numbers
        assert_eq!(d3_format_default_locale_6g(0.5), "0.5");
        assert_eq!(d3_format_default_locale_6g(0.001), "0.001");
        assert_eq!(d3_format_default_locale_6g(0.0001), "0.0001");
        
        // Test large numbers with SI prefix
        assert_eq!(d3_format_default_locale_6g(1e6), "1M");
        assert_eq!(d3_format_default_locale_6g(1.5e6), "1.5M");
        assert_eq!(d3_format_default_locale_6g(1e9), "1G");
        assert_eq!(d3_format_default_locale_6g(1.23e9), "1.23G");
        
        // Test numbers that should use exponential notation
        assert_eq!(d3_format_default_locale_6g(1e-5), "1e-5");
        assert_eq!(d3_format_default_locale_6g(1.23e-5), "1.23e-5");
    }
    
    #[test]
    fn test_format_with_si_prefix() {
        // Test basic SI prefix formatting
        assert_eq!(format_with_si_prefix(1000.0, 6), "1k");
        assert_eq!(format_with_si_prefix(1500.0, 6), "1.5k");
        assert_eq!(format_with_si_prefix(1000000.0, 6), "1M");
        assert_eq!(format_with_si_prefix(1500000.0, 6), "1.5M");
        assert_eq!(format_with_si_prefix(1000000000.0, 6), "1G");
        
        // Test negative numbers
        assert_eq!(format_with_si_prefix(-1000.0, 6), "-1k");
        assert_eq!(format_with_si_prefix(-1500000.0, 6), "-1.5M");
    }

    // Note: The TickFormat override functionality is implemented in the main code 
    // where the label is generated. The logic checks self.tick_format first,
    // then self.locale, then falls back to d3_format_default_locale_6g.
    // This ensures that when tick_format is present, it takes precedence.
    
    #[test]
    fn test_tick_arguments_linear() {
        use crate::axis::axis_constructors::axis_bottom;
        use crate::scale::linear::ScaleLinear;
        
        let scale = ScaleLinear::new([0.0, 100.0], [0.0, 500.0]);
        
        // Test with default tick_count
        let axis = axis_bottom(scale.clone());
        let ticks = axis.ticks();
        let default_count = ticks.len();
        
        // Test with tick_arguments containing custom count
        let axis_custom = axis_bottom(scale.clone())
            .tick_arguments(vec![5.0]);
        let ticks_custom = axis_custom.ticks();
        
        // Should have approximately 5 ticks (D3 tick generation is approximate)
        assert!(ticks_custom.len() > 0);
        assert!(ticks_custom.len() <= 10); // Reasonable upper bound
        
        // Test with tick_arguments containing count and potential specifier
        let axis_with_spec = axis_bottom(scale.clone())
            .tick_arguments(vec![3.0, 1.0]);
        let ticks_with_spec = axis_with_spec.ticks();
        
        // Should have approximately 3 ticks
        assert!(ticks_with_spec.len() > 0);
        assert!(ticks_with_spec.len() <= 8); // Reasonable upper bound
        
        println!("Linear axis - Default: {}, Custom (5): {}, With spec (3): {}", 
                default_count, ticks_custom.len(), ticks_with_spec.len());
    }
    
    #[test]
    fn test_tick_arguments_time() {
        use crate::axis::axis_constructors::axis_bottom;
        use crate::scale::time::ScaleTime;
        use chrono::NaiveDate;
        
        let start_date = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2020, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap();
        let time_scale = ScaleTime::new([start_date, end_date], [0.0, 500.0]);
        
        // Test with default tick_count
        let time_axis = axis_bottom(time_scale.clone());
        let time_ticks = time_axis.ticks();
        let default_count = time_ticks.len();
        
        // Test with tick_arguments for time scale
        let time_axis_custom = axis_bottom(time_scale.clone())
            .tick_arguments(vec![6.0]);
        let time_ticks_custom = time_axis_custom.ticks();
        
        // Should have approximately 6 ticks
        assert!(time_ticks_custom.len() > 0);
        assert!(time_ticks_custom.len() <= 15); // Reasonable upper bound
        
        // Test with tick_arguments and potential specifier for time scale
        let time_axis_with_spec = axis_bottom(time_scale.clone())
            .tick_arguments(vec![4.0, 2.0]);
        let time_ticks_with_spec = time_axis_with_spec.ticks();
        
        // Should have approximately 4 ticks
        assert!(time_ticks_with_spec.len() > 0);
        assert!(time_ticks_with_spec.len() <= 12); // Reasonable upper bound
        
        println!("Time axis - Default: {}, Custom (6): {}, With spec (4): {}", 
                default_count, time_ticks_custom.len(), time_ticks_with_spec.len());
    }
}

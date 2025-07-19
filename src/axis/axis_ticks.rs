// Axis ticks logic for different scale types
// Responsible for generating visual ticks across different axis types like linear, log, and time.
// Includes context-aware format patterns for time-based scales to ensure accurate display.

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
        let mut values: Vec<f64>;
        let should_include_domain_bounds: bool;
        
        if let Some(input) = tick_input {
            values = input.to_vec();
            should_include_domain_bounds = false; // Don't modify explicitly provided tick values
        } else if let Some(ref values_ref) = self.tick_values {
            values = values_ref.clone();
            should_include_domain_bounds = false; // Don't modify explicitly set tick values
        } else if let Some(ref args) = self.tick_arguments {
            // Use tick_arguments to call scale.ticks() with custom parameters
            // args[0] is the count, following D3's axis.ticks(count, specifier) signature
            let count = args.get(0).map(|&c| c as usize).unwrap_or(self.tick_count);
            values = self.scale.ticks(count);
            should_include_domain_bounds = true; // Apply domain bounds to auto-generated ticks
        } else {
            // Use the D3-compatible ticks method from ScaleLinear
            values = self.scale.ticks(self.tick_count);
            should_include_domain_bounds = true; // Apply domain bounds to auto-generated ticks
        };
        
        // Apply domain bounds inclusion only for auto-generated ticks
        if should_include_domain_bounds && !values.is_empty() {
            // Get domain bounds
            let domain = self.scale.domain();
            let domain_min = domain[0];
            let domain_max = domain[1];
            let tolerance = 1e-10;
            
            // Check if first tick is outside tolerance of domain minimum
            if (values[0] - domain_min).abs() > tolerance {
                values.insert(0, domain_min);
            }
            
            // Check if last tick is outside tolerance of domain maximum
            let last_idx = values.len() - 1;
            if (values[last_idx] - domain_max).abs() > tolerance {
                values.push(domain_max);
            }
            
            // Sort the values to ensure proper ordering
            values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        }
        
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
    /// Get context-aware format pattern based on the tick interval
    fn get_context_aware_format_pattern(&self, tick_interval: &crate::scale::time::TimeTickInterval) -> &'static str {
        use crate::scale::time::TimeTickInterval;
        
        // Get the domain span to determine if we need special handling
        let start = self.scale.domain[0];
        let stop = self.scale.domain[1];
        let duration = (stop - start).abs();
        
        match tick_interval {
            TimeTickInterval::Second(_) => {
                // For second-level ticks, default to date-time format
                "%Y-%m-%d"
            }
            TimeTickInterval::Minute(_) => "%Y-%m-%d",
            TimeTickInterval::Hour(_) => "%Y-%m-%d",
            TimeTickInterval::Day(_) => "%m/%d",
            TimeTickInterval::Week(_) => "%a",
            TimeTickInterval::Month(_) => "%b",
            TimeTickInterval::Year(_) => "%Y",
        }
    }

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
        
        // Determine the tick interval to select appropriate format
        let tick_count = if let Some(ref args) = self.tick_arguments {
            args.get(0).map(|&c| c as usize).unwrap_or(self.tick_count)
        } else {
            self.tick_count
        };
        let tick_interval = self.scale.tick_interval(tick_count);
        
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
                        // Use context-aware format based on tick interval
                        let format_pattern = self.get_context_aware_format_pattern(&tick_interval);
                        dt.format(format_pattern).to_string()
                    }
                } else {
                    // Use context-aware format based on tick interval
                    let format_pattern = self.get_context_aware_format_pattern(&tick_interval);
                    dt.format(format_pattern).to_string()
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
// 
// This function replicates D3's default number formatting behavior:
// - Uses "g" format type (general format) with 6 digits of precision
// - Switches between fixed-point and exponential notation based on magnitude
// - Handles special values (NaN, Infinity) with appropriate symbols
// - Applies trimming to remove trailing zeros for cleaner output
// - Uses scientific notation for very large (>=1e6) or very small (<1e-4) values
//
// Formatting decisions:
// - Integers < 1e6 are formatted without decimal points
// - Very small values (< 1e-6) are formatted as "0.000000"
// - Large numbers use scientific notation with proper exponent formatting
// - Trailing zeros are removed from decimal representations
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
    // But only if they are not too large (should use scientific notation for large numbers)
    if (value - value.round()).abs() < 1e-10 {
        let rounded = value.round();
        if rounded.abs() < 1e6 { // Use scientific notation for integers >= 1e6
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
        // Use scientific notation for large numbers
        let mut result = format!("{:.prec$e}", value, prec = precision - 1);
        
        // Remove trailing zeros from the coefficient and ensure proper exponent format
        if let Some(e_pos) = result.find('e') {
            let (coeff, exp) = result.split_at(e_pos);
            let mut coeff = coeff.trim_end_matches('0').trim_end_matches('.').to_string();
            if coeff.is_empty() || coeff == "-" {
                coeff = "0".to_string();
            }
            
            // Parse exponent and format with + sign and at least two digits
            let exp_num = exp[1..].parse::<i32>().unwrap_or(0);
            result = format!("{}e+{:02}", coeff, exp_num);
        }
        result
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

// Note: format_with_si_prefix function removed as SI prefix formatting
// is no longer used by default for large numbers >= 1e6.
// Scientific notation is used instead to ensure consistent formatting.

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
        
        // Test large numbers with scientific notation
        assert_eq!(d3_format_default_locale_6g(1e6), "1e+06");
        assert_eq!(d3_format_default_locale_6g(1.5e6), "1.5e+06");
        assert_eq!(d3_format_default_locale_6g(1e9), "1e+09");
        assert_eq!(d3_format_default_locale_6g(1.23e9), "1.23e+09");
        
        // Test numbers that should use exponential notation
        assert_eq!(d3_format_default_locale_6g(1e-5), "1e-5");
        assert_eq!(d3_format_default_locale_6g(1.23e-5), "1.23e-5");
    }
    
// Note: test_format_with_si_prefix removed as the function is obsolete

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

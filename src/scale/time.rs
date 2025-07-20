#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, Datelike};
    use super::*;

    #[test]
    fn test_ticks_time_scale() {
        let start_date = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2020, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap();
        let time_scale = ScaleTime::new([start_date, end_date], [0.0, 500.0]);

        // Test tick count and validity
        let ticks = time_scale.ticks(10);
        assert!(!ticks.is_empty(), "Ticks should not be empty");
        assert!(ticks.len() >= 2, "Should be at least 2 ticks including both domain ends");
        assert_eq!(ticks.first().unwrap(), &start_date, "First tick should match domain start");
        assert_eq!(ticks.last().unwrap(), &end_date, "Last tick should match domain end");
        for i in 1..ticks.len() {
            assert!(ticks[i] > ticks[i - 1], "Ticks should be ordered");
        }

        // Printing for debug, can be removed if needed
        println!("Generated ticks: {:?}", ticks);
    }
    
    #[test]
    fn test_ticks_time_scale_reverse() {
        let start_date = NaiveDate::from_ymd_opt(2020, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap();
        let time_scale = ScaleTime::new([start_date, end_date], [0.0, 500.0]);

        // Test tick count and validity in reverse order
        let ticks = time_scale.ticks(10);
        assert!(!ticks.is_empty(), "Ticks should not be empty");
        assert!(ticks.len() >= 2, "Should be at least 2 ticks including both domain ends");
        assert_eq!(ticks.first().unwrap(), &start_date, "First tick should match domain start in reverse");
        assert_eq!(ticks.last().unwrap(), &end_date, "Last tick should match domain end in reverse");
        for i in 1..ticks.len() {
            assert!(ticks[i] < ticks[i - 1], "Ticks should be ordered reversing");
        }
        
        // Printing for debug, can be removed if needed
        println!("Generated reverse ticks: {:?}", ticks);
    }
    
    #[test]
    fn test_ticks_time_scale_different_counts() {
        let start_date = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2020, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap();
        let time_scale = ScaleTime::new([start_date, end_date], [0.0, 500.0]);

        // Test different tick counts
        let ticks_5 = time_scale.ticks(5);
        let ticks_10 = time_scale.ticks(10);
        let ticks_20 = time_scale.ticks(20);
        
        println!("5 ticks: {:?}", ticks_5);
        println!("10 ticks: {:?}", ticks_10);
        println!("20 ticks: {:?}", ticks_20);
        
        // All should have at least 2 ticks (start and end)
        assert!(ticks_5.len() >= 2, "5 ticks should be at least 2");
        assert!(ticks_10.len() >= 2, "10 ticks should be at least 2");
        assert!(ticks_20.len() >= 2, "20 ticks should be at least 2");
        
        // All should start and end at the same place
        assert_eq!(ticks_5.first().unwrap(), &start_date);
        assert_eq!(ticks_5.last().unwrap(), &end_date);
        assert_eq!(ticks_10.first().unwrap(), &start_date);
        assert_eq!(ticks_10.last().unwrap(), &end_date);
        assert_eq!(ticks_20.first().unwrap(), &start_date);
        assert_eq!(ticks_20.last().unwrap(), &end_date);
        
        // Check that tick counts are reasonable (not exact since D3 uses approximate counts)
        assert!(ticks_5.len() <= 15, "5 ticks should not exceed 15");
        assert!(ticks_10.len() <= 20, "10 ticks should not exceed 20");
        assert!(ticks_20.len() <= 35, "20 ticks should not exceed 35");
    }
    
    #[test]
    fn test_ticks_time_scale_short_span() {
        let start_date = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2020, 1, 2).unwrap().and_hms_opt(0, 0, 0).unwrap();
        let time_scale = ScaleTime::new([start_date, end_date], [0.0, 500.0]);

        // Test short time span (1 day)
        let ticks = time_scale.ticks(10);
        println!("Short span ticks: {:?}", ticks);
        
        assert!(!ticks.is_empty(), "Short span should still have ticks");
        assert!(ticks.len() >= 2, "Should be at least 2 ticks");
        assert_eq!(ticks.first().unwrap(), &start_date);
        assert_eq!(ticks.last().unwrap(), &end_date);
    }
    
    #[test]
    fn test_ticks_time_scale_long_span() {
        let start_date = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2020, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap();
        let time_scale = ScaleTime::new([start_date, end_date], [0.0, 500.0]);

        // Test long time span (20 years)
        let ticks = time_scale.ticks(10);
        println!("Long span ticks: {:?}", ticks);
        
        assert!(!ticks.is_empty(), "Long span should have ticks");
        assert!(ticks.len() >= 2, "Should be at least 2 ticks");
        assert_eq!(ticks.first().unwrap(), &start_date);
        assert_eq!(ticks.last().unwrap(), &end_date);
        
        // For 20 years, we should get yearly ticks
        // The exact number depends on the algorithm but should be reasonable
        assert!(ticks.len() <= 30, "Long span should not have too many ticks");
    }
    
    #[test]
    fn test_ticks_within_domain_bounds() {
        let start_date = NaiveDate::from_ymd_opt(2007, 4, 23).unwrap().and_hms_opt(0, 0, 0).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2012, 5, 1).unwrap().and_hms_opt(0, 0, 0).unwrap();
        let time_scale = ScaleTime::new([start_date, end_date], [40.0, 898.0]);

        let ticks = time_scale.ticks(11);
        
        // All ticks should be within the domain bounds
        for tick in &ticks {
            assert!(*tick >= start_date && *tick <= end_date, 
                   "Tick {:?} is outside domain [{:?}, {:?}]", tick, start_date, end_date);
        }
        
        // Check that scaled positions are within range bounds
        for tick in &ticks {
            let scaled_pos = time_scale.scale(*tick);
            assert!(scaled_pos >= 40.0 && scaled_pos <= 898.0, 
                   "Scaled position {} for tick {:?} is outside range [40.0, 898.0]", 
                   scaled_pos, tick);
        }
        
        println!("Tick positions within domain test passed for {} ticks", ticks.len());
    }
    
    #[test]
    fn test_tick_spacing_consistency() {
        let start_date = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap();
        let time_scale = ScaleTime::new([start_date, end_date], [0.0, 500.0]);

        let ticks = time_scale.ticks(6);
        
        // For yearly intervals, spacing should be consistent
        if ticks.len() > 2 {
            let first_gap = ticks[1].signed_duration_since(ticks[0]);
            
            // Check that all gaps are approximately equal (within reasonable tolerance)
            for i in 2..ticks.len() {
                let gap = ticks[i].signed_duration_since(ticks[i-1]);
                let gap_diff = (gap - first_gap).num_days().abs();
                assert!(gap_diff <= 31, // Allow up to 31 days difference for month variations
                       "Inconsistent spacing: gap {} differs from first gap {} by {} days", 
                       gap, first_gap, gap_diff);
            }
        }
        
        println!("Tick spacing consistency test passed for {} ticks", ticks.len());
    }
    
    #[test]
    fn test_monthly_ticks_alignment() {
        let start_date = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2020, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap();
        let time_scale = ScaleTime::new([start_date, end_date], [0.0, 500.0]);

        let ticks = time_scale.ticks(12);
        
        // Check that monthly ticks are aligned to month boundaries
        for tick in &ticks {
            // Monthly ticks should typically be at the start of months
            // (except for the explicit start/end dates)
            if *tick != start_date && *tick != end_date {
                // Allow some flexibility, but day should be 1 for most monthly ticks
                assert!(tick.day() <= 2, 
                       "Monthly tick {:?} is not aligned to month start (day {})", 
                       tick, tick.day());
            }
        }
        
        println!("Monthly tick alignment test passed for {} ticks", ticks.len());
    }
    
    #[test]
    fn test_chart_ui_scenario() {
        // Test the exact scenario from chart_ui that was problematic
        let start_date = NaiveDate::from_ymd_opt(2007, 4, 23).unwrap().and_hms_opt(0, 0, 0).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2012, 5, 1).unwrap().and_hms_opt(0, 0, 0).unwrap();
        let time_scale = ScaleTime::new([start_date, end_date], [40.0, 898.0]);

        let ticks = time_scale.ticks(11);
        
        // Verify first and last ticks
        assert_eq!(ticks.first().unwrap(), &start_date, "First tick should be domain start");
        assert_eq!(ticks.last().unwrap(), &end_date, "Last tick should be domain end");
        
        // All ticks should be within domain
        for tick in &ticks {
            assert!(*tick >= start_date && *tick <= end_date, 
                   "Tick {:?} outside domain", tick);
        }
        
        // Check scaled positions
        for tick in &ticks {
            let pos = time_scale.scale(*tick);
            assert!(pos >= 40.0 && pos <= 898.0, 
                   "Scaled position {} outside range [40.0, 898.0] for tick {:?}", 
                   pos, tick);
        }
        
        // Verify tick count is reasonable
        assert!(ticks.len() >= 5 && ticks.len() <= 15, 
               "Tick count {} should be between 5 and 15 for requested count 11", 
               ticks.len());
        
        println!("Chart UI scenario test passed: {} ticks generated", ticks.len());
        for (i, tick) in ticks.iter().enumerate() {
            println!("  Tick {}: {:?} -> position {:.1}", i, tick, time_scale.scale(*tick));
        }
    }
    
    #[test]
    fn test_tick_ordering_and_uniqueness() {
        let start_date = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2025, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap();
        let time_scale = ScaleTime::new([start_date, end_date], [0.0, 1000.0]);

        let ticks = time_scale.ticks(8);
        
        // Check ordering
        for i in 1..ticks.len() {
            assert!(ticks[i] > ticks[i-1], 
                   "Ticks not in ascending order: {:?} > {:?}", 
                   ticks[i-1], ticks[i]);
        }
        
        // Check uniqueness
        for i in 0..ticks.len() {
            for j in i+1..ticks.len() {
                assert!(ticks[i] != ticks[j], 
                       "Duplicate tick found: {:?} appears at positions {} and {}", 
                       ticks[i], i, j);
            }
        }
        
        println!("Tick ordering and uniqueness test passed for {} ticks", ticks.len());
    }
    
    #[test]
    fn test_edge_case_single_day() {
        let start_date = NaiveDate::from_ymd_opt(2020, 6, 15).unwrap().and_hms_opt(0, 0, 0).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2020, 6, 15).unwrap().and_hms_opt(23, 59, 59).unwrap();
        let time_scale = ScaleTime::new([start_date, end_date], [0.0, 100.0]);

        let ticks = time_scale.ticks(5);
        
        // Should have at least the start and end
        assert!(ticks.len() >= 2, "Should have at least 2 ticks for single day");
        assert_eq!(ticks.first().unwrap(), &start_date);
        assert_eq!(ticks.last().unwrap(), &end_date);
        
        // All ticks should be within the same day
        for tick in &ticks {
            assert!(tick.date() == start_date.date(), 
                   "Tick {:?} is not on the same day as domain", tick);
        }
        
        println!("Single day edge case test passed for {} ticks", ticks.len());
    }
}

// d3-scale: ScaleTime
use crate::time::{Day, Hour, Minute, Month, Second, TimeInterval, Week, Year, utc_format};
use chrono::{Datelike, Duration, NaiveDateTime};
#[derive(Debug, Clone)]
pub struct ScaleTime {
    pub domain: [NaiveDateTime; 2],
    pub range: [f64; 2],
    pub clamp: bool,
    pub interpolate: fn(f64, f64, f64) -> f64,
    pub unknown: Option<f64>,
}

impl ScaleTime {
    pub fn new(domain: [NaiveDateTime; 2], range: [f64; 2]) -> Self {
        Self {
            domain,
            range,
            clamp: false,
            interpolate: linear_interpolate,
            unknown: None,
        }
    }

    pub fn scale(&self, x: NaiveDateTime) -> f64 {
        let mut x_millis = x.and_utc().timestamp_millis() as f64;
        let domain_start = self.domain[0].and_utc().timestamp_millis() as f64;
        let domain_end = self.domain[1].and_utc().timestamp_millis() as f64;

        if self.clamp {
            x_millis = x_millis.max(domain_start).min(domain_end);
        }

        let t = (x_millis - domain_start) / (domain_end - domain_start);
        (self.interpolate)(self.range[0], self.range[1], t)
    }

    pub fn invert(&self, y: f64) -> NaiveDateTime {
        let t = (y - self.range[0]) / (self.range[1] - self.range[0]);
        let domain_start = self.domain[0].and_utc().timestamp_millis() as f64;
        let domain_end = self.domain[1].and_utc().timestamp_millis() as f64;

        let millis = domain_start + t * (domain_end - domain_start);
        let mut result_millis = millis;

        if self.clamp {
            result_millis = result_millis.max(domain_start).min(domain_end);
        }

        chrono::DateTime::<chrono::Utc>::from_timestamp(
            (result_millis / 1000.0) as i64,
            ((result_millis % 1000.0) * 1_000_000.0) as u32,
        )
        .unwrap_or_else(|| self.domain[0].and_utc())
        .naive_utc()
    }

    pub fn domain(&self) -> [NaiveDateTime; 2] {
        self.domain
    }

    pub fn range(&self) -> [f64; 2] {
        self.range
    }

    pub fn clamp(mut self, clamp: bool) -> Self {
        self.clamp = clamp;
        self
    }

    pub fn interpolate(mut self, interpolate: fn(f64, f64, f64) -> f64) -> Self {
        self.interpolate = interpolate;
        self
    }

    pub fn unknown(mut self, value: f64) -> Self {
        self.unknown = Some(value);
        self
    }

    pub fn ticks(&self, count: usize) -> Vec<NaiveDateTime> {
        let start = self.domain[0];
        let stop = self.domain[1];

        // Handle reverse case
        let reverse = stop < start;
        let (start, stop) = if reverse { (stop, start) } else { (start, stop) };

        // Choose appropriate time interval based on domain span
        let interval = self.tick_interval(count);

        // Generate ticks with domain-aware boundaries
        let mut ticks = match interval {
            TimeTickInterval::Second(step) => {
                let sec = Second;
                self.generate_domain_aware_ticks(sec, start, stop, step as i32)
            }
            TimeTickInterval::Minute(step) => {
                let min = Minute;
                self.generate_domain_aware_ticks(min, start, stop, step as i32)
            }
            TimeTickInterval::Hour(step) => {
                let hour = Hour;
                self.generate_domain_aware_ticks(hour, start, stop, step as i32)
            }
            TimeTickInterval::Day(step) => {
                let day = Day;
                self.generate_domain_aware_ticks(day, start, stop, step as i32)
            }
            TimeTickInterval::Week(step) => {
                let week = Week;
                self.generate_domain_aware_ticks(week, start, stop, step as i32)
            }
            TimeTickInterval::Month(step) => {
                let month = Month;
                self.generate_domain_aware_ticks(month, start, stop, step as i32)
            }
            TimeTickInterval::Year(step) => {
                let year = Year;
                self.generate_domain_aware_ticks(year, start, stop, step as i32)
            }
        };

        // Return in original order
        if reverse {
            ticks.reverse();
        }
        ticks
    }

    pub fn tick_interval(&self, count: usize) -> TimeTickInterval {
        let start = self.domain[0];
        let stop = self.domain[1];
        
        // Handle reverse case for calculations
        let (start_calc, stop_calc) = if stop < start { (stop, start) } else { (start, stop) };
        
        // D3 tick intervals in milliseconds (matching D3's duration.js)
        const DURATION_SECOND: i64 = 1000;
        const DURATION_MINUTE: i64 = DURATION_SECOND * 60;
        const DURATION_HOUR: i64 = DURATION_MINUTE * 60;
        const DURATION_DAY: i64 = DURATION_HOUR * 24;
        const DURATION_WEEK: i64 = DURATION_DAY * 7;
        const DURATION_MONTH: i64 = DURATION_DAY * 30;
        const DURATION_YEAR: i64 = DURATION_DAY * 365;
        
        // D3 tick intervals array (matching D3's ticks.js)
        let tick_intervals = vec![
            (TimeTickInterval::Second(1), 1, DURATION_SECOND),
            (TimeTickInterval::Second(5), 5, 5 * DURATION_SECOND),
            (TimeTickInterval::Second(15), 15, 15 * DURATION_SECOND),
            (TimeTickInterval::Second(30), 30, 30 * DURATION_SECOND),
            (TimeTickInterval::Minute(1), 1, DURATION_MINUTE),
            (TimeTickInterval::Minute(5), 5, 5 * DURATION_MINUTE),
            (TimeTickInterval::Minute(15), 15, 15 * DURATION_MINUTE),
            (TimeTickInterval::Minute(30), 30, 30 * DURATION_MINUTE),
            (TimeTickInterval::Hour(1), 1, DURATION_HOUR),
            (TimeTickInterval::Hour(2), 2, 2 * DURATION_HOUR),
            (TimeTickInterval::Hour(3), 3, 3 * DURATION_HOUR),
            (TimeTickInterval::Hour(6), 6, 6 * DURATION_HOUR),
            (TimeTickInterval::Hour(12), 12, 12 * DURATION_HOUR),
            (TimeTickInterval::Day(1), 1, DURATION_DAY),
            (TimeTickInterval::Day(2), 2, 2 * DURATION_DAY),
            (TimeTickInterval::Week(1), 1, DURATION_WEEK),
            (TimeTickInterval::Month(1), 1, DURATION_MONTH),
            (TimeTickInterval::Month(3), 3, 3 * DURATION_MONTH),
            (TimeTickInterval::Year(1), 1, DURATION_YEAR),
        ];

        let start_millis = start_calc.and_utc().timestamp_millis();
        let stop_millis = stop_calc.and_utc().timestamp_millis();
        let _target = (stop_millis - start_millis).abs() / count as i64;

        // For each interval, calculate how many ticks it would actually produce
        let mut best_interval = TimeTickInterval::Year(1);
        let mut best_score = f64::INFINITY;
        
        for &(ref interval, _step, _duration) in &tick_intervals {
            // Calculate actual tick count for this interval
            let actual_ticks = self.count_ticks_for_interval(start_calc, stop_calc, interval);
            
            // Score based on how close the actual tick count is to the requested count
            let score = ((actual_ticks as f64) - (count as f64)).abs();
            
            if score < best_score {
                best_score = score;
                best_interval = interval.clone();
            }
        }
        
        // Also consider year intervals with different steps
        for year_step in 1..=10 {
            let year_interval = TimeTickInterval::Year(year_step);
            let actual_ticks = self.count_ticks_for_interval(start_calc, stop_calc, &year_interval);
            let score = ((actual_ticks as f64) - (count as f64)).abs();
            
            if score < best_score {
                best_score = score;
                best_interval = year_interval;
            }
        }
        best_interval
    }
    
    fn count_ticks_for_interval(&self, start: chrono::NaiveDateTime, stop: chrono::NaiveDateTime, interval: &TimeTickInterval) -> usize {
        // Calculate tick count without generating full range to avoid hanging
        let count = match interval {
            TimeTickInterval::Second(step) => {
                let duration_secs = (stop - start).num_seconds().abs();
                (duration_secs / (*step as i64)).max(1) as usize + 1 // +1 for domain bounds
            }
            TimeTickInterval::Minute(step) => {
                let duration_mins = (stop - start).num_minutes().abs();
                (duration_mins / (*step as i64)).max(1) as usize + 1
            }
            TimeTickInterval::Hour(step) => {
                let duration_hours = (stop - start).num_hours().abs();
                (duration_hours / (*step as i64)).max(1) as usize + 1
            }
            TimeTickInterval::Day(step) => {
                let duration_days = (stop - start).num_days().abs();
                (duration_days / (*step as i64)).max(1) as usize + 1
            }
            TimeTickInterval::Week(step) => {
                let duration_weeks = (stop - start).num_weeks().abs();
                (duration_weeks / (*step as i64)).max(1) as usize + 1
            }
            TimeTickInterval::Month(step) => {
                let start_months = start.year() * 12 + start.month() as i32;
                let stop_months = stop.year() * 12 + stop.month() as i32;
                let duration_months = (stop_months - start_months).abs();
                (duration_months / (*step as i32)).max(1) as usize + 1
            }
            TimeTickInterval::Year(step) => {
                let duration_years = (stop.year() - start.year()).abs();
                (duration_years / (*step as i32)).max(1) as usize + 1
            }
        };
        
        count
    }
    
    fn generate_domain_aware_ticks<T: TimeInterval>(&self, interval: T, start: chrono::NaiveDateTime, stop: chrono::NaiveDateTime, step: i32) -> Vec<chrono::NaiveDateTime> {
        let mut ticks = Vec::new();
        
        // Always start with the domain start
        ticks.push(start);
        
        // Find the next interval boundary after the start
        let mut current = interval.floor(start);
        if current < start {
            current = interval.offset(current, step);
        }
        
        // Generate intermediate ticks with safeguard against infinite loops
        let mut iteration_count = 0;
        const MAX_ITERATIONS: i32 = 10000; // Safety limit
        
        while current < stop && iteration_count < MAX_ITERATIONS {
            if current > start {
                ticks.push(current);
            }
            let next_current = interval.offset(current, step);
            
            // Safety check: if offset doesn't advance, break to avoid infinite loop
            if next_current <= current {
                break;
            }
            
            current = next_current;
            iteration_count += 1;
        }
        
        // Always end with the domain stop (if not already added)
        if ticks.last() != Some(&stop) {
            ticks.push(stop);
        }
        
        ticks
    }


    pub fn tick_format(
        &self,
        count: usize,
        specifier: Option<&str>,
    ) -> impl Fn(&NaiveDateTime) -> String {
        let spec = specifier.unwrap_or(self.default_format_specifier(count));
        let spec = spec.to_string();

        move |d| utc_format(&spec, d)
    }

    fn default_format_specifier(&self, count: usize) -> &str {
        let interval = self.tick_interval(count);
        
        // Check if this is a very short span (less than 10 seconds) - use date format
        let _start = self.domain[0];
        let _stop = self.domain[1];
        
        match interval {
            TimeTickInterval::Second(_) => "%Y-%m-%d",
            TimeTickInterval::Minute(_) => "%Y-%m-%d",
            TimeTickInterval::Hour(_) => "%Y-%m-%d",
            TimeTickInterval::Day(_) => "%m/%d",
            TimeTickInterval::Week(_) => "%a",
            TimeTickInterval::Month(_) => "%b",
            TimeTickInterval::Year(_) => "%Y",
        }
    }

    pub fn nice(&mut self, count: Option<usize>) {
        let count = count.unwrap_or(10);
        let interval = self.tick_interval(count);

        match interval {
            TimeTickInterval::Second(_) => {
                let sec = Second;
                self.domain[0] = sec.floor(self.domain[0]);
                self.domain[1] = sec.ceil(self.domain[1]);
            }
            TimeTickInterval::Minute(_) => {
                let min = Minute;
                self.domain[0] = min.floor(self.domain[0]);
                self.domain[1] = min.ceil(self.domain[1]);
            }
            TimeTickInterval::Hour(_) => {
                let hour = Hour;
                self.domain[0] = hour.floor(self.domain[0]);
                self.domain[1] = hour.ceil(self.domain[1]);
            }
            TimeTickInterval::Day(_) => {
                let day = Day;
                self.domain[0] = day.floor(self.domain[0]);
                self.domain[1] = day.ceil(self.domain[1]);
            }
            TimeTickInterval::Week(_) => {
                let week = Week;
                self.domain[0] = week.floor(self.domain[0]);
                self.domain[1] = week.ceil(self.domain[1]);
            }
            TimeTickInterval::Month(_) => {
                let month = Month;
                self.domain[0] = month.floor(self.domain[0]);
                self.domain[1] = month.ceil(self.domain[1]);
            }
            TimeTickInterval::Year(_) => {
                let year = Year;
                self.domain[0] = year.floor(self.domain[0]);
                self.domain[1] = year.ceil(self.domain[1]);
            }
        }
    }

    pub fn copy(&self) -> Self {
        Self {
            domain: self.domain,
            range: self.range,
            clamp: self.clamp,
            interpolate: self.interpolate,
            unknown: self.unknown,
        }
    }

    pub fn range_round(mut self, range: [f64; 2]) -> Self {
        self.range = range;
        self.interpolate = round_interpolate;
        self
    }
}

#[derive(Debug, Clone)]
pub enum TimeTickInterval {
    Second(i64),
    Minute(i64),
    Hour(i64),
    Day(i64),
    Week(i64),
    Month(i64),
    Year(i64),
}

// Default linear interpolation
fn linear_interpolate(a: f64, b: f64, t: f64) -> f64 {
    a + t * (b - a)
}

// Rounding interpolation
fn round_interpolate(a: f64, b: f64, t: f64) -> f64 {
    (a + t * (b - a)).round()
}

impl Default for ScaleTime {
    fn default() -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self::new([now, now + Duration::try_hours(1).unwrap()], [0.0, 1.0])
    }
}

// Implementation for axis rendering
impl crate::axis::axis_common::TimeScaleForAxis for ScaleTime {
    fn scale_timestamp(&self, timestamp_millis: f64) -> f64 {
        let domain_start = self.domain[0].and_utc().timestamp_millis() as f64;
        let domain_end = self.domain[1].and_utc().timestamp_millis() as f64;
        
        let mut millis = timestamp_millis;
        if self.clamp {
            millis = millis.max(domain_start).min(domain_end);
        }
        
        let t = (millis - domain_start) / (domain_end - domain_start);
        (self.interpolate)(self.range[0], self.range[1], t)
    }
}

impl crate::axis::axis_common::ScaleWithRange for ScaleTime {
    fn range(&self) -> [f64; 2] {
        self.range()
    }
}

// d3-scale: ScaleTime
use crate::time::{Day, Hour, Minute, Month, Second, TimeInterval, Week, Year, utc_format};
use chrono::{Duration, NaiveDateTime};

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

        // Generate ticks with inclusive stop (D3 behavior)
        let mut ticks = match interval {
            TimeTickInterval::Second(step) => {
                let sec = Second;
                let mut result = sec.range(start, stop, step as i32);
                // Add inclusive stop if not already present
                if !result.is_empty() && result.last().unwrap() < &stop {
                    result.push(stop);
                }
                result
            }
            TimeTickInterval::Minute(step) => {
                let min = Minute;
                let mut result = min.range(start, stop, step as i32);
                if !result.is_empty() && result.last().unwrap() < &stop {
                    result.push(stop);
                }
                result
            }
            TimeTickInterval::Hour(step) => {
                let hour = Hour;
                let mut result = hour.range(start, stop, step as i32);
                if !result.is_empty() && result.last().unwrap() < &stop {
                    result.push(stop);
                }
                result
            }
            TimeTickInterval::Day(step) => {
                let day = Day;
                let mut result = day.range(start, stop, step as i32);
                if !result.is_empty() && result.last().unwrap() < &stop {
                    result.push(stop);
                }
                result
            }
            TimeTickInterval::Week(step) => {
                let week = Week;
                let mut result = week.range(start, stop, step as i32);
                if !result.is_empty() && result.last().unwrap() < &stop {
                    result.push(stop);
                }
                result
            }
            TimeTickInterval::Month(step) => {
                let month = Month;
                let mut result = month.range(start, stop, step as i32);
                if !result.is_empty() && result.last().unwrap() < &stop {
                    result.push(stop);
                }
                result
            }
            TimeTickInterval::Year(step) => {
                let year = Year;
                let mut result = year.range(start, stop, step as i32);
                if !result.is_empty() && result.last().unwrap() < &stop {
                    result.push(stop);
                }
                result
            }
        };

        // Return in original order
        if reverse {
            ticks.reverse();
        }
        ticks
    }

    fn tick_interval(&self, count: usize) -> TimeTickInterval {
        let start = self.domain[0];
        let stop = self.domain[1];
        
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

        let start_millis = start.and_utc().timestamp_millis();
        let stop_millis = stop.and_utc().timestamp_millis();
        let target = (stop_millis - start_millis).abs() / count as i64;

        // Binary search for the appropriate interval (matching D3's bisector logic)
        let mut i = 0;
        while i < tick_intervals.len() && tick_intervals[i].2 < target {
            i += 1;
        }

        if i == tick_intervals.len() {
            // Use years with tick_step logic
            let years = Self::tick_step(start_millis / DURATION_YEAR, stop_millis / DURATION_YEAR, count);
            return TimeTickInterval::Year(years.max(1));
        }

        if i == 0 {
            // Use milliseconds/seconds with tick_step logic
            let step = Self::tick_step(start_millis, stop_millis, count).max(1);
            return TimeTickInterval::Second(step / 1000);
        }

        // Choose between current and previous interval based on proximity
        let current_step = tick_intervals[i].2;
        let prev_step = tick_intervals[i - 1].2;
        
        if (target as f64) / (prev_step as f64) < (current_step as f64) / (target as f64) {
            tick_intervals[i - 1].0.clone()
        } else {
            tick_intervals[i].0.clone()
        }
    }

    fn tick_step(start: i64, stop: i64, count: usize) -> i64 {
        let step = (stop - start) / count.max(1) as i64;
        let power = (step as f64).log10().floor() as i32;
        let error = step as f64 / 10.0_f64.powi(power);
        
        let factor = if error >= 50.0_f64.sqrt() {
            10.0
        } else if error >= 10.0_f64.sqrt() {
            5.0
        } else if error >= 2.0_f64.sqrt() {
            2.0
        } else {
            1.0
        };
        
        (10.0_f64.powi(power) * factor) as i64
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

        match interval {
            TimeTickInterval::Second(_) => "%H:%M:%S",
            TimeTickInterval::Minute(_) => "%H:%M",
            TimeTickInterval::Hour(_) => "%H:%M",
            TimeTickInterval::Day(_) => "%m/%d",
            TimeTickInterval::Week(_) => "%m/%d",
            TimeTickInterval::Month(_) => "%Y-%m",
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
enum TimeTickInterval {
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

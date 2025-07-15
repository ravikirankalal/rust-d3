// d3-scale: ScaleTime
use chrono::{NaiveDateTime, Duration};
use crate::time::{
    TimeInterval, Second, Minute, Hour, Day, Week, Month, Year,
    utc_format
};

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
            ((result_millis % 1000.0) * 1_000_000.0) as u32
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
        
        // Choose appropriate time interval based on domain span
        let interval = self.tick_interval(count);
        
        match interval {
            TimeTickInterval::Second(step) => {
                let sec = Second;
                if step == 1 {
                    sec.range(start, stop, 1)
                } else {
                    sec.range(start, stop, step as i32)
                }
            },
            TimeTickInterval::Minute(step) => {
                let min = Minute;
                min.range(start, stop, step as i32)
            },
            TimeTickInterval::Hour(step) => {
                let hour = Hour;
                hour.range(start, stop, step as i32)
            },
            TimeTickInterval::Day(step) => {
                let day = Day;
                day.range(start, stop, step as i32)
            },
            TimeTickInterval::Week(step) => {
                let week = Week;
                week.range(start, stop, step as i32)
            },
            TimeTickInterval::Month(step) => {
                let month = Month;
                month.range(start, stop, step as i32)
            },
            TimeTickInterval::Year(step) => {
                let year = Year;
                year.range(start, stop, step as i32)
            },
        }
    }
    
    fn tick_interval(&self, count: usize) -> TimeTickInterval {
        let start = self.domain[0];
        let stop = self.domain[1];
        let duration = stop.signed_duration_since(start);
        
        let total_seconds = duration.num_seconds();
        let target_interval = total_seconds / count as i64;
        
        // Choose appropriate interval based on target interval
        if target_interval < 1 {
            TimeTickInterval::Second(1)
        } else if target_interval < 60 {
            TimeTickInterval::Second(Self::nice_step(target_interval, &[1, 5, 15, 30]))
        } else if target_interval < 3600 {
            let minutes = target_interval / 60;
            TimeTickInterval::Minute(Self::nice_step(minutes, &[1, 5, 15, 30]))
        } else if target_interval < 86400 {
            let hours = target_interval / 3600;
            TimeTickInterval::Hour(Self::nice_step(hours, &[1, 3, 6, 12]))
        } else if target_interval < 604800 {
            let days = target_interval / 86400;
            TimeTickInterval::Day(Self::nice_step(days, &[1, 2, 3, 7]))
        } else if target_interval < 2419200 {
            let weeks = target_interval / 604800;
            TimeTickInterval::Week(Self::nice_step(weeks, &[1, 2, 4]))
        } else if target_interval < 31536000 {
            let months = target_interval / 2419200;
            TimeTickInterval::Month(Self::nice_step(months, &[1, 2, 3, 6]))
        } else {
            let years = target_interval / 31536000;
            TimeTickInterval::Year(Self::nice_step(years, &[1, 2, 5, 10]))
        }
    }
    
    fn nice_step(target: i64, steps: &[i64]) -> i64 {
        steps.iter()
            .find(|&&step| step >= target)
            .copied()
            .unwrap_or(*steps.last().unwrap())
    }
    
    pub fn tick_format(&self, count: usize, specifier: Option<&str>) -> impl Fn(&NaiveDateTime) -> String {
        let spec = specifier.unwrap_or(self.default_format_specifier(count));
        let spec = spec.to_string();
        
        move |d| {
            utc_format(&spec, d)
        }
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
            },
            TimeTickInterval::Minute(_) => {
                let min = Minute;
                self.domain[0] = min.floor(self.domain[0]);
                self.domain[1] = min.ceil(self.domain[1]);
            },
            TimeTickInterval::Hour(_) => {
                let hour = Hour;
                self.domain[0] = hour.floor(self.domain[0]);
                self.domain[1] = hour.ceil(self.domain[1]);
            },
            TimeTickInterval::Day(_) => {
                let day = Day;
                self.domain[0] = day.floor(self.domain[0]);
                self.domain[1] = day.ceil(self.domain[1]);
            },
            TimeTickInterval::Week(_) => {
                let week = Week;
                self.domain[0] = week.floor(self.domain[0]);
                self.domain[1] = week.ceil(self.domain[1]);
            },
            TimeTickInterval::Month(_) => {
                let month = Month;
                self.domain[0] = month.floor(self.domain[0]);
                self.domain[1] = month.ceil(self.domain[1]);
            },
            TimeTickInterval::Year(_) => {
                let year = Year;
                self.domain[0] = year.floor(self.domain[0]);
                self.domain[1] = year.ceil(self.domain[1]);
            },
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

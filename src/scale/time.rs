// Time scale (D3.js scaleTime) for Rust D3
use chrono::{DateTime, Utc, TimeZone};

pub struct TimeScale {
    domain: (DateTime<Utc>, DateTime<Utc>),
    range: (f64, f64),
}

impl TimeScale {
    pub fn new(domain: (DateTime<Utc>, DateTime<Utc>), range: (f64, f64)) -> Self {
        Self { domain, range }
    }
    pub fn scale(&self, value: DateTime<Utc>) -> f64 {
        let (d0, d1) = self.domain;
        let (r0, r1) = self.range;
        let total = d1.timestamp_millis() - d0.timestamp_millis();
        let val = value.timestamp_millis() - d0.timestamp_millis();
        if total == 0 { return r0; }
        r0 + (val as f64) * (r1 - r0) / (total as f64)
    }
    pub fn domain(&self) -> (DateTime<Utc>, DateTime<Utc>) {
        self.domain
    }
    pub fn range(&self) -> (f64, f64) {
        self.range
    }
    pub fn set_domain(&mut self, domain: (DateTime<Utc>, DateTime<Utc>)) {
        self.domain = domain;
    }
    pub fn set_range(&mut self, range: (f64, f64)) {
        self.range = range;
    }
    pub fn invert(&self, value: f64) -> DateTime<Utc> {
        let (d0, d1) = self.domain;
        let (r0, r1) = self.range;
        if r1 == r0 {
            return d0;
        }
        let total = d1.timestamp_millis() - d0.timestamp_millis();
        let t = (value - r0) / (r1 - r0);
        let millis = d0.timestamp_millis() + (t * total as f64) as i64;
        match Utc.timestamp_millis_opt(millis) {
            chrono::LocalResult::Single(dt) => dt,
            _ => d0,
        }
    }
    pub fn clamp(&mut self) {
        let (d0, d1) = self.domain;
        let (r0, r1) = self.range;
        let min_d = if d0 < d1 { d0 } else { d1 };
        let max_d = if d0 > d1 { d0 } else { d1 };
        let min_r = r0.min(r1);
        let max_r = r0.max(r1);
        self.domain = (min_d, max_d);
        self.range = (min_r, max_r);
    }
    pub fn nice(&mut self) {
        // Simple nice: round to nearest day
        let (d0, d1) = self.domain;
        let d0_nice = Utc.from_utc_datetime(&d0.date_naive().and_hms_opt(0, 0, 0).unwrap());
        let d1_nice = Utc.from_utc_datetime(&d1.date_naive().and_hms_opt(23, 59, 59).unwrap());
        self.domain = (d0_nice, d1_nice);
    }
    pub fn ticks(&self, count: usize) -> Vec<DateTime<Utc>> {
        let (d0, d1) = self.domain;
        let total = d1.timestamp_millis() - d0.timestamp_millis();
        if count < 2 || total <= 0 { return vec![d0, d1]; }
        let step = total as f64 / (count as f64 - 1.0);
        (0..count)
            .map(|i| {
                let millis = d0.timestamp_millis() + (i as f64 * step) as i64;
                match Utc.timestamp_millis_opt(millis) {
                    chrono::LocalResult::Single(dt) => dt,
                    _ => d0,
                }
            })
            .collect()
    }
    pub fn tick_format(&self, fmt: &str) -> impl Fn(DateTime<Utc>) -> String {
        let fmt = fmt.to_string();
        move |dt| dt.format(&fmt).to_string()
    }
}

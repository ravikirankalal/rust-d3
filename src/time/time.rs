use chrono::{NaiveDateTime, NaiveDate, Duration, Datelike};

pub struct TimeScale {
    domain: (NaiveDateTime, NaiveDateTime),
    range: (f64, f64),
}

impl TimeScale {
    pub fn new(domain: (NaiveDateTime, NaiveDateTime), range: (f64, f64)) -> Self {
        Self { domain, range }
    }

    pub fn scale(&self, value: NaiveDateTime) -> f64 {
        let (d0, d1) = self.domain;
        let (r0, r1) = self.range;
        let total = d1.signed_duration_since(d0).num_seconds() as f64;
        let part = value.signed_duration_since(d0).num_seconds() as f64;
        if total == 0.0 {
            r0
        } else {
            r0 + (part / total) * (r1 - r0)
        }
    }
}

pub fn format_time(dt: NaiveDateTime, fmt: &str) -> String {
    dt.format(fmt).to_string()
}

/// Placeholder for d3-time API parity.
/// See: https://github.com/d3/d3-time#api-reference
/// TODO: Implement full API parity with d3-time (timeInterval, timeDay, timeWeek, timeMonth, timeYear, etc.)
pub fn time_day(start: NaiveDateTime, end: NaiveDateTime) -> Vec<NaiveDateTime> {
    let mut days = Vec::new();
    let mut current = start;
    while current <= end {
        days.push(current);
        current += Duration::days(1);
    }
    days
}

pub fn time_week(start: NaiveDateTime, end: NaiveDateTime) -> Vec<NaiveDateTime> {
    let mut weeks = Vec::new();
    let mut current = start;
    while current <= end {
        weeks.push(current);
        current += Duration::weeks(1);
    }
    weeks
}

pub fn time_month(start: NaiveDateTime, end: NaiveDateTime) -> Vec<NaiveDateTime> {
    let mut months = Vec::new();
    let mut current = start;
    while current <= end {
        months.push(current);
        let (y, m) = (current.date().year(), current.date().month());
        let next_month = if m == 12 {
            NaiveDate::from_ymd_opt(y + 1, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
        } else {
            NaiveDate::from_ymd_opt(y, m + 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
        };
        current = next_month;
    }
    months
}

pub fn time_year(start: NaiveDateTime, end: NaiveDateTime) -> Vec<NaiveDateTime> {
    let mut years = Vec::new();
    let mut current = start;
    while current <= end {
        years.push(current);
        let next_year = NaiveDate::from_ymd_opt(current.date().year() + 1, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        current = next_year;
    }
    years
}

mod time;
pub use time::{TimeScale, format_time, time_day, time_week, time_month, time_year};

// --- Advanced time utilities (from time_adv) ---
use chrono::{NaiveDateTime, Duration};

/// D3.js: d3.timeTicks
pub fn time_ticks(start: NaiveDateTime, end: NaiveDateTime, count: usize) -> Vec<NaiveDateTime> {
    let total = end.signed_duration_since(start).num_seconds();
    if count == 0 || total <= 0 {
        return vec![];
    }
    let step = total / count as i64;
    (0..=count)
        .map(|i| start + Duration::seconds(i as i64 * step))
        .collect()
}

/// D3.js: d3.timeInterval.every (stub)
pub fn time_interval_every_placeholder() -> &'static str {
    "timeInterval.every not implemented"
}

/// Checks if a year is a leap year.
pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

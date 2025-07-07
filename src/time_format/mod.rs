// D3 time format module for Rust D3
// Provides date/time formatting utilities similar to d3-time-format.

use chrono::{NaiveDateTime, DateTime, Utc};

pub fn format_time(dt: NaiveDateTime, fmt: &str) -> String {
    dt.format(fmt).to_string()
}

pub fn parse_time(s: &str, fmt: &str) -> Option<NaiveDateTime> {
    NaiveDateTime::parse_from_str(s, fmt).ok()
}

/// Placeholder for d3-time-format API parity.
/// See: https://github.com/d3/d3-time-format#api-reference
/// TODO: Implement full API parity with d3-time-format (timeFormat, timeParse, utcFormat, utcParse, etc.)

pub fn time_format(dt: NaiveDateTime, fmt: &str) -> String {
    dt.format(fmt).to_string()
}

pub fn time_parse(s: &str, fmt: &str) -> Option<NaiveDateTime> {
    NaiveDateTime::parse_from_str(s, fmt).ok()
}

pub fn utc_format(dt: NaiveDateTime, fmt: &str) -> String {
    let dt_utc: DateTime<Utc> = DateTime::from_naive_utc_and_offset(dt, Utc);
    dt_utc.format(fmt).to_string()
}

pub fn utc_parse(s: &str, fmt: &str) -> Option<NaiveDateTime> {
    DateTime::parse_from_str(s, fmt).ok().map(|dt| dt.naive_utc())
}

// --- Advanced time formatting (from time_format_adv) ---
/// D3.js: d3.timeMultiFormat (stub)
pub fn time_multi_format(_dt: NaiveDateTime) -> String {
    // TODO: Implement multi-format logic
    String::new()
}

/// D3.js: d3.timeFormatLocale (stub)
pub fn time_format_locale_placeholder() -> &'static str {
    "timeFormatLocale not implemented"
}

/// Formats a date using multiple formats based on conditions (like d3.timeMultiFormat).
pub fn time_format_multi(dt: NaiveDateTime) -> String {
    use chrono::Datelike;
    if dt.year() < 2000 {
        dt.format("%Y-%m-%d").to_string()
    } else {
        dt.format("%d %b %Y").to_string()
    }
}

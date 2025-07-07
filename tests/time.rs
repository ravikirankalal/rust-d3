//! Unit tests for d3 time
use rust_d3::time::{TimeScale, format_time, time_day, time_week, time_month, time_year};
use chrono::NaiveDateTime;

#[test]
fn test_time_scale() {
    let d0 = NaiveDateTime::parse_from_str("2020-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let d1 = NaiveDateTime::parse_from_str("2020-01-02 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let scale = TimeScale::new((d0, d1), (0.0, 100.0));
    let mid = NaiveDateTime::parse_from_str("2020-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    assert!((scale.scale(mid) - 50.0).abs() < 1e-6);
}

#[test]
fn test_format_time() {
    let dt = NaiveDateTime::parse_from_str("2020-01-01 15:30:00", "%Y-%m-%d %H:%M:%S").unwrap();
    assert_eq!(format_time(dt, "%Y-%m-%d %H:%M"), "2020-01-01 15:30");
}

#[test]
fn test_time_day() {
    let start = NaiveDateTime::parse_from_str("2020-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let end = NaiveDateTime::parse_from_str("2020-01-03 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let days = time_day(start, end);
    assert_eq!(days.len(), 3);
}

#[test]
fn test_time_week() {
    let start = NaiveDateTime::parse_from_str("2020-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let end = NaiveDateTime::parse_from_str("2020-01-15 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let weeks = time_week(start, end);
    assert_eq!(weeks.len(), 3);
}

#[test]
fn test_time_month() {
    let start = NaiveDateTime::parse_from_str("2020-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let end = NaiveDateTime::parse_from_str("2020-03-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let months = time_month(start, end);
    assert_eq!(months.len(), 3);
}

#[test]
fn test_time_year() {
    let start = NaiveDateTime::parse_from_str("2020-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let end = NaiveDateTime::parse_from_str("2022-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let years = time_year(start, end);
    assert_eq!(years.len(), 3);
}

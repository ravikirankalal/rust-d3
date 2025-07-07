//! Tests for time_adv module
use rust_d3::time_adv::time_ticks;
use chrono::NaiveDateTime;

#[test]
fn test_time_ticks() {
    let start = NaiveDateTime::parse_from_str("2020-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let end = NaiveDateTime::parse_from_str("2020-01-02 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let ticks = time_ticks(start, end, 4);
    assert_eq!(ticks.len(), 5);
    assert_eq!(ticks[0], start);
    assert_eq!(ticks.last().unwrap(), &end);
}

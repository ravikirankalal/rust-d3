//! Integration test for scale module

use rust_d3::scale::{LinearScale, BandScale, OrdinalScale, LogScale, PowScale, SqrtScale, TimeScale};
use chrono::{Utc, TimeZone, Timelike};

#[test]
fn test_linear_scale() {
    let scale = LinearScale::new((0.0, 10.0), (0.0, 100.0));
    assert_eq!(scale.scale(0.0), 0.0);
    assert_eq!(scale.scale(5.0), 50.0);
    assert_eq!(scale.scale(10.0), 100.0);
}

#[test]
fn test_band_scale() {
    let scale = BandScale::new(vec!["a", "b", "c"], (0.0, 30.0), 0.1);
    assert_eq!(scale.bandwidth(), 10.0);
    assert_eq!(scale.scale(&"a"), Some(0.0));
    assert_eq!(scale.scale(&"b"), Some(10.0));
    assert_eq!(scale.scale(&"c"), Some(20.0));
}

#[test]
fn test_ordinal_scale() {
    let scale = OrdinalScale::new(vec!["a", "b", "c"], vec![1, 2, 3]);
    assert_eq!(scale.scale(&"a"), Some(1));
    assert_eq!(scale.scale(&"b"), Some(2));
    assert_eq!(scale.scale(&"c"), Some(3));
    assert_eq!(scale.scale(&"d"), None);
}

#[test]
fn test_log_scale() {
    let scale = LogScale::new((1.0, 100.0), (0.0, 2.0), 10.0);
    assert!((scale.scale(1.0) - 0.0).abs() < 1e-6);
    assert!((scale.scale(10.0) - 1.0).abs() < 1e-6);
    assert!((scale.scale(100.0) - 2.0).abs() < 1e-6);
}

#[test]
fn test_pow_scale() {
    let scale = PowScale::new((0.0, 2.0), (0.0, 8.0), 3.0);
    assert!((scale.scale(0.0) - 0.0).abs() < 1e-6);
    assert!((scale.scale(1.0) - 1.0).abs() < 1e-6);
    assert!((scale.scale(2.0) - 8.0).abs() < 1e-6);
}

#[test]
fn test_sqrt_scale() {
    let scale = SqrtScale::new((0.0, 4.0), (0.0, 2.0));
    assert!((scale.scale(0.0) - 0.0).abs() < 1e-6);
    assert!((scale.scale(1.0) - 1.0).abs() < 1e-6);
    assert!((scale.scale(4.0) - 2.0).abs() < 1e-6);
}

#[test]
fn test_time_scale_basic() {
    let d0 = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
    let d1 = Utc.with_ymd_and_hms(2020, 1, 2, 0, 0, 0).unwrap();
    let scale = TimeScale::new((d0, d1), (0.0, 24.0));
    assert!((scale.scale(d0) - 0.0).abs() < 1e-6);
    assert!((scale.scale(d1) - 24.0).abs() < 1e-6);
}

#[test]
fn test_time_scale_invert() {
    let d0 = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
    let d1 = Utc.with_ymd_and_hms(2020, 1, 2, 0, 0, 0).unwrap();
    let scale = TimeScale::new((d0, d1), (0.0, 24.0));
    let dt = scale.invert(12.0);
    assert_eq!(dt.hour(), 12);
}

#[test]
fn test_time_scale_ticks_and_format() {
    let d0 = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
    let d1 = Utc.with_ymd_and_hms(2020, 1, 2, 0, 0, 0).unwrap();
    let scale = TimeScale::new((d0, d1), (0.0, 24.0));
    let ticks = scale.ticks(3);
    assert_eq!(ticks.len(), 3);
    let fmt = scale.tick_format("%H:%M");
    let labels: Vec<String> = ticks.iter().map(|dt| fmt(*dt)).collect();
    assert_eq!(labels[0], "00:00");
}

#[test]
fn test_time_scale_nice_and_clamp() {
    let d0 = Utc.with_ymd_and_hms(2020, 1, 1, 5, 30, 0).unwrap();
    let d1 = Utc.with_ymd_and_hms(2020, 1, 2, 18, 45, 0).unwrap();
    let mut scale = TimeScale::new((d0, d1), (0.0, 24.0));
    scale.nice();
    let (nice0, nice1) = scale.domain();
    assert_eq!(nice0.hour(), 0);
    assert_eq!(nice1.hour(), 23);
    scale.clamp();
    let (clamp0, clamp1) = scale.domain();
    assert!(clamp0 <= clamp1);
}
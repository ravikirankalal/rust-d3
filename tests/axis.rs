//! Unit tests for d3 axis

use rust_d3::scale::{LinearScale, LogScale, PowScale, SqrtScale, OrdinalScale, BandScale, TimeScale};
use chrono::{Utc, TimeZone};
use rust_d3::axis::{Axis, AxisOrientation, AxisScale};

#[test]
fn test_axis_generate_linear() {
    let scale = LinearScale::new((0.0, 10.0), (0.0, 100.0));
    let axis = Axis::new(5, AxisOrientation::Bottom);
    let ticks = axis.generate(AxisScale::Linear(&scale));
    let expected = vec![
        (0.0, "0".to_string()),
        (2.5, "2.5".to_string()),
        (5.0, "5".to_string()),
        (7.5, "7.5".to_string()),
        (10.0, "10".to_string()),
    ];
    assert_eq!(ticks, expected);
}

#[test]
fn test_axis_generate_linear_with_tick_format() {
    let scale = LinearScale::new((0.0, 10.0), (0.0, 100.0));
    let axis = Axis::new(3, AxisOrientation::Left)
        .with_tick_format(|v| format!("{:.1} units", v));
    let ticks = axis.generate(AxisScale::Linear(&scale));
    let expected = vec![
        (0.0, "0.0 units".to_string()),
        (5.0, "5.0 units".to_string()),
        (10.0, "10.0 units".to_string()),
    ];
    assert_eq!(ticks, expected);
}

#[test]
fn test_axis_generate_linear_with_custom_ticks() {
    let scale = LinearScale::new((0.0, 10.0), (0.0, 100.0));
    let axis = Axis::new(5, AxisOrientation::Top)
        .with_custom_ticks(vec![1.0, 3.0, 7.0]);
    let ticks = axis.generate(AxisScale::Linear(&scale));
    let expected = vec![
        (1.0, "1".to_string()),
        (3.0, "3".to_string()),
        (7.0, "7".to_string()),
    ];
    assert_eq!(ticks, expected);
}

#[test]
fn test_axis_generate_log() {
    let scale = LogScale::new((1.0, 100.0), (0.0, 2.0), 10.0);
    let axis = Axis::new(3, AxisOrientation::Bottom);
    let ticks = axis.generate(AxisScale::Log(&scale));
    let expected = vec![
        (1.0, "1".to_string()),
        (10.0, "10".to_string()),
        (100.0, "100".to_string()),
    ];
    assert_eq!(ticks, expected);
}

#[test]
fn test_axis_generate_pow() {
    let scale = PowScale::new((0.0, 8.0), (0.0, 64.0), 3.0);
    let axis = Axis::new(3, AxisOrientation::Left);
    let ticks = axis.generate(AxisScale::Pow(&scale));
    let expected = vec![
        (0.0, "0".to_string()),
        (4.0, "4".to_string()),
        (8.0, "8".to_string()),
    ];
    assert_eq!(ticks, expected);
}

#[test]
fn test_axis_generate_sqrt() {
    let scale = SqrtScale::new((0.0, 9.0), (0.0, 3.0));
    let axis = Axis::new(4, AxisOrientation::Right);
    let ticks = axis.generate(AxisScale::Sqrt(&scale));
    let expected = vec![
        (0.0, "0".to_string()),
        (3.0, "3".to_string()),
        (6.0, "6".to_string()),
        (9.0, "9".to_string()),
    ];
    assert_eq!(ticks, expected);
}

#[test]
fn test_axis_generate_ordinal() {
    let domain = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let range = vec!["red".to_string(), "green".to_string(), "blue".to_string()];
    let scale = OrdinalScale::new(domain, range);
    let axis = Axis::new(3, AxisOrientation::Bottom);
    let ticks = axis.generate(AxisScale::Ordinal(&scale));
    let expected = vec![
        (0.0, "0".to_string()),
        (1.0, "1".to_string()),
        (2.0, "2".to_string()),
    ];
    assert_eq!(ticks, expected);
}

#[test]
fn test_axis_generate_band() {
    let domain = vec!["A".to_string(), "B".to_string(), "C".to_string()];
    let scale = BandScale::new(domain, (0.0, 300.0), 0.0);
    let axis = Axis::new(3, AxisOrientation::Top);
    let ticks = axis.generate(AxisScale::Band(&scale));
    let expected = vec![
        (0.0, "0".to_string()),
        (1.0, "1".to_string()),
        (2.0, "2".to_string()),
    ];
    assert_eq!(ticks, expected);
}

#[test]
fn test_axis_svg_and_label() {
    let scale = LinearScale::new((0.0, 10.0), (0.0, 100.0));
    let axis = Axis::new(3, AxisOrientation::Bottom)
        .with_label("Test Axis");
    let svg = axis.to_svg(AxisScale::Linear(&scale));
    assert!(svg.contains("axis-label"));
    assert!(svg.contains("Test Axis"));
    assert!(svg.contains("tick"));
}

#[test]
fn test_axis_svg_with_time_scale() {
    let d0 = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
    let d1 = Utc.with_ymd_and_hms(2020, 1, 2, 0, 0, 0).unwrap();
    let _scale = TimeScale::new((d0, d1), (0.0, 24.0));
    let axis = Axis::new(3, AxisOrientation::Bottom)
        .with_label("Time Axis");
    let svg = axis.to_svg(AxisScale::Linear(&LinearScale::new((0.0, 10.0), (0.0, 100.0))));
    // This is a stub: update to use AxisScale::Time(&scale) when Axis supports it
    assert!(svg.contains("axis-label"));
    assert!(svg.contains("Time Axis"));
}

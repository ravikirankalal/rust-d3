// Axis module tests moved from src/axis/mod.rs
// These tests cover D3 parity for axis construction, tick generation, layout, and custom options

use rust_d3::axis::*;
use rust_d3::scale::{ScaleLinear, ScaleLog, ScaleTime, ScaleBand, ScalePoint};
use chrono::NaiveDate;

#[test]
fn test_linear_axis_ticks() {
    let scale = ScaleLinear::new([0.0, 10.0], [0.0, 100.0]);
    let axis = Axis::new(scale, AxisOrientation::Bottom).tick_count(5);
    let ticks = axis.ticks();
    assert_eq!(ticks.len(), 5);
    assert!((ticks[0].value - 0.0).abs() < 1e-6);
    assert!((ticks[4].value - 10.0).abs() < 1e-6);
    assert!((ticks[2].position - 50.0).abs() < 1e-6);
    assert_eq!(ticks[0].label, "0.000000");
}

#[test]
fn test_log_axis_ticks() {
    let scale = ScaleLog::new([1.0, 1000.0], [0.0, 100.0], 10.0);
    let axis = Axis::new(scale, AxisOrientation::Left).tick_count(4);
    let ticks = axis.ticks();
    assert!(ticks.iter().any(|t| (t.value - 1.0).abs() < 1e-6));
    assert!(ticks.iter().any(|t| (t.value - 10.0).abs() < 1e-6));
    assert!(ticks.iter().any(|t| (t.value - 100.0).abs() < 1e-6));
    assert!(ticks.iter().any(|t| (t.value - 1000.0).abs() < 1e-6));
}

#[test]
fn test_time_axis_ticks() {
    let start = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap();
    let end = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap().and_hms_opt(0, 0, 4).unwrap();
    let scale = ScaleTime::new([start, end], [0.0, 100.0]);
    let axis = Axis::new(scale, AxisOrientation::Bottom).tick_count(5);
    let ticks = axis.ticks();
    assert_eq!(ticks.len(), 5);
    assert_eq!(ticks[0].label, "2020-01-01");
    assert_eq!(ticks[4].label, "2020-01-01");
}

#[test]
fn test_band_axis_ticks() {
    let scale = ScaleBand::new(vec!["a", "b", "c"], [0.0, 120.0], 0.1, 0.1, 0.5);
    let axis = Axis::new(scale, AxisOrientation::Bottom);
    let ticks = axis.ticks();
    assert_eq!(ticks.len(), 3);
    assert_eq!(ticks[0].label, "a");
    assert_eq!(ticks[1].label, "b");
    assert_eq!(ticks[2].label, "c");
}

#[test]
fn test_point_axis_ticks() {
    let scale = ScalePoint::new(vec!["x", "y", "z"], [0.0, 100.0], 0.5);
    let axis = Axis::new(scale, AxisOrientation::Left);
    let ticks = axis.ticks();
    assert_eq!(ticks.len(), 3);
    assert_eq!(ticks[0].label, "x");
    assert_eq!(ticks[1].label, "y");
    assert_eq!(ticks[2].label, "z");
}

#[test]
fn test_linear_axis_custom_ticks() {
    let scale = ScaleLinear::new([0.0, 10.0], [0.0, 100.0]);
    let axis = Axis::new(scale, AxisOrientation::Bottom)
        .tick_values(vec![2.0, 5.0, 8.0]);
    let ticks = axis.ticks();
    assert_eq!(ticks.len(), 3);
    assert!((ticks[0].value - 2.0).abs() < 1e-6);
    assert!((ticks[1].value - 5.0).abs() < 1e-6);
    assert!((ticks[2].value - 8.0).abs() < 1e-6);
}

#[test]
fn test_axis_layout_linear() {
    let scale = ScaleLinear::new([0.0, 10.0], [0.0, 100.0]);
    let axis = Axis::new(scale, AxisOrientation::Bottom)
        .tick_count(3)
        .tick_size_inner(8.0)
        .tick_size_outer(12.0)
        .tick_padding(5.0);
    let ticks = axis.ticks();
    let layout = axis.layout(0.0, 100.0, ticks.clone());
    assert_eq!(layout.orientation, AxisOrientation::Bottom);
    assert_eq!(layout.ticks.len(), 3);
    assert_eq!(layout.tick_size_inner, 8.0);
    assert_eq!(layout.tick_size_outer, 12.0);
    assert_eq!(layout.tick_padding, 5.0);
    assert_eq!(layout.axis_start, 0.0);
    assert_eq!(layout.axis_end, 100.0);
    assert_eq!(layout.ticks[0].label, ticks[0].label);
}

#[test]
fn test_axis_layout_with_offset_and_locale() {
    let scale = ScaleLinear::new([0.0, 1.0], [0.0, 100.0]);
    let axis = Axis::new(scale, AxisOrientation::Top)
        .tick_count(2)
        .tick_size_inner(5.0)
        .tick_size_outer(7.0)
        .tick_padding(2.0)
        .offset(0.5)
        .locale("fr-FR");
    let ticks = axis.ticks();
    let layout = axis.layout(0.0, 100.0, ticks.clone());
    assert_eq!(layout.orientation, AxisOrientation::Top);
    assert_eq!(layout.offset, 0.5);
    assert_eq!(axis.locale.as_deref(), Some("fr-FR"));
}

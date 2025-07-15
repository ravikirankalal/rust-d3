// Axis module tests moved from src/axis/mod.rs
// These tests cover D3 parity for axis construction, tick generation, layout, and custom options

use chrono::NaiveDate;
use rust_d3::axis::axis_structs::{AxisLineStyle, GridStyle, TickLabelStyle, TitleStyle};
use rust_d3::axis::*;
use rust_d3::scale::{ScaleBand, ScaleLinear, ScaleLog, ScalePoint, ScaleTime};

#[test]
fn test_linear_axis_ticks() {
    let scale = ScaleLinear::new([0.0, 10.0], [0.0, 100.0]);
    let axis = Axis::new(scale, AxisOrientation::Bottom).tick_count(5);
    let ticks = axis.ticks();
    
    // D3 generates 6 ticks for domain [0,10] with tick_count 5: [0,2,4,6,8,10]
    assert_eq!(ticks.len(), 6);
    assert!((ticks[0].value - 0.0).abs() < 1e-6);
    assert!((ticks[5].value - 10.0).abs() < 1e-6);
    assert!((ticks[2].position - 40.0).abs() < 1e-6); // value 4 maps to position 40
    assert_eq!(ticks[0].label, "0");
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
    let start = NaiveDate::from_ymd_opt(2020, 1, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let end = NaiveDate::from_ymd_opt(2020, 1, 1)
        .unwrap()
        .and_hms_opt(0, 0, 4)
        .unwrap();
    let scale = ScaleTime::new([start, end], [0.0, 100.0]);
    let axis = Axis::new(scale, AxisOrientation::Bottom).tick_count(5);
    let ticks = axis.ticks();
    
    // Current time implementation produces 4 ticks for 4-second span
// Updated to match D3's 5 ticks behavior
    assert_eq!(ticks.len(), 5);
    assert_eq!(ticks[0].label, "2020-01-01");
    assert_eq!(ticks[3].label, "2020-01-01");
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
    let axis = Axis::new(scale, AxisOrientation::Bottom).tick_values(vec![2.0, 5.0, 8.0]);
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
fn test_axis_tick_size() {
    let scale = ScaleLinear::new([0.0, 10.0], [0.0, 100.0]);
    let axis = Axis::new(scale, AxisOrientation::Bottom)
        .tick_count(3)
        .tick_size(10.0); // This should set both inner and outer to 10.0
    let ticks = axis.ticks();
    let layout = axis.layout(0.0, 100.0, ticks.clone());
    
    // Both inner and outer tick sizes should be set to 10.0
    assert_eq!(layout.tick_size_inner, 10.0);
    assert_eq!(layout.tick_size_outer, 10.0);
}

#[test]
fn test_axis_tick_size_chainable() {
    let scale = ScaleLinear::new([0.0, 10.0], [0.0, 100.0]);
    
    // Test that tick_size can be chained with other methods
    let axis = Axis::new(scale, AxisOrientation::Bottom)
        .tick_count(5)
        .tick_size(8.0)
        .tick_padding(4.0)
        .offset(1.0);
    
    let ticks = axis.ticks();
    let layout = axis.layout(0.0, 100.0, ticks.clone());
    
    // Verify both tick sizes are set and chaining works
    assert_eq!(layout.tick_size_inner, 8.0);
    assert_eq!(layout.tick_size_outer, 8.0);
    assert_eq!(layout.tick_padding, 4.0);
    assert_eq!(layout.offset, 1.0);
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

#[test]
fn test_axis_grid_and_style() {
    let scale = ScaleLinear::new([0.0, 5.0], [0.0, 50.0]);
    let axis = Axis::new(scale, AxisOrientation::Right)
        .grid(true)
        .grid_style(GridStyle {
            color: "#f00".to_string(),
            width: 2.0,
            dasharray: Some("2,2".to_string()),
        });
    assert!(axis.grid);
    assert_eq!(axis.grid_style.as_ref().unwrap().color, "#f00");
}

#[test]
fn test_axis_title_and_style() {
    let scale = ScaleLinear::new([0.0, 1.0], [0.0, 10.0]);
    let axis = Axis::new(scale, AxisOrientation::Top)
        .title("Test Axis")
        .title_style(TitleStyle {
            font: "Arial".to_string(),
            color: "#00f".to_string(),
            position: Some((5.0, 5.0)),
        });
    assert_eq!(axis.title.as_deref(), Some("Test Axis"));
    assert_eq!(axis.title_style.as_ref().unwrap().font, "Arial");
}

#[test]
fn test_axis_minor_ticks_and_size() {
    let scale = ScaleLinear::new([0.0, 10.0], [0.0, 100.0]);
    let axis = Axis::new(scale, AxisOrientation::Bottom)
        .minor_ticks(vec![2.0, 4.0, 6.0])
        .minor_tick_size(3.5);
    assert_eq!(axis.minor_ticks.as_ref().unwrap().len(), 3);
    assert_eq!(axis.minor_tick_size.unwrap(), 3.5);
}

#[test]
fn test_axis_tick_label_angle_and_style() {
    let scale = ScaleLinear::new([0.0, 1.0], [0.0, 10.0]);
    let axis = Axis::new(scale, AxisOrientation::Left)
        .tick_label_angle(45.0)
        .tick_label_style(TickLabelStyle {
            font: "Verdana".to_string(),
            color: "#333".to_string(),
            padding: Some(2.0),
        });
    assert_eq!(axis.tick_label_angle.unwrap(), 45.0);
    assert_eq!(axis.tick_label_style.as_ref().unwrap().font, "Verdana");
}

#[test]
fn test_axis_line_style() {
    let scale = ScaleLinear::new([0.0, 1.0], [0.0, 10.0]);
    let axis = Axis::new(scale, AxisOrientation::Top).axis_line_style(AxisLineStyle {
        color: "#abc".to_string(),
        width: 1.5,
        dasharray: None,
    });
    assert_eq!(axis.axis_line_style.as_ref().unwrap().color, "#abc");
    assert_eq!(axis.axis_line_style.as_ref().unwrap().width, 1.5);
}

#[test]
fn test_axis_on_render_hook() {
    use std::sync::{Arc, Mutex};
    let scale = ScaleLinear::new([0.0, 1.0], [0.0, 10.0]);
    let called = Arc::new(Mutex::new(false));
    let called_clone = called.clone();
    let axis = Axis::new(scale, AxisOrientation::Bottom).on_render(move || {
        *called_clone.lock().unwrap() = true;
    });
    // Simulate calling the hook
    if let Some(hook) = &axis.on_render {
        hook();
    }
    assert!(*called.lock().unwrap());
}

#[test]
fn test_axis_empty_domain_range() {
    let scale = ScaleLinear::new([0.0, 0.0], [0.0, 0.0]);
    let axis = Axis::new(scale, AxisOrientation::Bottom);
    let ticks = axis.ticks();
    assert!(ticks.len() > 0); // Should still produce at least one tick
}

#[test]
fn test_axis_single_tick() {
    let scale = ScaleLinear::new([5.0, 5.0], [10.0, 10.0]);
    let axis = Axis::new(scale, AxisOrientation::Left).tick_count(1);
    let ticks = axis.ticks();
    assert_eq!(ticks.len(), 1);
}

#[test]
fn test_axis_default_offset() {
    let scale = ScaleLinear::new([0.0, 10.0], [0.0, 100.0]);
    let axis = Axis::new(scale, AxisOrientation::Bottom);
    
    // Default offset should be 0.5 for crisp lines
    assert_eq!(axis.offset, 0.5);
    
    let ticks = axis.ticks();
    let layout = axis.layout(0.0, 100.0, ticks);
    assert_eq!(layout.offset, 0.5);
}

#[test]
fn test_axis_custom_offset() {
    let scale = ScaleLinear::new([0.0, 10.0], [0.0, 100.0]);
    let axis = Axis::new(scale, AxisOrientation::Left).offset(1.0);
    
    // Custom offset should be respected
    assert_eq!(axis.offset, 1.0);
    
    let ticks = axis.ticks();
    let layout = axis.layout(0.0, 100.0, ticks);
    assert_eq!(layout.offset, 1.0);
}

#[test]
fn test_axis_zero_offset() {
    let scale = ScaleLinear::new([0.0, 10.0], [0.0, 100.0]);
    let axis = Axis::new(scale, AxisOrientation::Right).offset(0.0);
    
    // Zero offset should disable crisp lines
    assert_eq!(axis.offset, 0.0);
    
    let ticks = axis.ticks();
    let layout = axis.layout(0.0, 100.0, ticks);
    assert_eq!(layout.offset, 0.0);
}

#[test]
fn test_axis_offset_chaining() {
    let scale = ScaleLinear::new([0.0, 10.0], [0.0, 100.0]);
    let axis = Axis::new(scale, AxisOrientation::Top)
        .tick_count(5)
        .offset(0.25)
        .tick_padding(2.0);
    
    // Offset should be chainable with other methods
    assert_eq!(axis.offset, 0.25);
    assert_eq!(axis.tick_count, 5);
    assert_eq!(axis.tick_padding, 2.0);
}

#[test]
fn test_axis_offset_in_layout() {
    let scale = ScaleLinear::new([0.0, 10.0], [0.0, 100.0]);
    let axis = Axis::new(scale, AxisOrientation::Bottom)
        .tick_count(3)
        .offset(0.75);
    
    let ticks = axis.ticks();
    let layout = axis.layout(0.0, 100.0, ticks);
    
    // Layout should include the offset
    assert_eq!(layout.offset, 0.75);
    assert_eq!(layout.orientation, AxisOrientation::Bottom);
}

#[test]
fn test_band_axis_default_offset() {
    let scale = ScaleBand::new(vec!["a", "b", "c"], [0.0, 120.0], 0.1, 0.1, 0.5);
    let axis = Axis::new(scale, AxisOrientation::Bottom);
    
    // Band axis should also have default 0.5 offset
    assert_eq!(axis.offset, 0.5);
}

#[test]
fn test_point_axis_default_offset() {
    let scale = ScalePoint::new(vec!["x", "y", "z"], [0.0, 100.0], 0.5);
    let axis = Axis::new(scale, AxisOrientation::Left);
    
    // Point axis should also have default 0.5 offset
    assert_eq!(axis.offset, 0.5);
}

#[test]
fn test_log_axis_default_offset() {
    let scale = ScaleLog::new([1.0, 1000.0], [0.0, 100.0], 10.0);
    let axis = Axis::new(scale, AxisOrientation::Right);
    
    // Log axis should also have default 0.5 offset
    assert_eq!(axis.offset, 0.5);
}

#[test]
fn test_time_axis_default_offset() {
    let start = NaiveDate::from_ymd_opt(2020, 1, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let end = NaiveDate::from_ymd_opt(2020, 1, 1)
        .unwrap()
        .and_hms_opt(0, 0, 4)
        .unwrap();
    let scale = ScaleTime::new([start, end], [0.0, 100.0]);
    let axis = Axis::new(scale, AxisOrientation::Top);
    
    // Time axis should also have default 0.5 offset
    assert_eq!(axis.offset, 0.5);
}

#[test]
fn test_axis_transform_application() {
    use rust_d3::axis::axis_renderable::AxisRenderable;
    use rust_d3::selection::Selection;
    
    let scale = ScaleLinear::new([0.0, 10.0], [0.0, 100.0]);
    
    // Test Bottom axis - should apply horizontal translate
    let axis_bottom = Axis::new(scale.clone(), AxisOrientation::Bottom).offset(0.5);
    let mut selection_bottom = Selection::create("g");
    axis_bottom.render(&mut selection_bottom);
    
    let node_bottom = selection_bottom.node().unwrap();
    assert_eq!(node_bottom.attributes.get("transform"), Some(&"translate(0.5,0)".to_string()));
    
    // Test Left axis - should apply vertical translate
    let axis_left = Axis::new(scale.clone(), AxisOrientation::Left).offset(0.5);
    let mut selection_left = Selection::create("g");
    axis_left.render(&mut selection_left);
    
    let node_left = selection_left.node().unwrap();
    assert_eq!(node_left.attributes.get("transform"), Some(&"translate(0,0.5)".to_string()));
    
    // Test Top axis - should apply horizontal translate
    let axis_top = Axis::new(scale.clone(), AxisOrientation::Top).offset(0.5);
    let mut selection_top = Selection::create("g");
    axis_top.render(&mut selection_top);
    
    let node_top = selection_top.node().unwrap();
    assert_eq!(node_top.attributes.get("transform"), Some(&"translate(0.5,0)".to_string()));
    
    // Test Right axis - should apply vertical translate
    let axis_right = Axis::new(scale.clone(), AxisOrientation::Right).offset(0.5);
    let mut selection_right = Selection::create("g");
    axis_right.render(&mut selection_right);
    
    let node_right = selection_right.node().unwrap();
    assert_eq!(node_right.attributes.get("transform"), Some(&"translate(0,0.5)".to_string()));
}

#[test]
fn test_axis_tick_size_alias() {
    let scale = ScaleLinear::new([0.0, 10.0], [0.0, 100.0]);
    let axis = Axis::new(scale, AxisOrientation::Bottom)
        .tick_count(3)
        .tick_size(10.0); // This should set both inner and outer to 10.0
    let ticks = axis.ticks();
    let layout = axis.layout(0.0, 100.0, ticks.clone());
    
    // Both inner and outer tick sizes should be set to 10.0
    assert_eq!(layout.tick_size_inner, 10.0);
    assert_eq!(layout.tick_size_outer, 10.0);
    
    // Test getter methods
    assert_eq!(axis.tick_size_inner_value(), 10.0);
    assert_eq!(axis.tick_size_outer_value(), 10.0);
}

#[test]
fn test_axis_offset_half_visual_diff() {
    let scale = ScaleLinear::new([0.0, 10.0], [0.0, 100.0]);
    
    // Test with offset = 0.5 (default for crisp lines)
    let axis_half = Axis::new(scale.clone(), AxisOrientation::Bottom).offset(0.5);
    let ticks_half = axis_half.ticks();
    let layout_half = axis_half.layout(0.0, 100.0, ticks_half);
    
    // Test with offset = 0.0 (no crisp lines)
    let axis_zero = Axis::new(scale.clone(), AxisOrientation::Bottom).offset(0.0);
    let ticks_zero = axis_zero.ticks();
    let layout_zero = axis_zero.layout(0.0, 100.0, ticks_zero);
    
    // Verify the offset difference
    assert_eq!(layout_half.offset, 0.5);
    assert_eq!(layout_zero.offset, 0.0);
    
    // The visual difference should be 0.5 pixels
    assert_eq!(layout_half.offset - layout_zero.offset, 0.5);
}

#[test]
fn test_axis_custom_tick_format_string() {
    let scale = ScaleLinear::new([0.0, 10.0], [0.0, 100.0]);
    
    // Test with custom tick format function
    let custom_format = |value: f64| -> String {
        format!("{:.1}%", value * 10.0)
    };
    
    let axis = Axis::new(scale, AxisOrientation::Bottom)
        .tick_count(3)
        .tick_format(custom_format);
    
    let ticks = axis.ticks();
    
    // Check that custom format is applied
    assert_eq!(ticks.len(), 3);
    // The first tick should be 0.0 which becomes "0.0%"
    assert_eq!(ticks[0].label, "0.0%");
    // The last tick should be 10.0 which becomes "100.0%"
    assert_eq!(ticks[2].label, "100.0%");
}

#[test]
fn test_axis_custom_offset_transform() {
    use rust_d3::axis::axis_renderable::AxisRenderable;
    use rust_d3::selection::Selection;
    
    let scale = ScaleLinear::new([0.0, 10.0], [0.0, 100.0]);
    
    // Test custom offset values
    let axis_custom = Axis::new(scale.clone(), AxisOrientation::Bottom).offset(1.25);
    let mut selection_custom = Selection::create("g");
    axis_custom.render(&mut selection_custom);
    
    let node_custom = selection_custom.node().unwrap();
    assert_eq!(node_custom.attributes.get("transform"), Some(&"translate(1.25,0)".to_string()));
    
    // Test zero offset
    let axis_zero = Axis::new(scale.clone(), AxisOrientation::Left).offset(0.0);
    let mut selection_zero = Selection::create("g");
    axis_zero.render(&mut selection_zero);
    
    let node_zero = selection_zero.node().unwrap();
    assert_eq!(node_zero.attributes.get("transform"), Some(&"translate(0,0)".to_string()));
}

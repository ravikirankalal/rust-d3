mod fixture_helper;

use rust_d3::axis::*;
use rust_d3::scale::ScaleLinear;
use fixture_helper::*;

#[test]
fn test_linear_axis_basic_ticks() {
    let fixtures = load_axis_fixtures();
    let fixture = &fixtures.linear["basic_ticks"];
    
    let scale = ScaleLinear::new(fixture.domain, fixture.range);
    let axis = Axis::new(scale, AxisOrientation::Bottom).tick_count(fixture.tick_count.unwrap_or(5) as usize);
    let ticks = axis.ticks();
    
    // Verify against fixture expectations
    assert_eq!(ticks.len(), fixture.expected.len());
    
    for (i, tick) in ticks.iter().enumerate() {
        assert_tick_matches_expectation(tick, &fixture.expected[i], 1e-6);
    }
}

#[test]
fn test_linear_axis_custom_ticks() {
    let fixtures = load_axis_fixtures();
    let fixture = &fixtures.linear["custom_ticks"];
    
    let scale = ScaleLinear::new(fixture.domain, fixture.range);
    let axis = Axis::new(scale, AxisOrientation::Bottom)
        .tick_values(fixture.tick_values.as_ref().unwrap().clone());
    let ticks = axis.ticks();
    
    // Verify against fixture expectations
    assert_eq!(ticks.len(), fixture.expected.len());
    
    for (i, tick) in ticks.iter().enumerate() {
        assert_tick_matches_expectation(tick, &fixture.expected[i], 1e-6);
    }
}

#[test]
fn test_linear_axis_negative_domain() {
    let fixtures = load_axis_fixtures();
    let fixture = &fixtures.linear["negative_domain"];
    
    let scale = ScaleLinear::new(fixture.domain, fixture.range);
    let axis = Axis::new(scale, AxisOrientation::Bottom).tick_count(fixture.tick_count.unwrap_or(5) as usize);
    let ticks = axis.ticks();
    
    // Verify against fixture expectations
    assert_eq!(ticks.len(), fixture.expected.len());
    
    for (i, tick) in ticks.iter().enumerate() {
        assert_tick_matches_expectation(tick, &fixture.expected[i], 1e-6);
    }
}

#[test]
fn test_linear_axis_fractional_domain() {
    let fixtures = load_axis_fixtures();
    let fixture = &fixtures.linear["fractional_domain"];
    
    let scale = ScaleLinear::new(fixture.domain, fixture.range);
    let axis = Axis::new(scale, AxisOrientation::Left).tick_count(fixture.tick_count.unwrap_or(8) as usize);
    let ticks = axis.ticks();
    
    // Verify against fixture expectations
    assert_eq!(ticks.len(), fixture.expected.len());
    
    for (i, tick) in ticks.iter().enumerate() {
        assert_tick_matches_expectation(tick, &fixture.expected[i], 1e-6);
    }
}

#[test]
fn test_linear_axis_large_numbers() {
    let fixtures = load_axis_fixtures();
    let fixture = &fixtures.linear["large_numbers"];
    
    let scale = ScaleLinear::new(fixture.domain, fixture.range);
    let axis = Axis::new(scale, AxisOrientation::Bottom).tick_count(fixture.tick_count.unwrap_or(10) as usize);
    let ticks = axis.ticks();
    
    // Verify against fixture expectations
    assert_eq!(ticks.len(), fixture.expected.len());
    
    for (i, tick) in ticks.iter().enumerate() {
        assert_tick_matches_expectation(tick, &fixture.expected[i], 1e-6);
    }
}

#[test]
fn test_linear_axis_small_numbers() {
    let fixtures = load_axis_fixtures();
    let fixture = &fixtures.linear["small_numbers"];
    
    let scale = ScaleLinear::new(fixture.domain, fixture.range);
    let axis = Axis::new(scale, AxisOrientation::Left).tick_count(fixture.tick_count.unwrap_or(9) as usize);
    let ticks = axis.ticks();
    
    // Verify against fixture expectations
    assert_eq!(ticks.len(), fixture.expected.len());
    
    for (i, tick) in ticks.iter().enumerate() {
        assert_tick_matches_expectation(tick, &fixture.expected[i], 1e-6);
    }
}

#[test]
fn test_linear_axis_reverse_range() {
    let fixtures = load_axis_fixtures();
    let fixture = &fixtures.linear["reverse_range"];
    
    let scale = ScaleLinear::new(fixture.domain, fixture.range);
    let axis = Axis::new(scale, AxisOrientation::Bottom).tick_count(fixture.tick_count.unwrap_or(10) as usize);
    let ticks = axis.ticks();
    
    // Verify against fixture expectations
    assert_eq!(ticks.len(), fixture.expected.len());
    
    for (i, tick) in ticks.iter().enumerate() {
        assert_tick_matches_expectation(tick, &fixture.expected[i], 1e-6);
    }
}

#[test]
fn test_linear_axis_empty_domain() {
    let fixtures = load_axis_fixtures();
    let fixture = &fixtures.linear["empty_domain"];
    
    let scale = ScaleLinear::new(fixture.domain, fixture.range);
    let axis = Axis::new(scale, AxisOrientation::Bottom).tick_count(fixture.tick_count.unwrap_or(5) as usize);
    let ticks = axis.ticks();
    
    // Verify against fixture expectations
    assert_eq!(ticks.len(), fixture.expected.len());
    
    for (i, tick) in ticks.iter().enumerate() {
        assert_tick_matches_expectation(tick, &fixture.expected[i], 1e-6);
    }
}

#[test]
fn test_linear_axis_single_value() {
    let fixtures = load_axis_fixtures();
    let fixture = &fixtures.linear["single_value"];
    
    let scale = ScaleLinear::new(fixture.domain, fixture.range);
    let axis = Axis::new(scale, AxisOrientation::Left).tick_count(fixture.tick_count.unwrap_or(1) as usize);
    let ticks = axis.ticks();
    
    // Verify against fixture expectations
    assert_eq!(ticks.len(), fixture.expected.len());
    
    for (i, tick) in ticks.iter().enumerate() {
        assert_tick_matches_expectation(tick, &fixture.expected[i], 1e-6);
    }
}


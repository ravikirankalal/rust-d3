use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TickExpectation {
    pub value: serde_json::Value,
    pub position: f64,
    pub label: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LinearFixture {
    pub domain: [f64; 2],
    pub range: [f64; 2],
    pub tick_count: Option<u32>,
    pub tick_values: Option<Vec<f64>>,
    pub expected: Vec<TickExpectation>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LogFixture {
    pub domain: [f64; 2],
    pub range: [f64; 2],
    pub base: f64,
    pub tick_count: Option<u32>,
    pub tick_values: Option<Vec<f64>>,
    pub expected: Vec<TickExpectation>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TimeFixture {
    pub domain: [String; 2],
    pub range: [f64; 2],
    pub tick_count: Option<u32>,
    pub tick_values: Option<Vec<String>>,
    pub expected: Vec<TickExpectation>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BandFixture {
    pub domain: Vec<String>,
    pub range: [f64; 2],
    pub inner_padding: f64,
    pub outer_padding: f64,
    pub align: f64,
    pub expected: Vec<TickExpectation>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PointFixture {
    pub domain: Vec<String>,
    pub range: [f64; 2],
    pub padding: f64,
    pub expected: Vec<TickExpectation>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LayoutFixture {
    pub orientation: String,
    pub tick_size_inner: f64,
    pub tick_size_outer: f64,
    pub tick_padding: f64,
    pub offset: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GridStyleFixture {
    pub color: String,
    pub width: f64,
    pub dasharray: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TitleStyleFixture {
    pub font: String,
    pub color: String,
    pub position: [f64; 2],
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TickLabelStyleFixture {
    pub font: String,
    pub color: String,
    pub padding: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AxisLineStyleFixture {
    pub color: String,
    pub width: f64,
    pub dasharray: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StylingFixtures {
    pub grid_style: GridStyleFixture,
    pub title_style: TitleStyleFixture,
    pub tick_label_style: TickLabelStyleFixture,
    pub axis_line_style: AxisLineStyleFixture,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AxisComprehensiveFixtures {
    pub linear: HashMap<String, LinearFixture>,
    pub log: HashMap<String, LogFixture>,
    pub time: HashMap<String, TimeFixture>,
    pub band: HashMap<String, BandFixture>,
    pub point: HashMap<String, PointFixture>,
    pub layout: HashMap<String, LayoutFixture>,
    pub styling: StylingFixtures,
}

pub fn load_axis_fixtures() -> AxisComprehensiveFixtures {
    let fixture_path = "tests/fixtures/axis_comprehensive.json";
    let fixture_content = fs::read_to_string(fixture_path)
        .expect("Failed to read axis comprehensive fixtures");
    serde_json::from_str(&fixture_content)
        .expect("Failed to parse axis comprehensive fixtures")
}

pub fn load_d3_reference_fixtures() -> serde_json::Value {
    let fixture_path = "tests/fixtures/d3_axis_reference.json";
    let fixture_content = fs::read_to_string(fixture_path)
        .expect("Failed to read D3 reference fixtures");
    serde_json::from_str(&fixture_content)
        .expect("Failed to parse D3 reference fixtures")
}

// Helper functions for common test operations
pub fn assert_tick_matches_expectation(actual_tick: &rust_d3::axis::Tick, expected: &TickExpectation, tolerance: f64) {
    // Handle different value types (f64 for numeric, String for time/categorical)
    match &expected.value {
        serde_json::Value::Number(n) => {
            let expected_value = n.as_f64().expect("Expected numeric value");
            assert!((actual_tick.value - expected_value).abs() < tolerance, 
                "Tick value mismatch: expected {}, got {}", expected_value, actual_tick.value);
        },
        _ => {
            // For time and categorical scales, we mainly care about position and label
        }
    }
    
    assert!((actual_tick.position - expected.position).abs() < tolerance,
        "Tick position mismatch: expected {}, got {}", expected.position, actual_tick.position);
    
    assert_eq!(actual_tick.label, expected.label,
        "Tick label mismatch: expected '{}', got '{}'", expected.label, actual_tick.label);
}

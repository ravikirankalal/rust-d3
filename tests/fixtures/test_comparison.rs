use std::fs;
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use rust_d3::axis::*;
use rust_d3::scale::{ScaleLinear, ScaleTime};

#[derive(Debug, Serialize, Deserialize)]
struct TickReference {
    value: serde_json::Value,
    position: f64,
    label: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LinearReference {
    normal: Vec<TickReference>,
    #[serde(rename = "zeroSpan")]
    zero_span: Vec<TickReference>,
    #[serde(rename = "singleValue")]
    single_value: Vec<TickReference>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TimeReference {
    seconds: Vec<TickReference>,
    minutes: Vec<TickReference>,
    hours: Vec<TickReference>,
    days: Vec<TickReference>,
    months: Vec<TickReference>,
    years: Vec<TickReference>,
}

fn load_linear_reference() -> LinearReference {
    let json_data = fs::read_to_string("tests/fixtures/linear_reference.json")
        .expect("Should be able to read linear reference file");
    serde_json::from_str(&json_data).expect("Should be able to parse linear reference JSON")
}

fn load_time_reference() -> TimeReference {
    let json_data = fs::read_to_string("tests/fixtures/time_reference.json")
        .expect("Should be able to read time reference file");
    serde_json::from_str(&json_data).expect("Should be able to parse time reference JSON")
}

fn compare_linear_ticks() {
    let reference = load_linear_reference();
    
    println!("=== LINEAR SCALE COMPARISON ===\n");
    
    // Normal domain [0, 10]
    println!("Normal domain [0, 10]:");
    let scale = ScaleLinear::new([0.0, 10.0], [0.0, 100.0]);
    let axis = Axis::new(scale, AxisOrientation::Bottom).tick_count(5);
    let ticks = axis.ticks();
    
    println!("  D3 Reference: {} ticks", reference.normal.len());
    println!("  Rust Implementation: {} ticks", ticks.len());
    
    for (i, (d3_tick, rust_tick)) in reference.normal.iter().zip(ticks.iter()).enumerate() {
        println!("  Tick {}: D3=[value={}, pos={}, label=\"{}\"], Rust=[value={}, pos={}, label=\"{}\"]",
                i, d3_tick.value, d3_tick.position, d3_tick.label,
                rust_tick.value, rust_tick.position, rust_tick.label);
    }
    
    // Zero span domain [0, 0]
    println!("\nZero span domain [0, 0]:");
    let scale = ScaleLinear::new([0.0, 0.0], [0.0, 100.0]);
    let axis = Axis::new(scale, AxisOrientation::Bottom);
    let ticks = axis.ticks();
    
    println!("  D3 Reference: {} ticks", reference.zero_span.len());
    println!("  Rust Implementation: {} ticks", ticks.len());
    
    for (i, (d3_tick, rust_tick)) in reference.zero_span.iter().zip(ticks.iter()).enumerate() {
        println!("  Tick {}: D3=[value={}, pos={}, label=\"{}\"], Rust=[value={}, pos={}, label=\"{}\"]",
                i, d3_tick.value, d3_tick.position, d3_tick.label,
                rust_tick.value, rust_tick.position, rust_tick.label);
    }
    
    // Single value domain [5, 5]
    println!("\nSingle value domain [5, 5]:");
    let scale = ScaleLinear::new([5.0, 5.0], [10.0, 10.0]);
    let axis = Axis::new(scale, AxisOrientation::Left).tick_count(1);
    let ticks = axis.ticks();
    
    println!("  D3 Reference: {} ticks", reference.single_value.len());
    println!("  Rust Implementation: {} ticks", ticks.len());
    
    for (i, (d3_tick, rust_tick)) in reference.single_value.iter().zip(ticks.iter()).enumerate() {
        println!("  Tick {}: D3=[value={}, pos={}, label=\"{}\"], Rust=[value={}, pos={}, label=\"{}\"]",
                i, d3_tick.value, d3_tick.position, d3_tick.label,
                rust_tick.value, rust_tick.position, rust_tick.label);
    }
}

fn compare_time_ticks() {
    let reference = load_time_reference();
    
    println!("\n=== TIME SCALE COMPARISON ===\n");
    
    // Seconds
    println!("Seconds scale [2020-01-01 00:00:00 to 2020-01-01 00:00:04]:");
    let start = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap();
    let end = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap().and_hms_opt(0, 0, 4).unwrap();
    let scale = ScaleTime::new([start, end], [0.0, 100.0]);
    let axis = Axis::new(scale, AxisOrientation::Bottom).tick_count(5);
    let ticks = axis.ticks();
    
    println!("  D3 Reference: {} ticks", reference.seconds.len());
    println!("  Rust Implementation: {} ticks", ticks.len());
    
    for (i, d3_tick) in reference.seconds.iter().enumerate() {
        if let Some(rust_tick) = ticks.get(i) {
            println!("  Tick {}: D3=[value={}, pos={}, label=\"{}\"], Rust=[value={}, pos={}, label=\"{}\"]",
                    i, d3_tick.value, d3_tick.position, d3_tick.label,
                    rust_tick.value, rust_tick.position, rust_tick.label);
        } else {
            println!("  Tick {}: D3=[value={}, pos={}, label=\"{}\"], Rust=[MISSING]",
                    i, d3_tick.value, d3_tick.position, d3_tick.label);
        }
    }
}

fn main() {
    println!("D3 vs Rust Axis Implementation Comparison\n");
    println!("==========================================\n");
    
    compare_linear_ticks();
    compare_time_ticks();
    
    println!("\n==========================================");
    println!("Comparison complete!");
}

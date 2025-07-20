use serde::{Serialize, Deserialize};
use serde_json;
use std::fs;

// Generic function to deserialize JSON files into strongly-typed structs
pub fn load_fixture<T: for<'de> Deserialize<'de>>(file_path: &str) -> T {
    let json_data = fs::read_to_string(file_path)
        .expect("Should be able to read the fixture file");
    serde_json::from_str(&json_data)
        .expect("Should be able to parse the fixture JSON")
}

// Generic function to compare reference and generated ticks
// Returns true if they match, false otherwise. Prints detailed diff on failure.
pub fn compare_ticks(reference: &[Tick], generated: &[Tick]) -> bool {
    let mut matches = true;
    
    if reference.len() != generated.len() {
        println!("❌ Tick count mismatch: Reference={}, Generated={}", reference.len(), generated.len());
        matches = false;
    }
    
    let max_len = reference.len().max(generated.len());
    for i in 0..max_len {
        match (reference.get(i), generated.get(i)) {
            (Some(ref_tick), Some(gen_tick)) => {
                if ref_tick != gen_tick {
                    println!("❌ Tick mismatch at index {}:", i);
                    println!("  Reference: value={:?}, position={}, label='{}'", 
                           ref_tick.value, ref_tick.position, ref_tick.label);
                    println!("  Generated: value={:?}, position={}, label='{}'", 
                           gen_tick.value, gen_tick.position, gen_tick.label);
                    
                    // Show specific field differences
                    if ref_tick.value != gen_tick.value {
                        println!("    ↳ Value differs");
                    }
                    if (ref_tick.position - gen_tick.position).abs() > 1e-10 {
                        println!("    ↳ Position differs by {}", (ref_tick.position - gen_tick.position).abs());
                    }
                    if ref_tick.label != gen_tick.label {
                        println!("    ↳ Label differs");
                    }
                    println!();
                    matches = false;
                }
            }
            (Some(ref_tick), None) => {
                println!("❌ Missing tick at index {}: {:?}", i, ref_tick);
                matches = false;
            }
            (None, Some(gen_tick)) => {
                println!("❌ Extra tick at index {}: {:?}", i, gen_tick);
                matches = false;
            }
            (None, None) => unreachable!(),
        }
    }
    
    if matches {
        println!("✅ All {} ticks match!", reference.len());
    }
    
    matches
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Tick {
    pub value: serde_json::Value,
    pub position: f64,
    pub label: String,
    // Optional fields for extensibility
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<serde_json::Value>,
}

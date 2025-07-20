use serde::{Deserialize, Serialize};
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

// Simple test structures
#[derive(Debug, Serialize, Deserialize)]
struct LinearReference {
    normal: Vec<Tick>,
    #[serde(rename = "zeroSpan")]
    zero_span: Vec<Tick>,
    #[serde(rename = "singleValue")]
    single_value: Vec<Tick>,
}

fn main() {
    println!("🚀 Rust D3 Fixture Helpers Demo");
    println!("=================================\n");
    
    // Demo 1: Loading fixtures
    println!("📁 Demo 1: Loading fixture files");
    println!("----------------------------------");
    
    match std::fs::metadata("tests/fixtures/linear_reference.json") {
        Ok(_) => {
            let linear_ref: LinearReference = load_fixture("tests/fixtures/linear_reference.json");
            
            println!("✅ Successfully loaded linear reference fixture");
            println!("  • Normal ticks: {} items", linear_ref.normal.len());
            println!("  • Zero span ticks: {} items", linear_ref.zero_span.len());
            println!("  • Single value ticks: {} items", linear_ref.single_value.len());
            
            if let Some(first_tick) = linear_ref.normal.first() {
                println!("  • First normal tick: {:?}", first_tick);
            }
        }
        Err(_) => {
            println!("⚠️  Fixture file not found, skipping fixture loading demo");
        }
    }
    
    // Demo 2: Successful tick comparison
    println!("\n🔍 Demo 2: Successful tick comparison");
    println!("-------------------------------------");
    
    let tick1 = Tick {
        value: serde_json::Value::Number(serde_json::Number::from(10)),
        position: 100.0,
        label: "10".to_string(),
        style: None,
    };
    
    let tick2 = Tick {
        value: serde_json::Value::Number(serde_json::Number::from(10)),
        position: 100.0,
        label: "10".to_string(),
        style: None,
    };

    let reference = vec![tick1];
    let generated = vec![tick2];
    
    let matches = compare_ticks(&reference, &generated);
    assert!(matches, "Identical ticks should match");
    
    // Demo 3: Failed tick comparison with detailed diff
    println!("\n❌ Demo 3: Failed tick comparison (detailed diff)");
    println!("------------------------------------------------");
    
    let tick1 = Tick {
        value: serde_json::Value::Number(serde_json::Number::from(5)),
        position: 50.0,
        label: "5".to_string(),
        style: None,
    };
    
    let tick2 = Tick {
        value: serde_json::Value::Number(serde_json::Number::from(6)),
        position: 65.0,
        label: "6".to_string(),
        style: None,
    };

    let reference = vec![tick1];
    let generated = vec![tick2];
    
    let matches = compare_ticks(&reference, &generated);
    println!("ℹ️  Result: {} (as expected, for demo purposes)", if matches { "match" } else { "no match" });
    
    // Demo 4: Length mismatch detection
    println!("\n📏 Demo 4: Length mismatch detection");
    println!("------------------------------------");
    
    let tick1 = Tick {
        value: serde_json::Value::Number(serde_json::Number::from(1)),
        position: 10.0,
        label: "1".to_string(),
        style: None,
    };
    
    let tick2 = Tick {
        value: serde_json::Value::Number(serde_json::Number::from(2)),
        position: 20.0,
        label: "2".to_string(),
        style: None,
    };

    let reference = vec![tick1];
    let generated = vec![tick2.clone(), tick2];  // Two ticks vs one
    
    let matches = compare_ticks(&reference, &generated);
    println!("ℹ️  Result: {} (as expected, for demo purposes)", if matches { "match" } else { "no match" });
    
    // Demo 5: Optional fields support
    println!("\n🎨 Demo 5: Optional fields support");
    println!("----------------------------------");
    
    let tick_with_style = Tick {
        value: serde_json::Value::Number(serde_json::Number::from(42)),
        position: 420.0,
        label: "42".to_string(),
        style: Some(serde_json::json!({"color": "red", "font-size": "12px"})),
    };
    
    let tick_without_style = Tick {
        value: serde_json::Value::Number(serde_json::Number::from(42)),
        position: 420.0,
        label: "42".to_string(),
        style: None,
    };
    
    println!("✅ Successfully created ticks with and without optional fields");
    println!("  • Tick with style: {:?}", tick_with_style);
    println!("  • Tick without style: {:?}", tick_without_style);
    
    let reference = vec![tick_with_style];
    let generated = vec![tick_without_style];
    
    let matches = compare_ticks(&reference, &generated);
    println!("ℹ️  Result: {} (different due to style field)", if matches { "match" } else { "no match" });
    
    println!("\n✨ Demo completed! The fixture helper module provides:");
    println!("  • Generic fixture loading from JSON files");
    println!("  • Detailed tick comparison with diff output");
    println!("  • Support for optional fields (backward compatibility)");
    println!("  • Reusable, strongly-typed structures");
}

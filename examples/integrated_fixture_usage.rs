use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

// Copy the fixture helper functions for this example
pub fn load_fixture<T: for<'de> Deserialize<'de>>(file_path: &str) -> T {
    let json_data = fs::read_to_string(file_path)
        .expect("Should be able to read the fixture file");
    serde_json::from_str(&json_data)
        .expect("Should be able to parse the fixture JSON")
}

pub fn compare_ticks(reference: &[FixtureTick], generated: &[RustTick]) -> bool {
    let mut matches = true;
    
    if reference.len() != generated.len() {
        println!("❌ Tick count mismatch: Reference={}, Generated={}", reference.len(), generated.len());
        matches = false;
    }
    
    for (i, (ref_tick, gen_tick)) in reference.iter().zip(generated.iter()).enumerate() {
        // Convert generated tick to comparable values
        let gen_value = match gen_tick.value {
            _ => serde_json::Value::Number(serde_json::Number::from_f64(gen_tick.value).unwrap()),
        };
        
        if ref_tick.value != gen_value 
            || (ref_tick.position - gen_tick.position).abs() > 1e-10
            || ref_tick.label != gen_tick.label {
            
            println!("❌ Tick mismatch at index {}:", i);
            println!("  Reference: value={:?}, position={}, label='{}'", 
                   ref_tick.value, ref_tick.position, ref_tick.label);
            println!("  Generated: value={}, position={}, label='{}'", 
                   gen_tick.value, gen_tick.position, gen_tick.label);
            
            if ref_tick.value != gen_value {
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
    
    if matches {
        println!("✅ All {} ticks match!", reference.len());
    }
    
    matches
}

// Fixture tick structure (from JSON)
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct FixtureTick {
    pub value: serde_json::Value,
    pub position: f64,
    pub label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<serde_json::Value>,
}

// Rust implementation tick structure (what the library generates)
#[derive(Debug, Clone)]
pub struct RustTick {
    pub value: f64,
    pub position: f64,
    pub label: String,
}

// Structures matching the existing fixtures
#[derive(Debug, Serialize, Deserialize)]
struct LinearReference {
    normal: Vec<FixtureTick>,
    #[serde(rename = "zeroSpan")]
    zero_span: Vec<FixtureTick>,
    #[serde(rename = "singleValue")]
    single_value: Vec<FixtureTick>,
}

fn main() {
    println!("🔧 Integrated Fixture Usage Example");
    println!("====================================\n");
    
    println!("This demonstrates how the fixture helpers can integrate with existing structures");
    println!("from test_comparison.rs while providing enhanced functionality.\n");
    
    // Load the existing linear reference fixture
    match std::fs::metadata("tests/fixtures/linear_reference.json") {
        Ok(_) => {
            let linear_ref: LinearReference = load_fixture("tests/fixtures/linear_reference.json");
            
            println!("📁 Loaded fixture: {} normal ticks", linear_ref.normal.len());
            
            // Simulate what a Rust axis implementation might generate
            let generated_ticks = vec![
                RustTick { value: 0.0, position: 0.0, label: "0".to_string() },
                RustTick { value: 2.0, position: 20.0, label: "2".to_string() },
                RustTick { value: 4.0, position: 40.0, label: "4".to_string() },
                RustTick { value: 6.0, position: 60.0, label: "6".to_string() },
                RustTick { value: 8.0, position: 80.0, label: "8".to_string() },
                RustTick { value: 10.0, position: 100.0, label: "10".to_string() },
            ];
            
            println!("🔍 Comparing reference vs generated ticks:");
            println!("------------------------------------------");
            let matches = compare_ticks(&linear_ref.normal, &generated_ticks);
            
            if matches {
                println!("\n🎉 Perfect! The Rust implementation matches the D3 reference.");
            } else {
                println!("\n⚠️  The Rust implementation differs from D3 reference.");
            }
            
            // Demonstrate failure case with different data
            println!("\n🔍 Demonstrating mismatch detection:");
            println!("------------------------------------");
            let wrong_generated_ticks = vec![
                RustTick { value: 0.0, position: 0.0, label: "0".to_string() },
                RustTick { value: 3.0, position: 25.0, label: "3".to_string() }, // Different!
                RustTick { value: 6.0, position: 50.0, label: "6".to_string() },  // Different!
            ];
            
            let _matches = compare_ticks(&linear_ref.normal[0..3], &wrong_generated_ticks);
        }
        Err(_) => {
            println!("⚠️  Fixture file not found. Make sure to run from the project root directory.");
        }
    }
    
    println!("\n✨ Key Benefits:");
    println!("  • Type-safe fixture loading");
    println!("  • Detailed comparison with specific field differences");
    println!("  • Works with existing fixture files");
    println!("  • Can compare different struct types (FixtureTick vs RustTick)");
    println!("  • Extensible for new optional fields");
}

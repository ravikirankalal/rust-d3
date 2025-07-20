use serde::{Deserialize, Serialize};

mod fixtures;
use fixtures::{load_fixture, compare_ticks, Tick};

// Simple test structures
#[derive(Debug, Serialize, Deserialize)]
struct LinearReference {
    normal: Vec<Tick>,
    #[serde(rename = "zeroSpan")]
    zero_span: Vec<Tick>,
    #[serde(rename = "singleValue")]
    single_value: Vec<Tick>,
}

#[cfg(test)]
mod demo_tests {
    use super::*;

    #[test]
    fn demo_fixture_loading() {
        println!("=== Demonstrating Fixture Loading ===");
        
        // Load linear reference fixture
        let linear_ref: LinearReference = load_fixture("tests/fixtures/linear_reference.json");
        
        // Show what we loaded
        println!("✅ Successfully loaded linear reference fixture");
        println!("  Normal ticks: {} items", linear_ref.normal.len());
        println!("  Zero span ticks: {} items", linear_ref.zero_span.len());
        println!("  Single value ticks: {} items", linear_ref.single_value.len());
        
        // Show a sample tick
        if let Some(first_tick) = linear_ref.normal.first() {
            println!("  First normal tick: {:?}", first_tick);
        }
        
        assert!(!linear_ref.normal.is_empty(), "Should load normal ticks");
        assert!(!linear_ref.zero_span.is_empty(), "Should load zero span ticks");
        assert!(!linear_ref.single_value.is_empty(), "Should load single value ticks");
    }

    #[test]
    fn demo_tick_comparison_success() {
        println!("\n=== Demonstrating Successful Tick Comparison ===");
        
        // Create identical ticks
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
    }

    #[test] 
    fn demo_tick_comparison_failure() {
        println!("\n=== Demonstrating Failed Tick Comparison (with detailed diff) ===");
        
        // Create different ticks
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
        assert!(!matches, "Different ticks should not match");
        
        println!("⚠️  As expected, the ticks don't match (this is intentional for demo)");
    }

    #[test]
    fn demo_length_mismatch() {
        println!("\n=== Demonstrating Length Mismatch Detection ===");
        
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
        assert!(!matches, "Different lengths should not match");
        
        println!("⚠️  As expected, different lengths detected (this is intentional for demo)");
    }

    #[test]
    fn demo_optional_fields() {
        println!("\n=== Demonstrating Optional Fields Support ===");
        
        // Create a tick with style information
        let tick_with_style = Tick {
            value: serde_json::Value::Number(serde_json::Number::from(42)),
            position: 420.0,
            label: "42".to_string(),
            style: Some(serde_json::json!({"color": "red", "font-size": "12px"})),
        };
        
        // Create a tick without style (backward compatibility)  
        let tick_without_style = Tick {
            value: serde_json::Value::Number(serde_json::Number::from(42)),
            position: 420.0,
            label: "42".to_string(),
            style: None,
        };
        
        println!("✅ Successfully created ticks with and without optional fields");
        println!("  Tick with style: {:?}", tick_with_style);
        println!("  Tick without style: {:?}", tick_without_style);
        
        // These should be different due to style field
        let reference = vec![tick_with_style];
        let generated = vec![tick_without_style];
        
        let matches = compare_ticks(&reference, &generated);
        assert!(!matches, "Ticks with different styles should not match");
    }
}

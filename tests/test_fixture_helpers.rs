use serde::{Deserialize, Serialize};

mod fixtures;
use fixtures::{load_fixture, compare_ticks, Tick};

// Define structures for different fixture types
#[derive(Debug, Serialize, Deserialize)]
struct LinearReference {
    normal: Vec<Tick>,
    #[serde(rename = "zeroSpan")]
    zero_span: Vec<Tick>,
    #[serde(rename = "singleValue")]
    single_value: Vec<Tick>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TimeReference {
    seconds: Vec<Tick>,
    minutes: Vec<Tick>,
    hours: Vec<Tick>,
    days: Vec<Tick>,
    months: Vec<Tick>,
    years: Vec<Tick>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AxisComprehensive {
    linear: LinearTestCases,
    log: LogTestCases,
    time: TimeTestCases,
}

#[derive(Debug, Serialize, Deserialize)]
struct LinearTestCases {
    basic_ticks: TestCase,
    custom_ticks: TestCase,
    negative_domain: TestCase,
    fractional_domain: TestCase,
    large_numbers: TestCase,
    small_numbers: TestCase,
    reverse_range: TestCase,
    empty_domain: TestCase,
    single_value: TestCase,
}

#[derive(Debug, Serialize, Deserialize)]
struct LogTestCases {
    basic_ticks: LogTestCase,
    multiple_decades: LogTestCase,
    base_2: LogTestCase,
    custom_values: LogTestCase,
}

#[derive(Debug, Serialize, Deserialize)]
struct TimeTestCases {
    seconds_interval: TestCase,
    minutes_interval: TestCase,
    // Add more time test cases as needed
}

#[derive(Debug, Serialize, Deserialize)]
struct TestCase {
    domain: [f64; 2],
    range: [f64; 2],
    tick_count: Option<u32>,
    tick_values: Option<Vec<f64>>,
    expected: Vec<Tick>,
}

#[derive(Debug, Serialize, Deserialize)]
struct LogTestCase {
    domain: [f64; 2],
    range: [f64; 2],
    base: Option<f64>,
    tick_count: Option<u32>,
    tick_values: Option<Vec<f64>>,
    expected: Vec<Tick>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_linear_reference() {
        let linear_ref: LinearReference = load_fixture("tests/fixtures/linear_reference.json");
        
        // Test that we can load the fixture correctly
        assert!(!linear_ref.normal.is_empty());
        assert!(!linear_ref.zero_span.is_empty());
        assert!(!linear_ref.single_value.is_empty());
        
        // Print some info about what we loaded
        println!("Loaded linear reference with {} normal ticks", linear_ref.normal.len());
        println!("First normal tick: {:?}", linear_ref.normal[0]);
    }

    #[test]
    fn test_load_time_reference() {
        let time_ref: TimeReference = load_fixture("tests/fixtures/time_reference.json");
        
        // Test that we can load the fixture correctly
        assert!(!time_ref.seconds.is_empty());
        assert!(!time_ref.minutes.is_empty());
        assert!(!time_ref.hours.is_empty());
        
        // Print some info about what we loaded
        println!("Loaded time reference with {} seconds ticks", time_ref.seconds.len());
        println!("First seconds tick: {:?}", time_ref.seconds[0]);
    }

    #[test]
    fn test_load_comprehensive_fixture() {
        let comprehensive: AxisComprehensive = load_fixture("tests/fixtures/axis_comprehensive.json");
        
        // Test that we can load the comprehensive fixture
        assert!(!comprehensive.linear.basic_ticks.expected.is_empty());
        
        // Print some info about what we loaded
        println!("Loaded comprehensive fixture with {} basic linear ticks", 
                 comprehensive.linear.basic_ticks.expected.len());
        println!("First basic tick: {:?}", comprehensive.linear.basic_ticks.expected[0]);
    }

    #[test]
    fn test_compare_ticks_identical() {
        let tick1 = Tick {
            value: serde_json::Value::Number(serde_json::Number::from(5)),
            position: 50.0,
            label: "5".to_string(),
            style: None,
        };
        let tick2 = Tick {
            value: serde_json::Value::Number(serde_json::Number::from(5)),
            position: 50.0,
            label: "5".to_string(),
            style: None,
        };

        let reference = vec![tick1];
        let generated = vec![tick2];
        
        // This should return true and print success
        let matches = compare_ticks(&reference, &generated);
        assert!(matches, "Identical ticks should match");
    }

    #[test]
    fn test_compare_ticks_different() {
        let tick1 = Tick {
            value: serde_json::Value::Number(serde_json::Number::from(5)),
            position: 50.0,
            label: "5".to_string(),
            style: None,
        };
        let tick2 = Tick {
            value: serde_json::Value::Number(serde_json::Number::from(6)),
            position: 60.0,
            label: "6".to_string(),
            style: None,
        };

        let reference = vec![tick1];
        let generated = vec![tick2];
        
        // This should print mismatches
        compare_ticks(&reference, &generated);
        println!("Different ticks comparison completed");
    }

    #[test]
    fn test_compare_ticks_different_lengths() {
        let tick1 = Tick {
            value: serde_json::Value::Number(serde_json::Number::from(5)),
            position: 50.0,
            label: "5".to_string(),
            style: None,
        };
        let tick2 = Tick {
            value: serde_json::Value::Number(serde_json::Number::from(6)),
            position: 60.0,
            label: "6".to_string(),
            style: None,
        };

        let reference = vec![tick1];
        let generated = vec![tick2.clone(), tick2];
        
        // This should print length mismatch
        compare_ticks(&reference, &generated);
        println!("Different length ticks comparison completed");
    }
}

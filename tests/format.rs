//! Unit tests for d3 format
use rust_d3::format::{format, format_comma, format_fixed, format_prefix, FormatSpecifier};

#[test]
fn test_format_basic() {
    let f = format(".2");
    assert_eq!(f(3.14159), "3.14");
}

#[test]
fn test_format_comma() {
    assert_eq!(format_comma(1234567), "1,234,567");
}

#[test]
fn test_format_comma_spec() {
    let f = format(",");
    assert_eq!(f(12345.0), "12,345");
}

#[test]
fn test_format_fixed() {
    assert_eq!(format_fixed(3.14159, 2), "3.14");
    assert_eq!(format_fixed(2.0, 0), "2");
}

#[test]
fn test_format_percent() {
    let f = format("%");
    assert_eq!(f(0.25), "25%".to_string());
}

#[test]
fn test_format_prefix() {
    let f = format_prefix("", 12345.0);
    assert_eq!(f(12345.0), "12.35k");
    let f2 = format_prefix("", 1_234_567.0);
    assert_eq!(f2(1_234_567.0), "1.23M");
}

#[test]
fn test_format_specifier_parse() {
    let spec = FormatSpecifier::parse(".2,");
    assert_eq!(spec.precision, Some(2));
    assert!(spec.comma);
    assert!(!spec.percent);
}

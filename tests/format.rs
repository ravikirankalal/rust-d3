//! Tests for d3-format Rust port (D3.js API parity)

use rust_d3::format::*;

#[test]
fn test_basic_format() {
    let f = format(".2f").unwrap();
    assert_eq!(f(3.14159), "3.14"); // TODO: update when logic is implemented
}

#[test]
fn test_format_specifier_parse() {
    let spec = FormatSpecifier::parse(",.2f").unwrap();
    assert_eq!(spec.precision, Some(2));
    assert_eq!(spec.comma, true);
    assert_eq!(spec.ty, Some('f'));
}

#[test]
fn test_format_prefix() {
    let s = format_prefix(".2s", 12345.0).unwrap();
    assert_eq!(s, "12.35k");
    let s = format_prefix(".1s", 0.00123).unwrap();
    assert_eq!(s, "1.2m");
    let s = format_prefix(">8.2s", 12345.0).unwrap();
    assert_eq!(s, "  12.35k");
}

#[test]
fn test_sign_format() {
    let f = format("+.2f").unwrap();
    assert_eq!(f(3.14), "+3.14");
    assert_eq!(f(-3.14), "-3.14");
    let f = format(" .2f").unwrap();
    assert_eq!(f(3.14), " 3.14");
    let f = format("(.2f").unwrap();
    assert_eq!(f(-3.14), "(3.14)");
}

#[test]
fn test_width_fill_align() {
    let f = format(">8.2f").unwrap();
    assert_eq!(f(3.14), "    3.14");
    let f = format("<8.2f").unwrap();
    assert_eq!(f(3.14), "3.14    ");
    let f = format("^8.2f").unwrap();
    assert_eq!(f(3.14), "  3.14  ");
    let f = format("*<8.2f").unwrap();
    assert_eq!(f(3.14), "3.14****");
}

#[test]
fn test_si_prefix() {
    let f = format(".2s").unwrap();
    assert_eq!(f(1234.0), "1.23k");
    assert_eq!(f(1_234_567.0), "1.23M");
    assert_eq!(f(0.00123), "1.23m");
    assert_eq!(f(-1234.0), "-1.23k");
}

#[test]
fn test_percent_format() {
    let f = format(".2%").unwrap();
    assert_eq!(f(0.1234), "12.34%");
    let f = format("%").unwrap();
    assert_eq!(f(0.5), "50%" );
}

#[test]
fn test_exponential_format() {
    let f = format(".2e").unwrap();
    assert_eq!(f(1234.0), "1.23e3");
    let f = format("e").unwrap();
    assert!(f(0.00123).contains("e"));
}

#[test]
fn test_general_format() {
    let f = format(".2g").unwrap();
    assert_eq!(f(1234.0), "1.2e3"); // Rust's general format uses scientific for large numbers
    let f = format("g").unwrap();
    assert_eq!(f(12.34), "12.34");
}

#[test]
fn test_trim_specifier() {
    let f = format("~.4f").unwrap();
    assert_eq!(f(3.1400), "3.14");
    let f = format("~.4f").unwrap();
    assert_eq!(f(3.0000), "3");
    let f = format("~.4f").unwrap();
    assert_eq!(f(3.1234), "3.1234");
}

#[test]
fn test_zero_padding() {
    let f = format("08.2f").unwrap();
    assert_eq!(f(3.14), "00003.14");
    let f = format("0>8.2f").unwrap();
    assert_eq!(f(-3.14), "-0003.14");
    let f = format("0=8.2f").unwrap();
    assert_eq!(f(-3.14), "-0003.14");
    let f = format("0^8.2f").unwrap();
    assert_eq!(f(3.14), "003.1400");
}

// TODO: Add tests for all D3.js format features, edge cases, and error handling.

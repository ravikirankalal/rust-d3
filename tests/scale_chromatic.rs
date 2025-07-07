//! Tests for d3-scale-chromatic API parity
use rust_d3::scale_chromatic::*;

#[test]
fn test_scheme_category10() {
    let scheme = scheme_category10();
    assert_eq!(scheme.len(), 10);
    assert_eq!(scheme[0], "#1f77b4");
}

#[test]
fn test_scheme_accent() {
    let scheme = scheme_accent();
    assert_eq!(scheme.len(), 8);
    assert_eq!(scheme[1], "#beaed4");
}

#[test]
fn test_interpolate_viridis() {
    assert_eq!(interpolate_viridis(0.0), "#440154");
    assert_eq!(interpolate_viridis(1.0), "#35b779");
}

#[test]
fn test_interpolate_inferno() {
    assert_eq!(interpolate_inferno(0.0), "#000004");
    assert_eq!(interpolate_inferno(1.0), "#fca50a");
}

#[test]
fn test_interpolate_plasma() {
    assert_eq!(interpolate_plasma(0.0), "#0d0887");
    assert_eq!(interpolate_plasma(1.0), "#f0f921");
}

#[test]
fn test_interpolate_magma() {
    assert_eq!(interpolate_magma(0.0), "#000004");
    assert_eq!(interpolate_magma(1.0), "#fcfdbf");
}

//! D3 Geo Advanced module
//! Advanced projections and path generators for D3.js API parity.

use std::f64::consts::FRAC_PI_2;

/// D3.js: d3.geoMercator
pub fn geo_mercator(lon: f64, lat: f64) -> (f64, f64) {
    // Mercator projection (spherical, not ellipsoidal)
    let x = lon.to_radians();
    let y = (lat.to_radians() / 2.0 + FRAC_PI_2 / 2.0).tan().ln();
    (x, y)
}

/// D3.js: d3.geoOrthographic
pub fn geo_orthographic(lon: f64, lat: f64, center_lon: f64, center_lat: f64, radius: f64) -> Option<(f64, f64)> {
    // Orthographic projection (returns None if point is not visible)
    let lon = lon.to_radians();
    let lat = lat.to_radians();
    let center_lon = center_lon.to_radians();
    let center_lat = center_lat.to_radians();
    let cos_c = (center_lat.sin() * lat.sin()) + (center_lat.cos() * lat.cos() * (lon - center_lon).cos());
    if cos_c < 0.0 {
        return None; // Point is on the far side of the globe
    }
    let x = radius * lat.cos() * (lon - center_lon).sin();
    let y = radius * (center_lat.cos() * lat.sin() - center_lat.sin() * lat.cos() * (lon - center_lon).cos());
    Some((x, y))
}

/// D3.js: d3.geoPath (advanced, minimal SVG path for a polyline)
pub fn geo_path_advanced(coords: &[(f64, f64)]) -> String {
    if coords.is_empty() {
        return String::new();
    }
    let mut d = format!("M {} {}", coords[0].0, coords[0].1);
    for &(x, y) in &coords[1..] {
        d.push_str(&format!(" L {} {}", x, y));
    }
    d
}

/// Placeholder for geo advanced functionality.
pub fn geo_projection_placeholder() -> &'static str {
    "geo projection not implemented"
}

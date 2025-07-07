//! D3 Geo Projections module
//! Various map projections for D3.js API parity.

/// D3.js: d3.geoAlbers (stub)
pub fn geo_albers_placeholder() -> &'static str {
    "geoAlbers not implemented"
}

/// D3.js: d3.geoAzimuthalEqualArea (stub)
pub fn geo_azimuthal_equal_area_placeholder() -> &'static str {
    "geoAzimuthalEqualArea not implemented"
}

/// D3.js: d3.geoConicEqualArea (stub)
pub fn geo_conic_equal_area_placeholder() -> &'static str {
    "geoConicEqualArea not implemented"
}

/// D3.js: d3.geoEquirectangular (simple implementation)
pub fn geo_equirectangular(lon: f64, lat: f64) -> (f64, f64) {
    (lon.to_radians(), lat.to_radians())
}

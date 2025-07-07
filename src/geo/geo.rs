//! D3 Geo module
//! Equirectangular projection with scaling and translation.

pub struct Equirectangular {
    pub scale: f64,
    pub translate: (f64, f64),
}

impl Equirectangular {
    pub fn new(scale: f64, translate: (f64, f64)) -> Self {
        Self { scale, translate }
    }

    pub fn project(&self, lon: f64, lat: f64) -> (f64, f64) {
        let x = self.scale * lon.to_radians() + self.translate.0;
        let y = self.scale * -lat.to_radians() + self.translate.1;
        (x, y)
    }
}

pub fn equirectangular(lon: f64, lat: f64) -> (f64, f64) {
    // Default: scale=1, translate=(0,0)
    Equirectangular::new(1.0, (0.0, 0.0)).project(lon, lat)
}

/// Placeholder for d3-geo API parity.
/// See: https://github.com/d3/d3-geo#api-reference
/// TODO: Implement full API parity with d3-geo (geoPath, geoProjection, geoArea, geoLength, geoDistance, etc.)

use std::f64::consts::PI;

pub fn geo_path(coords: &[(f64, f64)]) -> String {
    // SVG path string for a sequence of points
    if coords.is_empty() {
        return String::new();
    }
    let mut d = format!("M {} {}", coords[0].0, coords[0].1);
    for &(x, y) in &coords[1..] {
        d.push_str(&format!(" L {} {}", x, y));
    }
    d
}

pub fn geo_area(_coords: &[(f64, f64)]) -> f64 {
    // Placeholder: polygon area on sphere (not accurate for large areas)
    0.0
}

pub fn geo_length(coords: &[(f64, f64)]) -> f64 {
    coords.windows(2).map(|w| geo_distance(w[0].0, w[0].1, w[1].0, w[1].1)).sum()
}

pub fn geo_distance(lon1: f64, lat1: f64, lon2: f64, lat2: f64) -> f64 {
    // Haversine formula
    let r = 6371.0; // Earth radius in km
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let a = (dlat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    r * c
}

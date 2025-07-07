//! D3 Geo module
//! Equirectangular projection with scaling and translation.

pub trait Projection {
    fn project(&self, lon: f64, lat: f64) -> (f64, f64);
}

pub struct Equirectangular {
    pub scale: f64,
    pub translate: (f64, f64),
}

impl Equirectangular {
    pub fn new(scale: f64, translate: (f64, f64)) -> Self {
        Self { scale, translate }
    }
}

impl Projection for Equirectangular {
    fn project(&self, lon: f64, lat: f64) -> (f64, f64) {
        let x = self.scale * lon.to_radians() + self.translate.0;
        let y = self.scale * -lat.to_radians() + self.translate.1;
        (x, y)
    }
}

pub fn equirectangular(lon: f64, lat: f64) -> (f64, f64) {
    // Default: scale=1, translate=(0,0)
    Equirectangular::new(1.0, (0.0, 0.0)).project(lon, lat)
}

pub struct Mercator {
    pub scale: f64,
    pub translate: (f64, f64),
}

impl Mercator {
    pub fn new(scale: f64, translate: (f64, f64)) -> Self {
        Self { scale, translate }
    }
}

impl Projection for Mercator {
    fn project(&self, lon: f64, lat: f64) -> (f64, f64) {
        let x = self.scale * lon.to_radians() + self.translate.0;
        let y = self.scale * ((std::f64::consts::PI / 4.0) + (lat.to_radians() / 2.0)).tan().ln() + self.translate.1;
        (x, y)
    }
}

pub fn mercator() -> Mercator {
    Mercator::new(156.96, (480.0, 250.0))
}

pub struct GeoPathGenerator<F> {
    projection: F,
}

impl<F>
    GeoPathGenerator<F>
where
    F: Fn((f64, f64)) -> (f64, f64),
{
    pub fn new(projection: F) -> Self {
        Self { projection }
    }
    /// Accepts a polygon as Vec<Vec<(f64, f64)>> (like GeoJSON coordinates)
    pub fn path(&self, polygon: &[Vec<(f64, f64)>]) -> String {
        let mut d = String::new();
        for ring in polygon {
            let mut iter = ring.iter();
            if let Some(&first) = iter.next() {
                let (x, y) = (self.projection)(first);
                d += &format!("M{x},{y}", x = x, y = y);
                for &pt in iter {
                    let (x, y) = (self.projection)(pt);
                    d += &format!("L{x},{y}", x = x, y = y);
                }
                d += "Z";
            }
        }
        d
    }
}

pub fn geo_path_generator<F>(projection: F) -> GeoPathGenerator<F>
where
    F: Fn((f64, f64)) -> (f64, f64),
{
    GeoPathGenerator::new(projection)
}
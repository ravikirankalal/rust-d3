mod geo;
pub use geo::{equirectangular, Equirectangular};

pub struct GeoPathGenerator<F> {
    projection: F,
}

impl<F> GeoPathGenerator<F>
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

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

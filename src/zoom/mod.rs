//! D3 Zoom module
//! Provides pan/zoom behavior for selections (see d3-zoom in JS).

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Zoom {
    pub scale: f64,
    pub translate: f64,
    pub min_scale: f64,
    pub max_scale: f64,
}

impl Zoom {
    pub fn new(min_scale: f64, max_scale: f64) -> Self {
        Self { scale: 1.0, translate: 0.0, min_scale, max_scale }
    }

    pub fn zoom_identity() -> Self {
        Zoom { scale: 1.0, translate: 0.0, min_scale: 1.0, max_scale: 10.0 }
    }

    pub fn zoom_transform(&self) -> (f64, f64) {
        (self.scale, self.translate)
    }

    pub fn zoom_scale_extent(&mut self, min: f64, max: f64) {
        self.min_scale = min;
        self.max_scale = max;
        self.scale = self.scale.clamp(self.min_scale, self.max_scale);
    }

    pub fn zoom_translate_extent(&mut self, min: f64, max: f64) {
        // For simplicity, just clamp translate to [min, max]
        self.translate = self.translate.clamp(min, max);
    }

    pub fn on<F>(&mut self, _event: &str, _listener: F)
    where F: Fn() + 'static {
        // Event system stub: no-op for now
    }

    pub fn zoom_by(&mut self, factor: f64) {
        let new_scale = (self.scale * factor).clamp(self.min_scale, self.max_scale);
        self.scale = new_scale;
    }

    pub fn pan_by(&mut self, delta: f64) {
        self.translate += delta;
    }

    pub fn reset(&mut self) {
        self.scale = 1.0;
        self.translate = 0.0;
    }
}

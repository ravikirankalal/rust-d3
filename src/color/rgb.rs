use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub opacity: f32,
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8, opacity: f32) -> Self {
        Rgb { r, g, b, opacity }
    }

    pub fn displayable(&self) -> bool {
        self.opacity >= 0.0 && self.opacity <= 1.0
    }

    pub fn brighter(&self, k: Option<f32>) -> Self {
        let k = k.unwrap_or(1.0);
        let t = 1.0 / 0.7_f32.powf(k);
        Rgb::new(
            ((self.r as f32 * t).round() as u8).min(255),
            ((self.g as f32 * t).round() as u8).min(255),
            ((self.b as f32 * t).round() as u8).min(255),
            self.opacity,
        )
    }

    pub fn darker(&self, k: Option<f32>) -> Self {
        let k = k.unwrap_or(1.0);
        let t = 0.7_f32.powf(k);
        Rgb::new(
            ((self.r as f32 * t).round() as u8).max(0),
            ((self.g as f32 * t).round() as u8).max(0),
            ((self.b as f32 * t).round() as u8).max(0),
            self.opacity,
        )
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.opacity == 1.0 {
            write!(f, "rgb({},{},{})", self.r, self.g, self.b)
        } else {
            write!(f, "rgba({},{},{},{})", self.r, self.g, self.b, self.opacity)
        }
    }
}

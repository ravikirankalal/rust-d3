
// src/color/rgb.rs

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

    // Add methods specific to Rgb here, e.g., to_hex(), to_string()
}

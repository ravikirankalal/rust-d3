
// src/color/hsl.rs

#[derive(Debug, Clone, PartialEq)]
pub struct Hsl {
    pub h: f32,
    pub s: f32,
    pub l: f32,
    pub opacity: f32,
}

impl Hsl {
    pub fn new(h: f32, s: f32, l: f32, opacity: f32) -> Self {
        Hsl { h, s, l, opacity }
    }

    // Add methods specific to Hsl here
}

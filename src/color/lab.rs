
// src/color/lab.rs

#[derive(Debug, Clone, PartialEq)]
pub struct Lab {
    pub l: f32,
    pub a: f32,
    pub b: f32,
    pub opacity: f32,
}

impl Lab {
    pub fn new(l: f32, a: f32, b: f32, opacity: f32) -> Self {
        Lab { l, a, b, opacity }
    }

    // Add methods specific to Lab here
}

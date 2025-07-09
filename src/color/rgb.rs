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
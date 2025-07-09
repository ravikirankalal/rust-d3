use std::fmt;

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
}

impl fmt::Display for Lab {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.opacity == 1.0 {
            write!(f, "lab({},{},{})", self.l, self.a, self.b)
        } else {
            write!(f, "laba({},{},{},{})", self.l, self.a, self.b, self.opacity)
        }
    }
}
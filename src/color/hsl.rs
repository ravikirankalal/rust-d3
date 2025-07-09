use std::fmt;

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
}

impl fmt::Display for Hsl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.opacity == 1.0 {
            write!(f, "hsl({},{}%,{}%)", self.h, self.s, self.l)
        } else {
            write!(f, "hsla({},{}%,{}%,{})", self.h, self.s, self.l, self.opacity)
        }
    }
}
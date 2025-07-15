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

    pub fn brighter(&self, k: Option<f32>) -> Self {
        let k = k.unwrap_or(1.0);
        let t = 1.0 / 0.7_f32.powf(k);
        Lab::new((self.l * t).min(100.0), self.a, self.b, self.opacity)
    }

    pub fn darker(&self, k: Option<f32>) -> Self {
        let k = k.unwrap_or(1.0);
        let t = 0.7_f32.powf(k);
        Lab::new((self.l * t).max(0.0), self.a, self.b, self.opacity)
    }

    pub fn interpolate(&self, other: &Lab, t: f32) -> Self {
        Lab::new(
            self.l + (other.l - self.l) * t,
            self.a + (other.a - self.a) * t,
            self.b + (other.b - self.b) * t,
            self.opacity + (other.opacity - self.opacity) * t,
        )
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

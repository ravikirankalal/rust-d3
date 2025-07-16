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

    pub fn brighter(&self, k: Option<f32>) -> Self {
        let k = k.unwrap_or(1.0);
        let t = 1.0 / 0.7_f32.powf(k);
        Hsl::new(self.h, self.s, (self.l * t).min(100.0), self.opacity)
    }

    pub fn darker(&self, k: Option<f32>) -> Self {
        let k = k.unwrap_or(1.0);
        let t = 0.7_f32.powf(k);
        Hsl::new(self.h, self.s, (self.l * t).max(0.0), self.opacity)
    }
}

impl fmt::Display for Hsl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.opacity == 1.0 {
            write!(f, "hsl({},{}%,{}%)", self.h, self.s, self.l)
        } else {
            write!(
                f,
                "hsla({},{}%,{}%,{})",
                self.h, self.s, self.l, self.opacity
            )
        }
    }
}

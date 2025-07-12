// src/color/hcl.rs

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Hcl {
    pub h: f32,
    pub c: f32,
    pub l: f32,
    pub opacity: f32,
}

impl Hcl {
    pub fn new(h: f32, c: f32, l: f32, opacity: f32) -> Self {
        Hcl { h, c, l, opacity }
    }

    pub fn brighter(&self, k: Option<f32>) -> Self {
        let k = k.unwrap_or(1.0);
        let l = self.l + 18.0 * k;
        Hcl::new(
            self.h,
            self.c,
            l.min(100.0),
            self.opacity,
        )
    }

    pub fn darker(&self, k: Option<f32>) -> Self {
        let k = k.unwrap_or(1.0);
        let l = self.l - 18.0 * k;
        Hcl::new(
            self.h,
            self.c,
            l.max(0.0),
            self.opacity,
        )
    }

    pub fn opacity(&self, value: f32) -> Self {
        Hcl::new(self.h, self.c, self.l, value)
    }

    pub fn clamp(&self) -> Self {
        Hcl::new(
            self.h.rem_euclid(360.0),
            self.c.max(0.0),
            self.l.max(0.0).min(100.0),
            self.opacity.max(0.0).min(1.0),
        )
    }
}

impl fmt::Display for Hcl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.opacity == 1.0 {
            write!(f, "hcl({},{},{})", self.h, self.c, self.l)
        } else {
            write!(f, "hcla({},{},{},{})", self.h, self.c, self.l, self.opacity)
        }
    }
}

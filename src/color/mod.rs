// Color module root

mod scale;

pub use scale::ColorScale;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn hsl(h: f64, s: f64, l: f64) -> Self {
        // Convert HSL to RGB
        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let h_ = h / 60.0;
        let x = c * (1.0 - ((h_ % 2.0) - 1.0).abs());
        let (r1, g1, b1) = match h_ as u32 {
            0 => (c, x, 0.0),
            1 => (x, c, 0.0),
            2 => (0.0, c, x),
            3 => (0.0, x, c),
            4 => (x, 0.0, c),
            5 | _ => (c, 0.0, x),
        };
        let m = l - c / 2.0;
        let (r, g, b) = (r1 + m, g1 + m, b1 + m);
        Color {
            r: (r * 255.0).round().clamp(0.0, 255.0) as u8,
            g: (g * 255.0).round().clamp(0.0, 255.0) as u8,
            b: (b * 255.0).round().clamp(0.0, 255.0) as u8,
        }
    }
    pub fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

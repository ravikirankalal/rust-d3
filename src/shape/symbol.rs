// d3-shape: symbol generator
// Implements: type, size; symbol types: circle, cross, diamond, square, star, triangle, wye

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SymbolType {
    Circle,
    Cross,
    Diamond,
    Square,
    Star,
    Triangle,
    Wye,
}

impl Default for SymbolType {
    fn default() -> Self {
        SymbolType::Circle
    }
}

pub struct Symbol {
    symbol_type: SymbolType,
    size: f64, // area in square pixels
}

impl Default for Symbol {
    fn default() -> Self {
        Symbol {
            symbol_type: SymbolType::Circle,
            size: 64.0,
        }
    }
}

impl Symbol {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn symbol_type(mut self, symbol_type: SymbolType) -> Self {
        self.symbol_type = symbol_type;
        self
    }
    pub fn size(mut self, size: f64) -> Self {
        self.size = size;
        self
    }
    pub fn to_path(&self) -> String {
        if !self.size.is_finite() || self.size <= 0.0 {
            return String::new();
        }
        match self.symbol_type {
            SymbolType::Circle => symbol_circle(self.size),
            SymbolType::Cross => symbol_cross(self.size),
            SymbolType::Diamond => symbol_diamond(self.size),
            SymbolType::Square => symbol_square(self.size),
            SymbolType::Star => symbol_star(self.size),
            SymbolType::Triangle => symbol_triangle(self.size),
            SymbolType::Wye => symbol_wye(self.size),
        }
    }
}

// --- Symbol path generators ---
use std::f64::consts::PI;

fn symbol_circle(size: f64) -> String {
    let r = (size / PI).sqrt();
    format!("M{},0A{},{} 0 1,1 {},0A{},{} 0 1,1 {},0Z", r, r, r, -r, r, r, r)
}

fn symbol_cross(size: f64) -> String {
    let s = (size / 5.0).sqrt();
    let r = s / 2.0;
    // D3 cross: vertical and horizontal bar
    format!(
        "M{},{}H{}V{}H{}V{}H{}V{}Z",
        -3.0*r, -r, -r, r, 3.0*r, r, r, -r
    )
}

fn symbol_diamond(size: f64) -> String {
    let y = (size / (2.0 * (3.0f64).sqrt())).sqrt();
    let x = y * (3.0f64).sqrt();
    format!("M0,{}L{},0L0,{}L{},0Z", -y, x, y, -x)
}

fn symbol_square(size: f64) -> String {
    let w = size.sqrt() / 2.0;
    format!("M{},{}H{}V{}H{}Z", -w, -w, w, w, -w)
}

fn symbol_star(size: f64) -> String {
    let ka = 0.8908130915292852;
    let kr = (1.0 / 2.0) * (1.0 + (5.0f64).sqrt());
    let r = (size * ka).sqrt();
    let a = PI / 5.0;
    let mut path = String::new();
    for i in 0..10 {
        let radius = if i % 2 == 0 { r } else { r / kr };
        let angle = i as f64 * a - PI / 2.0;
        let x = radius * angle.cos();
        let y = radius * angle.sin();
        if i == 0 {
            path.push_str(&format!("M{},{}", x, y));
        } else {
            path.push_str(&format!("L{},{}", x, y));
        }
    }
    path.push('Z');
    path
}

fn symbol_triangle(size: f64) -> String {
    let y = -((size / (3.0f64).sqrt()) as f64).sqrt();
    let x = y * (3.0f64).sqrt();
    format!("M0,{}L{},{}L{},{}Z", y, x, -y/2.0, -x, -y/2.0)
}

fn symbol_wye(size: f64) -> String {
    let s = (size / (3.0f64.sqrt() * 6.0)).sqrt();
    let a = s / 2.0;
    let b = a * (3.0f64).sqrt();
    // D3's wye is a 3-armed shape; here is a simple approximation
    format!("M0,{}L{},{}M0,{}L{},{}M0,{}L{},{}Z", -b, a, b, -b, -a, b, b, 0.0, -b)
}

pub trait SymbolOutput {
    fn move_to(&mut self, x: f64, y: f64);
    fn arc_to(&mut self, rx: f64, ry: f64, x: f64);
    fn line_to(&mut self, x: f64, y: f64);
    fn close(&mut self) {}
}

impl SymbolOutput for String {
    fn move_to(&mut self, x: f64, y: f64) {
        self.push_str(&format!("M{},{}", x, y));
    }
    fn arc_to(&mut self, rx: f64, ry: f64, x: f64) {
        self.push_str(&format!("A{},{} 0 1,1 {},0", rx, ry, x));
    }
    fn line_to(&mut self, x: f64, y: f64) {
        self.push_str(&format!("L{},{}", x, y));
    }
}

impl Symbol {
    pub fn to_custom<O: SymbolOutput>(&self, out: &mut O) {
        if !self.size.is_finite() || self.size <= 0.0 {
            return;
        }
        match self.symbol_type {
            SymbolType::Circle => {
                let r = (self.size / PI).sqrt();
                out.move_to(r, 0.0);
                out.arc_to(r, r, -r);
                out.arc_to(r, r, r);
                out.close();
            },
            SymbolType::Square => {
                let w = self.size.sqrt() / 2.0;
                out.move_to(-w, -w);
                out.line_to(w, -w);
                out.line_to(w, w);
                out.line_to(-w, w);
                out.close();
            },
            SymbolType::Diamond => {
                let y = (self.size / (2.0 * (3.0f64).sqrt())).sqrt();
                let x = y * (3.0f64).sqrt();
                out.move_to(0.0, -y);
                out.line_to(x, 0.0);
                out.line_to(0.0, y);
                out.line_to(-x, 0.0);
                out.close();
            },
            SymbolType::Triangle => {
                let y = -((self.size / (3.0f64).sqrt()) as f64).sqrt();
                let x = y * (3.0f64).sqrt();
                out.move_to(0.0, y);
                out.line_to(x, -y/2.0);
                out.line_to(-x, -y/2.0);
                out.close();
            },
            SymbolType::Cross => {
                let s = (self.size / 5.0).sqrt();
                let r = s / 2.0;
                out.move_to(-3.0*r, -r);
                out.line_to(-r, -r);
                out.line_to(-r, -3.0*r);
                out.line_to(r, -3.0*r);
                out.line_to(r, -r);
                out.line_to(3.0*r, -r);
                out.line_to(3.0*r, r);
                out.line_to(r, r);
                out.line_to(r, 3.0*r);
                out.line_to(-r, 3.0*r);
                out.line_to(-r, r);
                out.line_to(-3.0*r, r);
                out.close();
            },
            SymbolType::Star => {
                let ka = 0.8908130915292852;
                let kr = (1.0 / 2.0) * (1.0 + (5.0f64).sqrt());
                let r = (self.size * ka).sqrt();
                let a = PI / 5.0;
                for i in 0..10 {
                    let radius = if i % 2 == 0 { r } else { r / kr };
                    let angle = i as f64 * a - PI / 2.0;
                    let x = radius * angle.cos();
                    let y = radius * angle.sin();
                    if i == 0 {
                        out.move_to(x, y);
                    } else {
                        out.line_to(x, y);
                    }
                }
                out.close();
            },
            SymbolType::Wye => {
                let s = (self.size / (3.0f64.sqrt() * 6.0)).sqrt();
                let a = s / 2.0;
                let b = a * (3.0f64).sqrt();
                for (dx, dy) in [(0.0, -b), (a, b), (-a, b)] {
                    out.move_to(0.0, 0.0);
                    out.line_to(dx, dy);
                }
                out.close();
            },
        }
    }
}

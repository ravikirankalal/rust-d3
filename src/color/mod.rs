// src/color/mod.rs

//! d3-color (Rust port)
//!
//! This module provides utilities for representing and manipulating colors.

pub mod rgb;
pub mod hsl;
pub mod lab;
pub mod convert;

use std::str::FromStr;

use rgb::Rgb;
use hsl::Hsl;
use lab::Lab;

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Rgb(Rgb),
    Hsl(Hsl),
    Lab(Lab),
    // Add other color models as needed
}

impl FromStr for Color {
    type Err = ColorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("rgb(") && s.ends_with(")") {
            let inner = &s[4..s.len() - 1];
            let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
            if parts.len() == 3 {
                let r = parts[0].parse().map_err(|_| ColorParseError)?;
                let g = parts[1].parse().map_err(|_| ColorParseError)?;
                let b = parts[2].parse().map_err(|_| ColorParseError)?;
                return Ok(Color::Rgb(Rgb::new(r, g, b, 1.0)));
            } else if parts.len() == 4 {
                let r = parts[0].parse().map_err(|_| ColorParseError)?;
                let g = parts[1].parse().map_err(|_| ColorParseError)?;
                let b = parts[2].parse().map_err(|_| ColorParseError)?;
                let a = parts[3].parse().map_err(|_| ColorParseError)?;
                return Ok(Color::Rgb(Rgb::new(r, g, b, a)));
            }
        } else if s.starts_with("rgba(") && s.ends_with(")") {
            let inner = &s[5..s.len() - 1];
            let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
            if parts.len() == 4 {
                let r = parts[0].parse().map_err(|_| ColorParseError)?;
                let g = parts[1].parse().map_err(|_| ColorParseError)?;
                let b = parts[2].parse().map_err(|_| ColorParseError)?;
                let a = parts[3].parse().map_err(|_| ColorParseError)?;
                return Ok(Color::Rgb(Rgb::new(r, g, b, a)));
            }
        } else if s.starts_with("hsl(") && s.ends_with(")") {
            let inner = &s[4..s.len() - 1];
            let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
            if parts.len() == 3 {
                let h = parts[0].parse().map_err(|_| ColorParseError)?;
                let s = parts[1].trim_end_matches('%').parse().map_err(|_| ColorParseError)?;
                let l = parts[2].trim_end_matches('%').parse().map_err(|_| ColorParseError)?;
                return Ok(Color::Hsl(Hsl::new(h, s, l, 1.0)));
            } else if parts.len() == 4 {
                let h = parts[0].parse().map_err(|_| ColorParseError)?;
                let s = parts[1].trim_end_matches('%').parse().map_err(|_| ColorParseError)?;
                let l = parts[2].trim_end_matches('%').parse().map_err(|_| ColorParseError)?;
                let a = parts[3].parse().map_err(|_| ColorParseError)?;
                return Ok(Color::Hsl(Hsl::new(h, s, l, a)));
            }
        } else if s.starts_with("hsla(") && s.ends_with(")") {
            let inner = &s[5..s.len() - 1];
            let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
            if parts.len() == 4 {
                let h = parts[0].parse().map_err(|_| ColorParseError)?;
                let s = parts[1].trim_end_matches('%').parse().map_err(|_| ColorParseError)?;
                let l = parts[2].trim_end_matches('%').parse().map_err(|_| ColorParseError)?;
                let a = parts[3].parse().map_err(|_| ColorParseError)?;
                return Ok(Color::Hsl(Hsl::new(h, s, l, a)));
            }
        } else if s.starts_with("#") && s.len() == 7 {
            let r = u8::from_str_radix(&s[1..3], 16).map_err(|_| ColorParseError)?;
            let g = u8::from_str_radix(&s[3..5], 16).map_err(|_| ColorParseError)?;
            let b = u8::from_str_radix(&s[5..7], 16).map_err(|_| ColorParseError)?;
            return Ok(Color::Rgb(Rgb::new(r, g, b, 1.0)));
        }
        Err(ColorParseError)
    }
}

#[derive(Debug)]
pub struct ColorParseError;

impl Color {
    pub fn rgb(&self) -> Rgb {
        match self {
            Color::Rgb(rgb) => rgb.clone(),
            Color::Hsl(hsl) => {
                let h = hsl.h;
                let s = hsl.s / 100.0;
                let l = hsl.l / 100.0;

                let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
                let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
                let m = l - c / 2.0;

                let (r_prime, g_prime, b_prime) = if (0.0..60.0).contains(&h) {
                    (c, x, 0.0)
                } else if (60.0..120.0).contains(&h) {
                    (x, c, 0.0)
                } else if (120.0..180.0).contains(&h) {
                    (0.0, c, x)
                } else if (180.0..240.0).contains(&h) {
                    (0.0, x, c)
                } else if (240.0..300.0).contains(&h) {
                    (x, 0.0, c)
                } else {
                    (c, 0.0, x)
                };

                Rgb::new(
                    ((r_prime + m) * 255.0).round() as u8,
                    ((g_prime + m) * 255.0).round() as u8,
                    ((b_prime + m) * 255.0).round() as u8,
                    hsl.opacity,
                )
            }
            Color::Lab(lab) => {
                let (x, y, z) = convert::lab_to_xyz(lab.l, lab.a, lab.b);
                let (r, g, b) = convert::xyz_to_rgb(x, y, z);
                Rgb::new(
                    (r * 255.0).round() as u8,
                    (g * 255.0).round() as u8,
                    (b * 255.0).round() as u8,
                    lab.opacity,
                )
            }
        }
    }

    pub fn hsl(&self) -> Hsl {
        match self {
            Color::Hsl(hsl) => hsl.clone(),
            Color::Rgb(rgb) => {
                let r = rgb.r as f32 / 255.0;
                let g = rgb.g as f32 / 255.0;
                let b = rgb.b as f32 / 255.0;

                let max = r.max(g).max(b);
                let min = r.min(g).min(b);
                let h;
                let s;
                let l = (max + min) / 2.0;

                if max == min {
                    h = 0.0;
                    s = 0.0;
                } else {
                    let d = max - min;
                    s = if l > 0.5 { d / (2.0 - max - min) } else { d / (max + min) };

                    h = if max == r {
                        (g - b) / d + (if g < b { 6.0 } else { 0.0 })
                    } else if max == g {
                        (b - r) / d + 2.0
                    } else {
                        (r - g) / d + 4.0
                    } / 6.0;
                }

                Hsl::new(
                    h * 360.0,
                    s * 100.0,
                    l * 100.0,
                    rgb.opacity,
                )
            }
            Color::Lab(lab) => {
                let rgb_color = Color::Lab(lab.clone()).rgb();
                Color::Rgb(rgb_color).hsl()
            }
        }
    }

    pub fn lab(&self) -> Lab {
        match self {
            Color::Lab(lab) => lab.clone(),
            Color::Rgb(rgb) => {
                let (x, y, z) = convert::rgb_to_xyz(rgb.r as f32, rgb.g as f32, rgb.b as f32);
                let (l, a, b) = convert::xyz_to_lab(x, y, z);
                Lab::new(l, a, b, rgb.opacity)
            }
            Color::Hsl(hsl) => {
                let rgb_color = Color::Hsl(hsl.clone()).rgb();
                Color::Rgb(rgb_color).lab()
            }
        }
    }

    pub fn brighter(&self, k: Option<f32>) -> Color {
        let k = k.unwrap_or(1.0);
        let r = self.rgb();
        let t = 1.0 / 0.7_f32.powf(k);
        Color::Rgb(
            Rgb::new(
                (r.r as f32 * t).round() as u8,
                (r.g as f32 * t).round() as u8,
                (r.b as f32 * t).round() as u8,
                r.opacity,
            )
        )
    }

    pub fn darker(&self, k: Option<f32>) -> Color {
        let k = k.unwrap_or(1.0);
        let r = self.rgb();
        let t = 0.7_f32.powf(k);
        Color::Rgb(
            Rgb::new(
                (r.r as f32 * t).round() as u8,
                (r.g as f32 * t).round() as u8,
                (r.b as f32 * t).round() as u8,
                r.opacity,
            )
        )
    }

    pub fn opacity(&self, value: f32) -> Color {
        match self {
            Color::Rgb(rgb) => Color::Rgb(Rgb::new(rgb.r, rgb.g, rgb.b, value)),
            Color::Hsl(hsl) => Color::Hsl(Hsl::new(hsl.h, hsl.s, hsl.l, value)),
            Color::Lab(lab) => Color::Lab(Lab::new(lab.l, lab.a, lab.b, value)),
        }
    }

    pub fn gamma(&self, k: f32) -> Color {
        let r = self.rgb();
        let k_inv = 1.0 / k;
        Color::Rgb(
            Rgb::new(
                ((r.r as f32 / 255.0).powf(k_inv) * 255.0).round() as u8,
                ((r.g as f32 / 255.0).powf(k_inv) * 255.0).round() as u8,
                ((r.b as f32 / 255.0).powf(k_inv) * 255.0).round() as u8,
                r.opacity,
            )
        )
    }

    pub fn clamp(&self) -> Color {
        match self {
            Color::Rgb(rgb) => Color::Rgb(Rgb::new(
                rgb.r.max(0).min(255),
                rgb.g.max(0).min(255),
                rgb.b.max(0).min(255),
                rgb.opacity.max(0.0).min(1.0),
            )),
            Color::Hsl(hsl) => Color::Hsl(Hsl::new(
                hsl.h.rem_euclid(360.0),
                hsl.s.max(0.0).min(100.0),
                hsl.l.max(0.0).min(100.0),
                hsl.opacity.max(0.0).min(1.0),
            )),
            Color::Lab(lab) => Color::Lab(Lab::new(
                lab.l.max(0.0).min(100.0),
                lab.a,
                lab.b,
                lab.opacity.max(0.0).min(1.0),
            )),
        }
    }

    pub fn format_hex(&self) -> String {
        let rgb = self.rgb();
        format!("#{:02x}{:02x}{:02x}", rgb.r, rgb.g, rgb.b)
    }

    pub fn format_rgb(&self) -> String {
        let rgb = self.rgb();
        rgb.to_string()
    }

    pub fn format_hsl(&self) -> String {
        let hsl = self.hsl();
        hsl.to_string()
    }

    pub fn format_lab(&self) -> String {
        let lab = self.lab();
        lab.to_string()
    }

    pub fn copy(&self) -> Color {
        self.clone()
    }
}


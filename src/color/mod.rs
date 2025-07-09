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
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::rgb::Rgb;
    use super::hsl::Hsl;
    use super::lab::Lab;

    #[test]
    fn test_color_from_str_rgb() {
        assert_eq!("rgb(255,0,0)".parse::<Color>().unwrap(), Color::Rgb(Rgb::new(255, 0, 0, 1.0)));
        assert_eq!("rgb(0, 128, 255)".parse::<Color>().unwrap(), Color::Rgb(Rgb::new(0, 128, 255, 1.0)));
    }

    #[test]
    fn test_color_from_str_rgba() {
        assert_eq!("rgba(255,0,0,0.5)".parse::<Color>().unwrap(), Color::Rgb(Rgb::new(255, 0, 0, 0.5)));
        assert_eq!("rgba(0, 128, 255, 1.0)".parse::<Color>().unwrap(), Color::Rgb(Rgb::new(0, 128, 255, 1.0)));
    }

    #[test]
    fn test_color_from_str_hex() {
        assert_eq!("#ff0000".parse::<Color>().unwrap(), Color::Rgb(Rgb::new(255, 0, 0, 1.0)));
        assert_eq!("#0080ff".parse::<Color>().unwrap(), Color::Rgb(Rgb::new(0, 128, 255, 1.0)));
    }

    #[test]
    fn test_color_from_str_hsl() {
        assert_eq!("hsl(0,100%,50%)".parse::<Color>().unwrap(), Color::Hsl(Hsl::new(0.0, 100.0, 50.0, 1.0)));
        assert_eq!("hsl(120,50%,75%)".parse::<Color>().unwrap(), Color::Hsl(Hsl::new(120.0, 50.0, 75.0, 1.0)));
    }

    #[test]
    fn test_color_from_str_hsla() {
        assert_eq!("hsla(0,100%,50%,0.5)".parse::<Color>().unwrap(), Color::Hsl(Hsl::new(0.0, 100.0, 50.0, 0.5)));
        assert_eq!("hsla(120,50%,75%,1.0)".parse::<Color>().unwrap(), Color::Hsl(Hsl::new(120.0, 50.0, 75.0, 1.0)));
    }

    #[test]
    fn test_color_from_str_invalid() {
        assert!("invalid".parse::<Color>().is_err());
        assert!("#123".parse::<Color>().is_err());
        assert!("rgb(1,2)".parse::<Color>().is_err());
        assert!("rgba(1,2,3)".parse::<Color>().is_err());
        assert!("hsl(1,2)".parse::<Color>().is_err());
        assert!("hsla(1,2,3)".parse::<Color>().is_err());
    }

    #[test]
    fn test_rgb_to_hsl_conversion() {
        let red = Color::Rgb(Rgb::new(255, 0, 0, 1.0));
        let red_hsl = red.hsl();
        // Allow for some floating point inaccuracies
        assert!((red_hsl.h - 0.0).abs() < 0.1);
        assert!((red_hsl.s - 100.0).abs() < 0.1);
        assert!((red_hsl.l - 50.0).abs() < 0.1);
        assert!((red_hsl.opacity - 1.0).abs() < 0.1);

        let green = Color::Rgb(Rgb::new(0, 255, 0, 1.0));
        let green_hsl = green.hsl();
        assert!((green_hsl.h - 120.0).abs() < 0.1);
        assert!((green_hsl.s - 100.0).abs() < 0.1);
        assert!((green_hsl.l - 50.0).abs() < 0.1);
        assert!((green_hsl.opacity - 1.0).abs() < 0.1);

        let blue = Color::Rgb(Rgb::new(0, 0, 255, 1.0));
        let blue_hsl = blue.hsl();
        assert!((blue_hsl.h - 240.0).abs() < 0.1);
        assert!((blue_hsl.s - 100.0).abs() < 0.1);
        assert!((blue_hsl.l - 50.0).abs() < 0.1);
        assert!((blue_hsl.opacity - 1.0).abs() < 0.1);

        let gray = Color::Rgb(Rgb::new(128, 128, 128, 1.0));
        let gray_hsl = gray.hsl();
        let expected_l = (128.0 / 255.0 + 128.0 / 255.0) / 2.0 * 100.0;
        assert!((gray_hsl.h - 0.0).abs() < 0.1);
        assert!((gray_hsl.s - 0.0).abs() < 0.1);
        assert!((gray_hsl.l - expected_l).abs() < 0.1);
        assert!((gray_hsl.opacity - 1.0).abs() < 0.1);
    }

    #[test]
    fn test_hsl_to_rgb_conversion() {
        let red_hsl = Color::Hsl(Hsl::new(0.0, 100.0, 50.0, 1.0));
        let red_rgb = red_hsl.rgb();
        assert_eq!(red_rgb, Rgb::new(255, 0, 0, 1.0));

        let green_hsl = Color::Hsl(Hsl::new(120.0, 100.0, 50.0, 1.0));
        let green_rgb = green_hsl.rgb();
        assert_eq!(green_rgb, Rgb::new(0, 255, 0, 1.0));

        let blue_hsl = Color::Hsl(Hsl::new(240.0, 100.0, 50.0, 1.0));
        let blue_rgb = blue_hsl.rgb();
        assert_eq!(blue_rgb, Rgb::new(0, 0, 255, 1.0));

        let gray_hsl = Color::Hsl(Hsl::new(0.0, 0.0, 50.2, 1.0));
        let gray_rgb = gray_hsl.rgb();
        assert_eq!(gray_rgb, Rgb::new(128, 128, 128, 1.0));
    }

    #[test]
    fn test_brighter() {
        let color = Color::Rgb(Rgb::new(128, 128, 128, 1.0));
        let brighter_color = color.brighter(None);
        assert_eq!(brighter_color, Color::Rgb(Rgb::new(183, 183, 183, 1.0)));

        let brighter_color_k2 = color.brighter(Some(2.0));
        assert_eq!(brighter_color_k2, Color::Rgb(Rgb::new(255, 255, 255, 1.0)));
    }

    #[test]
    fn test_darker() {
        let color = Color::Rgb(Rgb::new(128, 128, 128, 1.0));
        let darker_color = color.darker(None);
        assert_eq!(darker_color, Color::Rgb(Rgb::new(90, 90, 90, 1.0)));

        let darker_color_k2 = color.darker(Some(2.0));
        assert_eq!(darker_color_k2, Color::Rgb(Rgb::new(63, 63, 63, 1.0)));
    }

    #[test]
    fn test_rgb_to_lab_conversion() {
        let red = Color::Rgb(Rgb::new(255, 0, 0, 1.0));
        let red_lab = red.lab();
        assert!((red_lab.l - 53.23).abs() < 0.05);
        assert!((red_lab.a - 80.11).abs() < 0.02);
        assert!((red_lab.b - 67.22).abs() < 0.02);
        assert!((red_lab.opacity - 1.0).abs() < 0.01);

        let green = Color::Rgb(Rgb::new(0, 255, 0, 1.0));
        let green_lab = green.lab();
        assert!((green_lab.l - 87.73).abs() < 0.01);
        assert!((green_lab.a - (-86.18)).abs() < 0.01);
        assert!((green_lab.b - 83.18).abs() < 0.01);
        assert!((green_lab.opacity - 1.0).abs() < 0.01);

        let blue = Color::Rgb(Rgb::new(0, 0, 255, 1.0));
        let blue_lab = blue.lab();
        assert!((blue_lab.l - 32.30).abs() < 0.01);
        assert!((blue_lab.a - 79.19).abs() < 0.01);
        assert!((blue_lab.b - (-107.86)).abs() < 0.01);
        assert!((blue_lab.opacity - 1.0).abs() < 0.01);

        let gray = Color::Rgb(Rgb::new(128, 128, 128, 1.0));
        let gray_lab = gray.lab();
        assert!((gray_lab.l - 53.59).abs() < 0.01);
        assert!((gray_lab.a - 0.0).abs() < 0.01);
        assert!((gray_lab.b - 0.0).abs() < 0.01);
        assert!((gray_lab.opacity - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_lab_to_rgb_conversion() {
        let red_lab = Color::Lab(Lab::new(53.23, 80.11, 67.22, 1.0));
        let red_rgb = red_lab.rgb();
        assert_eq!(red_rgb, Rgb::new(255, 0, 0, 1.0));

        let green_lab = Color::Lab(Lab::new(87.73, -86.18, 83.18, 1.0));
        let green_rgb = green_lab.rgb();
        assert_eq!(green_rgb, Rgb::new(0, 255, 0, 1.0));

        let blue_lab = Color::Lab(Lab::new(32.30, 79.19, -107.86, 1.0));
        let blue_rgb = blue_lab.rgb();
        assert_eq!(blue_rgb, Rgb::new(0, 0, 255, 1.0));

        let gray_lab = Color::Lab(Lab::new(53.59, 0.0, 0.0, 1.0));
        let gray_rgb = gray_lab.rgb();
        assert_eq!(gray_rgb, Rgb::new(128, 128, 128, 1.0));
    }

    #[test]
    fn test_lab_to_hsl_conversion() {
        let red_lab = Color::Lab(Lab::new(53.23, 80.11, 67.22, 1.0));
        let red_hsl = red_lab.hsl();
        assert!((red_hsl.h - 0.0).abs() < 0.1);
        assert!((red_hsl.s - 100.0).abs() < 0.1);
        assert!((red_hsl.l - 50.0).abs() < 0.1);
        assert!((red_hsl.opacity - 1.0).abs() < 0.1);

        let green_lab = Color::Lab(Lab::new(87.73, -86.18, 83.18, 1.0));
        let green_hsl = green_lab.hsl();
        assert!((green_hsl.h - 120.0).abs() < 0.1);
        assert!((green_hsl.s - 100.0).abs() < 0.1);
        assert!((green_hsl.l - 50.0).abs() < 0.1);
        assert!((green_hsl.opacity - 1.0).abs() < 0.1);

        let blue_lab = Color::Lab(Lab::new(32.30, 79.19, -107.86, 1.0));
        let blue_hsl = blue_lab.hsl();
        assert!((blue_hsl.h - 240.0).abs() < 0.1);
        assert!((blue_hsl.s - 100.0).abs() < 0.1);
        assert!((blue_hsl.l - 50.0).abs() < 0.1);
        assert!((blue_hsl.opacity - 1.0).abs() < 0.1);

        let gray_lab = Color::Lab(Lab::new(53.59, 0.0, 0.0, 1.0));
        let gray_hsl = gray_lab.hsl();
        assert!((gray_hsl.h - 0.0).abs() < 0.1);
        assert!((gray_hsl.s - 0.0).abs() < 0.1);
        assert!((gray_hsl.l - 50.2).abs() < 0.1);
        assert!((gray_hsl.opacity - 1.0).abs() < 0.1);
    }
}
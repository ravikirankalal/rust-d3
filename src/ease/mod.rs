//! d3-ease (Rust port)
//!
//! This module ports the D3.js d3-ease API to Rust.
//! Provides a variety of easing functions for use in transitions and animations.

/// Linear easing (identity)
pub fn linear(t: f32) -> f32 {
    t
}

/// Quad easing
pub fn quad_in(t: f32) -> f32 { t * t }
pub fn quad_out(t: f32) -> f32 { t * (2.0 - t) }
pub fn quad_inout(t: f32) -> f32 { if t < 0.5 { 2.0 * t * t } else { -1.0 + (4.0 - 2.0 * t) * t } }

/// Cubic easing
pub fn cubic_in(t: f32) -> f32 { t * t * t }
pub fn cubic_out(t: f32) -> f32 { 1.0 + (t - 1.0) * (t - 1.0) * (t - 1.0) }
pub fn cubic_inout(t: f32) -> f32 { if t < 0.5 { 4.0 * t * t * t } else { (t - 1.0) * (2.0 * t - 2.0) * (2.0 * t - 2.0) + 1.0 } }

/// Sin easing
pub fn sin_in(t: f32) -> f32 { 1.0 - (t * std::f32::consts::PI / 2.0).cos() }
pub fn sin_out(t: f32) -> f32 { (t * std::f32::consts::PI / 2.0).sin() }
pub fn sin_inout(t: f32) -> f32 { -0.5 * ((std::f32::consts::PI * t).cos() - 1.0) }

/// Exp easing
pub fn exp_in(t: f32) -> f32 { if t == 0.0 { 0.0 } else { 2.0f32.powf(10.0 * (t - 1.0)) } }
pub fn exp_out(t: f32) -> f32 { if t == 1.0 { 1.0 } else { 1.0 - 2.0f32.powf(-10.0 * t) } }
pub fn exp_inout(t: f32) -> f32 {
    if t == 0.0 { 0.0 }
    else if t == 1.0 { 1.0 }
    else if t < 0.5 { 0.5 * 2.0f32.powf(20.0 * t - 10.0) }
    else { 1.0 - 0.5 * 2.0f32.powf(-20.0 * t + 10.0) }
}

/// Circle easing
pub fn circle_in(t: f32) -> f32 { 1.0 - (1.0 - t * t).sqrt() }
pub fn circle_out(t: f32) -> f32 { (1.0 - (t - 1.0) * (t - 1.0)).sqrt() }
pub fn circle_inout(t: f32) -> f32 {
    let mut t = t * 2.0;
    if t <= 1.0 {
        -0.5 * ((1.0 - t * t).sqrt() - 1.0)
    } else {
        t -= 2.0;
        0.5 * ((1.0 - t * t).sqrt() + 1.0)
    }
}

/// Back easing
pub fn back_in(t: f32) -> f32 {
    let c1 = 1.70158;
    let c3 = c1 + 1.0;
    t * t * (c3 * t - c1)
}
pub fn back_out(t: f32) -> f32 {
    let c1 = 1.70158;
    let c3 = c1 + 1.0;
    let t = t - 1.0;
    t * t * (c3 * t + c1) + 1.0
}
pub fn back_inout(t: f32) -> f32 {
    let c1 = 1.70158 * 1.525;
    if t < 0.5 {
        let t = 2.0 * t;
        0.5 * (t * t * ((c1 + 1.0) * t - c1))
    } else {
        let t = 2.0 * t - 2.0;
        0.5 * (t * t * ((c1 + 1.0) * t + c1) + 2.0)
    }
}

/// Bounce out (D3 default)
pub fn bounce_out(t: f32) -> f32 {
    if t < 1.0/2.75 {
        7.5625 * t * t
    } else if t < 2.0/2.75 {
        let t = t - 1.5/2.75;
        7.5625 * t * t + 0.75
    } else if t < 2.5/2.75 {
        let t = t - 2.25/2.75;
        7.5625 * t * t + 0.9375
    } else {
        let t = t - 2.625/2.75;
        7.5625 * t * t + 0.984375
    }
}

/// Elastic out (D3 default)
pub fn elastic_out(t: f32) -> f32 {
    if t == 0.0 || t == 1.0 {
        t
    } else {
        let p = 0.3;
        2.0f32.powf(-10.0 * t) * (t * (2.0 * std::f32::consts::PI) / p).sin() + 1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_linear() {
        assert_eq!(linear(0.0), 0.0);
        assert_eq!(linear(1.0), 1.0);
    }
    #[test]
    fn test_quad() {
        assert_eq!(quad_in(0.5), 0.25);
        assert!((quad_out(0.5) - 0.75).abs() < 1e-6);
        assert!((quad_inout(0.25) - 0.125).abs() < 1e-6);
        assert!((quad_inout(0.75) - 0.875).abs() < 1e-6);
    }
    #[test]
    fn test_cubic() {
        assert_eq!(cubic_in(0.5), 0.125);
        assert!((cubic_out(0.5) - 0.875).abs() < 1e-6);
        assert!((cubic_inout(0.25) - 0.0625).abs() < 1e-6);
        assert!((cubic_inout(0.75) - 0.9375).abs() < 1e-6);
    }
    #[test]
    fn test_sin() {
        assert!((sin_in(0.5) - (1.0 - (std::f32::consts::PI / 4.0).cos())).abs() < 1e-6);
        assert!((sin_out(0.5) - (std::f32::consts::PI / 4.0).sin()).abs() < 1e-6);
        assert!((sin_inout(0.5) - 0.5).abs() < 1e-6);
    }
    #[test]
    fn test_exp() {
        assert_eq!(exp_in(0.0), 0.0);
        assert_eq!(exp_out(1.0), 1.0);
        assert!(exp_inout(0.0).abs() < 1e-6);
        assert!((exp_inout(1.0) - 1.0).abs() < 1e-6);
    }
    #[test]
    fn test_circle() {
        assert!((circle_in(0.5) - (1.0 - (1.0_f32 - 0.25_f32).sqrt())).abs() < 1e-6);
        assert!((circle_out(0.5) - (1.0_f32 - 0.25_f32).sqrt()).abs() < 1e-6);
        assert!((circle_inout(0.5) - 0.5).abs() < 1e-6);
    }
    #[test]
    fn test_back() {
        assert!((back_in(0.5) - (-0.0876975)).abs() < 1e-5);
        assert!((back_out(0.5) - 1.0876975).abs() < 1e-5);
        assert!((back_inout(0.5) - 0.5).abs() < 1e-6);
    }
    #[test]
    fn test_bounce_out() {
        assert!((bounce_out(0.5) - 0.765625).abs() < 1e-5);
    }
    #[test]
    fn test_elastic_out() {
        assert!((elastic_out(0.5) - 1.0).abs() < 0.1);
    }
}

pub use linear as ease_linear;
pub use quad_in as ease_quad;

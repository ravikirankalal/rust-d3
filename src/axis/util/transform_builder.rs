// `TransformBuilder` struct for building SVG transforms.
// Ensures proper ordering (translate, rotate, scale) and no duplicates.

use crate::px;

#[derive(Default)]
pub struct TransformBuilder {
    existing: Option<String>,
    translate: Option<(f64, f64)>,
    rotate: Option<(f64, f64, f64)>,
    scale: Option<(f64, f64)>,
}

impl TransformBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Start with an existing transform string
    pub fn with_existing(existing: Option<String>) -> Self {
        Self {
            existing,
            ..Default::default()
        }
    }

    /// Add a translation transform
    pub fn translate(mut self, x: f64, y: f64) -> Self {
        self.translate = Some((x, y));
        self
    }

    /// Add a rotation transform
    pub fn rotate(mut self, angle: f64, cx: f64, cy: f64) -> Self {
        self.rotate = Some((angle, cx, cy));
        self
    }

    /// Add a scale transform
    pub fn scale(mut self, sx: f64, sy: f64) -> Self {
        self.scale = Some((sx, sy));
        self
    }

    /// Build the final transform string
    /// Always returns a non-empty string to mirror D3 behavior
    pub fn build(self) -> String {
        let mut parts = vec![];
        
        // Add existing transform first if present
        if let Some(existing) = self.existing {
            if !existing.trim().is_empty() {
                parts.push(existing.trim().to_string());
            }
        }
        
        // Add new transforms in proper order: translate, rotate, scale
        if let Some((x, y)) = self.translate {
            parts.push(format!("translate({},{})", px(x), px(y)));
        }
        if let Some((angle, cx, cy)) = self.rotate {
            parts.push(format!("rotate({} {} {})", px(angle), px(cx), px(cy)));
        }
        if let Some((sx, sy)) = self.scale {
            parts.push(format!("scale({},{})", px(sx), px(sy)));
        }
        
        let result = parts.join(" ").trim().to_string();
        
        // Always return a transform, even if it's just "translate(0,0)"
        // to mirror D3 behavior
        if result.is_empty() {
            "translate(0,0)".to_string()
        } else {
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_precision_formatting() {
        let transform = TransformBuilder::new()
            .translate(1.0/3.0, 2.5000)
            .rotate(45.123456789, 10.0000, 20.50000)
            .scale(1.333333333, 0.666666666)
            .build();
        
        // Should use px function formatting: round to 6 decimals, trim trailing zeros
        assert_eq!(transform, "translate(0.333333,2.5) rotate(45.123457 10 20.5) scale(1.333333,0.666667)");
    }

    #[test] 
    fn test_zero_values() {
        let transform = TransformBuilder::new()
            .translate(0.0, -0.0)
            .scale(1.000000, 2.000000)
            .build();
        
        // Should format zeros properly and trim trailing zeros
        assert_eq!(transform, "translate(0,0) scale(1,2)");
    }
}

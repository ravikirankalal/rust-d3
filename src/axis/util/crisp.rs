// Crisp pixel utilities for axis rendering
// Provides device pixel ratio aware offset calculations for crisp line rendering
//
// Design decisions:
// - Centralized offset logic to replace hard-coded 0.5 values across renderers
// - WASM/web support with device pixel ratio detection
// - Fallback to standard offset for non-web environments
// - Configurable custom offset override for special use cases

/// Default device pixel ratio detection
/// Returns the device pixel ratio for WASM/web environments, 
/// or allows override via AXIS_DPR environment variable for testing
pub fn default_device_pixel_ratio() -> f64 {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window()
            .and_then(|w| Some(w.device_pixel_ratio()))
            .unwrap_or(1.0)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Allow overriding DPR via environment variable for unit tests
        if let Ok(dpr_str) = std::env::var("AXIS_DPR") {
            dpr_str.parse().unwrap_or(1.0)
        } else {
            1.0
        }
    }
}

/// Deprecated: use default_device_pixel_ratio() instead
/// Kept for backwards compatibility
pub fn default_offset() -> f64 {
    default_device_pixel_ratio()
}

/// Calculate crisp pixel offset based on device pixel ratio
/// Returns 0.0 for high DPI displays (>1.0) and 0.5 for standard displays
pub fn crisp_offset(dpr: f64) -> f64 {
    if dpr > 1.0 {
        0.0
    } else {
        0.5
    }
}

/// Get the effective offset for crisp pixel rendering
/// Uses custom offset if provided, otherwise calculates based on device pixel ratio
pub fn effective_offset(custom: Option<f64>) -> f64 {
    match custom {
        Some(offset) => offset,
        None => crisp_offset(default_offset()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crisp_offset() {
        // Standard DPI should use 0.5 offset for crisp lines
        assert_eq!(crisp_offset(1.0), 0.5);
        
        // High DPI should use 0.0 offset
        assert_eq!(crisp_offset(2.0), 0.0);
        assert_eq!(crisp_offset(1.5), 0.0);
        
        // Edge case
        assert_eq!(crisp_offset(1.0), 0.5);
    }

    #[test]
    fn test_effective_offset() {
        // Custom offset should be used when provided
        assert_eq!(effective_offset(Some(0.25)), 0.25);
        assert_eq!(effective_offset(Some(1.0)), 1.0);
        
        // Should fall back to computed offset when None
        let computed = effective_offset(None);
        assert!(computed == 0.5 || computed == 0.0); // Depends on environment
    }

    #[test] 
    fn test_default_offset() {
        // Should return a valid DPR value
        let dpr = default_offset();
        assert!(dpr >= 1.0); // DPR should be at least 1.0
    }
    
    #[test]
    fn test_env_var_dpr_override() {
        // Test that AXIS_DPR environment variable works
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Set environment variable and test
            unsafe {
                std::env::set_var("AXIS_DPR", "2.5");
            }
            let dpr = default_device_pixel_ratio();
            assert_eq!(dpr, 2.5);
            
            // Test with standard DPI
            unsafe {
                std::env::set_var("AXIS_DPR", "1.0");
            }
            let dpr = default_device_pixel_ratio();
            assert_eq!(dpr, 1.0);
            
            // Clean up
            unsafe {
                std::env::remove_var("AXIS_DPR");
            }
        }
    }
}

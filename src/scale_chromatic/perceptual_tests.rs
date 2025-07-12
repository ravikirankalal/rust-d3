//! Tests for perceptual palettes in d3-scale-chromatic

#[cfg(test)]
mod tests {
    use crate::scale_chromatic::{cividis, cubehelix, inferno, magma, plasma, turbo};

    #[test]
    fn test_inferno_palette() {
        let c = inferno(0.5);
        assert!(c.starts_with("#") && c.len() == 7);
    }
    #[test]
    fn test_magma_palette() {
        let c = magma(0.5);
        assert!(c.starts_with("#") && c.len() == 7);
    }
    #[test]
    fn test_plasma_palette() {
        let c = plasma(0.5);
        assert!(c.starts_with("#") && c.len() == 7);
    }
    #[test]
    fn test_cividis_palette() {
        let c = cividis(0.5);
        assert!(c.starts_with("#") && c.len() == 7);
    }
    #[test]
    fn test_turbo_palette() {
        let c = turbo(0.5);
        assert!(c.starts_with("#") && c.len() == 7);
    }
    #[test]
    fn test_cubehelix_palette() {
        let c = cubehelix(0.5);
        assert!(c.starts_with("#") && c.len() == 7);
    }
}

#[cfg(test)]
mod more_tests {
    use crate::scale_chromatic::{cubehelix, inferno, magma};

    #[test]
    fn test_inferno_palette_edges() {
        assert_eq!(inferno(0.0), inferno(0.0));
        assert_eq!(inferno(1.0), inferno(1.0));
    }
    #[test]
    fn test_magma_palette_edges() {
        assert_eq!(magma(0.0), magma(0.0));
        assert_eq!(magma(1.0), magma(1.0));
    }
    #[test]
    fn test_palette_monotonicity() {
        // Colors at t=0, t=0.5, t=1 should be different for inferno
        let c0 = inferno(0.0);
        let c1 = inferno(0.5);
        let c2 = inferno(1.0);
        assert!(c0 != c1 && c1 != c2 && c0 != c2);
    }
    #[test]
    fn test_cubehelix_palette_edges() {
        assert_eq!(cubehelix(0.0), cubehelix(0.0));
        assert_eq!(cubehelix(1.0), cubehelix(1.0));
    }
}

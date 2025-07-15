// d3-scale parity root
// Re-export all submodules here as you implement them

pub mod linear;
pub use linear::ScaleLinear;
pub mod log;
pub use log::ScaleLog;
pub mod pow;
pub use pow::ScalePow;
pub mod sqrt;
pub use sqrt::ScaleSqrt;
pub mod symlog;
pub use symlog::ScaleSymlog;
pub mod time;
pub use time::ScaleTime;
pub mod band;
pub use band::ScaleBand;
pub mod point;
pub use point::ScalePoint;
pub mod utc;
pub use utc::ScaleUtc;

// New scale implementations
pub mod identity;
pub use identity::ScaleIdentity;
pub mod ordinal;
pub use ordinal::ScaleOrdinal;
pub mod quantile;
pub use quantile::ScaleQuantile;
pub mod quantize;
pub use quantize::ScaleQuantize;
pub mod threshold;
pub use threshold::ScaleThreshold;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_export() {
        let s = ScaleLinear::new([0.0, 1.0], [0.0, 10.0]);
        assert_eq!(s.scale(0.5), 5.0);
        assert_eq!(s.invert(5.0), 0.5);
    }

    #[test]
    fn test_log_export() {
        let s = ScaleLog::new([1.0, 10.0], [0.0, 1.0], 10.0);
        let mid = s.scale(3.1622776601683795); // sqrt(10)
        assert!((mid - 0.5).abs() < 1e-6);
        let inv = s.invert(mid);
        assert!((inv - 3.1622776601683795).abs() < 1e-6);
    }

    #[test]
    fn test_pow_export() {
        let s = ScalePow::new([0.0, 1.0], [0.0, 100.0], 2.0);
        assert_eq!(s.scale(0.5), 25.0);
        assert_eq!(s.invert(25.0), 0.5);
    }

    #[test]
    fn test_sqrt_export() {
        let s = ScaleSqrt::new([0.0, 1.0], [0.0, 100.0], 0.5);
        assert_eq!(s.scale(0.25), 50.0);
        assert!((s.invert(50.0) - 0.25).abs() < 1e-6);
    }

    #[test]
    fn test_symlog_export() {
        let s = ScaleSymlog::new([-10.0, 10.0], [0.0, 100.0], 1.0);
        let mid = s.scale(0.0);
        assert!((mid - 50.0).abs() < 1e-6);
        let inv = s.invert(mid);
        assert!((inv - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_band_export() {
        let s = ScaleBand::new(vec!["a", "b", "c"], [0.0, 120.0], 0.1, 0.1, 0.5);
        let bw = s.bandwidth();
        assert!((bw - 34.8387).abs() < 1e-3);
        let a = s.scale(&"a").unwrap();
        assert!((a - 1.9355).abs() < 1e-3);
    }

    #[test]
    fn test_point_export() {
        let s = ScalePoint::new(vec!["a", "b", "c"], [0.0, 100.0], 0.5);
        assert!((s.scale(&"a").unwrap() - 0.0).abs() < 1e-6);
        assert!((s.scale(&"b").unwrap() - 50.0).abs() < 1e-6);
        assert!((s.scale(&"c").unwrap() - 100.0).abs() < 1e-6);
    }

    #[test]
    fn test_identity_export() {
        let s = ScaleIdentity::new([0.0, 1.0], [0.0, 1.0]);
        assert_eq!(s.scale(0.5), 0.5);
        assert_eq!(s.invert(0.5), 0.5);
    }

    #[test]
    fn test_ordinal_export() {
        let s = ScaleOrdinal::new(vec!["a", "b", "c"], vec!["red", "green", "blue"]);
        assert_eq!(s.scale(&"a"), Some("red"));
        assert_eq!(s.scale(&"b"), Some("green"));
        assert_eq!(s.scale(&"c"), Some("blue"));
    }

    #[test]
    fn test_quantile_export() {
        let domain = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
        let range = vec!["low", "high"];
        let s = ScaleQuantile::new(domain, range);
        assert_eq!(s.scale(0.0), Some("low"));
        assert_eq!(s.scale(5.0), Some("high"));
    }

    #[test]
    fn test_quantize_export() {
        let s = ScaleQuantize::new([0.0, 1.0], vec!["a", "b"]);
        assert_eq!(s.scale(0.25), Some("a"));
        assert_eq!(s.scale(0.75), Some("b"));
    }

    #[test]
    fn test_threshold_export() {
        let s = ScaleThreshold::new(vec![0.0, 1.0], vec!["red", "white", "blue"]);
        assert_eq!(s.scale(-1.0), Some("red"));
        assert_eq!(s.scale(0.5), Some("white"));
        assert_eq!(s.scale(1.5), Some("blue"));
    }
}

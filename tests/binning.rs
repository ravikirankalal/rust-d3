//! Unit tests for d3 binning (histogram)
use rust_d3::binning::{histogram, bin};

#[test]
fn test_histogram() {
    let data = [1.0, 2.0, 2.5, 3.0, 4.0];
    let bins = histogram(&data, 3);
    assert_eq!(bins.len(), 3);
    assert!(bins[0].contains(&1.0));
    assert!(bins[1].contains(&2.0));
    assert!(bins[1].contains(&2.5));
    assert!(bins[2].contains(&3.0));
    assert!(bins[2].contains(&4.0));
}

#[test]
fn test_bin() {
    let data = [1.0, 2.0, 2.5, 3.0, 4.0, 10.0];
    let bins = bin(&data);
    let flat: Vec<f64> = bins.iter().flat_map(|b| b.iter().map(|&&x| x)).collect();
    for &v in &data {
        assert!(flat.contains(&v));
    }
    // Should have 10 bins by default
    assert_eq!(bins.len(), 10);
    // Edge case: empty
    let empty: [f64; 0] = [];
    let bins = bin(&empty);
    assert!(bins.is_empty());
    // Edge case: all values the same
    let same = [5.0, 5.0, 5.0];
    let bins = bin(&same);
    let flat: Vec<f64> = bins.iter().flat_map(|b| b.iter().map(|&&x| x)).collect();
    assert_eq!(flat, vec![5.0, 5.0, 5.0]);
}

#[test]
fn test_histogram_with_edges_and_accessor() {
    let data = [(1, "a"), (2, "b"), (3, "c"), (4, "d")];
    let edges = [0.0, 2.0, 4.0];
    let bins = rust_d3::binning::histogram_with(&data, &edges, |x| x.0 as f64);
    assert_eq!(bins.len(), 2);
    assert_eq!(bins[0].len(), 1); // (1, "a")
    assert_eq!(bins[1].len(), 3); // (2, "b"), (3, "c"), (4, "d")
}

#[test]
fn test_bin_with_custom_count_and_accessor() {
    let data = [(1, "a"), (2, "b"), (3, "c"), (4, "d")];
    let bins = rust_d3::binning::bin_with(&data, 2, |x| x.0 as f64);
    assert_eq!(bins.len(), 2);
    assert!(bins.iter().map(|b| b.len()).sum::<usize>() == data.len());
}

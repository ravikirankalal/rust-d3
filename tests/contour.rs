//! Unit test for d3 contour
use rust_d3::contour::{contours, ContourLine};

#[test]
fn test_contours_simple() {
    let values = vec![
        vec![0.0, 1.0],
        vec![1.0, 2.0],
    ];
    let lines = contours(&values, 1.0);
    assert!(!lines.is_empty());
    for line in &lines {
        assert_eq!(line.value, 1.0);
        assert!(line.points.len() >= 2);
    }
}

#[test]
fn test_contours_empty() {
    let values: Vec<Vec<f64>> = vec![];
    let lines = contours(&values, 1.0);
    assert!(lines.is_empty());
}

#[test]
fn test_contours_single_row_column() {
    let row = vec![vec![0.0, 2.0, 1.0]];
    let lines = contours(&row, 1.0);
    // Should handle single row
    assert!(lines.iter().all(|l| l.value == 1.0));
    let col = vec![vec![0.0], vec![2.0], vec![1.0]];
    let lines = contours(&col, 1.0);
    assert!(lines.iter().all(|l| l.value == 1.0));
}

#[test]
fn test_contours_all_below_above_threshold() {
    let values = vec![vec![0.0, 0.0], vec![0.0, 0.0]];
    let lines = contours(&values, 1.0);
    assert!(lines.is_empty());
    let values = vec![vec![2.0, 2.0], vec![2.0, 2.0]];
    let lines = contours(&values, 1.0);
    // All values above threshold: implementation may return no lines
    assert!(lines.is_empty());
}

#[test]
fn test_contours_negative_zero_threshold() {
    let values = vec![vec![-1.0, 0.0], vec![1.0, 2.0]];
    let lines = contours(&values, 0.0);
    assert!(lines.iter().all(|l| l.value == 0.0));
    let lines = contours(&values, -1.0);
    assert!(lines.iter().all(|l| l.value == -1.0));
}

#[test]
fn test_contours_non_square_grid() {
    let values = vec![vec![0.0, 1.0, 2.0], vec![2.0, 1.0, 0.0]];
    let lines = contours(&values, 1.0);
    assert!(lines.iter().all(|l| l.value == 1.0));
}

#[test]
fn test_contours_marching_squares_cases() {
    // Helper to build a 2x2 grid
    let mut grid = |a, b, c, d| vec![vec![a, b], vec![d, c]];
    let t = 1.0;
    // Case 1: only top-left >= t
    let lines = contours(&grid(2.0, 0.0, 0.0, 0.0), t);
    assert!(!lines.is_empty());
    // Case 2: only top-right >= t
    let lines = contours(&grid(0.0, 2.0, 0.0, 0.0), t);
    assert!(!lines.is_empty());
    // Case 3: top row >= t
    let lines = contours(&grid(2.0, 2.0, 0.0, 0.0), t);
    assert!(!lines.is_empty());
    // Case 4: only bottom-right >= t
    let lines = contours(&grid(0.0, 0.0, 2.0, 0.0), t);
    assert!(!lines.is_empty());
    // Case 5: top-left and bottom-right >= t
    let lines = contours(&grid(2.0, 0.0, 2.0, 0.0), t);
    assert!(!lines.is_empty());
    // Case 6: top-right and bottom-right >= t
    let lines = contours(&grid(0.0, 2.0, 2.0, 0.0), t);
    assert!(!lines.is_empty());
    // Case 7: all except bottom-left >= t
    let lines = contours(&grid(2.0, 2.0, 2.0, 0.0), t);
    assert!(!lines.is_empty());
    // Case 8: only bottom-left >= t
    let lines = contours(&grid(0.0, 0.0, 0.0, 2.0), t);
    assert!(!lines.is_empty());
    // Case 9: top-left and bottom-left >= t
    let lines = contours(&grid(2.0, 0.0, 0.0, 2.0), t);
    assert!(!lines.is_empty());
    // Case 10: top-right and bottom-left >= t
    let lines = contours(&grid(0.0, 2.0, 0.0, 2.0), t);
    assert!(!lines.is_empty());
    // Case 11: all except top-left >= t
    let lines = contours(&grid(0.0, 2.0, 2.0, 2.0), t);
    assert!(!lines.is_empty());
    // Case 12: bottom row >= t
    let lines = contours(&grid(0.0, 0.0, 2.0, 2.0), t);
    assert!(!lines.is_empty());
    // Case 13: all except top-right >= t
    let lines = contours(&grid(2.0, 0.0, 2.0, 2.0), t);
    assert!(!lines.is_empty());
    // Case 14: all except bottom-right >= t
    let lines = contours(&grid(2.0, 2.0, 0.0, 2.0), t);
    assert!(!lines.is_empty());
    // Case 15: all >= t (should be empty)
    let lines = contours(&grid(2.0, 2.0, 2.0, 2.0), t);
    assert!(lines.is_empty());
}

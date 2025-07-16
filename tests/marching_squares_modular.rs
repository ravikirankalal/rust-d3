use rust_d3::contour::marching_squares::{cell_mask, marching_case, marching_segments};

#[test]
fn test_marching_case() {
    // t0, t1, t2, t3
    assert_eq!(marching_case(false, false, false, false), 0);
    assert_eq!(marching_case(true, false, false, false), 1);
    assert_eq!(marching_case(false, true, false, false), 2);
    assert_eq!(marching_case(false, false, true, false), 4);
    assert_eq!(marching_case(false, false, false, true), 8);
    assert_eq!(marching_case(true, true, true, true), 15);
}

#[test]
fn test_cell_mask() {
    let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    // 2x3 grid, cell at (0,0): i=0, dx=2
    assert_eq!(cell_mask(&values, 0, 2, 3.5), (false, false, false, true));
    // cell at (1,0): i=1, dx=2
    assert_eq!(cell_mask(&values, 1, 2, 2.5), (false, true, true, true));
}

#[test]
fn test_marching_segments() {
    // Case 5: two segments
    let segs = marching_segments(5, 1, 2);
    assert_eq!(segs.len(), 2);
    // Check that the segments are offset by (1,2)
    for (start, end) in segs {
        assert!(start[0] >= 1.0 && start[1] >= 2.0);
        assert!(end[0] >= 1.0 && end[1] >= 2.0);
    }
}

#[test]
fn test_marching_segments_case0() {
    // Case 0: no segments
    let segs = marching_segments(0, 0, 0);
    assert_eq!(segs.len(), 0);
}

#[test]
fn test_marching_segments_case15() {
    // Case 15: no segments (all corners above threshold)
    let segs = marching_segments(15, 0, 0);
    assert_eq!(segs.len(), 0);
}

#[test]
fn test_marching_segments_case1() {
    // Case 1: one segment
    let segs = marching_segments(1, 0, 0);
    assert_eq!(segs.len(), 1);
    let (start, end) = segs[0];
    // Should match the CASES table for case 1
    assert!((start[0] - 0.5).abs() < 1e-6 && (start[1] - 1.0).abs() < 1e-6);
    assert!((end[0] - 0.0).abs() < 1e-6 && (end[1] - 0.5).abs() < 1e-6);
}

#[test]
fn test_marching_segments_offset() {
    // Case 1 at (2,3)
    let segs = marching_segments(1, 2, 3);
    assert_eq!(segs.len(), 1);
    let (start, end) = segs[0];
    assert!((start[0] - 2.5).abs() < 1e-6 && (start[1] - 4.0).abs() < 1e-6);
    assert!((end[0] - 2.0).abs() < 1e-6 && (end[1] - 3.5).abs() < 1e-6);
}

#[test]
fn test_marching_segments_case2() {
    // Case 2: one segment
    let segs = marching_segments(2, 0, 0);
    assert_eq!(segs.len(), 1);
    let (start, end) = segs[0];
    assert!((start[0] - 1.0).abs() < 1e-6 && (start[1] - 0.5).abs() < 1e-6);
    assert!((end[0] - 0.5).abs() < 1e-6 && (end[1] - 1.0).abs() < 1e-6);
}

#[test]
fn test_marching_segments_case3() {
    // Case 3: one segment
    let segs = marching_segments(3, 0, 0);
    assert_eq!(segs.len(), 1);
    let (start, end) = segs[0];
    assert!((start[0] - 1.0).abs() < 1e-6 && (start[1] - 0.5).abs() < 1e-6);
    assert!((end[0] - 0.0).abs() < 1e-6 && (end[1] - 0.5).abs() < 1e-6);
}

#[test]
fn test_marching_segments_case4() {
    // Case 4: one segment
    let segs = marching_segments(4, 0, 0);
    assert_eq!(segs.len(), 1);
    let (start, end) = segs[0];
    assert!((start[0] - 0.0).abs() < 1e-6 && (start[1] - 0.5).abs() < 1e-6);
    assert!((end[0] - 0.5).abs() < 1e-6 && (end[1] - 0.0).abs() < 1e-6);
}

#[test]
fn test_marching_segments_case6() {
    // Case 6: one segment
    let segs = marching_segments(6, 0, 0);
    assert_eq!(segs.len(), 1);
    let (start, end) = segs[0];
    assert!((start[0] - 1.0).abs() < 1e-6 && (start[1] - 0.5).abs() < 1e-6);
    assert!((end[0] - 0.5).abs() < 1e-6 && (end[1] - 0.0).abs() < 1e-6);
}

#[test]
fn test_marching_segments_case7() {
    // Case 7: one segment
    let segs = marching_segments(7, 0, 0);
    assert_eq!(segs.len(), 1);
    let (start, end) = segs[0];
    assert!((start[0] - 1.0).abs() < 1e-6 && (start[1] - 0.5).abs() < 1e-6);
    assert!((end[0] - 0.0).abs() < 1e-6 && (end[1] - 0.5).abs() < 1e-6);
}

#[test]
fn test_marching_segments_case8() {
    // Case 8: one segment
    let segs = marching_segments(8, 0, 0);
    assert_eq!(segs.len(), 1);
    let (start, end) = segs[0];
    assert!((start[0] - 0.0).abs() < 1e-6 && (start[1] - 0.5).abs() < 1e-6);
    assert!((end[0] - 0.5).abs() < 1e-6 && (end[1] - 1.0).abs() < 1e-6);
}

#[test]
fn test_marching_segments_case9() {
    // Case 9: one segment
    let segs = marching_segments(9, 0, 0);
    assert_eq!(segs.len(), 1);
    let (start, end) = segs[0];
    assert!((start[0] - 0.5).abs() < 1e-6 && (start[1] - 0.0).abs() < 1e-6);
    assert!((end[0] - 0.5).abs() < 1e-6 && (end[1] - 1.0).abs() < 1e-6);
}

#[test]
fn test_marching_segments_case11() {
    // Case 11: one segment
    let segs = marching_segments(11, 0, 0);
    assert_eq!(segs.len(), 1);
    let (start, end) = segs[0];
    assert!((start[0] - 0.5).abs() < 1e-6 && (start[1] - 0.0).abs() < 1e-6);
    assert!((end[0] - 1.0).abs() < 1e-6 && (end[1] - 0.5).abs() < 1e-6);
}

#[test]
fn test_marching_segments_case12() {
    // Case 12: one segment
    let segs = marching_segments(12, 0, 0);
    assert_eq!(segs.len(), 1);
    let (start, end) = segs[0];
    assert!((start[0] - 0.0).abs() < 1e-6 && (start[1] - 0.5).abs() < 1e-6);
    assert!((end[0] - 0.5).abs() < 1e-6 && (end[1] - 0.0).abs() < 1e-6);
}

#[test]
fn test_marching_segments_case13() {
    // Case 13: one segment
    let segs = marching_segments(13, 0, 0);
    assert_eq!(segs.len(), 1);
    let (start, end) = segs[0];
    assert!((start[0] - 0.0).abs() < 1e-6 && (start[1] - 0.5).abs() < 1e-6);
    assert!((end[0] - 0.5).abs() < 1e-6 && (end[1] - 1.0).abs() < 1e-6);
}

#[test]
fn test_marching_segments_case14() {
    // Case 14: one segment
    let segs = marching_segments(14, 0, 0);
    assert_eq!(segs.len(), 1);
    let (start, end) = segs[0];
    assert!((start[0] - 0.5).abs() < 1e-6 && (start[1] - 1.0).abs() < 1e-6);
    assert!((end[0] - 0.5).abs() < 1e-6 && (end[1] - 0.0).abs() < 1e-6);
}

use rust_d3::contour::marching_squares::isorings;

#[test]
fn test_isorings_single_cell_case8() {
    let values = vec![0.0, 0.0, 0.0, 1.0];
    let mut rings = Vec::new();
    isorings(&values, 0.5, 2, 2, |ring: Vec<[f64; 2]>| rings.push(ring));
    assert_eq!(rings.len(), 1);
    let ring = &rings[0];
    assert!(ring.contains(&[0.0, 0.5]) || ring.contains(&[0.5, 1.0]));
}

#[test]
fn test_isorings_no_contour() {
    let values = vec![0.0, 0.0, 0.0, 0.0];
    let mut rings = Vec::new();
    isorings(&values, 0.5, 2, 2, |ring: Vec<[f64; 2]>| rings.push(ring));
    assert_eq!(rings.len(), 0);
}

#[test]
fn test_isorings_full_contour() {
    let values = vec![1.0, 1.0, 1.0, 1.0];
    let mut rings = Vec::new();
    isorings(&values, 0.5, 2, 2, |ring: Vec<[f64; 2]>| rings.push(ring));
    assert_eq!(rings.len(), 0);
}

#[test]
fn test_isorings_diagonal() {
    let values = vec![1.0, 0.0, 0.0, 1.0];
    let mut rings = Vec::new();
    isorings(&values, 0.5, 2, 2, |ring: Vec<[f64; 2]>| rings.push(ring));
    assert!(rings.len() >= 1);
}

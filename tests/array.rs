//! Integration test for array_utils (group, rollup)

use rust_d3::array::{group, rollup, flat_group, fsum, Adder};

#[test]
fn test_group() {
    let data = [1, 2, 3, 4, 5, 6];
    let grouped = group(&data, |x| x % 2);
    assert_eq!(grouped[&0], vec![&2, &4, &6]);
    assert_eq!(grouped[&1], vec![&1, &3, &5]);
}

#[test]
fn test_rollup() {
    let data = [1, 2, 3, 4, 5, 6];
    let rolled = rollup(&data, |x| x % 2, |group| group.iter().copied().sum::<i32>());
    assert_eq!(rolled[&0], 12); // 2+4+6
    assert_eq!(rolled[&1], 9);  // 1+3+5
}

#[test]
fn test_flat_group() {
    let data = [1, 2, 3, 4];
    let fg = flat_group(&data, |x| x % 2);
    assert!(fg.contains(&(0, vec![&2, &4])));
    assert!(fg.contains(&(1, vec![&1, &3])));
}

#[test]
fn test_fsum() {
    let data = [1e100, 1.0, -1e100];
    let accurate = rust_d3::array::fsum(data.iter().copied());
    assert!((accurate - 1.0).abs() < 1e-12);
}

#[test]
fn test_adder() {
    let mut adder = Adder::new();
    adder.add(1e100);
    adder.add(1.0);
    adder.add(-1e100);
    assert!((adder.value() - 1.0).abs() < 1e-12);
}

#[test]
fn test_sum() {
    let data = [1, 2, 3, 4, 5];
    assert_eq!(rust_d3::array::sum(&data), 15.0);
    let empty: [i32; 0] = [];
    assert_eq!(rust_d3::array::sum(&empty), 0.0);
}

#[test]
fn test_mean() {
    let data = [2, 4, 6, 8];
    assert_eq!(rust_d3::array::mean(&data), Some(5.0));
    let empty: [i32; 0] = [];
    assert_eq!(rust_d3::array::mean(&empty), None);
}

#[test]
fn test_median() {
    let data = [1, 3, 5, 7, 9];
    assert_eq!(rust_d3::array::median(&data), Some(5.0));
    let even = [1, 2, 3, 4];
    assert_eq!(rust_d3::array::median(&even), Some(2.5));
    let empty: [i32; 0] = [];
    assert_eq!(rust_d3::array::median(&empty), None);
}

#[test]
fn test_mode() {
    let data = [1, 2, 2, 3, 4, 2, 5];
    assert_eq!(rust_d3::array::mode(&data), Some(2));
    let unique = [1, 2, 3, 4];
    assert!(rust_d3::array::mode(&unique).is_some()); // Any value is valid
    let empty: [i32; 0] = [];
    assert_eq!(rust_d3::array::mode(&empty), None);
}

#[test]
fn test_variance() {
    let data = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    let var = rust_d3::array::variance(&data).unwrap();
    assert!((var - 4.0).abs() < 1e-6);
    let empty: [f64; 0] = [];
    assert_eq!(rust_d3::array::variance(&empty), None);
}

#[test]
fn test_deviation() {
    let data = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    let dev = rust_d3::array::deviation(&data).unwrap();
    assert!((dev - 2.0).abs() < 1e-6);
    let empty: [f64; 0] = [];
    assert_eq!(rust_d3::array::deviation(&empty), None);
}

#[test]
fn test_pairs() {
    let data = [1, 2, 3, 4];
    let pairs = rust_d3::array::pairs(&data);
    assert_eq!(pairs, vec![(1, 2), (2, 3), (3, 4)]);
    let single = [1];
    assert!(rust_d3::array::pairs(&single).is_empty());
}

#[test]
fn test_transpose() {
    let data = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let transposed = rust_d3::array::transpose(&data);
    assert_eq!(transposed, vec![vec![1, 4], vec![2, 5], vec![3, 6]]);
    let empty: Vec<Vec<i32>> = vec![];
    assert!(rust_d3::array::transpose(&empty).is_empty());
}

#[test]
fn test_zip() {
    let a = [1, 2, 3];
    let b = [4, 5, 6];
    let zipped = rust_d3::array::zip(&[&a, &b]);
    assert_eq!(zipped, vec![vec![1, 4], vec![2, 5], vec![3, 6]]);
    let empty: [&[i32]; 0] = [];
    assert!(rust_d3::array::zip(&empty).is_empty());
}

#[test]
fn test_least_greatest() {
    let data = [3, 1, 4, 2];
    assert_eq!(rust_d3::array::least(&data), Some(1));
    assert_eq!(rust_d3::array::greatest(&data), Some(4));
    let empty: [i32; 0] = [];
    assert_eq!(rust_d3::array::least(&empty), None);
    assert_eq!(rust_d3::array::greatest(&empty), None);
}

#[test]
fn test_shuffle_permute() {
    let mut data = [1, 2, 3, 4, 5];
    let orig = data.clone();
    rust_d3::array::shuffle(&mut data);
    // Shuffled array should have same elements as original
    let mut sorted = data.to_vec();
    sorted.sort();
    assert_eq!(sorted, orig);
    let perm = rust_d3::array::permute(&orig, &[4, 3, 2, 1, 0]);
    assert_eq!(perm, vec![5, 4, 3, 2, 1]);
}

#[test]
fn test_ascending_descending() {
    assert_eq!(rust_d3::array::ascending(1, 2), -1);
    assert_eq!(rust_d3::array::ascending(2, 1), 1);
    assert_eq!(rust_d3::array::ascending(2, 2), 0);
    assert_eq!(rust_d3::array::descending(1, 2), 1);
    assert_eq!(rust_d3::array::descending(2, 1), -1);
    assert_eq!(rust_d3::array::descending(2, 2), 0);
}

#[test]
fn test_bisect() {
    let data = [1, 2, 4, 4, 5, 7];
    assert_eq!(rust_d3::array::bisect(&data, &4), 2);
    assert_eq!(rust_d3::array::bisect(&data, &3), 2);
    assert_eq!(rust_d3::array::bisect(&data, &8), 6);
    assert_eq!(rust_d3::array::bisect(&data, &0), 0);
}

#[test]
fn test_merge() {
    let arrays = vec![vec![1, 2], vec![3, 4], vec![5]];
    let merged = rust_d3::array::merge(&arrays);
    assert_eq!(merged, vec![1, 2, 3, 4, 5]);
    let empty: Vec<Vec<i32>> = vec![];
    assert!(rust_d3::array::merge(&empty).is_empty());
}

#[test]
fn test_union() {
    let a = [1, 2, 3];
    let b = [3, 4, 5];
    let mut result = rust_d3::array::union(&a, &b);
    result.sort();
    assert_eq!(result, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_intersection() {
    let a = [1, 2, 3];
    let b = [2, 3, 4];
    let mut result = rust_d3::array::intersection(&a, &b);
    result.sort();
    assert_eq!(result, vec![2, 3]);
}

#[test]
fn test_difference() {
    let a = [1, 2, 3];
    let b = [2, 4];
    let mut result = rust_d3::array::difference(&a, &b);
    result.sort();
    assert_eq!(result, vec![1, 3]);
}

#[test]
fn test_cross() {
    let a = [1, 2];
    let b = ['a', 'b'];
    let mut result = rust_d3::array::cross(&a, &b);
    result.sort_by(|x, y| x.0.cmp(&y.0).then(x.1.cmp(&y.1)));
    assert_eq!(result, vec![(1, 'a'), (1, 'b'), (2, 'a'), (2, 'b')]);
}

#[test]
fn test_bisect_left_right_by() {
    let data = [1, 2, 4, 4, 5];
    assert_eq!(rust_d3::array::bisect_left(&data, &4), 2);
    assert_eq!(rust_d3::array::bisect_right(&data, &4), 4);
    let cmp = |a: &i32, b: &i32| a.cmp(b);
    assert_eq!(rust_d3::array::bisect_by(&data, &4, cmp), 2);
}

#[test]
fn test_fsum_precision() {
    let data = [1e100, 1.0, -1e100, 1.0, 1.0];
    let accurate = rust_d3::array::fsum(data.iter().copied());
    assert!((accurate - 3.0).abs() < 1e-12);
}

#[test]
fn test_tick_step() {
    let step = rust_d3::array::tick_step(0.0, 10.0, 5);
    assert!((step - 2.5).abs() < 1e-12);
}

#[test]
fn test_blur() {
    use rust_d3::array::blur;

    let mut data = vec![0.0, 1.0, 0.0];
    blur(&mut data, 1.0);
    // Expected: [0.25, 0.5, 0.25] after one pass, then further blurred
    // Due to three passes, the values will be more spread out.
    // A simple test for now, more rigorous tests can be added later.
    assert!((data[0] - 0.4305555555555555).abs() < 0.001);
    assert!((data[1] - 0.4259259259259259).abs() < 0.001);
    assert!((data[2] - 0.4305555555555555).abs() < 0.001);

    let mut data2 = vec![1.0, 1.0, 1.0, 1.0, 1.0];
    blur(&mut data2, 0.0);
    assert_eq!(data2, vec![1.0, 1.0, 1.0, 1.0, 1.0]); // Radius 0 should not change data

    let mut data3 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    blur(&mut data3, 2.0);
    // Expected values after blur, these are approximate and depend on the exact blur implementation
    // For radius 2, the blur is significant.
    assert!((data3[0] - 2.75).abs() < 0.001);
    assert!((data3[1] - 2.875).abs() < 0.001);
    assert!((data3[2] - 3.0).abs() < 0.001);
    assert!((data3[3] - 3.125).abs() < 0.001);
    assert!((data3[4] - 3.25).abs() < 0.001);
}

#[test]
fn test_interner() {
    let mut interner = rust_d3::array::Interner::new();

    let s1_val = "hello".to_string();
    let s2_val = "world".to_string();
    let s3_val = "hello".to_string();

    // Test interning of s1_val
    let r1 = interner.intern(&s1_val).clone();
    assert_eq!(r1, s1_val);

    // Test interning of s2_val
    let r2 = interner.intern(&s2_val).clone();
    assert_eq!(r2, s2_val);

    // Test interning of s3_val (which is identical to s1_val)
    let r3 = interner.intern(&s3_val).clone();
    assert_eq!(r3, s3_val);
    assert_eq!(r1, r3);

    // Test that different strings return different values
    assert_ne!(r1, r2);

    // Test with another type (e.g., i32)
    let mut int_interner = rust_d3::array::Interner::new();
    let i1_val = 10;
    let i2_val = 20;
    let i3_val = 10;

    // Test interning of i1_val
    let ir1 = int_interner.intern(&i1_val).clone();
    assert_eq!(ir1, i1_val);

    // Test interning of i2_val
    let ir2 = int_interner.intern(&i2_val).clone();
    assert_eq!(ir2, i2_val);

    // Test interning of i3_val (which is identical to i1_val)
    let ir3 = int_interner.intern(&i3_val).clone();
    assert_eq!(ir3, i3_val);
    assert_eq!(ir1, ir3);

    // Test that different integers return different values
    assert_ne!(ir1, ir2);
}
use rust_d3::array_utils::{fsum, Adder};

#[test]
fn test_fsum() {
    let data = [1e100, 1.0, -1e100];
    let naive = data.iter().copied().sum::<f64>();
    let accurate = fsum(data);
    assert!(naive != 1.0); // naive sum is not accurate
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

pub fn extent<T: PartialOrd + Copy>(array: &[T]) -> Option<(T, T)> {
    if array.is_empty() {
        return None;
    }

    let mut min_val = array[0];
    let mut max_val = array[0];

    for &item in array.iter().skip(1) {
        if item < min_val {
            min_val = item;
        }
        if item > max_val {
            max_val = item;
        }
    }
    Some((min_val, max_val))
}

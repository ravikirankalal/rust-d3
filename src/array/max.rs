pub fn max<T: PartialOrd + Copy>(array: &[T]) -> Option<T> {
    if array.is_empty() {
        return None;
    }

    let mut max_val = array[0];
    for &item in array.iter().skip(1) {
        if item > max_val {
            max_val = item;
        }
    }
    Some(max_val)
}

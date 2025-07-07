pub fn extent<T: Ord + Copy>(data: &[T]) -> Option<(T, T)> {
    if data.is_empty() {
        None
    } else {
        let mut min = data[0];
        let mut max = data[0];
        for &v in data.iter().skip(1) {
            if v < min {
                min = v;
            }
            if v > max {
                max = v;
            }
        }
        Some((min, max))
    }
}

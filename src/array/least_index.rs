pub fn least_index<T, U, F>(array: &[T], accessor: F) -> Option<usize>
where
    T: Clone,
    U: PartialOrd,
    F: Fn(&T) -> U,
{
    if array.is_empty() {
        return None;
    }

    let mut min_index = 0;
    let mut min_val = accessor(&array[0]);

    for i in 1..array.len() {
        let current_val = accessor(&array[i]);
        if current_val < min_val {
            min_val = current_val;
            min_index = i;
        }
    }
    Some(min_index)
}

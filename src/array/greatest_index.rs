pub fn greatest_index<T, U, F>(array: &[T], accessor: F) -> Option<usize>
where
    T: Clone,
    U: PartialOrd,
    F: Fn(&T) -> U,
{
    if array.is_empty() {
        return None;
    }

    let mut max_index = 0;
    let mut max_val = accessor(&array[0]);

    for i in 1..array.len() {
        let current_val = accessor(&array[i]);
        if current_val > max_val {
            max_val = current_val;
            max_index = i;
        }
    }
    Some(max_index)
}

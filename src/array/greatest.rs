pub fn greatest<T, U, F>(array: &[T], accessor: F) -> Option<T>
where
    T: Clone,
    U: PartialOrd,
    F: Fn(&T) -> U,
{
    if array.is_empty() {
        return None;
    }

    let mut max_val = accessor(&array[0]);
    let mut max_item = array[0].clone();

    for item in array.iter().skip(1) {
        let current_val = accessor(item);
        if current_val > max_val {
            max_val = current_val;
            max_item = item.clone();
        }
    }
    Some(max_item)
}

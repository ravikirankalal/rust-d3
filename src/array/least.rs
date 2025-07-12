pub fn least<T, U, F>(array: &[T], accessor: F) -> Option<T>
where
    T: Clone,
    U: PartialOrd,
    F: Fn(&T) -> U,
{
    if array.is_empty() {
        return None;
    }

    let mut min_val = accessor(&array[0]);
    let mut min_item = array[0].clone();

    for item in array.iter().skip(1) {
        let current_val = accessor(item);
        if current_val < min_val {
            min_val = current_val;
            min_item = item.clone();
        }
    }
    Some(min_item)
}

use std::cmp::Ordering;

pub fn scan<T, F>(array: &[T], comparator: F) -> Option<usize>
where
    F: Fn(&T, &T) -> Ordering,
{
    if array.is_empty() {
        return None;
    }

    let mut min_index = 0;
    for i in 1..array.len() {
        if comparator(&array[i], &array[min_index]) == Ordering::Less {
            min_index = i;
        }
    }
    Some(min_index)
}

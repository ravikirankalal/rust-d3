use std::cmp::Ordering;

pub fn descending<T: PartialOrd>(a: &T, b: &T) -> Ordering {
    if a > b {
        Ordering::Less
    } else if a < b {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

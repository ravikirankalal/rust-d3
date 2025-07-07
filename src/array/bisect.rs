pub fn bisect_left<T: PartialOrd>(arr: &[T], x: &T, lo: usize, hi: usize) -> usize {
    let mut low = lo;
    let mut high = hi;
    while low < high {
        let mid = (low + high) / 2;
        if arr[mid] < *x {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
}

pub fn bisect_right<T: PartialOrd>(arr: &[T], x: &T, lo: usize, hi: usize) -> usize {
    let mut low = lo;
    let mut high = hi;
    while low < high {
        let mid = (low + high) / 2;
        if *x < arr[mid] {
            high = mid;
        } else {
            low = mid + 1;
        }
    }
    low
}

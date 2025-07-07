pub fn bisect_left<T: PartialOrd>(arr: &[T], x: &T) -> usize {
    let mut lo = 0;
    let mut hi = arr.len();
    while lo < hi {
        let mid = (lo + hi) / 2;
        if &arr[mid] < x {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}

pub fn bisect_right<T: PartialOrd>(arr: &[T], x: &T) -> usize {
    let mut lo = 0;
    let mut hi = arr.len();
    while lo < hi {
        let mid = (lo + hi) / 2;
        if &arr[mid] <= x {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}


pub struct Bisector;

impl Bisector {
    pub fn left<T, K, F>(array: &[T], x: &K, accessor: F) -> usize
    where
        F: Fn(&T) -> K,
        K: PartialOrd,
    {
        let mut lo = 0;
        let mut hi = array.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if accessor(&array[mid]) < *x {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo
    }
    pub fn right<T, K, F>(array: &[T], x: &K, accessor: F) -> usize
    where
        F: Fn(&T) -> K,
        K: PartialOrd,
    {
        let mut lo = 0;
        let mut hi = array.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if accessor(&array[mid]) <= *x {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo
    }
    pub fn center<T, K, F>(array: &[T], x: &K, accessor: F) -> usize
    where
        F: Fn(&T) -> K,
        K: PartialOrd,
    {
        let l = Self::left(array, x, &accessor);
        let r = Self::right(array, x, &accessor);
        (l + r) / 2
    }
}

//! Generates left/right bisect functions for a comparator or accessor, like d3-array's bisector.
//!
//! # Example
//! ```
//! let arr = vec![1, 3, 5, 7, 9];
//! let (left, right) = rust_d3::bisector(|a: &i32, b: &i32| a.cmp(b));
//! assert_eq!(left(&arr, &5), 2);
//! assert_eq!(right(&arr, &5), 3);
//! ```
/// Returns a pair of bisect functions (left, right) for a comparator or accessor.
pub fn bisector<T, F>(compare: F) -> (impl Fn(&[T], &T) -> usize, impl Fn(&[T], &T) -> usize)
where
    F: Fn(&T, &T) -> std::cmp::Ordering + Copy,
{
    let left = move |arr: &[T], x: &T| -> usize {
        let mut lo = 0;
        let mut hi = arr.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if compare(&arr[mid], x) == std::cmp::Ordering::Less {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo
    };
    let right = move |arr: &[T], x: &T| -> usize {
        let mut lo = 0;
        let mut hi = arr.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if compare(x, &arr[mid]) == std::cmp::Ordering::Less {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    };
    (left, right)
}

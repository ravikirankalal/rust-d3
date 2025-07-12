//! Partial sort to find the k-th smallest element (like d3-array's quickselect).
//!
//! # Example
//! ```
//! let mut arr = vec![9, 1, 8, 2, 7, 3, 6, 4, 5];
//! rust_d3::quickselect(&mut arr, 4);
//! assert_eq!(arr[4], 5); // arr[4] is now the 5th smallest element
//! ```
/// Rearranges the array so that the k-th element is the one that would be in that position in a sorted array.
/// Elements before k are less than or equal to it, elements after are greater or equal.
pub fn quickselect<T: Ord>(arr: &mut [T], k: usize) {
    if arr.is_empty() || k >= arr.len() {
        return;
    }
    let mut left = 0;
    let mut right = arr.len() - 1;
    loop {
        if left == right {
            break;
        }
        let pivot_index = partition(arr, left, right);
        if k == pivot_index {
            break;
        } else if k < pivot_index {
            right = pivot_index - 1;
        } else {
            left = pivot_index + 1;
        }
    }
}

fn partition<T: Ord>(arr: &mut [T], left: usize, right: usize) -> usize {
    let pivot = right;
    let mut store = left;
    for i in left..right {
        if arr[i] < arr[pivot] {
            arr.swap(store, i);
            store += 1;
        }
    }
    arr.swap(store, right);
    store
}

# Array Utilities (`d3-array` Parity)

This module provides Rust equivalents of D3.js array utilities, covering nearly all features from [d3-array](https://github.com/d3/d3-array):

## Features
- [`ascending`](./ascending.rs), [`descending`](./descending.rs): Sort comparators.
- [`bisect`](./bisect.rs), [`bisector`](./bisector.rs), `bisectLeft`, `bisectRight`: Binary search utilities.
- [`bin`](./histogram.rs), [`histogram`](./histogram.rs): Histogram binning.
- [`cumsum`](./cumsum.rs): Cumulative sum.
- [`deviation`](./deviation.rs), [`variance`](./variance.rs): Statistical deviation and variance.
- [`difference`](./difference.rs), [`union`](./union.rs), [`intersection`](./intersection.rs), [`symmetric_difference`](./symmetric_difference.rs): Set operations.
- [`extent`](./extent.rs), [`min`](./min.rs), [`max`](./max.rs): Range and extrema.
- [`group`](./group.rs), [`groups`](./group.rs), [`rollup`](./group.rs), [`rollups`](./group.rs), [`flat_group`](./flat_group.rs): Grouping and aggregation.
- [`greatest`](./greatest.rs), [`greatestIndex`](./greatest_index.rs), [`least`](./least.rs), [`leastIndex`](./least_index.rs): Find greatest/least values and indices.
- [`fsum`](./fsum.rs), [`sum`](./sum.rs), [`mean`](./mean.rs), [`median`](./median.rs): Summation and statistics.
- [`merge`](./merge.rs), [`pairs`](./pairs.rs), [`permute`](./permute.rs), [`zip`](./zip.rs), [`cross`](./cross.rs): Array transformations.
- [`quantile`](./quantile.rs), `quantileSorted`: Quantile calculations.
- [`range`](./range.rs), [`ticks`](./ticks.rs), [`tickStep`](./tick_step.rs), `tickIncrement`, [`nice`](./nice.rs): Range and tick generation.
- [`scan`](./scan.rs): Argmin/argmax.
- [`shuffle`](./shuffle.rs): Shuffle arrays.
- [`sort`](./sort.rs), [`sort_by`](./sort_by.rs): Sorting utilities.
- [`summarize`](./summarize.rs), [`transform`](./transform.rs): Data summarization and transformation.
- [`intern`](./intern.rs): Interning sets and maps.
- [`transpose`](./transpose.rs): Transpose a matrix (array of arrays).
- [`quickselect`](./quickselect.rs): Partial sort to find the k-th smallest element.

## Usage

### [`ascending`](./ascending.rs) / [`descending`](./descending.rs)
Sort comparators:
```rust
let mut arr = vec![3, 1, 2];
arr.sort_by(rust_d3::ascending);
assert_eq!(arr, vec![1, 2, 3]);
arr.sort_by(rust_d3::descending);
assert_eq!(arr, vec![3, 2, 1]);
```

### [`bisect`](./bisect.rs)
Find insertion index for a value:
```rust
let arr = vec![1, 3, 5, 7, 9];
let idx = rust_d3::bisect_left(&arr, &5);
assert_eq!(idx, 2);
```

### [`bin`](./histogram.rs) / [`histogram`](./histogram.rs)
Histogram binning:
```rust
let data = vec![1.0, 2.0, 2.5, 3.0, 4.0];
let bins = rust_d3::histogram(&data, 2.0, 4.0, 2);
assert_eq!(bins.len(), 2);
```

### [`cumsum`](./cumsum.rs)
Cumulative sum:
```rust
let arr = vec![1, 2, 3];
let cs = rust_d3::cumsum(&arr);
assert_eq!(cs, vec![1, 3, 6]);
```

### [`deviation`](./deviation.rs) / [`variance`](./variance.rs)
Statistical deviation and variance:
```rust
let arr = vec![1.0, 2.0, 3.0];
let dev = rust_d3::deviation(&arr);
let var = rust_d3::variance(&arr);
```

### [`difference`](./difference.rs) / [`union`](./union.rs) / [`intersection`](./intersection.rs) / [`symmetric_difference`](./symmetric_difference.rs)
Set operations:
```rust
let a = vec![1, 2, 3];
let b = vec![2, 3, 4];
let diff = rust_d3::difference(&a, &b);
let union = rust_d3::union(&a, &b);
let inter = rust_d3::intersection(&a, &b);
let sym = rust_d3::symmetric_difference(&a, &b);
```

### [`extent`](./extent.rs) / [`min`](./min.rs) / [`max`](./max.rs)
Range and extrema:
```rust
let arr = vec![1, 2, 3];
let (min, max) = rust_d3::extent(&arr);
assert_eq!((min, max), (Some(1), Some(3)));
```

### [`group`](./group.rs) / [`groups`](./group.rs) / [`rollup`](./group.rs) / [`rollups`](./group.rs) / [`flat_group`](./flat_group.rs)
Grouping and aggregation:
```rust
let arr = vec!["a", "b", "a"];
let grouped = rust_d3::group_by(&arr, |x| *x);
```

### [`greatest`](./greatest.rs) / [`greatestIndex`](./greatest_index.rs) / [`least`](./least.rs) / [`leastIndex`](./least_index.rs)
Find greatest/least values and indices:
```rust
let arr = vec![1, 3, 2];
let greatest = rust_d3::greatest(&arr);
let least_idx = rust_d3::least_index(&arr);
```

### [`fsum`](./fsum.rs) / [`sum`](./sum.rs) / [`mean`](./mean.rs) / [`median`](./median.rs)
Summation and statistics:
```rust
let arr = vec![1.0, 2.0, 3.0];
let sum = rust_d3::sum(&arr);
let mean = rust_d3::mean(&arr);
let median = rust_d3::median(&arr);
```

### [`merge`](./merge.rs) / [`pairs`](./pairs.rs) / [`permute`](./permute.rs) / [`zip`](./zip.rs) / [`cross`](./cross.rs)
Array transformations:
```rust
let a = vec![1, 2];
let b = vec![3, 4];
let zipped = rust_d3::zip(&a, &b);
```

### [`quantile`](./quantile.rs)
Quantile calculations:
```rust
let arr = vec![1.0, 2.0, 3.0];
let q = rust_d3::quantile(&arr, 0.5);
```

### [`range`](./range.rs) / [`ticks`](./ticks.rs) / [`tickStep`](./tick_step.rs) / [`nice`](./nice.rs)
Range and tick generation:
```rust
let r = rust_d3::range(0, 5, 1);
let ticks = rust_d3::ticks(0.0, 1.0, 5);
```

### [`scan`](./scan.rs)
Argmin/argmax:
```rust
let arr = vec![3, 1, 2];
let idx = rust_d3::scan(&arr);
```

### [`shuffle`](./shuffle.rs)
Shuffle arrays:
```rust
let mut arr = vec![1, 2, 3];
rust_d3::shuffle(&mut arr);
```

### [`sort`](./sort.rs) / [`sort_by`](./sort_by.rs)
Sorting utilities:
```rust
let mut arr = vec![3, 1, 2];
rust_d3::sort(&mut arr);
```

### [`summarize`](./summarize.rs) / [`transform`](./transform.rs)
Data summarization and transformation:
```rust
let arr = vec![1, 2, 3];
let summary = rust_d3::summarize(&arr);
```

### [`intern`](./intern.rs)
Interning sets and maps:
```rust
let arr = vec!["a", "b", "a"];
let set = rust_d3::intern_set(&arr);
```

### [`transpose`](./transpose.rs)
Transpose a matrix (array of arrays):
```rust
let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
let t = rust_d3::transpose(&matrix);
assert_eq!(t, vec![vec![1, 4], vec![2, 5], vec![3, 6]]);
```

### [`bisector`](./bisector.rs)
Generate left/right bisect functions for a comparator or accessor:
```rust
let arr = vec![1, 3, 5, 7, 9];
let (left, right) = rust_d3::bisector(|a: &i32, b: &i32| a.cmp(b));
assert_eq!(left(&arr, &5), 2);
assert_eq!(right(&arr, &5), 3);
```

### [`quickselect`](./quickselect.rs)
Partial sort to find the k-th smallest element:
```rust
let mut arr = vec![9, 1, 8, 2, 7, 3, 6, 4, 5];
rust_d3::quickselect(&mut arr, 4);
assert_eq!(arr[4], 5); // arr[4] is now the 5th smallest element
```

## Integration
All functions are available via the `array` module and many are re-exported at the crate root for convenient use.

---

For more, see the main [README](../README.md).

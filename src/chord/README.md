# d3-chord

This module provides a Chord layout generator, inspired by D3.js's d3-chord.

## Features

-   Generates chord diagrams from a square matrix.
-   Configurable padding between groups (`pad_angle`).
-   Customizable sorting for groups, subgroups, and chords.

## Usage

```rust
use rust_d3::chord::ChordLayout;

let matrix = vec![
    vec![11.0, 12.0, 13.0],
    vec![21.0, 22.0, 23.0],
    vec![31.0, 32.0, 33.0],
];

let chord_layout = ChordLayout::new()
    .pad_angle(0.05)
    .sort_groups(Some(Box::new(|a, b| a.value.partial_cmp(&b.value).unwrap())));

let chords = chord_layout.chords(matrix.clone());
let groups = chord_layout.groups(matrix.clone());

// Now you can use `chords` and `groups` to render your chord diagram.
// For example, print the first chord's source and target values:
// println!("First chord source value: {}", chords[0].source.value);
// println!("First chord target value: {}", chords[0].target.value);
```

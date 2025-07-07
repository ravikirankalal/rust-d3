//! Unit test for d3 chord
use rust_d3::chord::{chord, Chord};

#[test]
fn test_chord_matrix() {
    let matrix = vec![vec![1.0, 0.0], vec![2.0, 3.0]];
    let result = chord(matrix.clone());
    assert_eq!(result.matrix, matrix);
    assert!(result.chords.contains(&Chord { source: 0, target: 0, value: 1.0 }));
    assert!(result.chords.contains(&Chord { source: 1, target: 0, value: 2.0 }));
    assert!(result.chords.contains(&Chord { source: 1, target: 1, value: 3.0 }));
    assert_eq!(result.chords.len(), 3);
}

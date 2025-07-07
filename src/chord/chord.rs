pub struct ChordMatrix {
    pub matrix: Vec<Vec<f64>>,
    pub chords: Vec<Chord>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Chord {
    pub source: usize,
    pub target: usize,
    pub value: f64,
}

/// Compute chords from a matrix: each nonzero entry is a chord.
pub fn chord(matrix: Vec<Vec<f64>>) -> ChordMatrix {
    let mut chords = Vec::new();
    for (i, row) in matrix.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            if value != 0.0 {
                chords.push(Chord { source: i, target: j, value });
            }
        }
    }
    ChordMatrix { matrix, chords }
}

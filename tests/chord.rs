use rust_d3::chord::{
    Chord,
    ChordLayout,
    Group,
    Subgroup,
};
use rust_d3::chord::path::{ArcGenerator, RibbonGenerator};

#[test]
fn test_basic_chord_layout() {
    let matrix = vec![
        vec![11.0, 12.0, 13.0],
        vec![21.0, 22.0, 23.0],
        vec![31.0, 32.0, 33.0],
    ];

    let chord_layout = ChordLayout::new();
    let chords = chord_layout.chords(matrix.clone());
    let groups = chord_layout.groups(matrix.clone());

    // Basic assertions for groups
    assert_eq!(groups.len(), 3);
    assert_eq!(groups[0].index, 0);
    assert_eq!(groups[0].value, 36.0); // 11+12+13
    assert_eq!(groups[1].index, 1);
    assert_eq!(groups[1].value, 66.0); // 21+22+23
    assert_eq!(groups[2].index, 2);
    assert_eq!(groups[2].value, 96.0); // 31+32+33

    // Check angles (approximate due to floating point)
    let pi2 = std::f64::consts::PI * 2.0;
    let total_matrix_sum: f64 = matrix.iter().flat_map(|row| row.iter()).sum();
    let angle_per_value = pi2 / total_matrix_sum;

    assert!((groups[0].start_angle - 0.0).abs() < 1e-9);
    assert!((groups[0].end_angle - (36.0 * angle_per_value)).abs() < 1e-9);
    assert!((groups[1].start_angle - (36.0 * angle_per_value)).abs() < 1e-9);
    assert!((groups[1].end_angle - (36.0 * angle_per_value + 66.0 * angle_per_value)).abs() < 1e-9);

    // Basic assertions for chords
    assert_eq!(chords.len(), 9); // 3x3 matrix, all non-zero

    // Check a specific chord (e.g., from 0 to 1)
    let chord_0_1 = chords.iter().find(|c| c.source.index == 0 && c.source.subindex == 1).unwrap();
    assert_eq!(chord_0_1.source.value, 12.0);
    assert_eq!(chord_0_1.target.value, 21.0); // Flow from 1 to 0
}

#[test]
fn test_pad_angle() {
    let matrix = vec![
        vec![1.0, 0.0],
        vec![0.0, 1.0],
    ];

    let chord_layout = ChordLayout::new().pad_angle(0.1);
    let groups = chord_layout.groups(matrix.clone());

    assert!((groups[0].end_angle + 0.1 - groups[1].start_angle).abs() < 1e-9);
}

#[test]
fn test_sort_groups() {
    let matrix = vec![
        vec![10.0, 0.0],
        vec![0.0, 1.0],
    ];

    // Sort by value descending (default behavior) - D3 default is by index ascending
    let chord_layout_default = ChordLayout::new();
    let groups_default = chord_layout_default.groups(matrix.clone());
    assert_eq!(groups_default[0].index, 0); // Group 0 has value 10
    assert_eq!(groups_default[1].index, 1); // Group 1 has value 1

    // Sort by value ascending
    let chord_layout_asc = ChordLayout::new().sort_groups(Some(Box::new(|a, b| a.value.partial_cmp(&b.value).unwrap())));
    let groups_asc = chord_layout_asc.groups(matrix.clone());
    assert_eq!(groups_asc[0].index, 1); // Group 1 has value 1
    assert_eq!(groups_asc[1].index, 0); // Group 0 has value 10
}

#[test]
fn test_sort_subgroups() {
    let matrix = vec![
        vec![0.0, 5.0, 1.0],
        vec![0.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0],
    ];

    // Default sort (by value descending)
    let chord_layout_default = ChordLayout::new();
    let chords_default = chord_layout_default.chords(matrix.clone());
    
    // Find chords originating from group 0
    let chords_from_0: Vec<&Chord> = chords_default.iter()
        .filter(|c| c.source.index == 0)
        .collect();

    // Expect subgroup with value 5.0 to come before subgroup with value 1.0
    assert_eq!(chords_from_0[0].source.subindex, 1); // Value 5.0
    assert_eq!(chords_from_0[1].source.subindex, 2); // Value 1.0

    // Custom sort (by subindex ascending)
    let chord_layout_asc = ChordLayout::new().sort_subgroups(Some(Box::new(|a, b| a.subindex.cmp(&b.subindex))));
    let chords_asc = chord_layout_asc.chords(matrix.clone());

    let chords_from_0_asc: Vec<&Chord> = chords_asc.iter()
        .filter(|c| c.source.index == 0)
        .collect();
    
    assert_eq!(chords_from_0_asc[0].source.subindex, 1); // Value 5.0
    assert_eq!(chords_from_0_asc[1].source.subindex, 2); // Value 1.0
}

#[test]
fn test_sort_chords() {
    let matrix = vec![
        vec![0.0, 10.0, 1.0],
        vec![5.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0],
    ];

    // Default sort (no specific order guaranteed without custom sort)
    let chord_layout_default = ChordLayout::new();
    let _chords_default = chord_layout_default.chords(matrix.clone());

    // Custom sort (by source value ascending)
    let chord_layout_asc = ChordLayout::new().sort_chords(Some(Box::new(|a, b| a.source.value.partial_cmp(&b.source.value).unwrap())));
    let chords_asc = chord_layout_asc.chords(matrix.clone());

    // Expect chord with source value 1.0 to come before 5.0, then 10.0
    assert_eq!(chords_asc[0].source.value, 1.0);
    assert_eq!(chords_asc[1].source.value, 5.0);
    assert_eq!(chords_asc[2].source.value, 10.0);
}

#[test]
fn test_arc_generator_path() {
    let group = Group {
        start_angle: 0.0,
        end_angle: std::f64::consts::PI / 2.0,
        value: 10.0,
        index: 0,
    };
    let arc_gen = ArcGenerator::new().inner_radius(10.0).outer_radius(20.0);
    let path = arc_gen.path(&group);

    // Assert that the path string is not empty and contains expected SVG commands
    assert!(!path.is_empty());
    assert!(path.starts_with("M"));
    assert!(path.contains("A"));
    assert!(path.contains("L"));
    assert!(path.ends_with("Z"));
}

#[test]
fn test_ribbon_generator_path() {
    let source_subgroup = Subgroup {
        start_angle: 0.0,
        end_angle: std::f64::consts::PI / 4.0,
        value: 5.0,
        index: 0,
        subindex: 0,
    };
    let target_subgroup = Subgroup {
        start_angle: std::f64::consts::PI / 2.0,
        end_angle: 3.0 * std::f64::consts::PI / 4.0,
        value: 5.0,
        index: 1,
        subindex: 0,
    };
    let chord = Chord {
        source: source_subgroup,
        target: target_subgroup,
    };

    let ribbon_gen = RibbonGenerator::new().radius(100.0);
    let path = ribbon_gen.path(&chord);

    // Assert that the path string is not empty and contains expected SVG commands
    assert!(!path.is_empty());
    assert!(path.starts_with("M"));
    assert!(path.contains("A"));
    assert!(path.contains("Q"));
    assert!(path.ends_with("Z"));
}

#[test]
fn test_arc_generator_functional_radius() {
    let group = Group {
        start_angle: 0.0,
        end_angle: std::f64::consts::PI / 2.0,
        value: 10.0,
        index: 0,
    };
    let arc_gen = ArcGenerator::new()
        .inner_radius_fn(Box::new(|_| 15.0))
        .outer_radius_fn(Box::new(|g| 25.0 + g.value)); // Example: outer radius depends on group value
    let path = arc_gen.path(&group);

    assert!(!path.is_empty());
    assert!(path.starts_with("M"));
    assert!(path.contains("A"));
    assert!(path.contains("L"));
    assert!(path.ends_with("Z"));
}

#[test]
fn test_ribbon_generator_functional_radius() {
    let source_subgroup = Subgroup {
        start_angle: 0.0,
        end_angle: std::f64::consts::PI / 4.0,
        value: 5.0,
        index: 0,
        subindex: 0,
    };
    let target_subgroup = Subgroup {
        start_angle: std::f64::consts::PI / 2.0,
        end_angle: 3.0 * std::f64::consts::PI / 4.0,
        value: 5.0,
        index: 1,
        subindex: 0,
    };
    let chord = Chord {
        source: source_subgroup,
        target: target_subgroup,
    };

    let ribbon_gen = RibbonGenerator::new()
        .radius_fn(Box::new(|c| 100.0 + c.source.value + c.target.value)); // Example: radius depends on chord values
    let path = ribbon_gen.path(&chord);

    assert!(!path.is_empty());
    assert!(path.starts_with("M"));
    assert!(path.contains("A"));
    assert!(path.contains("Q"));
    assert!(path.ends_with("Z"));
}

// src/chord/mod.rs
pub mod path;

#[derive(Debug, Clone, PartialEq)]
pub struct Group {
    pub start_angle: f64,
    pub end_angle: f64,
    pub value: f64,
    pub index: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Subgroup {
    pub start_angle: f64,
    pub end_angle: f64,
    pub value: f64,
    pub index: usize,
    pub subindex: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Chord {
    pub source: Subgroup,
    pub target: Subgroup,
}

pub struct ChordLayout {
    pad_angle: f64,
    sort_groups: Option<Box<dyn Fn(&Group, &Group) -> std::cmp::Ordering>>,
    sort_subgroups: Option<Box<dyn Fn(&Subgroup, &Subgroup) -> std::cmp::Ordering>>,
    sort_chords: Option<Box<dyn Fn(&Chord, &Chord) -> std::cmp::Ordering>>,
}

impl ChordLayout {
    pub fn new() -> Self {
        ChordLayout {
            pad_angle: 0.0,
            sort_groups: None,
            sort_subgroups: None,
            sort_chords: None,
        }
    }

    pub fn pad_angle(mut self, angle: f64) -> Self {
        self.pad_angle = angle;
        self
    }

    pub fn sort_groups(mut self, compare: Option<Box<dyn Fn(&Group, &Group) -> std::cmp::Ordering>>) -> Self {
        self.sort_groups = compare;
        self
    }

    pub fn sort_subgroups(mut self, compare: Option<Box<dyn Fn(&Subgroup, &Subgroup) -> std::cmp::Ordering>>) -> Self {
        self.sort_subgroups = compare;
        self
    }

    pub fn sort_chords(mut self, compare: Option<Box<dyn Fn(&Chord, &Chord) -> std::cmp::Ordering>>) -> Self {
        self.sort_chords = compare;
        self
    }

    pub fn groups(&self, matrix: Vec<Vec<f64>>) -> Vec<Group> {
        let n = matrix.len();
        let mut groups: Vec<Group> = Vec::with_capacity(n);
        let mut sums: Vec<f64> = vec![0.0; n];

        // Calculate sum of outgoing and incoming flows for each group
        for i in 0..n {
            for j in 0..n {
                sums[i] += matrix[i][j];
            }
        }

        // Create initial Group objects
        for i in 0..n {
            groups.push(Group {
                index: i,
                value: sums[i],
                start_angle: 0.0,
                end_angle: 0.0,
            });
        }

        // Sort groups if a custom sort function is provided
        if let Some(sort_fn) = &self.sort_groups {
            groups.sort_by(|a, b| sort_fn(a, b));
        }

        // Allocate angles
        let mut total_angle = 0.0;
        let mut k = 0.0; // Sum of all matrix values
        for i in 0..n {
            k += sums[i];
        }

        let pi2 = std::f64::consts::PI * 2.0;
        let mut angle_per_value = (pi2 - self.pad_angle * n as f64) / k;

        if k == 0.0 {
            angle_per_value = 0.0;
        }

        for i in 0..n {
            let group = &mut groups[i];
            group.start_angle = total_angle;
            group.end_angle = total_angle + group.value * angle_per_value;
            total_angle = group.end_angle + self.pad_angle;
        }

        groups
    }

    pub fn chords(&self, matrix: Vec<Vec<f64>>) -> Vec<Chord> {
        let n = matrix.len();
        let mut chords: Vec<Chord> = Vec::new();
        let mut subgroups: Vec<Vec<Subgroup>> = Vec::with_capacity(n);

        // Initialize subgroups for each group
        for i in 0..n {
            subgroups.push(Vec::with_capacity(n));
            for j in 0..n {
                subgroups[i].push(Subgroup {
                    index: i,
                    subindex: j,
                    value: matrix[i][j],
                    start_angle: 0.0,
                    end_angle: 0.0,
                });
            }
        }

        // Get sorted groups with allocated angles
        let groups = self.groups(matrix.clone());

        // Sort subgroups within each group
        if let Some(sort_fn) = &self.sort_subgroups {
            for i in 0..n {
                subgroups[i].sort_by(|a, b| sort_fn(a, b));
            }
        }

        // Allocate angles for subgroups
        for i in 0..n {
            let group_start_angle = groups[i].start_angle;
            let group_end_angle = groups[i].end_angle;
            let group_value = groups[i].value;

            let mut current_angle = group_start_angle;
            let angle_per_value = if group_value == 0.0 { 0.0 } else { (group_end_angle - group_start_angle) / group_value };

            for j in 0..n {
                let subgroup = &mut subgroups[i][j];
                subgroup.start_angle = current_angle;
                subgroup.end_angle = current_angle + subgroup.value * angle_per_value;
                current_angle = subgroup.end_angle;
            }
        }

        // Create Chord objects
        for i in 0..n {
            for j in 0..n {
                if matrix[i][j] > 0.0 { // Changed condition
                    chords.push(Chord {
                        source: subgroups[i][j].clone(),
                        target: subgroups[j][i].clone(),
                    });
                }
            }
        }

        // Sort chords if a custom sort function is provided
        if let Some(sort_fn) = &self.sort_chords {
            chords.sort_by(|a, b| sort_fn(a, b));
        }

        chords
    }
}
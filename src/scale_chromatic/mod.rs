// Color schemes and interpolators for D3.js parity

/// Returns the D3 Category10 categorical color scheme.
pub fn scheme_category10() -> Vec<&'static str> {
    vec![
        "#1f77b4", "#ff7f0e", "#2ca02c", "#d62728", "#9467bd",
        "#8c564b", "#e377c2", "#7f7f7f", "#bcbd22", "#17becf"
    ]
}

/// Returns the D3 Accent categorical color scheme.
pub fn scheme_accent() -> Vec<&'static str> {
    vec![
        "#7fc97f", "#beaed4", "#fdc086", "#ffff99", "#386cb0",
        "#f0027f", "#bf5b17", "#666666"
    ]
}

/// Returns the D3 Dark2 categorical color scheme.
pub fn scheme_dark2() -> Vec<&'static str> {
    vec![
        "#1b9e77", "#d95f02", "#7570b3", "#e7298a", "#66a61e",
        "#e6ab02", "#a6761d", "#666666"
    ]
}

/// Returns the D3 Paired categorical color scheme.
pub fn scheme_paired() -> Vec<&'static str> {
    vec![
        "#a6cee3", "#1f78b4", "#b2df8a", "#33a02c", "#fb9a99", "#e31a1c",
        "#fdbf6f", "#ff7f00", "#cab2d6", "#6a3d9a", "#ffff99", "#b15928"
    ]
}

/// Returns the D3 Set1 categorical color scheme.
pub fn scheme_set1() -> Vec<&'static str> {
    vec![
        "#e41a1c", "#377eb8", "#4daf4a", "#984ea3", "#ff7f00",
        "#ffff33", "#a65628", "#f781bf", "#999999"
    ]
}

/// Returns the D3 Set2 categorical color scheme.
pub fn scheme_set2() -> Vec<&'static str> {
    vec![
        "#66c2a5", "#fc8d62", "#8da0cb", "#e78ac3", "#a6d854",
        "#ffd92f", "#e5c494", "#b3b3b3"
    ]
}

/// Returns the D3 Set3 categorical color scheme.
pub fn scheme_set3() -> Vec<&'static str> {
    vec![
        "#8dd3c7", "#ffffb3", "#bebada", "#fb8072", "#80b1d3", "#fdb462",
        "#b3de69", "#fccde5", "#d9d9d9", "#bc80bd", "#ccebc5", "#ffed6f"
    ]
}

/// Returns the D3 Pastel1 categorical color scheme.
pub fn scheme_pastel1() -> Vec<&'static str> {
    vec![
        "#fbb4ae", "#b3cde3", "#ccebc5", "#decbe4", "#fed9a6",
        "#ffffcc", "#e5d8bd", "#fddaec", "#f2f2f2"
    ]
}

/// Returns the D3 Pastel2 categorical color scheme.
pub fn scheme_pastel2() -> Vec<&'static str> {
    vec![
        "#b3e2cd", "#fdcdac", "#cbd5e8", "#f4cae4", "#e6f5c9",
        "#fff2ae", "#f1e2cc", "#cccccc"
    ]
}

/// Returns the D3 Tableau10 categorical color scheme.
pub fn scheme_tableau10() -> Vec<&'static str> {
    vec![
        "#4e79a7", "#f28e2b", "#e15759", "#76b7b2", "#59a14f",
        "#edc949", "#af7aa1", "#ff9da7", "#9c755f", "#bab0ab"
    ]
}

/// Returns a color from the Viridis sequential interpolator for t in [0, 1].
pub fn interpolate_viridis(t: f64) -> &'static str {
    // 6-point lookup table for demonstration (real d3 uses 256)
    const VIRIDIS: [&str; 6] = [
        "#440154", "#482777", "#3e4989", "#31688e", "#26828e", "#35b779"
    ];
    let idx = (t.clamp(0.0, 1.0) * (VIRIDIS.len() as f64 - 1.0)).round() as usize;
    VIRIDIS[idx]
}

/// Returns a color from the Inferno sequential interpolator for t in [0, 1].
pub fn interpolate_inferno(t: f64) -> &'static str {
    const INFERNO: [&str; 6] = [
        "#000004", "#1b0c41", "#4a0c6b", "#781c6d", "#b52f8c", "#fca50a"
    ];
    let idx = (t.clamp(0.0, 1.0) * (INFERNO.len() as f64 - 1.0)).round() as usize;
    INFERNO[idx]
}

/// Returns a color from the Plasma sequential interpolator for t in [0, 1].
pub fn interpolate_plasma(t: f64) -> &'static str {
    const PLASMA: [&str; 6] = [
        "#0d0887", "#6a00a8", "#b12a90", "#e16462", "#fca636", "#f0f921"
    ];
    let idx = (t.clamp(0.0, 1.0) * (PLASMA.len() as f64 - 1.0)).round() as usize;
    PLASMA[idx]
}

/// Returns a color from the Magma sequential interpolator for t in [0, 1].
pub fn interpolate_magma(t: f64) -> &'static str {
    const MAGMA: [&str; 6] = [
        "#000004", "#1c1044", "#51127c", "#b63679", "#fb8861", "#fcfdbf"
    ];
    let idx = (t.clamp(0.0, 1.0) * (MAGMA.len() as f64 - 1.0)).round() as usize;
    MAGMA[idx]
}

// Add more schemes/interpolators as needed for full d3-scale-chromatic API parity.

//! D3 Chord Advanced module: ribbon generator for chord diagrams
//!
//! Ribbons, subgroups, etc.

/// Generates SVG path data for a chord ribbon (arc between two points)
pub fn chord_ribbon(
    source_start_angle: f64,
    source_end_angle: f64,
    target_start_angle: f64,
    target_end_angle: f64,
    radius: f64,
) -> String {
    // D3's ribbon: draws an arc from source to target and back
    let sx0 = radius * source_start_angle.cos();
    let sy0 = radius * source_start_angle.sin();
    let sx1 = radius * source_end_angle.cos();
    let sy1 = radius * source_end_angle.sin();
    let tx0 = radius * target_start_angle.cos();
    let ty0 = radius * target_start_angle.sin();
    let tx1 = radius * target_end_angle.cos();
    let ty1 = radius * target_end_angle.sin();
    format!(
        "M{:.6},{:.6}A{r},{r} 0 0,1 {:.6},{:.6}Q0,0 {:.6},{:.6}A{r},{r} 0 0,1 {:.6},{:.6}Q0,0 {:.6},{:.6}Z",
        sx0, sy0, sx1, sy1, tx1, ty1, tx0, ty0, sx0, sy0, r=radius
    )
}

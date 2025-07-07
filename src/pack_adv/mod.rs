//! D3 Pack Advanced module: padding and radius for pack layouts

#[derive(Debug, Clone, Copy)]
pub struct PackConfig {
    pub radius: f64,
    pub padding: f64,
}

impl Default for PackConfig {
    fn default() -> Self {
        Self { radius: 1.0, padding: 0.0 }
    }
}

/// Pack layout with configurable padding and radius
pub fn pack_with_config<T: Clone>(
    root: &crate::hierarchy::Node<T>,
    config: PackConfig,
) -> Vec<crate::pack::PackNode<T>> {
    let mut result = Vec::new();
    fn walk<T: Clone>(
        node: &crate::hierarchy::Node<T>,
        x: f64,
        y: f64,
        r: f64,
        padding: f64,
        result: &mut Vec<crate::pack::PackNode<T>>,
    ) {
        result.push(crate::pack::PackNode {
            value: node.value.clone(),
            x,
            y,
            r,
        });
        let n = node.children.len();
        if n == 0 { return; }
        let child_r = (r - padding * (n as f64)) / (n as f64 * 2.0);
        let angle_step = std::f64::consts::TAU / n as f64;
        for (i, child) in node.children.iter().enumerate() {
            let angle = i as f64 * angle_step;
            let cx = x + (r - child_r - padding) * angle.cos();
            let cy = y + (r - child_r - padding) * angle.sin();
            walk(child, cx, cy, child_r, padding, result);
        }
    }
    walk(
        root,
        0.0,
        0.0,
        config.radius,
        config.padding,
        &mut result,
    );
    result
}

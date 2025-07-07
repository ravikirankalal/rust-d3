//! D3 Partition Advanced module: padding and size for partition layouts

#[derive(Debug, Clone, Copy)]
pub struct PartitionConfig {
    pub width: f64,
    pub height: f64,
    pub padding: f64,
}

impl Default for PartitionConfig {
    fn default() -> Self {
        Self { width: 1.0, height: 1.0, padding: 0.0 }
    }
}

/// Partition layout with configurable padding and size
pub fn partition_with_config<T: Clone>(
    root: &crate::hierarchy::Node<T>,
    config: PartitionConfig,
) -> Vec<crate::partition::PartitionNode<T>> {
    let mut result = Vec::new();
    fn walk<T: Clone>(
        node: &crate::hierarchy::Node<T>,
        x0: f64,
        x1: f64,
        y0: f64,
        y1: f64,
        padding: f64,
        result: &mut Vec<crate::partition::PartitionNode<T>>,
    ) {
        result.push(crate::partition::PartitionNode {
            value: node.value.clone(),
            x0,
            x1,
            y0,
            y1,
        });
        let n = node.children.len();
        if n == 0 { return; }
        let pad = padding * ((n - 1) as f64);
        let width = x1 - x0 - pad;
        let mut x = x0;
        for child in node.children.iter() {
            let child_x0 = x;
            let child_x1 = x + width / n as f64;
            walk(child, child_x0, child_x1, y0 + (y1 - y0) / 2.0, y1, padding, result);
            x = child_x1 + padding;
        }
    }
    walk(
        root,
        0.0,
        config.width,
        0.0,
        config.height,
        config.padding,
        &mut result,
    );
    result
}

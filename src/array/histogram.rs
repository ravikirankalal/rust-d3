pub struct Bin {
    pub x0: f64,
    pub x1: f64,
    pub values: Vec<f64>,
}

pub fn histogram(data: &[f64], num_bins: usize) -> Vec<Bin> {
    if data.is_empty() || num_bins == 0 {
        return Vec::new();
    }

    let min_val = *data
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max_val = *data
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    if min_val == max_val {
        // Handle the case where all values are the same
        return vec![Bin {
            x0: min_val,
            x1: min_val,
            values: data.to_vec(),
        }];
    }

    let bin_width = (max_val - min_val) / num_bins as f64;
    let mut bins: Vec<Bin> = (0..num_bins)
        .map(|i| {
            let x0 = min_val + i as f64 * bin_width;
            let x1 = if i == num_bins - 1 {
                max_val
            } else {
                min_val + (i + 1) as f64 * bin_width
            };
            Bin {
                x0,
                x1,
                values: Vec::new(),
            }
        })
        .collect();

    for &value in data {
        let mut bin_index = ((value - min_val) / bin_width).floor() as usize;
        if bin_index >= num_bins {
            bin_index = num_bins - 1; // Ensure the last value falls into the last bin
        }
        bins[bin_index].values.push(value);
    }

    bins
}

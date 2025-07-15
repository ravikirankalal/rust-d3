// d3-shape: pie layout
// Computes start/end angles for arcs from data

pub struct Pie<V, S, SA, EA, PA, T>
where
    V: Fn(&T) -> f64,
    S: Fn(&T, &T) -> std::cmp::Ordering,
    SA: Fn() -> f64,
    EA: Fn() -> f64,
    PA: Fn() -> f64,
{
    value: V,
    sort: Option<S>,
    start_angle: SA,
    end_angle: EA,
    pad_angle: PA,
    _phantom: std::marker::PhantomData<T>,
}

pub struct PieSlice<'a, T> {
    pub value: f64,
    pub index: usize,
    pub start_angle: f64,
    pub end_angle: f64,
    pub pad_angle: f64,
    pub data: &'a T,
}

impl<T>
    Pie<fn(&T) -> f64, fn(&T, &T) -> std::cmp::Ordering, fn() -> f64, fn() -> f64, fn() -> f64, T>
{
    pub fn new() -> Self {
        Self {
            value: |_d| 1.0,
            sort: None,
            start_angle: || 0.0,
            end_angle: || std::f64::consts::TAU,
            pad_angle: || 0.0,
            _phantom: std::marker::PhantomData,
        }
    }
}

pub trait PieOutput {
    fn slice(&mut self, start_angle: f64, end_angle: f64, value: f64, index: usize);
}

impl PieOutput for Vec<(f64, f64, f64, usize)> {
    fn slice(&mut self, start_angle: f64, end_angle: f64, value: f64, index: usize) {
        self.push((start_angle, end_angle, value, index));
    }
}

impl<V, S, SA, EA, PA, T> Pie<V, S, SA, EA, PA, T>
where
    V: Fn(&T) -> f64,
    S: Fn(&T, &T) -> std::cmp::Ordering,
    SA: Fn() -> f64,
    EA: Fn() -> f64,
    PA: Fn() -> f64,
{
    pub fn value<V2>(self, value: V2) -> Pie<V2, S, SA, EA, PA, T>
    where
        V2: Fn(&T) -> f64,
    {
        Pie {
            value,
            sort: self.sort,
            start_angle: self.start_angle,
            end_angle: self.end_angle,
            pad_angle: self.pad_angle,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn sort<S2>(self, sort: S2) -> Pie<V, S2, SA, EA, PA, T>
    where
        S2: Fn(&T, &T) -> std::cmp::Ordering,
    {
        Pie {
            value: self.value,
            sort: Some(sort),
            start_angle: self.start_angle,
            end_angle: self.end_angle,
            pad_angle: self.pad_angle,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn start_angle<SA2>(self, start_angle: SA2) -> Pie<V, S, SA2, EA, PA, T>
    where
        SA2: Fn() -> f64,
    {
        Pie {
            value: self.value,
            sort: self.sort,
            start_angle,
            end_angle: self.end_angle,
            pad_angle: self.pad_angle,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn end_angle<EA2>(self, end_angle: EA2) -> Pie<V, S, SA, EA2, PA, T>
    where
        EA2: Fn() -> f64,
    {
        Pie {
            value: self.value,
            sort: self.sort,
            start_angle: self.start_angle,
            end_angle,
            pad_angle: self.pad_angle,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn pad_angle<PA2>(self, pad_angle: PA2) -> Pie<V, S, SA, EA, PA2, T>
    where
        PA2: Fn() -> f64,
    {
        Pie {
            value: self.value,
            sort: self.sort,
            start_angle: self.start_angle,
            end_angle: self.end_angle,
            pad_angle,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn generate<'a>(&self, data: &'a [T]) -> Vec<PieSlice<'a, T>> {
        let mut indexed: Vec<(usize, &T)> = data.iter().enumerate().collect();
        if let Some(ref sort_fn) = self.sort {
            indexed.sort_by(|a, b| sort_fn(a.1, b.1));
        }
        let values: Vec<f64> = indexed.iter().map(|&(_i, d)| (self.value)(d)).collect();
        let total: f64 = values.iter().map(|&v| v.max(0.0)).sum();
        let n = indexed.len();
        let start_angle = (self.start_angle)();
        let end_angle = (self.end_angle)();
        let pad_angle = (self.pad_angle)();
        let angle_range = end_angle - start_angle;
        let mut current_angle = start_angle;
        let mut result = Vec::with_capacity(n);
        for ((orig_idx, datum), value) in indexed.into_iter().zip(values.into_iter()) {
            let angle = if total > 0.0 {
                value.max(0.0) / total * angle_range
            } else {
                0.0
            };
            let sa = current_angle;
            let ea = sa + angle;
            result.push(PieSlice {
                value,
                index: orig_idx,
                start_angle: sa,
                end_angle: ea,
                pad_angle,
                data: datum,
            });
            current_angle = ea + pad_angle;
        }
        // Sort result by original index for D3 parity
        result.sort_by_key(|s| s.index);
        result
    }
    pub fn generate_to<O: PieOutput>(&self, data: &[T], out: &mut O) {
        let slices = self.generate(data);
        for s in slices.iter() {
            out.slice(s.start_angle, s.end_angle, s.value, s.index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pie_basic() {
        let data = vec![1.0, 1.0, 2.0];
        let pie = Pie::new().value(|d: &f64| *d);
        let slices = pie.generate(&data);
        // println!("slice values: {:?}", slices.iter().map(|s| s.value).collect::<Vec<_>>());
        assert_eq!(slices.len(), 3);
        let total_angle: f64 = slices.iter().map(|s| s.end_angle - s.start_angle).sum();
        assert!((total_angle - std::f64::consts::TAU).abs() < 1e-6);
        assert_eq!(slices[2].value, 2.0);
    }
}

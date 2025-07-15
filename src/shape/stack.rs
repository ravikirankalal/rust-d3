// d3-shape: stack layout
// Computes stacked series for area/bar charts

pub struct Stack<K, V, O, F, T>
where
    K: Fn(&T) -> Vec<String>,
    V: Fn(&T, &str) -> f64,
    O: Fn(&[String]) -> Vec<String>,
    F: Fn(&[f64]) -> Vec<f64>,
{
    keys: K,
    value: V,
    order: Option<O>,
    offset: Option<F>,
    _phantom: std::marker::PhantomData<T>,
}

pub struct StackSeries {
    pub key: String,
    pub values: Vec<(f64, f64)>, // (start, end) for each datum
}

impl<T>
    Stack<
        fn(&T) -> Vec<String>,
        fn(&T, &str) -> f64,
        fn(&[String]) -> Vec<String>,
        fn(&[f64]) -> Vec<f64>,
        T,
    >
{
    pub fn new() -> Self {
        Self {
            keys: |_d| vec![],
            value: |_d, _k| 0.0,
            order: None,
            offset: None,
            _phantom: std::marker::PhantomData,
        }
    }
}

pub trait StackOutput {
    fn series(&mut self, key: &str, values: &[(f64, f64)]);
}

impl StackOutput for Vec<(String, Vec<(f64, f64)>)> {
    fn series(&mut self, key: &str, values: &[(f64, f64)]) {
        self.push((key.to_string(), values.to_vec()));
    }
}

impl<K, V, O, F, T> Stack<K, V, O, F, T>
where
    K: Fn(&T) -> Vec<String>,
    V: Fn(&T, &str) -> f64,
    O: Fn(&[String]) -> Vec<String>,
    F: Fn(&[f64]) -> Vec<f64>,
{
    pub fn keys<K2>(self, keys: K2) -> Stack<K2, V, O, F, T>
    where
        K2: Fn(&T) -> Vec<String>,
    {
        Stack {
            keys,
            value: self.value,
            order: self.order,
            offset: self.offset,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn value<V2>(self, value: V2) -> Stack<K, V2, O, F, T>
    where
        V2: Fn(&T, &str) -> f64,
    {
        Stack {
            keys: self.keys,
            value,
            order: self.order,
            offset: self.offset,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn order<O2>(self, order: O2) -> Stack<K, V, O2, F, T>
    where
        O2: Fn(&[String]) -> Vec<String>,
    {
        Stack {
            keys: self.keys,
            value: self.value,
            order: Some(order),
            offset: self.offset,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn offset<F2>(self, offset: F2) -> Stack<K, V, O, F2, T>
    where
        F2: Fn(&[f64]) -> Vec<f64>,
    {
        Stack {
            keys: self.keys,
            value: self.value,
            order: self.order,
            offset: Some(offset),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn generate(&self, data: &[T]) -> Vec<StackSeries> {
        if data.is_empty() {
            return vec![];
        }
        let keys = (self.keys)(&data[0]);
        let order = if let Some(ref o) = self.order {
            o(&keys)
        } else {
            keys.clone()
        };
        let n = data.len();
        let mut series: Vec<StackSeries> = order
            .iter()
            .map(|k| StackSeries {
                key: k.clone(),
                values: vec![(0.0, 0.0); n],
            })
            .collect();
        for (i, d) in data.iter().enumerate() {
            let mut acc = 0.0;
            for (j, k) in order.iter().enumerate() {
                let v = (self.value)(d, k);
                let start = acc;
                let end = acc + v;
                series[j].values[i] = (start, end);
                acc = end;
            }
        }
        // Offset support (not implemented, stub)
        // TODO: implement offset (e.g., expand, silhouette, wiggle)
        series
    }
    pub fn generate_to<O2: StackOutput>(&self, data: &[T], out: &mut O2) {
        let series = self.generate(data);
        for s in series.iter() {
            out.series(&s.key, &s.values);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_stack_basic() {
        let data = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let stack = Stack::new()
            .keys(|_d| vec!["a".to_string(), "b".to_string()])
            .value(|d: &Vec<f64>, k| match k {
                "a" => d[0],
                "b" => d[1],
                _ => 0.0,
            });
        let series = stack.generate(&data);
        assert_eq!(series.len(), 2);
        assert_eq!(series[0].key, "a");
        assert_eq!(series[1].key, "b");
        assert_eq!(series[0].values[0], (0.0, 1.0));
        assert_eq!(series[1].values[0], (1.0, 3.0));
        assert_eq!(series[0].values[1], (0.0, 3.0));
        assert_eq!(series[1].values[1], (3.0, 7.0));
    }
}

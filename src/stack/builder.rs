//! D3-style stack generator with builder pattern, key/value accessors, order/offset, and series metadata.

use std::marker::PhantomData;

pub struct SeriesMeta<K> {
    pub key: K,
    pub index: usize,
}

pub struct StackedSeries<K, P> {
    pub meta: SeriesMeta<K>,
    pub points: Vec<(f64, f64, P)>, // (y0, y1, original point)
}

pub struct Stack<D, K> {
    keys: Option<Vec<K>>,
    key_fn: Option<Box<dyn Fn(&D) -> K>>,
    value_fn: Option<Box<dyn Fn(&D, &K) -> f64>>,
    order_fn: Option<Box<dyn Fn(&[Vec<f64>]) -> Vec<usize>>>,
    offset_fn: Option<Box<dyn Fn(&mut [Vec<(f64, f64)>])>>,
    _phantom: PhantomData<D>,
}

impl<D, K> Stack<D, K>
where
    K: Clone + PartialEq + 'static,
    D: Clone + 'static,
{
    pub fn new() -> Self {
        Self {
            keys: None,
            key_fn: None,
            value_fn: None,
            order_fn: None,
            offset_fn: None,
            _phantom: PhantomData,
        }
    }
    pub fn keys(mut self, keys: Vec<K>) -> Self {
        self.keys = Some(keys);
        self
    }
    pub fn key_fn<F>(mut self, f: F) -> Self
    where
        F: Fn(&D) -> K + 'static,
    {
        self.key_fn = Some(Box::new(f));
        self
    }
    pub fn value_fn<F>(mut self, f: F) -> Self
    where
        F: Fn(&D, &K) -> f64 + 'static,
    {
        self.value_fn = Some(Box::new(f));
        self
    }
    pub fn order_fn<F>(mut self, f: F) -> Self
    where
        F: Fn(&[Vec<f64>]) -> Vec<usize> + 'static,
    {
        self.order_fn = Some(Box::new(f));
        self
    }
    pub fn offset_fn<F>(mut self, f: F) -> Self
    where
        F: Fn(&mut [Vec<(f64, f64)>]) + 'static,
    {
        self.offset_fn = Some(Box::new(f));
        self
    }
    pub fn stack(&self, data: &[D]) -> Vec<StackedSeries<K, D>>
    where
        K: PartialEq,
        D: Clone,
    {
        // 1. Determine keys
        let keys = if let Some(ref keys) = self.keys {
            keys.clone()
        } else if let Some(ref key_fn) = self.key_fn {
            let mut ks = vec![];
            for d in data {
                let k = key_fn(d);
                if !ks.contains(&k) {
                    ks.push(k);
                }
            }
            ks
        } else {
            panic!("No keys or key_fn provided");
        };
        // 2. Build value matrix
        let matrix: Vec<Vec<f64>> = keys
            .iter()
            .map(|k| data.iter().map(|d| (self.value_fn.as_ref().unwrap())(d, k)).collect())
            .collect();
        // 3. Order
        let order = if let Some(ref order_fn) = self.order_fn {
            order_fn(&matrix)
        } else {
            (0..matrix.len()).collect()
        };
        // 4. Stack
        let n = data.len();
        let mut result = vec![vec![(0.0, 0.0); n]; matrix.len()];
        for i in 0..n {
            let mut acc = 0.0;
            for &j in &order {
                let v = matrix[j][i];
                result[j][i] = (acc, acc + v);
                acc += v;
            }
        }
        // 5. Offset
        if let Some(ref offset_fn) = self.offset_fn {
            offset_fn(&mut result);
        }
        // 6. Attach metadata and original data
        let mut out = vec![];
        for (idx, &j) in order.iter().enumerate() {
            let meta = SeriesMeta { key: keys[j].clone(), index: idx };
            let points = result[j]
                .iter()
                .enumerate()
                .map(|(i, &(y0, y1))| (y0, y1, data[i].clone()))
                .collect();
            out.push(StackedSeries { meta, points });
        }
        out
    }
}

// d3-collection: nest implementation (WIP)
// See: https://github.com/d3/d3-collection#nest

// Placeholder for nest function and related types
// Port logic and tests from d3-collection as next step

use std::collections::HashMap;

/// A builder for grouping and aggregating data by keys, similar to d3-collection's nest.
pub struct Nest<K, V> {
    key_fn: Box<dyn Fn(&V) -> K>,
}

impl<K: Eq + std::hash::Hash, V> Nest<K, V> {
    pub fn new<F>(key_fn: F) -> Self
    where
        F: 'static + Fn(&V) -> K,
    {
        Self {
            key_fn: Box::new(key_fn),
        }
    }

    pub fn entries<'a>(&self, values: &'a [V]) -> HashMap<K, Vec<&'a V>> {
        let mut map: HashMap<K, Vec<&'a V>> = HashMap::new();
        for v in values {
            let k = (self.key_fn)(v);
            map.entry(k).or_default().push(v);
        }
        map
    }
}

/// Convenience function to create a Nest builder
pub fn nest<K: Eq + std::hash::Hash, V, F>(key_fn: F) -> Nest<K, V>
where
    F: 'static + Fn(&V) -> K,
{
    Nest::new(key_fn)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_nest_entries() {
        #[derive(Debug, PartialEq, Eq, Hash)]
        struct Person { name: &'static str, age: u32 }
        let people = vec![
            Person { name: "Alice", age: 30 },
            Person { name: "Bob", age: 25 },
            Person { name: "Charlie", age: 30 },
            Person { name: "David", age: 25 },
        ];
        let n = nest(|p: &Person| p.age);
        let grouped = n.entries(&people);
        assert_eq!(grouped[&30].len(), 2);
        assert_eq!(grouped[&25].len(), 2);
    }
}

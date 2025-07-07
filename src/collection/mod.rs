// d3-collection parity module root
// Add submodules here as you port features

pub mod nest;
pub use nest::nest;

pub mod map;
pub use map::Map;

pub mod set;
pub use set::Set;

pub mod keys;
pub use keys::keys;
pub mod values;
pub use values::values;
pub mod entries;
pub use entries::entries;
pub mod rollup;
pub use rollup::rollup;
pub mod index;
pub use index::index;
pub mod groups;
pub use groups::groups;
pub mod flat_group;
pub use flat_group::flat_group;
pub mod flat_rollup;
pub use flat_rollup::flat_rollup;
pub mod from_entries;
pub use from_entries::from_entries;
pub mod count;
pub use count::count;
pub mod count_map;
pub use count_map::count_map;
pub mod count_values;
pub use count_values::count_values;
pub mod filter_map;
pub use filter_map::filter_map;
pub mod map_map;
pub use map_map::map_map;
pub mod map_keys;
pub use map_keys::map_keys;
pub mod map_values;
pub use map_values::map_values;
pub mod merge_maps;
pub use merge_maps::merge_maps;
pub mod invert;
pub use invert::invert;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_keys() {
        let mut m = HashMap::new();
        m.insert("a", 1);
        m.insert("b", 2);
        let mut k = keys(&m);
        k.sort();
        assert_eq!(k, vec!["a", "b"]);
    }

    #[test]
    fn test_values() {
        let mut m = HashMap::new();
        m.insert("a", 1);
        m.insert("b", 2);
        let mut v = values(&m);
        v.sort();
        assert_eq!(v, vec![1, 2]);
    }

    #[test]
    fn test_entries() {
        let mut m = HashMap::new();
        m.insert("a", 1);
        m.insert("b", 2);
        let mut e = entries(&m);
        e.sort_by(|a, b| a.0.cmp(&b.0));
        assert_eq!(e, vec![("a", 1), ("b", 2)]);
    }

    #[test]
    fn test_rollup() {
        let data = vec![1, 2, 2, 3, 3, 3];
        let result = rollup(&data, |x| *x, |vs| vs.len());
        assert_eq!(result[&1], 1);
        assert_eq!(result[&2], 2);
        assert_eq!(result[&3], 3);
    }

    #[test]
    fn test_index() {
        let data = vec!["a", "b", "c"];
        let idx = index(&data, |x| *x);
        assert_eq!(idx[&"a"], &"a");
        assert_eq!(idx[&"b"], &"b");
        assert_eq!(idx[&"c"], &"c");
    }

    #[test]
    fn test_groups() {
        let data = vec![1, 2, 2, 3];
        let g = groups(&data, |x| *x);
        assert_eq!(g[&1], vec![&1]);
        assert_eq!(g[&2], vec![&2, &2]);
        assert_eq!(g[&3], vec![&3]);
    }

    #[test]
    fn test_flat_group() {
        let data = vec![1, 2, 2, 3];
        let mut g = flat_group(&data, |x| *x);
        g.sort_by_key(|(k, _)| *k);
        assert_eq!(g[0], (1, vec![&1]));
        assert_eq!(g[1], (2, vec![&2, &2]));
        assert_eq!(g[2], (3, vec![&3]));
    }

    #[test]
    fn test_flat_rollup() {
        let data = vec![1, 2, 2, 3];
        let mut r = flat_rollup(&data, |x| *x, |vs| vs.len());
        r.sort_by_key(|(k, _)| *k);
        assert_eq!(r, vec![(1, 1), (2, 2), (3, 1)]);
    }

    #[test]
    fn test_from_entries() {
        let entries = vec![("a", 1), ("b", 2)];
        let map = from_entries(entries.clone());
        assert_eq!(map.get("a"), Some(&1));
        assert_eq!(map.get("b"), Some(&2));
    }

    #[test]
    fn test_count() {
        let data = vec!["a", "b", "a", "c", "b", "a"];
        let counts = count(&data, |x| *x);
        assert_eq!(counts.get(&"a"), Some(&3));
        assert_eq!(counts.get(&"b"), Some(&2));
        assert_eq!(counts.get(&"c"), Some(&1));
    }

    #[test]
    fn test_count_map() {
        let data = vec!["a", "b", "a", "c", "b", "a"];
        let counts = count_map(&data, |x: &&str| x.chars().next().unwrap());
        assert_eq!(counts.get(&'a'), Some(&3));
        assert_eq!(counts.get(&'b'), Some(&2));
        assert_eq!(counts.get(&'c'), Some(&1));
    }

    #[test]
    fn test_count_values() {
        let data = vec![1, 2, 1, 3, 2, 1];
        let counts = count_values(&data, |x| *x);
        // count_values returns Vec<(K, usize)>
        let get_count = |k| counts.iter().find(|(key, _)| key == &k).map(|(_, v)| *v);
        assert_eq!(get_count(1), Some(3));
        assert_eq!(get_count(2), Some(2));
        assert_eq!(get_count(3), Some(1));
    }

    #[test]
    fn test_map_map() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        let mapped = map_map(&map, |k: &&str, v: &i32| (k.to_uppercase(), v * 10));
        assert_eq!(mapped.get("A"), Some(&10));
        assert_eq!(mapped.get("B"), Some(&20));
    }

    #[test]
    fn test_map_map_identity() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        let mapped = map_map(&map, |k: &&str, v: &i32| ((*k).to_string(), *v));
        assert_eq!(mapped.get("a"), Some(&1));
    }

    #[test]
    fn test_merge_maps_overlap() {
        let mut map1 = HashMap::new();
        map1.insert("x", 1);
        let mut map2 = HashMap::new();
        map2.insert("x", 2);
        let merged = merge_maps(&map1, &map2);
        assert_eq!(merged.get("x"), Some(&2));
    }

    #[test]
    fn test_invert() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        let inverted = invert(&map);
        assert_eq!(inverted.get(&1), Some(&"a"));
        assert_eq!(inverted.get(&2), Some(&"b"));
    }

    #[test]
    fn test_from_entries_empty() {
        let entries: Vec<(&str, i32)> = vec![];
        let map = from_entries(entries);
        assert!(map.is_empty());
    }

    #[test]
    fn test_count_empty() {
        let data: Vec<&str> = vec![];
        let counts = count(&data, |x| *x);
        assert!(counts.is_empty());
    }

    #[test]
    fn test_count_map_duplicates() {
        let data = vec!["apple", "apricot", "banana"];
        let counts = count_map(&data, |x: &&str| x.chars().next().unwrap());
        assert_eq!(counts.get(&'a'), Some(&2));
        assert_eq!(counts.get(&'b'), Some(&1));
    }

    #[test]
    fn test_count_values_non_string() {
        let data = vec![10, 20, 10, 30, 20, 10];
        let counts = count_values(&data, |x| *x);
        let get_count = |k| counts.iter().find(|(key, _)| key == &k).map(|(_, v)| *v);
        assert_eq!(get_count(10), Some(3));
        assert_eq!(get_count(20), Some(2));
        assert_eq!(get_count(30), Some(1));
    }

    #[test]
    fn test_filter_map_none_pass() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        let filtered = filter_map(&map, |_k, _v| false);
        assert!(filtered.is_empty());
    }

    #[test]
    fn test_filter_map_all_pass() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        let filtered = filter_map(&map, |_k, _v| true);
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_map_keys_non_string() {
        let mut map = HashMap::new();
        map.insert(1, "one");
        map.insert(2, "two");
        let mapped = map_keys(&map, |k| k * 10);
        assert_eq!(mapped.get(&10), Some(&"one"));
        assert_eq!(mapped.get(&20), Some(&"two"));
    }

    #[test]
    fn test_map_values_complex() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        let mapped = map_values(&map, |v| v.to_string());
        assert_eq!(mapped.get("a"), Some(&"1".to_string()));
        assert_eq!(mapped.get("b"), Some(&"2".to_string()));
    }

    #[test]
    fn test_invert_duplicate_values() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 1); // duplicate value
        let inverted = invert(&map);
        // Only one key will be kept for value 1
        assert!(inverted.get(&1) == Some(&"a") || inverted.get(&1) == Some(&"b"));
        assert_eq!(inverted.len(), 1);
    }

    #[test]
    fn test_groups_empty() {
        let data: Vec<i32> = vec![];
        let g = groups(&data, |x| *x);
        assert!(g.is_empty());
    }

    #[test]
    fn test_flat_group_empty() {
        let data: Vec<i32> = vec![];
        let g = flat_group(&data, |x| *x);
        assert!(g.is_empty());
    }

    #[test]
    fn test_flat_rollup_empty() {
        let data: Vec<i32> = vec![];
        let r = flat_rollup(&data, |x| *x, |vs| vs.len());
        assert!(r.is_empty());
    }

    #[test]
    fn test_rollup_nested() {
        let data = vec![("a", 1), ("a", 2), ("b", 3)];
        let result = rollup(&data, |x| x.0, |vs| vs.iter().map(|x| x.1).sum::<i32>());
        assert_eq!(result[&"a"], 3);
        assert_eq!(result[&"b"], 3);
    }
}


pub mod max;
pub mod min;
pub mod extent;
pub mod mean;
pub mod median;
pub mod sum;
pub mod deviation;
pub mod variance;
pub mod quantile;
pub mod histogram;
pub mod bisect;
pub mod ascending;
pub mod descending;
pub mod range;
pub mod merge;
pub mod shuffle;
pub mod tick_step;
pub mod ticks;
pub mod nice;
pub mod scan;
pub mod group;
pub mod flat_group;
pub mod pairs;
pub mod zip;
pub mod cross;
pub mod least;
pub mod greatest;
pub mod least_index;
pub mod greatest_index;
pub mod fsum;
pub mod blur;
pub mod union;
pub mod intersection;
pub mod difference;
pub mod symmetric_difference;
pub use union::union;
pub use intersection::intersection;
pub use difference::difference;
pub use symmetric_difference::symmetric_difference;
pub mod sort;
pub mod sort_by;
pub mod summarize;
pub mod transform;
pub use sort::sort;
pub use sort_by::sort_by;
pub use summarize::summarize;
pub use transform::transform;
pub mod intern;
pub use intern::{intern_set, intern_map};

#[cfg(test)]
mod tests {
    use super::max::max;
    use super::min::min;
    use super::extent::extent;
    use super::mean::mean;
    use super::median::median;
    use super::sum::sum;
    use super::deviation::deviation;
    use super::variance::variance;
    use super::quantile::quantile;
    use super::histogram::histogram;
    use super::bisect::{bisect_left, bisect_right};
    use super::ascending::ascending;
    use super::descending::descending;
    use super::range::range;
    use super::merge::merge;
    use super::shuffle::shuffle;
    use super::tick_step::tick_step;
    use super::ticks::ticks;
    use super::nice::nice;
    use super::scan::scan;
    use super::group::group;
    use super::flat_group::flat_group;
    use super::pairs::pairs;
    use super::zip::zip;
    use super::cross::cross;
    use super::least::least;
    use super::greatest::greatest;
    use super::least_index::least_index;
    use super::greatest_index::greatest_index;
    use super::fsum::fsum;
    use super::blur::blur1d;
    use super::{union, intersection, difference, symmetric_difference};
    use super::sort::{sort};
    use super::sort_by::{sort_by};
    use super::summarize::{summarize};
    use super::transform::{transform};
    use super::intern::{intern_set, intern_map};
    use std::cmp::Ordering;
    use std::collections::HashMap;

    #[test]
    fn test_max() {
        assert_eq!(max(&[1, 2, 3, 4, 5]), Some(5));
        assert_eq!(max(&[5, 4, 3, 2, 1]), Some(5));
        assert_eq!(max(&[1]), Some(1));
        assert_eq!(max(&[] as &[i32]), None);
        assert_eq!(max(&[1.0, 2.5, 0.5]), Some(2.5));
    }

    #[test]
    fn test_min() {
        assert_eq!(min(&[1, 2, 3, 4, 5]), Some(1));
        assert_eq!(min(&[5, 4, 3, 2, 1]), Some(1));
        assert_eq!(min(&[1]), Some(1));
        assert_eq!(min(&[] as &[i32]), None);
        assert_eq!(min(&[1.0, 2.5, 0.5]), Some(0.5));
    }

    #[test]
    fn test_extent() {
        assert_eq!(extent(&[1, 2, 3, 4, 5]), Some((1, 5)));
        assert_eq!(extent(&[5, 4, 3, 2, 1]), Some((1, 5)));
        assert_eq!(extent(&[1]), Some((1, 1)));
        assert_eq!(extent(&[] as &[i32]), None);
        assert_eq!(extent(&[1.0, 2.5, 0.5]), Some((0.5, 2.5)));
    }

    #[test]
    fn test_mean() {
        assert_eq!(mean(&[1.0, 2.0, 3.0, 4.0, 5.0]), Some(3.0));
        assert_eq!(mean(&[10.0]), Some(10.0));
        assert_eq!(mean(&[]), None);
        assert_eq!(mean(&[1.0, 2.0, 3.0]), Some(2.0));
    }

    #[test]
    fn test_median() {
        assert_eq!(median(&[1.0, 2.0, 3.0, 4.0, 5.0]), Some(3.0));
        assert_eq!(median(&[1.0, 2.0, 3.0, 4.0]), Some(2.5));
        assert_eq!(median(&[10.0]), Some(10.0));
        assert_eq!(median(&[]), None);
    }

    #[test]
    fn test_sum() {
        assert_eq!(sum(&[1.0, 2.0, 3.0, 4.0, 5.0]), Some(15.0));
        assert_eq!(sum(&[10.0]), Some(10.0));
        assert_eq!(sum(&[]), None);
        assert_eq!(sum(&[1.0, 2.0, 3.0]), Some(6.0));
    }

    #[test]
    fn test_deviation() {
        assert_eq!(deviation(&[1.0, 2.0, 3.0, 4.0, 5.0]), Some(1.5811388300841898));
        assert_eq!(deviation(&[2.0, 2.0, 2.0]), Some(0.0));
        assert_eq!(deviation(&[1.0, 2.0]), Some(0.7071067811865476));
        assert_eq!(deviation(&[10.0]), None);
        assert_eq!(deviation(&[]), None);
    }

    #[test]
    fn test_variance() {
        assert_eq!(variance(&[1.0, 2.0, 3.0, 4.0, 5.0]), Some(2.5));
        assert_eq!(variance(&[2.0, 2.0, 2.0]), Some(0.0));
        assert_eq!(variance(&[1.0, 2.0]), Some(0.5));
        assert_eq!(variance(&[10.0]), None);
        assert_eq!(variance(&[]), None);
    }

    #[test]
    fn test_quantile() {
        let arr = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(quantile(&arr, 0.5), Some(3.0));
        assert_eq!(quantile(&arr, 0.0), Some(1.0));
        assert_eq!(quantile(&arr, 1.0), Some(5.0));
        assert_eq!(quantile(&arr, 0.25), Some(2.0));
        assert_eq!(quantile(&arr, 0.75), Some(4.0));

        let arr2 = vec![1.0, 2.0, 3.0, 4.0];
        assert_eq!(quantile(&arr2, 0.5), Some(2.5));
        assert_eq!(quantile(&arr2, 0.0), Some(1.0));
        assert_eq!(quantile(&arr2, 1.0), Some(4.0));
        assert_eq!(quantile(&arr2, 0.25), Some(1.75));
        assert_eq!(quantile(&arr2, 0.75), Some(3.25));

        assert_eq!(quantile(&[] as &[f64], 0.5), None);
    }

    #[test]
    fn test_histogram() {
        let data = vec![1.0, 2.0, 2.5, 3.0, 4.0, 4.5, 5.0];
        let bins = histogram(&data, 3);

        assert_eq!(bins.len(), 3);

        assert_eq!(bins[0].x0, 1.0);
        assert!((bins[0].x1 - 2.3333333333333335).abs() < 1e-12);
        assert_eq!(bins[0].values, vec![1.0, 2.0]);

        assert!((bins[1].x0 - 2.3333333333333335).abs() < 1e-12);
        assert!((bins[1].x1 - 3.6666666666666665).abs() < 1e-12);
        assert_eq!(bins[1].values, vec![2.5, 3.0]);

        assert_eq!(bins[2].x0, 3.6666666666666665);
        assert_eq!(bins[2].x1, 5.0);
        assert_eq!(bins[2].values, vec![4.0, 4.5, 5.0]);

        let empty_data: Vec<f64> = Vec::new();
        let empty_bins = histogram(&empty_data, 5);
        assert!(empty_bins.is_empty());

        let single_value_data = vec![5.0];
        let single_value_bins = histogram(&single_value_data, 3);
        assert_eq!(single_value_bins.len(), 1);
        assert_eq!(single_value_bins[0].x0, 5.0);
        assert_eq!(single_value_bins[0].x1, 5.0);
        assert_eq!(single_value_bins[0].values, vec![5.0]);
    }

    #[test]
    fn test_bisect_left() {
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(bisect_left(&arr, &3, 0, arr.len()), 2);
        assert_eq!(bisect_left(&arr, &0, 0, arr.len()), 0);
        assert_eq!(bisect_left(&arr, &6, 0, arr.len()), 5);
        assert_eq!(bisect_left(&arr, &1, 0, arr.len()), 0);
        assert_eq!(bisect_left(&arr, &5, 0, arr.len()), 4);

        let arr_dup = vec![1, 2, 2, 3, 4];
        assert_eq!(bisect_left(&arr_dup, &2, 0, arr_dup.len()), 1);
    }

    #[test]
    fn test_bisect_right() {
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(bisect_right(&arr, &3, 0, arr.len()), 3);
        assert_eq!(bisect_right(&arr, &0, 0, arr.len()), 0);
        assert_eq!(bisect_right(&arr, &6, 0, arr.len()), 5);
        assert_eq!(bisect_right(&arr, &1, 0, arr.len()), 1);
        assert_eq!(bisect_right(&arr, &5, 0, arr.len()), 5);

        let arr_dup = vec![1, 2, 2, 3, 4];
        assert_eq!(bisect_right(&arr_dup, &2, 0, arr_dup.len()), 3);
    }

    #[test]
    fn test_ascending() {
        assert_eq!(ascending(&1, &2), Ordering::Less);
        assert_eq!(ascending(&2, &1), Ordering::Greater);
        assert_eq!(ascending(&1, &1), Ordering::Equal);
    }

    #[test]
    fn test_descending() {
        assert_eq!(descending(&1, &2), Ordering::Greater);
        assert_eq!(descending(&2, &1), Ordering::Less);
        assert_eq!(descending(&1, &1), Ordering::Equal);
    }

    #[test]
    fn test_range() {
        assert_eq!(range(0.0, 5.0, 1.0), vec![0.0, 1.0, 2.0, 3.0, 4.0]);
        assert_eq!(range(1.0, 5.0, 1.0), vec![1.0, 2.0, 3.0, 4.0]);
        let result = range(0.0, 1.0, 0.1);
        let expected = vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
        assert!((result.len() as isize - expected.len() as isize).abs() <= 1);
        for (a, b) in result.iter().zip(expected.iter()) {
            assert!((a - b).abs() < 1e-10);
        }
        assert_eq!(range(5.0, 0.0, -1.0), vec![5.0, 4.0, 3.0, 2.0, 1.0]);
        assert_eq!(range(0.0, 0.0, 1.0), Vec::<f64>::new());
        assert_eq!(range(0.0, 0.0, 0.0), vec![0.0]);
        assert_eq!(range(1.0, 1.0, 0.0), vec![1.0]);
    }

    #[test]
    fn test_merge() {
        let arr1 = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(merge(&arr1), vec![1, 2, 3, 4]);

        let arr2: Vec<Vec<i32>> = vec![];
        assert_eq!(merge(&arr2), Vec::<i32>::new());

        let arr3 = vec![vec![1], vec![2, 3], vec![4, 5, 6]];
        assert_eq!(merge(&arr3), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_shuffle() {
        let mut arr = vec![1, 2, 3, 4, 5];
        shuffle(&mut arr);
        // Cannot assert exact order due to randomness, but can assert length and content
        assert_eq!(arr.len(), 5);
        assert!(arr.contains(&1));
        assert!(arr.contains(&2));
        assert!(arr.contains(&3));
        assert!(arr.contains(&4));
        assert!(arr.contains(&5));
    }

    #[test]
    fn test_tick_step() {
        let actual = tick_step(0.0, 10.0, 10);
        println!("tick_step(0.0, 10.0, 10) = {}", actual);
        assert!((actual - 1.0).abs() < 1e-10);
        assert_eq!(tick_step(0.0, 100.0, 10), 10.0);
        assert_eq!(tick_step(0.0, 10.0, 3), 5.0);
        assert_eq!(tick_step(0.0, 1.0, 10), 0.1);
    }

    #[test]
    fn test_ticks() {
        let actual = ticks(0.0, 5.0, 5);
        println!("ticks(0.0, 5.0, 5) = {:?}", actual);
        assert_eq!(actual, vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(ticks(5.0, 0.0, 5), vec![5.0, 4.0, 3.0, 2.0, 1.0, 0.0]);
        assert_eq!(ticks(0.0, 1.0, 10), vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0]);
        assert_eq!(ticks(0.0, 0.0, 10), vec![0.0]);
    }

    #[test]
    fn test_nice() {
        assert_eq!(nice(0.123, 9.87, 10), (0.0, 10.0));
        assert_eq!(nice(9.87, 0.123, 10), (10.0, 0.0));
        assert_eq!(nice(0.0, 0.0, 10), (0.0, 0.0));
        assert_eq!(nice(1.0, 1.0, 10), (1.0, 1.0));
    }

    #[test]
    fn test_scan() {
        let arr = vec![4, 2, 7, 1, 5];
        assert_eq!(scan(&arr, |a, b| a.cmp(b)), Some(3));

        let arr_float = vec![4.0, 2.0, 7.0, 1.0, 5.0];
        assert_eq!(scan(&arr_float, |a, b| a.partial_cmp(b).unwrap()), Some(3));

        let empty_arr: Vec<i32> = vec![];
        assert_eq!(scan(&empty_arr, |a, b| a.cmp(b)), None);

        let single_arr = vec![10];
        assert_eq!(scan(&single_arr, |a, b| a.cmp(b)), Some(0));
    }

    #[test]
    fn test_group() {
        #[derive(Debug, PartialEq, Eq, Hash, Clone)]
        struct Person {
            name: String,
            age: u32,
        }

        let people = vec![
            Person { name: "Alice".to_string(), age: 30 },
            Person { name: "Bob".to_string(), age: 25 },
            Person { name: "Charlie".to_string(), age: 30 },
            Person { name: "David".to_string(), age: 25 },
        ];

        let grouped_by_age = group(&people, |p| p.age);

        let mut expected_map: HashMap<u32, Vec<Person>> = HashMap::new();
        expected_map.insert(30, vec![
            Person { name: "Alice".to_string(), age: 30 },
            Person { name: "Charlie".to_string(), age: 30 },
        ]);
        expected_map.insert(25, vec![
            Person { name: "Bob".to_string(), age: 25 },
            Person { name: "David".to_string(), age: 25 },
        ]);

        assert_eq!(grouped_by_age, expected_map);

        let empty_people: Vec<Person> = vec![];
        let empty_grouped = group(&empty_people, |p| p.age);
        assert!(empty_grouped.is_empty());
    }

    #[test]
    fn test_flat_group() {
        #[derive(Debug, PartialEq, Eq, Hash, Clone)]
        struct Person {
            name: String,
            age: u32,
        }

        let people = vec![
            Person { name: "Alice".to_string(), age: 30 },
            Person { name: "Bob".to_string(), age: 25 },
            Person { name: "Charlie".to_string(), age: 30 },
            Person { name: "David".to_string(), age: 25 },
        ];

        let flat_grouped_by_age = flat_group(&people, |p| p.age);

        let mut expected_vec: Vec<(u32, Vec<Person>)> = vec![
            (30, vec![
                Person { name: "Alice".to_string(), age: 30 },
                Person { name: "Charlie".to_string(), age: 30 },
            ]),
            (25, vec![
                Person { name: "Bob".to_string(), age: 25 },
                Person { name: "David".to_string(), age: 25 },
            ]),
        ];

        // Sort both for comparison as HashMap iteration order is not guaranteed
        flat_grouped_by_age.iter().for_each(|(k, v)| {
            let mut sorted_v = v.clone();
            sorted_v.sort_by_key(|p| p.name.clone());
            expected_vec.iter_mut().find(|(ek, _)| ek == k).unwrap().1 = sorted_v;
        });
        expected_vec.sort_by_key(|(k, _)| *k);

        let mut actual_vec = flat_grouped_by_age.clone();
        actual_vec.iter_mut().for_each(|(_k, v)| {
            v.sort_by_key(|p| p.name.clone());
        });
        actual_vec.sort_by_key(|(k, _)| *k);

        assert_eq!(actual_vec, expected_vec);

        let empty_people: Vec<Person> = vec![];
        let empty_flat_grouped = flat_group(&empty_people, |p| p.age);
        assert!(empty_flat_grouped.is_empty());
    }

    #[test]
    fn test_pairs() {
        let arr = vec![1, 2, 3, 4];
        assert_eq!(pairs(&arr), vec![(1, 2), (2, 3), (3, 4)]);

        let empty_arr: Vec<i32> = vec![];
        assert_eq!(pairs(&empty_arr), vec![]);

        let single_arr = vec![1];
        assert_eq!(pairs(&single_arr), vec![]);
    }

    #[test]
    fn test_zip() {
        let arr1 = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(zip(&arr1), vec![vec![1, 4], vec![2, 5], vec![3, 6]]);

        let arr2 = vec![vec![1, 2], vec![3, 4, 5]];
        assert_eq!(zip(&arr2), vec![vec![1, 3], vec![2, 4]]);

        let empty_arr: Vec<Vec<i32>> = vec![];
        assert_eq!(zip(&empty_arr), Vec::<Vec<i32>>::new());

        let single_arr = vec![vec![1, 2, 3]];
        assert_eq!(zip(&single_arr), vec![vec![1], vec![2], vec![3]]);
    }

    #[test]
    fn test_cross() {
        let a = vec![1, 2];
        let b = vec!["a", "b"];
        assert_eq!(cross(&a, &b), vec![(1, "a"), (1, "b"), (2, "a"), (2, "b")]);

        let empty_a: Vec<i32> = vec![];
        assert_eq!(cross(&empty_a, &b), vec![]);

        let empty_b: Vec<&str> = vec![];
        assert_eq!(cross(&a, &empty_b), vec![]);
    }

    #[test]
    fn test_least() {
        #[derive(Debug, PartialEq, Clone)]
        struct Item { value: f64 }
        let items = vec![Item { value: 3.0 }, Item { value: 1.0 }, Item { value: 2.0 }];
        assert_eq!(least(&items, |item| item.value), Some(Item { value: 1.0 }));

        let empty_items: Vec<Item> = vec![];
        assert_eq!(least(&empty_items, |item| item.value), None);
    }

    #[test]
    fn test_greatest() {
        #[derive(Debug, PartialEq, Clone)]
        struct Item { value: f64 }
        let items = vec![Item { value: 3.0 }, Item { value: 1.0 }, Item { value: 2.0 }];
        assert_eq!(greatest(&items, |item| item.value), Some(Item { value: 3.0 }));

        let empty_items: Vec<Item> = vec![];
        assert_eq!(greatest(&empty_items, |item| item.value), None);
    }

    #[test]
    fn test_least_index() {
        #[derive(Debug, PartialEq, Clone)]
        struct Item { value: f64 }
        let items = vec![Item { value: 3.0 }, Item { value: 1.0 }, Item { value: 2.0 }];
        assert_eq!(least_index(&items, |item| item.value), Some(1));

        let empty_items: Vec<Item> = vec![];
        assert_eq!(least_index(&empty_items, |item| item.value), None);
    }

    #[test]
    fn test_greatest_index() {
        #[derive(Debug, PartialEq, Clone)]
        struct Item { value: f64 }
        let items = vec![Item { value: 3.0 }, Item { value: 1.0 }, Item { value: 2.0 }];
        assert_eq!(greatest_index(&items, |item| item.value), Some(0));

        let empty_items: Vec<Item> = vec![];
        assert_eq!(greatest_index(&empty_items, |item| item.value), None);
    }

    #[test]
    fn test_fsum() {
        assert_eq!(fsum(&[0.1, 0.2, 0.3]), 0.6);
        assert_eq!(fsum(&[0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0]), 5.5);
        assert_eq!(fsum(&[]), 0.0);
        assert_eq!(fsum(&[1.0]), 1.0);
    }

    #[test]
    fn test_blur1d() {
        let input = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let blurred = blur1d(&input, 1);
        let expected = vec![1.5, 2.0, 3.0, 4.0, 4.5];
        for (b, e) in blurred.iter().zip(expected.iter()) {
            assert!((b - e).abs() < 1e-6, "blurred value {} != expected {}", b, e);
        }
    }

    #[test]
    fn test_set_ops() {
        let a = vec![1, 2, 3];
        let b = vec![3, 4, 5];
        let mut u = union(&a, &b);
        u.sort();
        assert_eq!(u, vec![1, 2, 3, 4, 5]);
        let mut i = intersection(&a, &b);
        i.sort();
        assert_eq!(i, vec![3]);
        let mut d = difference(&a, &b);
        d.sort();
        assert_eq!(d, vec![1, 2]);
        let mut s = symmetric_difference(&a, &b);
        s.sort();
        assert_eq!(s, vec![1, 2, 4, 5]);
    }

    #[test]
    fn test_sort_summarize_transform() {
        let arr = vec![3, 1, 4, 1, 5, 9, 2];
        let sorted = sort(&arr);
        assert_eq!(sorted, vec![1, 1, 2, 3, 4, 5, 9]);
        let sorted_by = sort_by(&arr, |a, b| b.cmp(a));
        assert_eq!(sorted_by, vec![9, 5, 4, 3, 2, 1, 1]);
        let arrf = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let summary = summarize(&arrf).unwrap();
        assert!((summary.0 - 1.0).abs() < 1e-6); // min
        assert!((summary.1 - 5.0).abs() < 1e-6); // max
        assert!((summary.2 - 3.0).abs() < 1e-6); // mean
        assert!((summary.3 - 3.0).abs() < 1e-6); // median
        assert!((summary.4 - 2.0).abs() < 1e-6); // variance
        let arr2 = vec![1, 2, 3];
        let transformed: Vec<String> = transform(&arr2, |x| format!("{}!", x));
        assert_eq!(transformed, vec!["1!", "2!", "3!"]);
    }

    #[test]
    fn test_intern() {
        use std::collections::{HashSet, HashMap};
        let mut set = HashSet::new();
        let a = intern_set(&mut set, "foo".to_string()).clone();
        let b = intern_set(&mut set, "foo".to_string()).clone();
        assert_eq!(a, b);
        let c = intern_set(&mut set, "bar".to_string()).clone();
        assert_ne!(a, c);

        let mut map = HashMap::new();
        let v1 = *intern_map(&mut map, "key1", 42);
        assert_eq!(v1, 42);
        let v2 = *intern_map(&mut map, "key1", 99);
        assert_eq!(v2, 42); // Should not overwrite
        let v3 = *intern_map(&mut map, "key2", 7);
        assert_eq!(v3, 7);
    }
}


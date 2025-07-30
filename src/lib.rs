pub mod prelude;
pub use prelude::IterExtra;

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn min_by_partial_key_basic() {
        let numbers = vec![3.2, 1.5, 2.8, 0.9];
        let min = numbers.iter().min_by_partial_key(|&x| x);
        assert_eq!(min, Some(&0.9));
    }

    #[test]
    fn min_by_partial_key_empty() {
        let empty: Vec<f64> = vec![];
        let min = empty.iter().min_by_partial_key(|&x| x);
        assert_eq!(min, None);
    }

    #[test]
    fn min_by_partial_key_single_element() {
        let single = vec![42.0];
        let min = single.iter().min_by_partial_key(|&x| x);
        assert_eq!(min, Some(&42.0));
    }

    #[test]
    fn min_by_partial_key_with_nan() {
        let with_nan = vec![1.0, f64::NAN, 2.0, 0.5];
        let min = with_nan.iter().min_by_partial_key(|&x| x);
        assert_eq!(min, Some(&0.5));
    }

    #[test]
    fn min_by_partial_key_all_nan() {
        let all_nan = vec![f64::NAN, f64::NAN, f64::NAN];
        let min = all_nan.iter().min_by_partial_key(|&x| x);
        assert!(min.is_some());
        assert!(min.unwrap().is_nan());
    }

    #[test]
    fn min_by_partial_key_with_key_function() {
        let people = vec![
            ("Alice", 25),
            ("Bob", 30),
            ("Charlie", 20),
            ("Diana", 35),
        ];
        let youngest = people.iter().min_by_partial_key(|(_, age)| *age);
        assert_eq!(youngest, Some(&("Charlie", 20)));
    }

    #[test]
    fn min_by_partial_key_negative_numbers() {
        let numbers = vec![-1.5, -3.2, -0.8, -2.1];
        let min = numbers.iter().min_by_partial_key(|&x| x);
        assert_eq!(min, Some(&-3.2));
    }

    #[test]
    fn max_by_partial_key_basic() {
        let numbers = vec![3.2, 1.5, 2.8, 0.9];
        let max = numbers.iter().max_by_partial_key(|&x| x);
        assert_eq!(max, Some(&3.2));
    }

    #[test]
    fn max_by_partial_key_empty() {
        let empty: Vec<f64> = vec![];
        let max = empty.iter().max_by_partial_key(|&x| x);
        assert_eq!(max, None);
    }

    #[test]
    fn max_by_partial_key_single_element() {
        let single = vec![42.0];
        let max = single.iter().max_by_partial_key(|&x| x);
        assert_eq!(max, Some(&42.0));
    }

    #[test]
    fn max_by_partial_key_with_nan() {
        let with_nan = vec![1.0, f64::NAN, 2.0, 0.5];
        let max = with_nan.iter().max_by_partial_key(|&x| x);
        assert_eq!(max, Some(&2.0));
    }

    #[test]
    fn max_by_partial_key_all_nan() {
        let all_nan = vec![f64::NAN, f64::NAN, f64::NAN];
        let max = all_nan.iter().max_by_partial_key(|&x| x);
        assert!(max.is_some());
        assert!(max.unwrap().is_nan());
    }

    #[test]
    fn max_by_partial_key_with_key_function() {
        let people = vec![
            ("Alice", 25),
            ("Bob", 30),
            ("Charlie", 20),
            ("Diana", 35),
        ];
        let oldest = people.iter().max_by_partial_key(|(_, age)| *age);
        assert_eq!(oldest, Some(&("Diana", 35)));
    }

    #[test]
    fn max_by_partial_key_negative_numbers() {
        let numbers = vec![-1.5, -3.2, -0.8, -2.1];
        let max = numbers.iter().max_by_partial_key(|&x| x);
        assert_eq!(max, Some(&-0.8));
    }

    #[test]
    fn min_max_with_equal_elements() {
        let equal = vec![5.0, 5.0, 5.0];
        let min = equal.iter().min_by_partial_key(|&x| x);
        let max = equal.iter().max_by_partial_key(|&x| x);
        assert_eq!(min, Some(&5.0));
        assert_eq!(max, Some(&5.0));
    }

    #[test]
    fn min_max_with_infinity() {
        let with_inf = vec![1.0, f64::INFINITY, -f64::INFINITY, 2.0];
        let min = with_inf.iter().min_by_partial_key(|&x| x);
        let max = with_inf.iter().max_by_partial_key(|&x| x);
        assert_eq!(min, Some(&f64::NEG_INFINITY));
        assert_eq!(max, Some(&f64::INFINITY));
    }

    #[test]
    fn min_max_with_option_flatten() {
        let arr = vec![Some(1.1f64), None, Some(1.2f64), Some(0.5f64)];
        assert_eq!(
            arr.iter().flatten().min_by_partial_key(|&x| x),
            Some(&0.5f64)
        );

        let arr = vec![Some(1.1f64), None, Some(1.2f64), Some(0.5f64)];
        assert_eq!(
            arr.iter().flatten().max_by_partial_key(|&x| x),
            Some(&1.2f64)
        );
    }

    #[test]
    fn min_max_with_complex_key_function() {
        let points = vec![(1.0, 2.0), (3.0, 1.0), (2.0, 3.0), (0.5, 0.5)];
        
        let closest_to_origin = points.iter().min_by_partial_key(|(x, y)| x * x + y * y);
        assert_eq!(closest_to_origin, Some(&(0.5, 0.5)));
        
        let farthest_from_origin = points.iter().max_by_partial_key(|(x, y)| x * x + y * y);
        assert_eq!(farthest_from_origin, Some(&(2.0, 3.0)));
    }

    #[test]
    fn min_max_with_string_length() {
        let words = vec!["hello", "world", "rust", "programming"];
        
        let shortest = words.iter().min_by_partial_key(|s| s.len());
        assert_eq!(shortest, Some(&"rust"));
        
        let longest = words.iter().max_by_partial_key(|s| s.len());
        assert_eq!(longest, Some(&"programming"));
    }
}

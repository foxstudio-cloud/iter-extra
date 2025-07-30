pub trait IterExtra: Iterator {
    /// Returns the element that gives the minimum value from the specified function.
    ///
    /// This method is similar to `Iterator::min_by_key`, but works with types that implement
    /// `PartialOrd` instead of `Ord`. When the comparison returns `None` (indicating
    /// incomparable values like NaN), it treats them as equal.
    ///
    /// # Arguments
    ///
    /// * `key` - A function that extracts a key from each element for comparison
    ///
    /// # Returns
    ///
    /// * `Some(item)` - The element that produces the minimum key value
    /// * `None` - If the iterator is empty
    ///
    /// # Examples
    ///
    /// ```
    /// use iter_extra::IterExtra;
    ///
    /// let numbers = vec![3.2, 1.5, 2.8, 0.9];
    /// let min = numbers.iter().min_by_partial_key(|&x| x);
    /// assert_eq!(min, Some(&0.9));
    ///
    /// // Works with NaN values
    /// let with_nan = vec![1.0, f64::NAN, 2.0];
    /// let min = with_nan.iter().min_by_partial_key(|&x| x);
    /// assert_eq!(min, Some(&1.0));
    /// ```
    fn min_by_partial_key<K: PartialOrd, F: FnMut(&Self::Item) -> K>(
        self,
        mut key: F,
    ) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.min_by(|x, y| {
            key(x)
                .partial_cmp(&key(y))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    /// Returns the element that gives the maximum value from the specified function.
    ///
    /// This method is similar to `Iterator::max_by_key`, but works with types that implement
    /// `PartialOrd` instead of `Ord`. When the comparison returns `None` (indicating
    /// incomparable values like NaN), it treats them as equal.
    ///
    /// # Arguments
    ///
    /// * `key` - A function that extracts a key from each element for comparison
    ///
    /// # Returns
    ///
    /// * `Some(item)` - The element that produces the maximum key value
    /// * `None` - If the iterator is empty
    ///
    /// # Examples
    ///
    /// ```
    /// use iter_extra::IterExtra;
    ///
    /// let numbers = vec![3.2, 1.5, 2.8, 0.9];
    /// let max = numbers.iter().max_by_partial_key(|&x| x);
    /// assert_eq!(max, Some(&3.2));
    ///
    /// // Works with NaN values
    /// let with_nan = vec![1.0, f64::NAN, 2.0];
    /// let max = with_nan.iter().max_by_partial_key(|&x| x);
    /// assert_eq!(max, Some(&2.0));
    /// ```
    fn max_by_partial_key<K: PartialOrd, F: FnMut(&Self::Item) -> K>(
        self,
        mut key: F,
    ) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.max_by(|x, y| {
            key(x)
                .partial_cmp(&key(y))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }
}

impl<I: Iterator<Item = T>, T> IterExtra for I {}

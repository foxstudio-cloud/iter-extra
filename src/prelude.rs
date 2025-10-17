pub struct Deltas<I: Iterator> {
    pub(crate) items: Vec<(I::Item, usize)>,
    pub(crate) enumerate_iter: std::iter::Enumerate<I>,
}

impl<I: Iterator> Deltas<I> {
    pub(crate) fn new(iter: I) -> Self {
        Deltas {
            items: Vec::new(),
            enumerate_iter: iter.enumerate(),
        }
    }
}

impl<I: Iterator> Iterator for Deltas<I>
where
    I::Item: std::cmp::PartialEq,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let (next_index, next_item) = self.enumerate_iter.next()?;

        let last_index = (self.items.iter().rev())
            .find_map(|(item, index)| (item == &next_item).then_some(*index));

        self.items.push((next_item, next_index));
        Some(last_index.map_or(next_index, |last_idx| next_index - last_idx - 1))
    }
}

pub struct DeltasBy<I: Iterator, F> {
    items: Vec<(I::Item, usize)>,
    enumerate_iter: std::iter::Enumerate<I>,
    cmp_fn: F,
}

impl<I: Iterator, F> DeltasBy<I, F> {
    pub(crate) fn new(iter: I, cmp_fn: F) -> Self {
        DeltasBy {
            items: Vec::new(),
            enumerate_iter: iter.enumerate(),
            cmp_fn,
        }
    }
}

impl<I: Iterator, F> Iterator for DeltasBy<I, F>
where
    F: FnMut(&I::Item, &I::Item) -> std::cmp::Ordering,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let (next_index, next_item) = self.enumerate_iter.next()?;

        let last_index = (self.items.iter().rev()).find_map(|(item, index)| {
            ((self.cmp_fn)(item, &next_item) == std::cmp::Ordering::Equal).then_some(*index)
        });

        self.items.push((next_item, next_index));
        Some(last_index.map_or(next_index, |last_idx| next_index - last_idx - 1))
    }
}

pub struct DeltasByKey<I: Iterator, F> {
    items: Vec<(I::Item, usize)>,
    enumerate_iter: std::iter::Enumerate<I>,
    key_fn: F,
}

impl<I: Iterator, F> DeltasByKey<I, F> {
    pub(crate) fn new(iter: I, key_fn: F) -> Self {
        DeltasByKey {
            items: Vec::new(),
            enumerate_iter: iter.enumerate(),
            key_fn,
        }
    }
}

impl<I: Iterator, K, F> Iterator for DeltasByKey<I, F>
where
    F: FnMut(&I::Item) -> K,
    K: std::cmp::PartialEq,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let (next_index, next_item) = self.enumerate_iter.next()?;

        let next_key = (self.key_fn)(&next_item);
        let last_index = (self.items.iter().rev())
            .find_map(|(item, index)| ((self.key_fn)(item) == next_key).then_some(*index));

        self.items.push((next_item, next_index));
        Some(last_index.map_or(next_index, |last_idx| next_index - last_idx - 1))
    }
}

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

    fn collect_some_vec(self) -> Option<Vec<Self::Item>>
    where
        Self: Sized,
    {
        Some(self.collect::<Vec<Self::Item>>()).filter(|v| !v.is_empty())
    }

    fn collect_ok_vec_or<E>(self, err: E) -> Result<Vec<Self::Item>, E>
    where
        Self: Sized,
    {
        Ok(self.collect::<Vec<Self::Item>>()).and_then(
            |v| {
                if v.is_empty() { Err(err) } else { Ok(v) }
            },
        )
    }

    fn collect_ok_vec_or_default<E: Default>(self) -> Result<Vec<Self::Item>, E>
    where
        Self: Sized,
    {
        Ok(self.collect::<Vec<Self::Item>>()).and_then(|v| {
            if v.is_empty() {
                Err(E::default())
            } else {
                Ok(v)
            }
        })
    }

    /// Returns an iterator that yields the distance from each element to its last occurrence.
    ///
    /// For each element in the iterator, this method returns the number of elements between
    /// the current element and the previous occurrence of the same element. If the element
    /// hasn't appeared before, it returns the current index.
    ///
    /// # Returns
    ///
    /// An iterator that yields `usize` values representing the delta for each element
    ///
    /// # Examples
    ///
    /// ```
    /// use iter_extra::IterExtra;
    ///
    /// let items = vec!['a', 'b', 'c', 'a', 'c'];
    /// let deltas: Vec<usize> = items.into_iter().deltas().collect();
    /// assert_eq!(deltas, vec![0, 1, 2, 2, 1]);
    /// ```
    fn deltas(self) -> Deltas<Self>
    where
        Self: Sized,
        Self::Item: std::cmp::PartialEq,
    {
        Deltas::new(self)
    }

    /// Returns an iterator that yields the distance from each element to its last occurrence,
    /// using a custom comparison function.
    ///
    /// Similar to `deltas`, but uses a custom comparison function to determine element equality.
    /// Two elements are considered equal when the comparison function returns `Ordering::Equal`.
    ///
    /// # Arguments
    ///
    /// * `cmp_fn` - A function that compares two elements and returns an `Ordering`
    ///
    /// # Returns
    ///
    /// An iterator that yields `usize` values representing the delta for each element
    ///
    /// # Examples
    ///
    /// ```
    /// use iter_extra::IterExtra;
    ///
    /// let items = vec![1.1, 2.2, 3.3, 1.2, 2.1];
    /// let deltas: Vec<usize> = items.into_iter()
    ///     .deltas_by(|a, b| a.floor().total_cmp(&b.floor()))
    ///     .collect();
    /// assert_eq!(deltas, vec![0, 1, 2, 2, 2]);
    /// ```
    fn deltas_by<F>(self, cmp_fn: F) -> DeltasBy<Self, F>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> std::cmp::Ordering,
    {
        DeltasBy::new(self, cmp_fn)
    }

    /// Returns an iterator that yields the distance from each element to its last occurrence,
    /// comparing elements by a key extracted from each element.
    ///
    /// Similar to `deltas`, but determines element equality by comparing the keys extracted
    /// by the provided key function. Two elements are considered equal if their keys are equal.
    ///
    /// # Arguments
    ///
    /// * `key_fn` - A function that extracts a key from each element for comparison
    ///
    /// # Returns
    ///
    /// An iterator that yields `usize` values representing the delta for each element
    ///
    /// # Examples
    ///
    /// ```
    /// use iter_extra::IterExtra;
    ///
    /// let items = vec!["apple", "banana", "apricot", "blueberry"];
    /// let deltas: Vec<usize> = items.into_iter()
    ///     .deltas_by_key(|s| s.chars().next())
    ///     .collect();
    /// assert_eq!(deltas, vec![0, 1, 1, 1]);
    /// ```
    fn deltas_by_key<K, F>(self, key_fn: F) -> DeltasByKey<Self, F>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> K,
        K: std::cmp::PartialEq,
    {
        DeltasByKey::new(self, key_fn)
    }
}

impl<I: Iterator<Item = T>, T> IterExtra for I {}

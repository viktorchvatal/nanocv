use std::{cmp::{min, max}, ops::Sub};

/// A half-open range bounded from start (inclusive) to end (exclusive).
/// 
/// Has the same meaning as std::ops::Range, but implements the Copy
/// trait so it can be stored inside copy structures.
///
/// # Examples
/// Create a new range
/// ```
/// use nanocv::geometry::Range;
/// let range = Range::new(1..4);
/// # assert_eq!(range.start, 1);
/// # assert_eq!(range.end, 4);
/// # assert_eq!(range.length(), 3);
/// ```
/// 
/// Test that `start` and `end` bounds are correctly set
/// ```
/// # use nanocv::geometry::Range;
/// # let range = Range::<u32>::from(1..4);
/// assert_eq!(range.start, 1);
/// assert_eq!(range.end, 4);
/// # assert_eq!(range.length(), 3);
/// ```
/// 
/// Test that `Range` length gets correctly computed
/// ```
/// # use nanocv::geometry::Range;
/// # let range = Range::<u32>::from(1..4);
/// # assert_eq!(range.start, 1);
/// # assert_eq!(range.end, 4);
/// assert_eq!(range.length(), 3);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Range<T> {
    pub start: T,
    pub end: T,
}

impl<T> Range<T> {
    /// New Range instance from system::ops::Range
    /// ```
    /// use nanocv::geometry::Range;
    /// let range = Range::new(1..4);
    ///  assert_eq!(range.start, 1);
    ///  assert_eq!(range.end, 4);
    /// ```
    pub fn new(range: std::ops::Range<T>) -> Self {
        Self { start: range.start, end: range.end }
    }
}

impl<T: Sub<Output=T> + Copy> Range<T> {
    /// Range length (number of elements a range includes)
    /// ```
    ///  use nanocv::geometry::Range;
    ///  let range = Range::<u32>::from(2..5);
    ///  assert_eq!(range.length(), 3);
    /// ```    
    pub fn length(&self) -> T {
        self.end - self.start
    }
}

impl<T> From<std::ops::Range<T>> for Range<T> {
    fn from(range: std::ops::Range<T>) -> Self {
        Self::new(range)
    }
}

impl From<Range<isize>> for Range<usize> {
    fn from(range: Range<isize>) -> Self {
        Self::new(range.start as usize..range.end as usize)
    }
}

impl<T: Ord + Copy> Range<T> {
    /// Intersection of two ranges
    ///
    /// # Example
    /// ```
    /// use nanocv::geometry::Range;
    /// assert_eq!(
    ///     Range::new(1..3).intersect(&Range::new(2..4)),
    ///     Range::new(2..3)
    /// );
    /// ```
    pub fn intersect(&self, other: &Range<T>) -> Self {
        Self::new(max(self.start, other.start)..min(self.end, other.end))
    }
}
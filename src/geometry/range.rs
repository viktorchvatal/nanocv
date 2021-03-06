use std::{cmp::{min, max}, ops::{Add, Sub}};

/// A half-open range bounded from start (inclusive) to end (exclusive).
/// 
/// Has the same meaning as std::ops::Range, but implements the Copy
/// trait so it can be stored inside copy structures.
///
/// # Examples
/// Create a new range
/// ```
/// use nanocv::Range;
/// let range = Range::new(1..4);
/// # assert_eq!(range.start, 1);
/// # assert_eq!(range.end, 4);
/// # assert_eq!(range.length(), 3);
/// ```
/// 
/// Test that `start` and `end` bounds are correctly set
/// ```
/// # use nanocv::Range;
/// # let range = Range::<u32>::from(1..4);
/// assert_eq!(range.start, 1);
/// assert_eq!(range.end, 4);
/// # assert_eq!(range.length(), 3);
/// ```
/// 
/// Test that `Range` length gets correctly computed
/// ```
/// # use nanocv::Range;
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
    /// use nanocv::Range;
    /// let range = Range::new(1..4);
    ///  assert_eq!(range.start, 1);
    ///  assert_eq!(range.end, 4);
    /// ```
    pub fn new(range: std::ops::Range<T>) -> Self {
        Self { start: range.start, end: range.end }
    }
}

impl<T: Copy> Range<T> {
    pub fn to_range(&self) -> std::ops::Range<T> {
        self.start..self.end
    }
}

impl<T: Sub<Output=T> + Copy> Range<T> {
    /// Range length (number of elements a range includes)
    /// ```
    ///  use nanocv::Range;
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

impl<T> From<Range<T>> for std::ops::Range<T> {
    fn from(range: Range<T>) -> Self {
        range.start..range.end
    }
}

impl From<Range<isize>> for Range<usize> {
    fn from(range: Range<isize>) -> Self {
        Self::new(range.start as usize..range.end as usize)
    }
}

impl From<Range<usize>> for Range<isize> {
    fn from(range: Range<usize>) -> Self {
        Self::new(range.start as isize..range.end as isize)
    }
}

impl<T: Ord + Copy> Range<T> {
    /// Intersection of two ranges
    ///
    /// # Example
    /// ```
    /// use nanocv::Range;
    /// assert_eq!(
    ///     Range::new(1..3).intersect(Range::new(2..4)),
    ///     Range::new(2..3)
    /// );
    /// ```
    pub fn intersect(&self, other: Range<T>) -> Self {
        Self::new(max(self.start, other.start)..min(self.end, other.end))
    }
}

impl<T: Add<T, Output=T> + Copy> Add<T> for Range<T> {
    type Output = Range<T>;

    fn add(self, scalar: T) -> Range<T> {
        Range {start: self.start + scalar, end: self.end + scalar}
    }
}

impl<T: Sub<T, Output=T> + Copy> Sub<T> for Range<T> {
    type Output = Range<T>;

    fn sub(self, scalar: T) -> Range<T> {
        Range {start: self.start - scalar, end: self.end - scalar}
    }
}

// ================================== TESTS ==================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_add() {
        assert_eq!(Range::new(1..3) + 2, Range::new(3..5));
    }


    #[test]
    fn test_range_sub() {
        assert_eq!(Range::new(1..3) - 1, Range::new(0..2));
    }
}

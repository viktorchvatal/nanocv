use std::ops::Sub;

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
    pub fn new(range: std::ops::Range<T>) -> Self {
        Self { start: range.start, end: range.end }
    }
}

impl<T: Sub<Output=T> + Copy> Range<T> {
    pub fn length(&self) -> T {
        self.end - self.start
    }
}

impl<T> From<std::ops::Range<T>> for Range<T> {
    fn from(range: std::ops::Range<T>) -> Self {
        Range::new(range)
    }
}

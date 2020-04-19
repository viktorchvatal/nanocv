
use std::ops::{Add, Sub};
use super::Range;
use crate::Vec2d;

/// A two directional (half-open) range describing an area within an image.
/// 
/// # Examples
/// Create a new 2D range
/// ```
/// use nanocv::Range2d;
/// let range = Range2d::new(0..2, 1..4);
/// ```
/// 
/// Test range bounds
/// ```
/// # use nanocv::Range2d;
/// # let range = Range2d::new(0..2, 1..4);
/// assert_eq!(range.x.start, 0);
/// assert_eq!(range.x.end, 2);
/// assert_eq!(range.y.start, 1);
/// assert_eq!(range.y.end, 4);
/// ```
/// 
/// Test range width and height
/// ```
/// # use nanocv::Range2d;
/// # let range = Range2d::new(0..2, 1..4);
/// assert_eq!(range.width(), 2);
/// assert_eq!(range.height(), 3);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Range2d<T> {
    pub x: Range<T>,
    pub y: Range<T>,
}

impl<T: Copy> Range2d<T> {
    /// Create a new range from two system::ops::Range values
    /// ```
    /// use nanocv::{Range, Range2d};
    /// let range = Range2d::new(0..2, 1..4);
    /// assert_eq!(range.x, Range::from(0..2));
    /// assert_eq!(range.y, Range::from(1..4));
    /// ```    
    pub fn new(x: std::ops::Range<T>, y: std::ops::Range<T>) -> Self {
        Self { x: Range::new(x), y: Range::new(y) }
    }

    /// Start 2D vector of the range
    /// ```
    /// use nanocv::{Range2d, Vec2d};
    /// let range = Range2d::new(0..2, 1..4);
    /// assert_eq!(range.start(), Vec2d::new(0, 1));
    /// ```    
    pub fn start(&self) -> Vec2d<T> {
        Vec2d::new(self.x.start, self.y.start)
    }

    /// End 2D vector of the range
    /// ```
    /// use nanocv::{Range2d, Vec2d};
    /// let range = Range2d::new(0..2, 1..4);
    /// assert_eq!(range.end(), Vec2d::new(2, 4));
    /// ```    
    pub fn end(&self) -> Vec2d<T> {
        Vec2d::new(self.x.end, self.y.end)
    }    
}

impl<T: Sub<Output=T> + Copy> Range2d<T> {
    /// Range width
    /// ```
    /// use nanocv::Range2d;
    /// let range = Range2d::new(0..2, 1..4);
    /// assert_eq!(range.width(), 2);
    /// ```       
    pub fn width(&self) -> T {
        self.x.length()
    }

    /// Range height
    /// ```
    /// use nanocv::Range2d;
    /// let range = Range2d::new(0..2, 1..4);
    /// assert_eq!(range.height(), 3);
    /// ```       
    pub fn height(&self) -> T {
        self.y.length()
    }
}

impl<T: Ord + Copy> Range2d<T> {
    /// Intersection of two 2D ranges
    ///
    /// # Example
    /// ```
    /// use nanocv::Range2d;
    /// assert_eq!(
    ///     Range2d::new(0..2, 1..3).intersect(Range2d::new(1..3, 2..5)),
    ///     Range2d::new(1..2, 2..3)
    /// );
    /// ```
    pub fn intersect(&self, other: Range2d<T>) -> Self {
        Self {
            x: self.x.intersect(other.x),
            y: self.y.intersect(other.y),
        }
    }
}

impl From<Range2d<isize>> for Range2d<usize> {
    fn from(range: Range2d<isize>) -> Self {
        Self {
            x: Range::from(range.x),
            y: Range::from(range.y),
        }
    }
}

impl<T: Add<T, Output=T> + Copy> Add<Vec2d<T>> for Range2d<T> {
    type Output = Range2d<T>;

    fn add(self, vector: Vec2d<T>) -> Range2d<T> {
        Range2d { x: self.x + vector.x, y: self.y + vector.y }
    }
}

impl<T: Sub<T, Output=T> + Copy> Sub<Vec2d<T>> for Range2d<T> {
    type Output = Range2d<T>;

    fn sub(self, vector: Vec2d<T>) -> Range2d<T> {
        Range2d { x: self.x - vector.x, y: self.y - vector.y }
    }
}

/// Defines range within an image using signed isize type to prevent 
/// zero underflows
pub type ImgRange = Range2d<isize>;

// ================================== TESTS ==================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range2d_add() {
        assert_eq!(
            Range2d::new(1..3, 2..5) + Vec2d::new(2, 1), 
            Range2d::new(3..5, 3..6)
        );
    }


    #[test]
    fn test_range2d_sub() {
        assert_eq!(
            Range2d::new(1..3, 2..5) - Vec2d::new(2, 1), 
            Range2d::new(-1..1, 1..4)
        );
    }
}

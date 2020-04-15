
use std::ops::Sub;
use crate::Range;

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
    pub fn new(x: std::ops::Range<T>, y: std::ops::Range<T>) -> Self {
        Self { x: Range::new(x), y: Range::new(y) }
    }
}

impl<T: Sub<Output=T> + Copy> Range2d<T> {
    pub fn width(&self) -> T {
        self.x.length()
    }

    pub fn height(&self) -> T {
        self.y.length()
    }
}

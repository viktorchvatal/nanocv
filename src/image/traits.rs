use crate::{ImgRange, geometry::{Range2d, Vec2d}};

/// Read-only access to image pixels, usually used as input data
pub trait Img<T> {
    /// Image width and height, in pixels
    fn size(&self) -> ImgSize;

    /// Non mutable access to image line pixel data,
    /// requires image line to be stored as a continous block of memory,
    /// but allows any layout of image lines
    ///
    /// Panics if image line does not exist
    fn line_ref(&self, line: usize) -> &[T];

    /// Image width in pixels
    fn width(&self) -> usize { self.size().x }

    /// Image height in pixels
    fn height(&self) -> usize  { self.size().y }

    /// Image size as a 2-dimensional range
    fn range(&self) -> ImgRange {
        Range2d::new(0..self.width() as isize, 0..self.height() as isize)
    }
}

/// Read-write access to image pixels, used as image data output
pub trait ImgMut<T>: Img<T> {
    /// Mutable access to specific image line
    ///
    /// Panics if image line does not exist
    fn line_mut(&mut self, line: usize) -> &mut [T];
}

/// Image width and height, specifies an image data size for any
/// Img and ImgMut implementations
pub type ImgSize = Vec2d<usize>;
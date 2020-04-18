use crate::geometry::{Range2d, Point};

/// Read-only access to image pixels, usually used as input data
pub trait Img<T> {
    /// Image width and height, in pixels
    fn size(&self) -> ImgSize;

    /// Non mutable access to image line pixel data,
    /// requires image line to be stored as a continous block of memory,
    /// but allows any layout of image lines
    fn line_ref(&self, line: usize) -> &[T];

    /// Image width in pixels
    fn width(&self) -> usize { self.size().x }

    /// Image height in pixels
    fn height(&self) -> usize  { self.size().y }

    fn range(&self) -> Range2d<isize> {
        Range2d::new(0..self.width() as isize, 0..self.height() as isize)
    }
}

/// Read-write access to image pixels, used as image data output
pub trait ImgMut<T>: Img<T> {
    /// Mutable access to specific image line
    fn line_mut(&mut self, line: usize) -> &mut [T];
}

pub type ImgSize = Point<usize>;
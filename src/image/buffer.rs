use super::{Img, ImgMut, ImgSize, dimensions::ImgDimensions};

/// Data buffer that stores image pixels and provides read access using
/// `Img` trait and mutable write access via `ImgMut` trait
/// 
/// Basic buffer implementation does not have any requirements for pixel type
/// `T`, but most functions require `T` to implement `Copy`
pub struct ImgBuf<T> {
    dimensions: ImgDimensions,
    /// Image pixels stored in a continous block of memory
    pixels: Vec<T>
}

impl<T> Img<T> for ImgBuf<T> {
    fn size(&self) -> ImgSize { self.dimensions.size }
    fn line_ref(&self, line: usize) -> &[T] { &self.pixels[self.line(line)] }
}

impl<T> ImgMut<T> for ImgBuf<T> {
    fn line_mut(&mut self, line: usize) -> &mut [T] {  
        let range = self.line(line);
        &mut self.pixels[range]
    }
}

impl<T> ImgBuf<T> {
    fn line(&self, line: usize) -> std::ops::Range<usize> {
        let start = line*self.dimensions.stride;
        (start)..(start + self.dimensions.size.x)
    }
}

impl<T: Copy> ImgBuf<T> {

}
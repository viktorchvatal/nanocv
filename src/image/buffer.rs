use super::{Img, ImgMut, ImgSize, dimensions::ImgBufLayout};

/// Data buffer for image pixels providing read access using
/// `Img` trait and write access via `ImgMut` trait
/// 
/// Basic buffer implementation does not have any requirements for pixel type
/// `T`, but most functions require `T` to implement `Copy`
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct ImgBuf<T> {
    dimensions: ImgBufLayout,
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

    pub fn dimensions(&self) -> ImgBufLayout {
        self.dimensions
    }
}

impl<T: Copy> ImgBuf<T> {
    pub fn from_vec_stride(dimensions: ImgBufLayout, pixels: Vec<T>) -> Self {
        dimensions.assert_data_size_correct(pixels.len());
        Self { dimensions, pixels }
    }

    pub fn from_vec(size: ImgSize, data: Vec<T>) -> Self {
        Self::from_vec_stride(ImgBufLayout { size, stride: size.x}, data)
    }
}

impl<T: Copy + Default> ImgBuf<T> {
    /// Create image with pixels initialized to default value of type `T`
    /// ```
    /// use nanocv::{ImgBuf, Img, ImgSize};
    /// let buf = ImgBuf::<u8>::new_default(ImgSize::new(3, 2));
    /// assert_eq!(buf.line_ref(0), &[0u8, 0u8, 0u8]);
    /// assert_eq!(buf.line_ref(1), &[0u8, 0u8, 0u8]);
    /// ```
    pub fn new_default(size: ImgSize) -> Self {
        Self::from_vec(size, vec![T::default(); size.product()])
    }
}
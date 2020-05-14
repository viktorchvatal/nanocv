use super::{Img, ImgMut, ImgSize, dimensions::ImgBufLayout};
use std::fmt::{Formatter, Debug, Error};

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

    /// Returns image dimensions
    /// ```
    /// use nanocv::{ImgBuf, ImgSize, ImgBufLayout};
    /// let buf = ImgBuf::<u8>::new(ImgSize::new(3, 2));
    /// assert_eq!(
    ///     buf.dimensions(),
    ///     ImgBufLayout { size: ImgSize::new(3, 2), stride: 3 }
    /// );
    /// ```
    pub fn dimensions(&self) -> ImgBufLayout {
        self.dimensions
    }

    /// Consumes image and returns underlying vector of pixel data
    /// ```
    /// use nanocv::{ImgBuf, ImgSize};
    /// let buf = ImgBuf::<u8>::from_vec(ImgSize::new(2, 2), vec![1, 2, 3, 4]);
    /// assert_eq!(buf.into_pixels(), vec![1, 2, 3, 4]);
    /// ```
    pub fn into_pixels(self) -> Vec<T> {
        self.pixels
    }
}

impl<T: Copy> ImgBuf<T> {
    /// Create image buffer of given size and row stride initialized 
    /// with provided data, if image width and stride are equal,
    /// `from_vec` function is more convenient 
    /// 
    /// Data vector length must correspond to 
    /// `dimensions.stride*dimensions.size.y`, otherwise this 
    /// function will panic
    /// ```
    /// use nanocv::{ImgBuf, Img, ImgSize, ImgBufLayout};
    /// let buf = ImgBuf::<u8>::from_vec_stride(
    ///     ImgBufLayout { size: ImgSize::new(1, 2), stride: 2 }, 
    ///     vec![1, 2, 3, 4]
    /// );
    /// assert_eq!(buf.line_ref(0), &[1]);
    /// assert_eq!(buf.line_ref(1), &[3]);
    /// ```
    pub fn from_vec_stride(dimensions: ImgBufLayout, pixels: Vec<T>) -> Self {
        dimensions.assert_data_size_correct(pixels.len());
        Self { dimensions, pixels }
    }

    /// Create image buffer of given size initialized with provided data
    /// 
    /// Data vector length must correspond to image `size`,
    /// otherwise this function will panic
    /// ```
    /// use nanocv::{ImgBuf, Img, ImgSize};
    /// let buf = ImgBuf::<u8>::from_vec(ImgSize::new(2, 2), vec![1, 2, 3, 4]);
    /// assert_eq!(buf.line_ref(0), &[1, 2]);
    /// assert_eq!(buf.line_ref(1), &[3, 4]);
    /// ```
    pub fn from_vec(size: ImgSize, data: Vec<T>) -> Self {
        Self::from_vec_stride(ImgBufLayout { size, stride: size.x}, data)
    }

    /// Create image buffer with pixels initialized to the `init` value
    /// ```
    /// use nanocv::{ImgBuf, Img, ImgSize};
    /// let buf = ImgBuf::<u8>::new_init(ImgSize::new(2, 2), 7u8);
    /// assert_eq!(buf.line_ref(0), &[7, 7]);
    /// assert_eq!(buf.line_ref(1), &[7, 7]);
    /// ```
    pub fn new_init(size: ImgSize, init: T) -> Self {
        Self::from_vec(size, vec![init; size.product()])
    }        
}

impl<T: Copy + Default> ImgBuf<T> {
    /// Create image buffer with pixels initialized to default value of type `T`
    /// ```
    /// use nanocv::{ImgBuf, Img, ImgSize};
    /// let buf = ImgBuf::<u8>::new(ImgSize::new(3, 2));
    /// assert_eq!(buf.line_ref(0), &[0, 0, 0]);
    /// assert_eq!(buf.line_ref(1), &[0, 0, 0]);
    /// ```
    pub fn new(size: ImgSize) -> Self {
        Self::from_vec(size, vec![T::default(); size.product()])
    }

    /// Create image buffer of the same type and same dimensions, as `other`
    /// ```
    /// use nanocv::{ImgBuf, Img, ImgSize};
    /// let buf = ImgBuf::<u8>::new(ImgSize::new(3, 2));
    /// assert_eq!(
    ///     ImgBuf::new_like(&buf).size(),
    ///     ImgSize::new(3, 2)
    /// );
    /// ```
    pub fn new_like(other: &Self) -> Self {
        Self::new(other.size())
    }    
}

impl<T: Debug> Debug for ImgBuf<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Image size: {:?} [\n", self.dimensions)?;

        for line in 0..self.height() {
            let line_pixels = self.line_ref(line);

            for value in line_pixels {
                write!(f, "{:5?}", value)?;
            }
            write!(f, "\n")?;
        }
        
        write!(f, "]\n")?;
        Ok(())
    }
}

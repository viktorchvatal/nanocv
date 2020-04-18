use super::ImgSize;

/// Image pixel data and allocated data size
pub struct ImgDimensions {
    /// Image data size specified by width and height
    pub size: ImgSize,
    /// Allocated image line width that can be larger than width to achieve
    /// data alignment, `stride >= width`
    pub stride: usize,
}

impl ImgDimensions {
    /// Number of allocated pixels
    pub fn data_length(&self) -> usize {
        self.size.y*self.stride
    }
}
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

    pub fn assert_data_size_correct(&self, data_size: usize) {
        assert_eq!(
            self.data_length(),
            data_size,
            "Vector of length {} cannot be used as an image {} X {} with \
            stride {}, correct length should be {}.",
            data_size,
            self.size.x,
            self.size.y,
            self.stride,
            self.data_length()
        )
    }
}

// ================================== TESTS ==================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_correct_data_size_should_be_ok() {
        let dimensions = ImgDimensions { size: ImgSize::new(2, 2), stride: 3 };
        dimensions.assert_data_size_correct(6)
    }

    #[test]
    #[should_panic]
    fn assert_correct_data_size_invalid_length() {
        let dimensions = ImgDimensions { size: ImgSize::new(2, 2), stride: 3 };
        dimensions.assert_data_size_correct(7)
    }    
}
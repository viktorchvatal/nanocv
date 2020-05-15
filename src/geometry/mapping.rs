use crate::{Vec2d, Range2d, ImgRange};

/// Mapping coordinates of an area in src image to area in dst image
/// 
/// src and dst must have same exactly same width and height, 
/// shift is difference of src and dst starting points, and
/// src lies within input image and dst list within output image
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct ImageMapping {
    /// Area in source image
    pub src: Range2d<usize>,
    /// Area in destination image
    pub dst: Range2d<usize>,
    /// Shift of src and dst areas
    pub shift: Vec2d<isize>,
}

impl ImageMapping {
    /// Create a new mapping from input to output image with respective ranges
    /// 
    /// # Arguments
    /// * `input_range` - 2D range in input image, cropped to input image size
    ///   and limited by `output_range` in size
    /// * `output_range` - 2D range in output image, cropped to output image
    ///   size and limited by `input_range` in size
    /// * `input_size` - input image size
    /// * `output_size` - output image size
    pub fn new(
        input_range: ImgRange, 
        output_range: ImgRange,
        input_size: ImgRange,
        output_size: ImgRange,
    ) -> Self {
        let shift = output_range.start() - input_range.start();

        let src = Range2d::<usize>::from(
            input_range.intersect(input_size).intersect(output_size - shift)
        );

        let dst = Range2d::<usize>::from(
            output_range.intersect(output_size).intersect(input_size + shift)
        );        

        Self { src, dst, shift }
    }
}
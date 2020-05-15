use crate::{Vec2d, Range2d, ImgRange, ImgSize};

/// Mapping coordinates of an area in src image to area in dst image,
/// src and dst should have same dimensions and shift is difference of
/// ranges starting points
pub struct ImageMapping {
    pub src: Range2d<usize>,
    pub dst: Range2d<usize>,
    pub shift: Vec2d<isize>,
}

impl ImageMapping {
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
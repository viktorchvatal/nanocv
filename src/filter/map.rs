use std::cmp::min;
use crate::{ImgMut, Img, ImgRange, Range2d};

/// Maps pixels from `input` at `input_range` into pixels
/// in `output` image in `output_range`
/// 
/// Pixels outside image ranges are ignored
/// 
/// # Arguments
///
/// * `input` - input read-only image
/// * `output` - output mutable image
/// * `input_range` - input pixel range
/// * `output_range` - output pixel range
/// * `operator` - combination function, maps either from both input and output image values 
/// into output valuem, e. g. `|input, output| input + output`, or just
/// from input to output image `|x, _| x + 1`
/// 
/// # Example
/// ```
/// use nanocv::{*, filter::map_range};
/// let size = ImgSize::new(3, 3);
/// 
/// let input = ImgBuf::<i8>::from_vec(size, vec![
///     1,  2,  3, 
///     4,  5,  6, 
///     7,  8,  9
/// ]);
/// 
/// let mut output = ImgBuf::new(size);
/// 
/// map_range(
///     &input, 
///     &mut output, 
///     Range2d::new(1..3, 1..3), 
///     Range2d::new(0..2, 0..2),
///     |x, _| -x
/// );
/// 
/// assert_eq!(
///     output,
///     ImgBuf::<i8>::from_vec(size, vec![
///         -5, -6,  0, 
///         -8, -9,  0, 
///          0,  0,  0
///     ])
/// )
/// ```
pub fn map_range<TI: Copy, TO: Copy, F>(
    input: &dyn Img<TI>,
    output: &mut dyn ImgMut<TO>,
    input_range: ImgRange,
    output_range: ImgRange,
    mut operator: F
) where F: FnMut(TI, TO) -> TO { 
    let shift = output_range.start() - input_range.start();

    let src_range = Range2d::<usize>::from(
        input_range.intersect(input.range()).intersect(output.range() - shift)
    );
    
    let dst_range = Range2d::<usize>::from(
        output_range.intersect(output.range()).intersect(input.range() + shift)
    );

    let height = min(src_range.height(), dst_range. height());

    for line in 0..height {
        let src = &input.line_ref(src_range.y.start + line)[src_range.x.to_range()];
        let dst = &mut output.line_mut(dst_range.y.start + line)[dst_range.x.to_range()];
        let max = min(src.len(), dst.len());

        for column in 0..max {
            dst[column] = operator(src[column], dst[column]);
        }
    }    
}


// ================================== TESTS ==================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ImgSize, ImgBuf};

    #[test]
    fn map_range_different_sizes() {
        let input = ImgBuf::<i8>::from_vec(
            ImgSize::new(2, 2), 
            vec![1, 2, 3, 4]
        );       

        let mut output = ImgBuf::<i8>::new(ImgSize::new(3, 3));

        map_range(
            &input, 
            &mut output, 
            ImgRange::new(0..2, 0..2), 
            ImgRange::new(1..3, 1..3), 
            |x, _| x
        );

        assert_eq!(
            output,
            ImgBuf::<i8>::from_vec(ImgSize::new(3, 3), vec![
                0,  0,  0,
                0,  1,  2,
                0,  3,  4,
            ])
        )        
    }
}
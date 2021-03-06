use std::cmp::min;
use crate::{ImgMut, ImgBuf, Img, ImgRange, ImageMapping};

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
/// * `operator` - either just mapping function from input to output `|x, _| -x`
/// or combinator that uses both input and output values to produce new 
/// output value `|i, o| i + o`
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
    let mapping = ImageMapping::new(input_range, output_range, input.range(), output.range());

    for line in 0..mapping.src.height() {
        let src = &input.line_ref(mapping.src.y.start + line)[mapping.src.x.to_range()];
        let dst = &mut output.line_mut(mapping.dst.y.start + line)[mapping.dst.x.to_range()];
        let max = min(src.len(), dst.len());

        for column in 0..max {
            dst[column] = operator(src[column], dst[column]);
        }
    }    
}

/// Maps pixels from `input` image onto `output` image
/// 
/// # Arguments
///
/// * `input` - input read-only image
/// * `output` - output mutable image
/// * `operator` - either just mapping function from input to output `|x, _| -x`
/// or combinator that uses both input and output values to produce new 
/// output value `|i, o| i + o`
/// 
/// # Example
/// 
/// Invert all pixels in image `input` and write values into `output`
/// ```
/// use nanocv::{*, filter::map};
/// let input = ImgBuf::<i8>::from_vec(ImgSize::new(2, 2), vec![1, 2, 3, 4]);
/// let mut output = ImgBuf::new(input.size());
/// map(&input, &mut output, |x, _| -x);
/// assert_eq!(output, ImgBuf::<i8>::from_vec(input.size(), vec![-1, -2, -3 ,-4]))
/// ```
/// 
/// Add values from image `a` and `b` and write result into `b`
/// ```
/// use nanocv::{*, filter::map};
/// let a = ImgBuf::<i8>::from_vec(ImgSize::new(2, 2), vec![1, 2, 3, 4]);
/// let mut b = ImgBuf::<i8>::from_vec(a.size(), vec![2, 4, 6, 8]);
/// map(&a, &mut b, |a, b| a + b);
/// assert_eq!(b, ImgBuf::<i8>::from_vec(a.size(), vec![3, 6, 9, 12]))
/// ```
pub fn map<TI: Copy, TO: Copy, F>(
    input: &dyn Img<TI>,
    output: &mut dyn ImgMut<TO>,
    operator: F
) where F: FnMut(TI, TO) -> TO { 
    let output_range = output.range();
    map_range(input, output, input.range(), output_range, operator);
}

/// Maps pixels from `input` image into newly created `ImgBuf` image with same size as `input`
/// 
/// # Example
/// 
/// Invert all pixels in image `input` and return result as `output`
/// 
/// ```
/// use nanocv::{*, filter::map_new};
/// let input = ImgBuf::<i8>::from_vec(ImgSize::new(2, 2), vec![1, 2, 3, 4]);
/// let output = map_new(&input, |x| -x);
/// assert_eq!(output, ImgBuf::<i8>::from_vec(input.size(), vec![-1, -2, -3 ,-4]))
/// ```
pub fn map_new<TI: Copy, TO: Copy + Default, F>(input: &dyn Img<TI>, mut operator: F) -> ImgBuf<TO> 
where F: FnMut(TI) -> TO { 
    let mut output = ImgBuf::new(input.size());
    let input_range = input.range();
    let output_range = output.range();
    map_range(input, &mut output, input_range, output_range, |x, _| operator(x));
    output
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
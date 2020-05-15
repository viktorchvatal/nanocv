use crate::{ImgMut, Img, ImgRange, ImageMapping, Range};
use std::{cmp::min};
use super::plan::{create_filter_plan, FilterIteration};

/// Horizontal image filter for whole image
/// 
/// Input image is considered infinite, replicating values of 
/// nearest existing pixels.
/// 
/// # Arguments
///
/// * `input` - input read-only image
/// * `output` - output mutable image
/// * `kernel` - filter kernel
/// * `operator` - operator between input, output and kernel, for convolution
///   filter, use `convolution_operator` function
/// 
/// # Example
/// ```
/// use nanocv::{*, filter::{horizontal_filter, convolution_operator}};
/// 
/// let input = ImgBuf::from_vec(
///     ImgSize::new(3, 2), 
///     vec![
///         1,  2,  0,
///         3,  4,  0,
///     ]
/// );
/// 
/// let mut output = ImgBuf::new_like(&input);
/// let kernel = [0, 0, -1];
/// horizontal_filter(&input, &mut output, &kernel, convolution_operator);
/// 
/// assert_eq!(
///     output,
///     ImgBuf::<i8>::from_vec(input.size(), vec![
///         -1, -1, -2,
///         -3, -3, -4,
///     ])
/// );
/// ```
pub fn horizontal_filter<T: Copy, F>(
    input: &dyn Img<T>,
    output: &mut dyn ImgMut<T>, 
    kernel: &[T], 
    operator: F
) where F: Fn(&[T], &mut [T], T) {
    let output_range = output.range();
    horizontal_filter_range(input, output, kernel, input.range(), output_range, operator)
}

/// Horizontal image filter for specific range
/// 
/// # Arguments
///
/// * `input` - input read-only image
/// * `output` - output mutable image
/// * `kernel` - filter kernel
/// * `input_range` - input pixel range
/// * `output_range` - output pixel range
/// * `operator` - operator between input, output and kernel, for convolution
///   filter, use `convolution_operator` function
pub fn horizontal_filter_range<T: Copy, F>(
    input: &dyn Img<T>,
    output: &mut dyn ImgMut<T>, 
    kernel: &[T], 
    input_range: ImgRange, 
    output_range: ImgRange, 
    operator: F
) where F: Fn(&[T], &mut [T], T) {
    let mapping = ImageMapping::new(input_range, output_range, input.range(), output.range());
    let (l, r) = (mapping.src.x.start, mapping.src.x.end);

    let plan = create_filter_plan(
        input.width(), kernel.len(), 
        Range::<isize>::from(mapping.src.x), 
        Range::<isize>::from(mapping.dst.x),
    );

    for line in mapping.src.y.to_range() {
        let src = input.line_ref(line as usize);
        let dst = output.line_mut((line as isize + mapping.shift.y) as usize);

        for index in 0..plan.len() {
            let ref bound: &FilterIteration = &plan[index];
            let value: T = kernel[bound.kernel_index];

            // Convolution with pixels outside image at the beginning
            for outside in 0..min(mapping.src.width(), bound.outside_start) {
                let src = &src[0..1];
                let dst = &mut dst[(outside + l)..(outside + l + 1)];
                operator(src, dst, value);
            }

            // Convolution with pixels within image
            {
                let src = &src[bound.src_range.to_range()];
                let dst = &mut dst[bound.dst_range.to_range()];
                operator(src, dst, value);
            }

            // Convolution with pixels outside image at the end
            for outside in 0..min(mapping.src.width(), bound.outside_end) {
                let col = r - outside - 1;
                let src = &src[(input.width() - 1)..input.width()];
                let dst = &mut dst[col..(col + 1)];
                operator(src, dst, value);
            }
        }
    }
}

// ================================== TESTS ==================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ImgSize, ImgBuf, Vec2d, filter::convolution_operator};

    fn test_image_1() -> ImgBuf<i16> {
        ImgBuf::from_vec(
            ImgSize::new(4, 3), 
            vec![
                1,  2,  3,  4,
                5,  6,  7,  8,
                9, 10, 11, 12
            ]
        )
    }

    /// Create unit test named $name, that tests that image $img
    /// convoluted with horizontal vector $kernel is equal to $expected,
    /// $img is treated as infinite, replicating values at its borders
    // 
    // Octave script to generate test matrices
    // 
    // ```
    // pkg load image;
    // A = [1 2 3 4; 5 6 7 8; 9 10 11 12]; % input image
    // K = [0 1 0]; % convolution kernel
    // margin = int32(numel(K))/2-1;
    // large_A = padarray(A, [margin margin], 'replicate');
    // large_B = conv2(large_A, K, 'same');
    // S = margin + 1;
    // H = size(large_A, 1);
    // W = size(large_A, 2);
    // B = large_B(S:(H - margin), S:(W - margin))
    // ```
    macro_rules! tst {
        ($name: ident, $img: expr, $kernel: expr, $expected: expr) => {
            #[test]
            fn $name() {
                let input = $img;
                let mut output = ImgBuf::new_like(&input);
        
                horizontal_filter_range(
                    &input, &mut output, &$kernel, 
                    input.range(), input.range(), convolution_operator
                );
        
                assert_eq!(output, $expected);
            }                    
        };
    }

    tst!(
        conv_matrix_4x3_kernel_1, test_image_1(), [1], 
        test_image_1()
    );

    tst!(
        conv_matrix_4x3_kernel_0_1_0, test_image_1(), [0, 1, 0], 
        test_image_1()
    );    

    tst!(
        conv_matrix_4x3_kernel_0_0_1_0_0, test_image_1(), [0, 0, 1, 0, 0], 
        test_image_1()
    );        

    tst!(
        conv_matrix_4x3_kernel_1_1_1, test_image_1(), [1, 1, 1], 
        ImgBuf::from_vec(
            ImgSize::new(4, 3), 
            vec![
               4,    6,    9,   11,
              16,   18,   21,   23,
              28,   30,   33,   35
            ],
        )    
    );       
    
    tst!(
        conv_matrix_4x3_kernel_1_2_3, test_image_1(), [1, 2, 3], 
        ImgBuf::from_vec(
            ImgSize::new(4, 3), 
            vec![
                 7,   10,   16,   21,
                31,   34,   40,   45,
                55,   58,   64,   69,
            ]
        )
    );

    #[test]
    fn conv_identity_to_larger_image() {
        let input = test_image_1();
        let mut output = ImgBuf::new(ImgSize::new(5, 4));

        horizontal_filter_range(
            &input, &mut output, &[1], 
            input.range(), input.range(), convolution_operator
        );

        assert_eq!(
            output, 
            ImgBuf::from_vec(
                ImgSize::new(5, 4), 
                vec![
                    1,  2,  3,  4, 0,
                    5,  6,  7,  8, 0,
                    9, 10, 11, 12, 0,               
                    0,  0,  0,  0, 0,
                ]
            )            
        );
    }  
    
    #[test]
    fn conv_identity_output_moved_down() {
        let input = test_image_1();
        let mut output = ImgBuf::new(ImgSize::new(5, 4));

        horizontal_filter_range(
            &input, &mut output, &[1], 
            input.range(), 
            input.range() + Vec2d::new(0, 1), 
            convolution_operator
        );

        assert_eq!(
            output, 
            ImgBuf::from_vec(
                ImgSize::new(5, 4), 
                vec![
                    0,  0,  0,  0, 0,
                    1,  2,  3,  4, 0,
                    5,  6,  7,  8, 0,
                    9, 10, 11, 12, 0,               
                ]
            )            
        );
    }    
    
    #[test]
    fn conv_identity_output_moved_down_more() {
        let input = test_image_1();
        let mut output = ImgBuf::new(ImgSize::new(5, 4));

        horizontal_filter_range(
            &input, &mut output, &[1], 
            input.range(), 
            input.range() + Vec2d::new(0, 2), 
            convolution_operator
        );

        assert_eq!(
            output, 
            ImgBuf::from_vec(
                ImgSize::new(5, 4), 
                vec![
                    0,  0,  0,  0, 0,
                    0,  0,  0,  0, 0,
                    1,  2,  3,  4, 0,
                    5,  6,  7,  8, 0,
                ]
            )            
        );
    }      

    #[test]
    fn conv_identity_output_moved_up() {
        let input = test_image_1();
        let mut output = ImgBuf::new(ImgSize::new(5, 4));

        horizontal_filter_range(
            &input, &mut output, &[1], 
            input.range(), 
            input.range() + Vec2d::new(0, -1), 
            convolution_operator
        );

        assert_eq!(
            output, 
            ImgBuf::from_vec(
                ImgSize::new(5, 4), 
                vec![
                    5,  6,  7,  8, 0,
                    9, 10, 11, 12, 0,               
                    0,  0,  0,  0, 0,
                    0,  0,  0,  0, 0,
                ]
            )            
        );
    }  
    
    #[test]
    fn conv_identity_output_moved_right() {
        let input = test_image_1();
        let mut output = ImgBuf::new(ImgSize::new(5, 4));

        horizontal_filter_range(
            &input, &mut output, &[1], 
            input.range(), 
            input.range() + Vec2d::new(1, 0), 
            convolution_operator
        );

        assert_eq!(
            output, 
            ImgBuf::from_vec(
                ImgSize::new(5, 4), 
                vec![
                    0,  1,  2,  3,  4,
                    0,  5,  6,  7,  8,
                    0,  9, 10, 11, 12,               
                    0,  0,  0,  0,  0,
                ]
            )            
        );
    }    
    

    #[test]
    fn conv_identity_output_moved_right_more() {
        let input = test_image_1();
        let mut output = ImgBuf::new(ImgSize::new(5, 4));

        horizontal_filter_range(
            &input, &mut output, &[1], 
            input.range(), 
            input.range() + Vec2d::new(2, 0), 
            convolution_operator
        );

        assert_eq!(
            output, 
            ImgBuf::from_vec(
                ImgSize::new(5, 4), 
                vec![
                    0,  0,  1,  2,  3,
                    0,  0,  5,  6,  7,
                    0,  0,  9, 10, 11, 
                    0,  0,  0,  0,  0,
                ]
            )            
        );
    }    
    

    #[test]
    fn conv_identity_output_moved_left() {
        let input = test_image_1();
        let mut output = ImgBuf::new(ImgSize::new(5, 4));

        horizontal_filter_range(
            &input, &mut output, &[1], 
            input.range(), 
            input.range() + Vec2d::new(-1, 0), 
            convolution_operator
        );

        assert_eq!(
            output, 
            ImgBuf::from_vec(
                ImgSize::new(5, 4), 
                vec![
                     2,  3,  4, 0, 0,
                     6,  7,  8, 0, 0,
                    10, 11, 12, 0, 0,               
                     0,  0,  0, 0, 0,
                ]
            )            
        );
    }      
}
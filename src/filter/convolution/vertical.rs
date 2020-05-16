use crate::{ImgMut, Img, ImgRange, ImageMapping, Range};
use std::{cmp::min};
use super::plan::{create_filter_plan, FilterIteration};

pub fn vertical_filter<T: Copy, F>(
    input: &dyn Img<T>,
    output: &mut dyn ImgMut<T>, 
    kernel: &[T], 
    operator: F
) where F: Fn(&[T], &mut [T], T) {
    let output_range = output.range();
    vertical_filter_range(input, output, kernel, input.range(), output_range, operator)
}

/// Vertical image filter for specific range
/// 
/// # Arguments
///
/// * `input` - input read-only image
/// * `output` - output mutable image
/// * `kernel` - filter kernel, must contain odd number of elements
/// * `input_range` - input pixel range
/// * `output_range` - output pixel range
/// * `operator` - operator between input, output and kernel, for convolution
///   filter, use `convolution_operator` function
pub fn vertical_filter_range<T: Copy, F>(
    input: &dyn Img<T>,
    output: &mut dyn ImgMut<T>, 
    kernel: &[T], 
    input_range: ImgRange, 
    output_range: ImgRange, 
    operator: F
) where F: Fn(&[T], &mut [T], T) {
    if kernel.len() % 2 == 0 {
        panic!("Only kernels with odd number of elements are supported");
    }

    let mapping = ImageMapping::new(input_range, output_range, input.range(), output.range());

    let plan = create_filter_plan(
        input.height(), kernel.len(), 
        Range::<isize>::from(mapping.src.y), 
        Range::<isize>::from(mapping.dst.y),
    );

    let columns = mapping.src.x;
    let (t, b) = (mapping.src.y.start, mapping.src.y.end);
    
    for index in 0..plan.len() {
        let ref bound: &FilterIteration = &plan[index];
        let value: T = kernel[bound.kernel_index];

        // Convolution with pixels outside image at the beginning
        for extend in 0..min(mapping.src.height(), bound.outside_start) {
            let src = &input.line_ref(0)[columns.to_range()];
            let dst = &mut output.line_mut(t + extend)[columns.to_range()];
            operator(src, dst, value);
        }

        // Convolution with pixels within image
        for offset in 0..(bound.src_range.end - bound.src_range.start) {
            let src = &input.line_ref(bound.src_range.start + offset)[columns.to_range()];
            let dst = &mut output.line_mut(bound.dst_range.start + offset)[columns.to_range()];
            operator(src, dst, value);
        }

        // Convolution with pixels outside image at the end
        for extend in 0..min(mapping.src.height(), bound.outside_end) {
            let line = b - extend - 1;
            let src = &input.line_ref(input.height() - 1)[columns.to_range()];
            let dst = &mut output.line_mut(line)[columns.to_range()];
            operator(src, dst, value);
        }
    }
}

// ================================== TESTS ==================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ImgSize, ImgBuf, filter::convolution_operator};

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
    // large_B = conv2(large_A, K', 'same');
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
        
                vertical_filter_range(
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
                 7,  10,  13,  16,
                15,  18,  21,  24,
                23,  26,  29,  32,
            ],
        )    
    );    

    tst!(
        conv_matrix_4x3_kernel_1_2_3, test_image_1(), [1, 2, 3], 
        ImgBuf::from_vec(
            ImgSize::new(4, 3), 
            vec![
                10,  16,  22,  28,
                22,  28,  34,  40,
                42,  48,  54,  60,
            ],
        )    
    );    
}
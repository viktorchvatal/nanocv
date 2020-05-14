use crate::{ImgMut, Img, ImgRange, Range2d};
use std::{cmp::min, ops::{Mul, Add}};
use super::plan::{create_filter_plan, FilterIteration};

pub fn horizontal_filter_range<T: Copy, F>(
    input: &dyn Img<T>,
    output: &mut dyn ImgMut<T>, 
    kernel: &[T], 
    input_range: ImgRange, 
    output_range: ImgRange, 
    operator: F
) where F: Fn(&[T], &mut [T], T) {
    let range = Range2d::<usize>::from(input.range().intersect(input_range));
    let (l, r, t, b) = (range.x.start, range.x.end, range.y.start, range.y.end);
    let plan = create_filter_plan(input.width(), kernel.len(), l, r);

    for line in t..b {
        let (src, dst) = (input.line_ref(line), output.line_mut(line));

        for index in 0..plan.len() {
            let ref bound: &FilterIteration = &plan[index];
            let value: T = kernel[bound.kernel_index];

            // Convolution with pixels outside image at the beginning
            for outside in 0..min(range.width(), bound.outside_start) {
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
            for outside in 0..min(range.width(), bound.outside_end) {
                let col = r - outside - 1;
                let src = &src[(input.width() - 1)..input.width()];
                let dst = &mut dst[col..(col + 1)];
                operator(src, dst, value);
            }
        }
    }
}

#[inline(never)]
pub fn convolution_operator<T>(
    src: &[T], 
    dst: &mut [T],
    kernel: T
)
where T: Add<T, Output=T> + Mul<T, Output=T> + Copy {
    let max = min(src.len(), dst.len());
    let src = &src[0..max];
    let dst = &mut dst[0..max];

    for index in 0..max {
        dst[index] = dst[index] + kernel*src[index];
    }
}

// ================================== TESTS ==================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ImgSize, ImgBuf};

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
}
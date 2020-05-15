use std::{cmp::min, ops::{Mul, Add}};

/// Convolution operator function
/// 
/// For every index `i` in src, computes `dst[i] += kernel*src[i]`
/// 
/// # Arguments
///
/// * `src` - source read-only slice
/// * `dst` - destination mutable slice
/// * `kernel` - kernel value
/// 
/// # Example
/// ```
/// use nanocv::{*, filter::convolution_operator};
/// let input = [1, 2, 3];
/// let mut output = [4, 5, 6];
/// let kernel = 3;
/// convolution_operator(&input, &mut output, kernel);
/// assert_eq!(output, [4 + 3*1, 5 + 3*2, 6 + 3*3]);
/// ```
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
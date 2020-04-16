use std::cmp::{min, max};
use crate::Range;

/// A recipe for one iteration of a convolution filter
/// 
/// Used for both vertical and horizontal filter implementations
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct FilterIteration {
    /// Range in one line/column of source image
    pub src_range: Range<usize>,
    /// Range in one line/column of destination image
    pub dst_range: Range<usize>,
    /// Index into convolution kernel
    pub kernel_index: usize,
    /// Number of pixels outside image to be replaced by first pixel value
    pub outside_start: usize,
    /// Number of pixels outside image to be replaced by last pixel value
    pub outside_end: usize
}

/// Prepare iteration plan for a filter
///
/// # Arguments
///
/// * `length` - length of image line for horizontal filter,
///   or image height for vertical filter
/// * `start` - start pixel (inclusive) in a line/column
/// * `end` - end pixel (exclusive) in a line/column
/// * `kernel_size` - size of a kernel
pub fn create_filter_plan(
    length: usize,
    kernel_size: usize,
    start: usize,
    end: usize,
) -> Vec<FilterIteration> {
    let center = ((kernel_size - 1) / 2) as isize;
    let first = - (center as isize);
    let last = kernel_size as isize - center as isize;

    (first..last)
        .map(|position| iteration(
            position, center, length as isize, start as isize, end as isize
        ))
        .collect()
}

fn iteration(
    shift: isize, 
    levels: isize, 
    length: isize, 
    start: isize, 
    end: isize
) -> FilterIteration {
    let src_range = Range::new(max(0, start + shift)..min(length, end + shift));

    FilterIteration { 
        src_range: Range::from(src_range),
        dst_range: Range::from(
            Range::new((src_range.start - shift)..(src_range.end - shift))
        ),
        kernel_index: (levels - shift) as usize,
        outside_start: max(0, - (start + shift)) as usize,
        outside_end: max(0, end - length + shift) as usize,
    }
}

// ================================== TESTS ==================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kernel_size_1_image_size_3_from_0_to_3() {
        assert_eq!(
            create_filter_plan(3, 1, 0, 3),
            vec![
                FilterIteration {
                    src_range: Range::new(0..3),
                    dst_range: Range::new(0..3),
                    kernel_index: 0,
                    outside_start: 0,
                    outside_end: 0
                }
            ]
        )
    }

    #[test]
    fn kernel_size_1_image_size_3_from_1_to_2() {
        assert_eq!(
            create_filter_plan(3, 1, 1, 2),
            vec![
                FilterIteration {
                    src_range: Range::new(1..2),
                    dst_range: Range::new(1..2),
                    kernel_index: 0,
                    outside_start: 0,
                    outside_end: 0
                }
            ]
        )
    } 

    #[test]
    fn kernel_size_3_image_size_3_from_0_to_3() {
        assert_eq!(
            create_filter_plan(3, 3, 0, 3),
            vec![
                FilterIteration {
                    src_range: Range::new(0..2),
                    dst_range: Range::new(1..3),
                    kernel_index: 2,
                    outside_start: 1,
                    outside_end: 0
                },
                FilterIteration {
                    src_range: Range::new(0..3),
                    dst_range: Range::new(0..3),
                    kernel_index: 1,
                    outside_start: 0,
                    outside_end: 0
                },
                FilterIteration {
                    src_range: Range::new(1..3),
                    dst_range: Range::new(0..2),
                    kernel_index: 0,
                    outside_start: 0,
                    outside_end: 1
                },                                
            ]
        )
    }    

    #[test]
    fn kernel_size_3_image_size_3_from_1_to_2() {
        assert_eq!(
            create_filter_plan(3, 3, 1, 2),
            vec![
                FilterIteration {
                    src_range: Range::new(0..1),
                    dst_range: Range::new(1..2),
                    kernel_index: 2,
                    outside_start: 0,
                    outside_end: 0
                },
                FilterIteration {
                    src_range: Range::new(1..2),
                    dst_range: Range::new(1..2),
                    kernel_index: 1,
                    outside_start: 0,
                    outside_end: 0
                },
                FilterIteration {
                    src_range: Range::new(2..3),
                    dst_range: Range::new(1..2),
                    kernel_index: 0,
                    outside_start: 0,
                    outside_end: 0
                },                                
            ]
        )
    }   
    
    #[test]
    fn kernel_size_3_image_size_1_from_0_to_1() {
        assert_eq!(
            create_filter_plan(1, 3, 0, 1),
            vec![
                FilterIteration {
                    src_range: Range::new(0..0),
                    dst_range: Range::new(1..1),
                    kernel_index: 2,
                    outside_start: 1,
                    outside_end: 0
                },
                FilterIteration {
                    src_range: Range::new(0..1),
                    dst_range: Range::new(0..1),
                    kernel_index: 1,
                    outside_start: 0,
                    outside_end: 0
                },
                FilterIteration {
                    src_range: Range::new(1..1),
                    dst_range: Range::new(0..0),
                    kernel_index: 0,
                    outside_start: 0,
                    outside_end: 1
                },                                
            ]
        )
    }    
}
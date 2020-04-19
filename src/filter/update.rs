

use crate::{Range2d, ImgMut};

/// Update specific range of the given image using an operator
/// 
/// # Examples
///
/// Update specific image range by increasing pixel value by 1
/// ```
/// use nanocv::{*, filter::update_range};
/// let mut img = ImgBuf::<u8>::from_vec(ImgSize::new(2, 2), vec![1, 2, 3, 4]);
/// update_range(&mut img, Range2d::new(0..1, 0..1), |x| x + 1);
/// assert_eq!(img.line_ref(0), &[2, 2]);
/// assert_eq!(img.line_ref(1), &[3, 4]);
/// ```
pub fn update_range<T: Copy, F>(image: &mut dyn ImgMut<T>, range: Range2d<isize>, operator: F) 
where F : Fn(T) -> T {
    // Assure that range is within image
    let range = Range2d::<usize>::from(range.intersect(image.range()));

    for line in range.y.start..range.y.end {
        let dst = &mut image.line_mut(line)[range.x.start..range.x.end];

        for col in 0..dst.len() {
            dst[col] = operator(dst[col]);
        }
    }    
}

/// Update the given image using an operator
/// 
/// # Examples
///
/// Update whole image by increasing pixel value by 1
/// ```
/// use nanocv::{*, filter::update};
/// let mut img = ImgBuf::<u8>::from_vec(ImgSize::new(2, 2), vec![1, 2, 3, 4]);
/// update(&mut img, |x| x + 1);
/// assert_eq!(img.line_ref(0), &[2, 3]);
/// assert_eq!(img.line_ref(1), &[4, 5]);
/// ```
pub fn update<T: Copy, F>(image: &mut dyn ImgMut<T>, operator: F) 
where F : Fn(T) -> T {
    let range = image.range();  
    update_range(image, range, operator)
}
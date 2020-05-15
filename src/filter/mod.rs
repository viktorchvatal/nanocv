//! Algorithms directly modifying or transforming images
//! 
//! Each function usually exists in more variants, for example `map_range`
//! is a powerful function that maps pixels in one image area to another
//! image and specified area, and allows a combination of values from both 
//! images. In case that just a whole image is about to be mapped to another image, 
//! a `map` convenient function available. And in case a new output image buffer has 
//! to be created automatically, `map_new` convenient suits even better.
//! 
//! Following conventions are used among all filter functions:
//!  * functions with `_range` suffix are usually the most powerful,
//!    allowing a specific range for the filter to operate on, however,
//!    not all functions support the feature
//!  * functions with `_new` suffix automatically create new output image
//!    buffer of `ImgBuf` type and do not require output buffer to be specified,
//!    however, no other buffer type constructors are supported
//! 
//! Notable function families are:
//!  * `update` - update image pixels in place:
//!    [update](fn.update.html), [update_range](fn.update_range.html)
//!  * `map` - map pixels from one image to another:
//!    [map](fn.map.html), [map_range](fn.map_range.html), 
//!    [map_new](fn.map_new.html)

mod update;
mod map;
mod convolution;

pub use update::{update, update_range};
pub use map::{map, map_range, map_new};

pub use convolution::{
    horizontal_filter_range, horizontal_filter, 
    convolution_operator
};
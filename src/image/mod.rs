//! Traits and structures for in-memory image buffers

mod traits;
mod buffer;
mod dimensions;

pub use traits::{Img, ImgMut, ImgSize};
pub use buffer::{ImgBuf};
pub use dimensions::ImgDimensions;
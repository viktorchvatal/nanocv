mod geometry;
mod image;

// Essential types for nanocv are exported to root module of the crate
pub use image::{Img, ImgMut, ImgSize, ImgBuf, ImgBufLayout};
pub use geometry::{Range, Range2d, ImgRange, Point};

// Specific algorithms and methods are defined in respective modules
pub mod filter;
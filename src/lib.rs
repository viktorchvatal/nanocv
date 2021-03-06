mod geometry;
mod image;

use geometry::{ImageMapping};

// Essential types for nanocv are exported to root module of the crate
pub use self::image::{Img, ImgMut, ImgSize, ImgBuf, ImgBufLayout};
pub use geometry::{Range, Range2d, ImgRange, Vec2d};

// Specific algorithms and methods are defined in respective modules
pub mod filter;
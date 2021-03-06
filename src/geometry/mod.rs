//! Basic geometric primitives to describe sizes, positions and dimensions

mod vec2d;
mod range;
mod range2d;
mod mapping;

pub use range::Range;
pub use range2d::{Range2d, ImgRange};
pub use vec2d::Vec2d;
pub use mapping::ImageMapping;
//! Algorithms directly modifying or transforming images

mod horizontal;
mod vertical;
mod plan;
mod update;
mod map;

pub use update::{update, update_range};
pub use map::{map_range};
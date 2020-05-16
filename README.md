# nanocv

[![Travis status](https://api.travis-ci.org/viktorchvatal/nanocv.svg?branch=master)](https://travis-ci.org/github/viktorchvatal/nanocv)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## nanocv is a work in progress far from any experimental build

### Motivation

 * Well, I wanted to make it myself ;)

### Primary goals

 * Minimal dependencies for basic usage
 * Batteries included with optional crate features turned on
 * No special optimizations, rather help the compiler to autovectorize the code
 * No unsafe code (except for few special SSE/AVX optimized functions 
   disabled by default)

### Secondary goals

 * Multi threaded versions of functions

### Non-goals

 * GPU support (there can be another crate for this)

## Basic usage

 * library contains its own `ImgBuf` structure to store image pixel data
 * however, all image processing functions work on any data structure that 
   implements `Img` and `ImgMut` traits

## Examples  

### Loading and saving images

[See load_save example](examples/load_save.rs)

`nanocv` is an image processing library, but lacks any image file loading / saving capabilities.
It is, however, super easy to use another library for this. Most examples use `image` crate for this.

Loading and image file into `ImgBuf` and saving result is as easy as

```rust
use image::{open, GrayImage};
use nanocv::{ImgBuf, ImgSize};

fn main() {
    // Load image using piston image
    let buf = open("examples/raster.png").unwrap().into_luma();
    // Convert into ImgBuf
    let size = ImgSize::new(buf.width() as usize, buf.height() as usize);
    let img = ImgBuf::from_vec(size, buf.into_vec());
    // Convert back to piston gray image
    let result = GrayImage::from_vec(size.x as u32, size.y as u32, img.into_vec()).unwrap();
    // Save result into target directory
    result.save("target/load_save.png").unwrap();
}
```

### Change pixel values in place

[See negative_image example](examples/negative_image.rs)

```rust
use nanocv::{ImgBuf, ImgSize, filter::update};

fn main() {
    let mut img: ImgBuf<u8> = ImgBuf::new(ImgSize::new(100, 100));
    // Compute negative image
    update(&mut img, |x| 255 - x);
}
```

### Horizontal and vertical convolution

[See horizontal_convolution example - horizontal convolution](examples/horizontal_convolution.rs)

Horizontal convolution filter
```rust
use nanocv::{ImgBuf, ImgSize, filter::{map_new, horizontal_filter, convolution_operator, update}};

fn main() {
    let img: ImgBuf<u8> = ImgBuf::new(ImgSize::new(100, 100));
    // Convert to 16-bit image buffer
    let img = map_new(&img, |x| x as u16);
    let mut result = ImgBuf::new_like(&img);
    // Horizontal convolution filter
    let kernel = [1, 1, 1, 1, 1, 1, 1, 1, 1];
    horizontal_filter(&img, &mut result, &kernel, convolution_operator);
    // Divide by 9 to fit into [0, 255] range
    update(&mut result, |x| x/10);
    // Convert back to 8-bit image 
    let _result = map_new(&result, |x| x as u8);
}
```

Vertical convolution is analogical to horizontal one

[See vertical_convolution example - vertical convolution](examples/vertical_convolution.rs)

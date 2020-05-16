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

[See full example](examples/load_save.rs)

`nanocv` is an image processing library, but lacks any image file loading / saving capabilities.
It is, however, super easy to use another library for this. Most examples use `image` crate for this.

Loading and image file into `ImgBuf` and saving result is as easy as
```
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
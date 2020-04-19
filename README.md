# nanocv

[![Travis status](https://api.travis-ci.org/viktorchvatal/nanocv.svg?branch=master)](https://travis-ci.org/github/viktorchvatal/nanocv)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## nanocv is a work in progress far from any experimental build

### Motivation

 * Well, i wanted to make it myself ;)

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


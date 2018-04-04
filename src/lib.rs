//! Simple module to create some generative art.
#![deny(missing_docs, warnings)]

#[cfg(test)]
#[macro_use]
extern crate maplit;

pub mod dragon;
pub mod drawing;
pub mod julia;
pub mod point;
pub mod quantize;
pub mod sierpinski;
pub mod utils;

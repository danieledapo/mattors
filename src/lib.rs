#![deny(warnings)]

#[cfg(test)]
#[macro_use]
extern crate maplit;

pub mod dragon;
pub mod julia;
pub mod point;
pub mod quantize;
pub mod utils;

pub use point::Point;

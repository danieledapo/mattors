//! Simple module to create some generative art.
#![deny(missing_docs, warnings)]

#[cfg(test)]
#[macro_use]
extern crate maplit;

#[cfg(test)]
#[macro_use]
extern crate proptest;

pub mod art;
pub mod color;
pub mod drawing;
pub mod geo;
pub mod utils;

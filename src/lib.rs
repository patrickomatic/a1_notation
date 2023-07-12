//! # a1_notation
//!
//! This is a library for parsing to and from A1-style notation.
//!
// TODO:
//
// * handle (ignore) `$`
//
mod a1;
mod a1_builder;
mod error;
mod position;
mod range_or_cell;

pub use a1::A1;
pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;

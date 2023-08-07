//! # a1_notation
//!
//! A library for parsing to and from A1 spreadsheet notation.  
//!
//! You can parse an A1-notation value using the `FromStr` trait:
//!
//! ```
//! use std::str::FromStr;
//! use a1_notation::A1;
//! let a1 = A1::from_str("A1").unwrap();
//!
//! assert_eq!(a1.x(), Some(0));
//! assert_eq!(a1.y(), Some(0));
//! ```
//!
//! Or if you have absolute coordinates and want to build A1 notation, you can use the builder
//! functions:
//!
//! ```
//! # use a1_notation::A1;
//! let a1_absolute = A1::builder()
//!                     .xy(0, 0)
//!                     .sheet_name("Important_stuff")
//!                     .build()
//!                     .unwrap();
//! // Cell A1
//! assert_eq!(a1_absolute.to_string(), "Important_stuff!A1");
//!
//! let a1_relative = A1::builder().x(0).build().unwrap();
//! // Column A
//! assert_eq!(a1_relative.to_string(), "A:A");
//!
//! let a1_range = A1::builder()
//!                 .range()
//!                 .from(A1::builder().x(0).build().unwrap())
//!                 .to(A1::builder().x(3).build().unwrap())
//!                 .build()
//!                 .unwrap();
//! // Range A:D
//! assert_eq!(a1_range.to_string(), "A:D");
//! ```
//!
//! Once you have an `A1`, you can shift/move it around using `shift_up`, `shift_down`,
//! `shift_left` and `shift_right`:
//!
//! ```
//! # use std::str::FromStr;
//! # use a1_notation::A1;
//! let a1 = A1::from_str("C3").unwrap();
//!
//! assert_eq!(a1.shift_down(2).to_string(), "C5");
//! assert_eq!(a1.shift_right(1).to_string(), "D3");
//! assert_eq!(a1.shift_left(1).to_string(), "B3");
//! assert_eq!(a1.shift_up(2).to_string(), "C1");
//! ```
//!
//! Here is a table illustrating A1 references:
//!
//! | **Reference**   | **Meaning**               |
//! |:----------------|:--------------------------|
//! | `"A1"`          | Cell A1                   |
//! | `"A1:B5"`       | Cells A1 through B5       |
//! | `"C5:D9,G9:H16"`| A multiple-area selection |
//! | `"A:A"`         | Column A                  |
//! | `"1:1"`         | Row 1                     |
//! | `"A:C"`         | Columns A through C       |
//! | `"1:5"`         | Rows 1 through 5          |
//! | `"1:1,3:3,8:8"` | Rows 1, 3, and 8          |
//! | `"A:A,C:C,F:F"` | Columns A, C, and F       |
//!
//
// TODO:
//
// * handle `$` between cells.
mod a1;
mod a1_builder;
mod error;
mod position;
mod range_or_cell;

pub use a1::A1;
pub use a1_builder::A1Builder;
pub use error::Error;
pub use position::Position;
pub use range_or_cell::RangeOrCell;

pub type Result<T> = std::result::Result<T, Error>;

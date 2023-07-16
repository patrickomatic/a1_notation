//! # RangeOrCell
//!
//! Parsing and displaying a cell value (which can pretty much always be either a cell or a range).
//! 
use serde::{Serialize, Deserialize};
use std::fmt;
use std::str;

use crate::{Error, Result};
use super::position::Position;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum RangeOrCell {
    /// A range with two sides:
    ///
    /// * `from` - Where the range begins
    ///
    /// * `to` - Where the range ends
    Range { 
        from: Position, 
        to: Position,
    },

    /// Just a single position
    Cell(Position),
}

impl str::FromStr for RangeOrCell {
    type Err = Error;

    fn from_str(a1: &str) -> Result<Self> {
        if let Some((l, r)) = a1.split_once(':') {
            Ok(RangeOrCell::Range {
                from: Position::from_str(l)?,
                to: Position::from_str(r)?,
            })
        } else {
            Ok(RangeOrCell::Cell(Position::from_str(a1)?))
        }
    }
}

impl fmt::Display for RangeOrCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Range { from, to } =>
                write!(f, "{}:{}", from.display_for_range(), to.display_for_range()),
            Self::Cell(p) =>
                write!(f, "{}", p),
        }
    }
}

#[cfg(test)] 
mod tests {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn display_cell() {
        assert_eq!(
            "A1", 
            RangeOrCell::Cell(Position::Absolute(0, 0)).to_string());
    }

    #[test]
    fn display_range() {
        assert_eq!(
            "1:3", 
            RangeOrCell::Range {
                from: Position::RowRelative(0),
                to: Position::RowRelative(2),
            }.to_string());
    }

    #[test]
    fn from_str_cell() {
        assert_eq!(
            RangeOrCell::Cell(Position::Absolute(0, 0)), 
            RangeOrCell::from_str("A1").unwrap());
    }

    #[test]
    fn from_str_range_row_relative() {
        assert_eq!(
            RangeOrCell::Range {
                from: Position::RowRelative(0),
                to: Position::RowRelative(5),
            },
            RangeOrCell::from_str("1:6").unwrap());
    }

    #[test]
    fn from_str_range_column_relative() {
        assert_eq!(
            RangeOrCell::Range {
                from: Position::ColumnRelative(0),
                to: Position::ColumnRelative(2),
            },
            RangeOrCell::from_str("A:C").unwrap());
    }
}

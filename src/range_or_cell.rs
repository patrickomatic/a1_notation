//! # RangeOrCell
//!
//! Parsing and displaying a cell value (which can pretty much always be either a cell or a range).
//! 
use serde::{Serialize, Deserialize};
use std::fmt;
use std::str;

use crate::{Error, Result, Shift};
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

impl Shift for RangeOrCell {
    fn shift_down(&self, rows: usize) -> Self {
        match self {
            Self::Range { from, to } =>
                Self::Range { 
                    from: from.shift_down(rows),
                    to: to.shift_down(rows),
                },
            Self::Cell(p) => Self::Cell(p.shift_down(rows)),
        }
    }

    fn shift_left(&self, columns: usize) -> Self {
        match self {
            Self::Range { from, to } =>
                Self::Range { 
                    from: from.shift_left(columns),
                    to: to.shift_left(columns),
                },
            Self::Cell(p) => Self::Cell(p.shift_left(columns)),
        }
    }

    fn shift_right(&self, columns: usize) -> Self {
        match self {
            Self::Range { from, to } =>
                Self::Range { 
                    from: from.shift_right(columns),
                    to: to.shift_right(columns),
                },
            Self::Cell(p) => Self::Cell(p.shift_right(columns)),
        }
    }

    fn shift_up(&self, rows: usize) -> Self {
        match self {
            Self::Range { from, to } =>
                Self::Range { 
                    from: from.shift_up(rows),
                    to: to.shift_up(rows),
                },
            Self::Cell(p) => Self::Cell(p.shift_up(rows)),
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

    #[test]
    fn shift_down_range() {
        assert_eq!(
            RangeOrCell::Range {
                from: Position::RowRelative(0),
                to: Position::RowRelative(5),
            }.shift_down(5),
            RangeOrCell::Range {
                from: Position::RowRelative(5),
                to: Position::RowRelative(10),
            });
    }

    #[test]
    fn shift_down_cell() {
        assert_eq!(
            RangeOrCell::Cell(Position::Absolute(0, 0)).shift_down(10), 
            RangeOrCell::Cell(Position::Absolute(0, 10)));
    }

    #[test]
    fn shift_left_range() {
        assert_eq!(
            RangeOrCell::Range {
                from: Position::RowRelative(0),
                to: Position::RowRelative(5),
            }.shift_left(5),
            RangeOrCell::Range {
                from: Position::RowRelative(0),
                to: Position::RowRelative(5),
            });
    }

    #[test]
    fn shift_left_cell() {
        assert_eq!(
            RangeOrCell::Cell(Position::Absolute(5, 0)).shift_left(10), 
            RangeOrCell::Cell(Position::Absolute(0, 0)));
    }

    #[test]
    fn shift_right_range() {
        assert_eq!(
            RangeOrCell::Range {
                from: Position::RowRelative(0),
                to: Position::RowRelative(5),
            }.shift_right(5),
            RangeOrCell::Range {
                from: Position::RowRelative(0),
                to: Position::RowRelative(5),
            });
    }

    #[test]
    fn shift_right_cell() {
        assert_eq!(
            RangeOrCell::Cell(Position::Absolute(0, 0)).shift_right(10), 
            RangeOrCell::Cell(Position::Absolute(10, 0)));
    }

    #[test]
    fn shift_up_range() {
        assert_eq!(
            RangeOrCell::Range {
                from: Position::RowRelative(15),
                to: Position::RowRelative(20),
            }.shift_up(5),
            RangeOrCell::Range {
                from: Position::RowRelative(10),
                to: Position::RowRelative(15),
            });
    }

    #[test]
    fn shift_up_cell() {
        assert_eq!(
            RangeOrCell::Cell(Position::Absolute(0, 100)).shift_up(10), 
            RangeOrCell::Cell(Position::Absolute(0, 90)));
    }
}

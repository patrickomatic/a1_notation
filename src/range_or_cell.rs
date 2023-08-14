//! # RangeOrCell
//!
//! Parsing and displaying a cell value (which can pretty much always be either a cell or a range).
//! 
use serde::{Serialize, Deserialize};
use std::fmt;
use std::str;
use crate::{A1, Error, Result};
use super::position::Position;

#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
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

impl RangeOrCell {
    pub fn column(&self) -> Option<Self> {
        match self {
            Self::Range { from, to } => 
                Some(Self::Range { from: from.column()?, to: to.column()? }),

            Self::Cell(p) => Some(Self::Cell(p.column()?)),
        }
    }

    pub fn row(&self) -> Option<Self> {
        match self {
            Self::Range { from, to } =>
                Some(Self::Range { from: from.row()?, to: to.row()? }),

            Self::Cell(p) => Some(Self::Cell(p.row()?)),
        }
    }

    pub fn shift_down(&self, rows: usize) -> Self {
        match self {
            Self::Range { from, to } =>
                Self::Range { from: from.shift_down(rows), to: to.shift_down(rows) },

            Self::Cell(p) => Self::Cell(p.shift_down(rows)),
        }
    }

    pub fn shift_left(&self, columns: usize) -> Self {
        match self {
            Self::Range { from, to } =>
                Self::Range { from: from.shift_left(columns), to: to.shift_left(columns) },

            Self::Cell(p) => Self::Cell(p.shift_left(columns)),
        }
    }

    pub fn shift_right(&self, columns: usize) -> Self {
        match self {
            Self::Range { from, to } =>
                Self::Range { from: from.shift_right(columns), to: to.shift_right(columns) },

            Self::Cell(p) => Self::Cell(p.shift_right(columns)),
        }
    }

    pub fn shift_up(&self, rows: usize) -> Self {
        match self {
            Self::Range { from, to } =>
                Self::Range { from: from.shift_up(rows), to: to.shift_up(rows) },

            Self::Cell(p) => Self::Cell(p.shift_up(rows)),
        }
    }

    pub fn with_x(&self, x: usize) -> Self {
        (match self {
            Self::Range { from, to } =>
                Self::Range { from: from.with_x(x), to: to.with_x(x) },

            Self::Cell(p) => Self::Cell(p.with_x(x)),
        }).simplify()
    }

    pub fn with_y(&self, y: usize) -> Self {
        (match self {
            Self::Range { from, to } =>
                Self::Range { from: from.with_y(y), to: to.with_y(y) },

            Self::Cell(p) => Self::Cell(p.with_y(y)),
        }).simplify()
    }

    /// Sometimes by manipulating a range with `with_x()`/`with_y()` you can end up with a range
    /// where `to` and `from` are the same.  In that case simplify it to just a cell
    fn simplify(&self) -> Self {
        if let Self::Range { from, to } = self {
            if from == to {
                // they're both the same, simplify to just one (it doesn't matter which since
                // they're the same)
                return Self::Cell(*from)
            }
        }

        *self
    }
}

impl fmt::Display for RangeOrCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Range { from, to } =>
                write!(f, "{}:{}", from.display_for_range(), to.display_for_range()),
            Self::Cell(p) =>
                write!(f, "{p}"),
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

/// We allow converting from a more specific type (RangeOrCell) to a more general one (A1) but 
/// it can't happen the other way around, so therefore we need to implement `Into` rather than
/// `From`
#[allow(clippy::from_over_into)]
impl Into<A1> for RangeOrCell {
    fn into(self) -> A1 {
        A1 { sheet_name: None, reference: self }
    }
}

#[cfg(test)] 
mod tests {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn column_none() {
        assert_eq!(None,
                   RangeOrCell::Cell(Position::RowRelative(5)).column());

        assert_eq!(None,
                   RangeOrCell::Range {
                       from: Position::RowRelative(5),
                       to: Position::RowRelative(10),
                   }.column());
    }

    #[test]
    fn column_some() {
        assert_eq!(Some(RangeOrCell::Cell(Position::ColumnRelative(5))),
                   RangeOrCell::Cell(Position::Absolute(5, 5)).column());
    }

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
    fn row_some() {
        assert_eq!(Some(RangeOrCell::Cell(Position::RowRelative(5))),
                   RangeOrCell::Cell(Position::Absolute(5, 5)).row());
    }

    #[test]
    fn row_none() {
        assert_eq!(None,
                   RangeOrCell::Cell(Position::ColumnRelative(5)).row());

        assert_eq!(None,
                   RangeOrCell::Range {
                       from: Position::ColumnRelative(5),
                       to: Position::ColumnRelative(10),
                   }.row());
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

    #[test]
    fn with_x_cell() {
        assert_eq!(
            RangeOrCell::Cell(Position::Absolute(0, 100)).with_x(10), 
            RangeOrCell::Cell(Position::Absolute(10, 100)));
    }

    #[test]
    fn with_x_range() {
        assert_eq!(
            RangeOrCell::Range {
                from: Position::RowRelative(0),
                to: Position::RowRelative(5),
            }.with_x(5),
            RangeOrCell::Range {
                from: Position::Absolute(5, 0),
                to: Position::Absolute(5, 5),
            });
    }

    #[test]
    fn with_x_range_simplify() {
        assert_eq!(
            RangeOrCell::Range {
                from: Position::ColumnRelative(0),
                to: Position::ColumnRelative(5),
            }.with_x(6),
            RangeOrCell::Cell(Position::ColumnRelative(6)));
    }

    #[test]
    fn with_y_cell() {
        assert_eq!(
            RangeOrCell::Cell(Position::Absolute(0, 100)).with_y(10), 
            RangeOrCell::Cell(Position::Absolute(0, 10)));
    }
    
    #[test]
    fn with_y_range() {
        assert_eq!(
            RangeOrCell::Range {
                from: Position::ColumnRelative(0),
                to: Position::ColumnRelative(5),
            }.with_y(6),
            RangeOrCell::Range {
                from: Position::Absolute(0, 6),
                to: Position::Absolute(5, 6),
            });
    }

    #[test]
    fn with_y_range_simplify() {
        assert_eq!(
            RangeOrCell::Range {
                from: Position::RowRelative(0),
                to: Position::RowRelative(5),
            }.with_y(6),
            RangeOrCell::Cell(Position::RowRelative(6)));
    }
}

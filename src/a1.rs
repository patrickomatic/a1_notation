//! # A1
//!
//! A position (location of a cell or range of cells) in a spreadsheet.  (0, 0) is the top left of
//! the spreadsheet. A lot of the logic here involves converting to and from A1-notation to a X/Y
//! based canonical representation.
//!
//! ### Links
//!
//! * [Google Sheets API Overview](https://developers.google.com/sheets/api/guides/concepts)
//! * [Refer to Cells and Ranges by Using A1 Notation](https://learn.microsoft.com/en-us/office/vba/excel/concepts/cells-and-ranges/refer-to-cells-and-ranges-by-using-a1-notation)
use serde::{Serialize, Deserialize};
use std::fmt;
use std::str;
use crate::{A1Builder, Error, Position, RangeOrCell, Result};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct A1 {
    pub sheet_name: Option<String>,
    pub reference: RangeOrCell,
}

impl A1 {
    /// Returns a builder that can be used to construct instances.
    pub fn builder() -> A1Builder {
        A1Builder::default()
    }

    pub fn with_sheet_name(self, sheet_name: &str) -> Self {
        Self {
            sheet_name: Some(sheet_name.to_owned()),
            ..self
        }
    }

    pub fn without_sheet_name(self) -> Self {
        Self {
            sheet_name: None,
            ..self
        }
    }

    /// Returns a new `A1` shifted downwards by `rows` rows.
    pub fn shift_down(self, rows: usize) -> Self {
        Self {
            sheet_name: self.sheet_name,
            reference: self.reference.shift_down(rows),
        }
    }

    /// Returns a new `A1` shifted left by `columns` columns.
    pub fn shift_left(self, columns: usize) -> Self {
        Self {
            sheet_name: self.sheet_name,
            reference: self.reference.shift_left(columns),
        }
    }

    /// Returns a new `A1` shifted right by `columns` columns.
    pub fn shift_right(self, columns: usize) -> Self {
        Self {
            sheet_name: self.sheet_name,
            reference: self.reference.shift_right(columns),
        }
    }

    /// Returns a new `A1` shifted up by `rows` rows.
    pub fn shift_up(self, rows: usize) -> Self {
        Self {
            sheet_name: self.sheet_name,
            reference: self.reference.shift_up(rows),
        }
    }

    /// The X component
    pub fn x(&self) -> Option<usize> {
        match self.cell_reference()? {
            Position::Absolute(x, _) | Position::ColumnRelative(x) => Some(x),
            _ => None,
        }
    }

    /// The Y component
    pub fn y(&self) -> Option<usize> {
        match self.cell_reference()? {
            Position::Absolute(_, y) | Position::RowRelative(y) => Some(y),
            _ => None,
        }
    }

    /// The X and Y components - only if both are set
    pub fn xy(&self) -> Option<(usize, usize)> {
        match self.cell_reference()? {
            Position::Absolute(x, y) => Some((x, y)),
            _ => None,
        }
    }

    fn cell_reference(&self) -> Option<Position> {
        match self.reference {
            RangeOrCell::Cell(p) => Some(p),
            _ => None,
        }
    }

    fn parse_sheet_name(a1: &str) -> Result<(Option<String>, &str)> {
        if let Some((sheet_name, rest)) = a1.split_once('!') {
            Ok((Some(sheet_name.to_string()), rest))
        } else {
            Ok((None, a1))
        }
    }
}

impl str::FromStr for A1 {
    type Err = Error;

    // TODO: 
    //
    // * handle commas? it might make it annoying to use this lib if the common cases is
    // 	 assuming a vector of ranges
    fn from_str(a1: &str) -> Result<Self> {
        let (sheet_name, rest) = Self::parse_sheet_name(a1)?;
        let reference = RangeOrCell::from_str(rest)?;

        Ok(A1 { sheet_name, reference })
    }
}

impl fmt::Display for A1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r = self.reference;
        if let Some(sheet_name) = &self.sheet_name {
            write!(f, "{sheet_name}!{r}")
        } else {
            write!(f, "{r}")
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn display() {
        let a1_ref = A1 {
            sheet_name: Some("Test1".to_string()),
            reference: RangeOrCell::Cell(Position::Absolute(1, 1)),
        };

        assert_eq!("Test1!B2", a1_ref.to_string());
    }

    #[test]
    fn display_without_sheet_name() {
        let a1_ref = A1 {
            sheet_name: None,
            reference: RangeOrCell::Cell(Position::Absolute(0, 0)),
        };

        assert_eq!("A1", a1_ref.to_string());
    }

    #[test]
    fn display_range() {
        let a1_ref = A1 {
            sheet_name: None,
            reference: RangeOrCell::Range {
                from: Position::ColumnRelative(1),
                to: Position::ColumnRelative(5),
            },
        };

        assert_eq!("B:F", a1_ref.to_string());
    }

    #[test]
    fn from_str() {
        let a1_ref = A1 {
            sheet_name: None,
            reference: RangeOrCell::Cell(Position::Absolute(0, 0)),
        };

        assert_eq!(a1_ref, A1::from_str("A1").unwrap());
    }

    #[test]
    fn from_str_sheet_name() {
        let a1_ref = A1 {
            sheet_name: Some("Foo".to_string()),
            reference: RangeOrCell::Cell(Position::Absolute(0, 0)),
        };

        assert_eq!(a1_ref, A1::from_str("Foo!A1").unwrap());
    }

    #[test]
    fn shift_down() {
        let a1_ref = A1 {
            sheet_name: Some("Test1".to_string()),
            reference: RangeOrCell::Cell(Position::Absolute(1, 1)),
        };

        assert_eq!("Test1!B3", a1_ref.shift_down(1).to_string());
    }

    #[test]
    fn shift_left() {
        let a1_ref = A1 {
            sheet_name: Some("Test1".to_string()),
            reference: RangeOrCell::Cell(Position::Absolute(1, 1)),
        };

        assert_eq!("Test1!A2", a1_ref.shift_left(1).to_string());
    }

    #[test]
    fn shift_right() {
        let a1_ref = A1 {
            sheet_name: None,
            reference: RangeOrCell::Cell(Position::Absolute(1, 1)),
        };

        assert_eq!("C2", a1_ref.shift_right(1).to_string());
    }

    #[test]
    fn shift_up() {
        let a1_ref = A1 {
            sheet_name: None,
            reference: RangeOrCell::Cell(Position::Absolute(1, 1)),
        };

        assert_eq!("B1", a1_ref.shift_up(1).to_string());
    }

    #[test]
    fn with_sheet_name() {
        let a1 = A1 {
            sheet_name: None,
            reference: RangeOrCell::Cell(Position::Absolute(1, 1)),
        };

        assert_eq!(Some("foo".to_string()), a1.with_sheet_name("foo").sheet_name);
    }

    #[test]
    fn without_sheet_name() {
        let a1 = A1 {
            sheet_name: Some("foo".to_string()),
            reference: RangeOrCell::Cell(Position::Absolute(1, 1)),
        };

        assert_eq!(None, a1.without_sheet_name().sheet_name);
    }
}

//! # Position
//!
use serde::{Serialize, Deserialize};
use std::fmt;
use std::str;
use crate::{Error, Result};

static ALPHA: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 
    'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Position {
    /// Absolute(x, y)
    ///
    /// * `x` - The column index
    /// * `y` - The row index
    Absolute(usize, usize),

    /// RowRelative(y)
    ///
    /// * `y` - The row index.  Starts at 0 being the very top row.
    RowRelative(usize),

    /// ColumnRelative(y)
    ///
    /// * `x` - The column index.  Starts at 0 being the left-most column.
    ColumnRelative(usize),
}

impl Position {
    /// When a relative reference is displayed within a range, the semantics are slightly different
    /// - for example when we print the column A by itself, it looks like:
    ///
    /// "A:A"
    ///
    /// however when it's part of a range, it's just "A":
    ///
    /// "A:C"
    pub fn display_for_range(&self) -> String {
        match self {
            Position::Absolute(_, _) => self.to_string(),
            Position::ColumnRelative(_) => self.a1_left(),
            Position::RowRelative(_) => self.a1_right(),
        }
    }

    pub fn shift_down(&self, rows: usize) -> Self {
        match self {
            Self::Absolute(x, y) => Self::Absolute(*x, y + rows),
            Self::ColumnRelative(_) => *self,
            Self::RowRelative(y) => Self::RowRelative(y + rows),
        }
    }

    pub fn shift_left(&self, columns: usize) -> Self {
        match self {
            Self::Absolute(x, y) => Self::Absolute(x.saturating_sub(columns), *y),
            Self::ColumnRelative(x) => Self::ColumnRelative(x.saturating_sub(columns)),
            Self::RowRelative(_) => *self,
        }
    }

    pub fn shift_right(&self, columns: usize) -> Self {
        match self {
            Self::Absolute(x, y) => Self::Absolute(x + columns, *y),
            Self::ColumnRelative(x) => Self::ColumnRelative(x + columns),
            Self::RowRelative(_) => *self,
        }
    }

    pub fn shift_up(&self, rows: usize) -> Self {
        match self {
            Self::Absolute(x, y) => Self::Absolute(*x, y.saturating_sub(rows)),
            Self::ColumnRelative(_) => *self,
            Self::RowRelative(y) => Self::RowRelative(y.saturating_sub(rows)),
        }
    }

    /// This function assumes that you've consumed the first part (the "A") of the A1 string and
    /// now we're just consuming the integer part
    fn parse_a1_y(a1: &str) -> Result<Option<usize>> {
        if !a1.ends_with(|c: char| c.is_ascii_digit()) {
            return Ok(None)
        };

        let n = match a1.parse::<usize>() {
            Ok(n) => n,
            Err(e) => return Err(Error::A1ParseError {
                bad_input: a1.to_owned(), 
                message: format!("Error parsing number part of A1 reference: {:?}", e),
            }),
        };

        if n < 1 {
            return Err(Error::A1ParseError {
                bad_input: n.to_string(),
                message: "A1 reference must be greater than 0".to_owned(),
            })
        }
        
        Ok(Some(n - 1))
    }

    fn parse_a1_x(a1: &str) -> Result<(Option<usize>, &str)> {
        if !a1.starts_with(|c: char| c.is_ascii_alphabetic()) {
            return Ok((None, a1))
        };

        let mut consumed = 0;
        let mut x = 0;
        for ch in a1.chars() {
            let uch = ch.to_ascii_uppercase();
            if let Some(ch_index) = ALPHA.iter().position(|&c| c == uch) {
                consumed += 1;
                x = x * 26 + ch_index + 1;
            } else if ch.is_ascii_digit() {
                break
            } else {
                return Err(Error::A1ParseError { 
                    bad_input: ch.to_string(), 
                    message: format!("Invalid character in A1 notation: {}", a1),
                })
            }
        }

        if consumed == 0 {
            Ok((None, a1))
        } else {
            Ok((Some(x - 1), &a1[consumed..]))
        }
    }

    /// Convert to the "A" part - 0 == 'A', 1 == 'B', etc.  we'll append to a string because
    /// if it's larger than 26, we'll have additional characters like AA1
    fn a1_left(&self) -> String {
        match self {
            Self::Absolute(x, _) | Self::ColumnRelative(x) => {
                let mut row_part = String::from("");
                let mut c = *x;
                
                loop {
                    row_part = format!("{}{}", ALPHA[c % 26], row_part);

                    let next_c = ((c as f64 / 26.0).floor() as isize) - 1;
                    if next_c < 0 {
                        break;
                    } 

                    c = next_c as usize;
                }

                row_part
            },
            Self::RowRelative(_) => self.a1_right(),
        }
    }

    /// This is the "1" part of "A1" which is easier because it's just our column index offset 
    /// by 1 instead of 0
    fn a1_right(&self) -> String {
        match self {
            Self::Absolute(_, y) | Self::RowRelative(y) => (y + 1).to_string(),
            Self::ColumnRelative(_) => self.a1_left(),
        }
    }
}

impl str::FromStr for Position {
    type Err = Error;

    fn from_str(a1: &str) -> Result<Self> {
        let (x, rest) = Self::parse_a1_x(a1)?;
        let y = Self::parse_a1_y(rest)?;

        if let Some(x) = x {
            if let Some(y) = y {
                Ok(Self::Absolute(x, y))
            } else {
                Ok(Self::ColumnRelative(x))
            }
        } else if let Some(y) = y {
            Ok(Self::RowRelative(y))
        } else {
            Err(Error::A1ParseError {
                bad_input: a1.to_owned(),
                message: "Error parsing A1 notation: could not determine a row or column".to_owned(),
            })
        }
    }
}

impl fmt::Display for Position {
    /// Converts a cell position to a String. The basic idea with A1 notation is that the row is
    /// represented by a letter A-Z and the column numerically, with the first position being `1`
    /// (not `0`).  So for example origin is `A1`:
    ///
    /// ```
    /// use a1_notation::Position;
    /// assert_eq!("A1", Position::Absolute(0, 0).to_string());
    /// ```
    ///
    /// And the position (1, 5) gives us `F2`. (F is the fifth letter, and 2 is the second cell
    /// when you start at 1):
    ///
    /// ```
    /// # use a1_notation::Position;
    /// assert_eq!("B6", Position::Absolute(1, 5).to_string());
    /// ```
    ///
    /// For relative cells we just have the alpha *or* numeric component:
    ///
    /// ```
    /// # use a1_notation::Position;
    /// assert_eq!("1:1", Position::RowRelative(0).to_string());
    /// assert_eq!("A:A", Position::ColumnRelative(0).to_string());
    /// ```
    ///
    /// yet another complication is once we get past column 26, we'll have to start stacking the 
    /// letters:
    /// ```
    /// # use a1_notation::Position;
    /// assert_eq!("Z1", Position::Absolute(25, 0).to_string());
    /// assert_eq!("AA1", Position::Absolute(26, 0).to_string());
    /// assert_eq!("AB1", Position::Absolute(27, 0).to_string());
    /// ```
    ///
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let left = self.a1_left();
        let right = self.a1_right();
        let separator = match self {
            Self::RowRelative(_) | Self::ColumnRelative(_) => ":",
            _ => "",
        };

        write!(f, "{left}{separator}{right}")
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn display_absolute() {
        assert_eq!("A1", Position::Absolute(0, 0).to_string());
        assert_eq!("C5", Position::Absolute(2, 4).to_string());
        assert_eq!("AY51", Position::Absolute(50, 50).to_string());
    }

    #[test]
    fn display_column_relative() {
        assert_eq!("A:A", Position::ColumnRelative(0).to_string());
        assert_eq!("AE:AE", Position::ColumnRelative(30).to_string());
    }

    #[test]
    fn display_row_relative() {
        assert_eq!("1:1", Position::RowRelative(0).to_string());
        assert_eq!("31:31", Position::RowRelative(30).to_string());
    }

    #[test]
    fn from_str_absolute() {
        assert_eq!(Position::Absolute(0, 0), Position::from_str("A1").unwrap());
        assert_eq!(Position::Absolute(50, 50), Position::from_str("AY51").unwrap());
    }
    
    #[test]
    fn from_str_column_relative() {
        assert_eq!(Position::ColumnRelative(0), Position::from_str("A").unwrap());
    }

    #[test]
    fn from_str_row_relative() {
        assert_eq!(Position::RowRelative(0), Position::from_str("1").unwrap());
    }

    #[test]
    fn from_str_invalid() {
        assert!(Position::from_str("").is_err());
        assert!(Position::from_str("/foo").is_err());
    }

    #[test]
    fn shift_down_absolute() {
        assert_eq!(Position::Absolute(2, 2).shift_down(1), Position::Absolute(2, 3));
        assert_eq!(Position::Absolute(2, 2).shift_down(10), Position::Absolute(2, 12));
    }

    #[test]
    fn shift_down_column_relative() {
        assert_eq!(Position::ColumnRelative(2).shift_down(1), Position::ColumnRelative(2));
    }

    #[test]
    fn shift_down_row_relative() {
        assert_eq!(Position::RowRelative(2).shift_down(1), Position::RowRelative(3));
        assert_eq!(Position::RowRelative(1).shift_down(100), Position::RowRelative(101));
    }

    #[test]
    fn shift_left_absolute() {
        assert_eq!(Position::Absolute(2, 2).shift_left(1), Position::Absolute(1, 2));
        assert_eq!(Position::Absolute(2, 2).shift_left(10), Position::Absolute(0, 2));
    }

    #[test]
    fn shift_left_column_relative() {
        assert_eq!(Position::ColumnRelative(2).shift_left(1), Position::ColumnRelative(1));
        assert_eq!(Position::ColumnRelative(2).shift_left(10), Position::ColumnRelative(0));
    }

    #[test]
    fn shift_left_row_relative() {
        assert_eq!(Position::RowRelative(2).shift_left(1), Position::RowRelative(2));
    }

    #[test]
    fn shift_right_absolute() {
        assert_eq!(Position::Absolute(2, 2).shift_right(1), Position::Absolute(3, 2));
        assert_eq!(Position::Absolute(2, 2).shift_right(10), Position::Absolute(12, 2));
    }

    #[test]
    fn shift_right_column_relative() {
        assert_eq!(Position::ColumnRelative(2).shift_right(1), Position::ColumnRelative(3));
    }

    #[test]
    fn shift_right_row_relative() {
        assert_eq!(Position::RowRelative(2).shift_right(1), Position::RowRelative(2));
    }

    #[test]
    fn shift_up_absolute() {
        assert_eq!(Position::Absolute(2, 2).shift_up(1), Position::Absolute(2, 1));
        assert_eq!(Position::Absolute(2, 2).shift_up(10), Position::Absolute(2, 0));
    }

    #[test]
    fn shift_up_column_relative() {
        assert_eq!(Position::ColumnRelative(2).shift_up(1), Position::ColumnRelative(2));
    }

    #[test]
    fn shift_up_row_relative() {
        assert_eq!(Position::RowRelative(2).shift_up(1), Position::RowRelative(1));
        assert_eq!(Position::RowRelative(2).shift_up(100), Position::RowRelative(0));
    }
}

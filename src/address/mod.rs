//! # Address
//! 
//! Represents a particular cell.  You treat an `Address` as any other type using the relevant
//! `AsRef` or `Into` implementations.
//!
use serde::{Serialize, Deserialize};
use crate::{Column, Index, Row};

mod as_ref;
mod display;
mod from_str;
mod into;

#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Address {
    pub column: Column,
    pub row: Row,
}

impl Address {
    pub fn new(column_index: Index, row_index: Index) -> Self {
        Self {
            column: column_index.into(),
            row: row_index.into(),
        }
    }

    /// Given that `a` and `b` form a finite range, is `self` within it? i.e. is_between `a` and
    /// `b`.
    pub fn is_between(&self, a: &Self, b: &Self) -> bool {
        // we "score" them by adding their X/Y coords so we can figure out which ones are closer to
        // origin and which is farther away.  The higher the score the farther it is from origin. 
        let a_score = a.origin_distance();
        let b_score = b.origin_distance();

        // the most top-leftest has the lowest score/closest to origin
        let top_left = if a_score < b_score { a } else { b };
        // the other one is bottom-right
        let bottom_right = if a_score < b_score { b } else { a };

        self.column >= top_left.column && self.row >= top_left.row
            && self.column <= bottom_right.column && self.row <= bottom_right.row
    }

    pub fn shift_down(&self, rows: Index) -> Self {
        Self { row: self.row.shift_down(rows), ..*self }
    }

    pub fn shift_left(&self, columns: Index) -> Self {
        Self { column: self.column.shift_left(columns), ..*self }
    }

    pub fn shift_right(&self, columns: Index) -> Self {
        Self { column: self.column.shift_right(columns), ..*self }
    }

    pub fn shift_up(&self, rows: Index) -> Self {
        Self { row: self.row.shift_up(rows), ..*self }
    }

    /// Set the `x` component with the following (hopefully sensical rules):
    pub fn with_x(&self, x: Index) -> Self {
        Self { column: x.into(), ..*self }
    }

    /// Set the `y` component with the following
    pub fn with_y(&self, y: Index) -> Self {
        Self { row: y.into(), ..*self }
    }

    pub(crate) fn origin_distance(&self) -> usize {
        self.column.x + self.row.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_between_true() {
        let a: Address = (5, 5).into();

        assert!(a.is_between(&(0, 0).into(), &(10, 10).into()));
        assert!(a.is_between(&(4, 5).into(), &(6, 5).into()));
    }

    #[test]
    fn is_between_false() {
        let a: Address = (5, 5).into();

        assert!(!a.is_between(&(8, 8).into(), &(10, 10).into()));
    }

    #[test]
    fn shift_down() {
        assert_eq!(Address::new(2, 2).shift_down(1), (2, 3).into());
        assert_eq!(Address::new(2, 2).shift_down(10), (2, 12).into());
    }

    #[test]
    fn shift_left() {
        assert_eq!(Address::new(2, 2).shift_left(1), (1, 2).into());
        assert_eq!(Address::new(2, 2).shift_left(10), (0, 2).into());
    }

    #[test]
    fn shift_right() {
        assert_eq!(Address::new(2, 2).shift_right(1), (3, 2).into());
        assert_eq!(Address::new(2, 2).shift_right(10), (12, 2).into());
    }

    #[test]
    fn shift_up() {
        assert_eq!(Address::new(2, 2).shift_up(1), (2, 1).into());
        assert_eq!(Address::new(2, 2).shift_up(10), (2, 0).into());
    }
}

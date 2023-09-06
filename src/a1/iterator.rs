use crate::A1;
use crate::range_or_cell::iterator::RangeOrCellIterator;
use std::iter;

pub struct A1Iterator<'a> {
    sheet_name: Option<String>,
    reference_iter: RangeOrCellIterator<'a>,
}

impl A1 {
    pub fn iter(&self) -> A1Iterator {
        A1Iterator {
            sheet_name: self.sheet_name.clone(),
            reference_iter: self.reference.iter(),
        }
    }
}

/// A thin wrapper around `RangeOrCellIterator` which also reflects the `sheet_name` of the `A1`.
impl<'a> iter::Iterator for A1Iterator<'a> {
    type Item = A1;
    
    // TODO: have this move the `sheet_name` around rather than clone()ing it each time
    fn next(&mut self) -> Option<Self::Item> {
        Some(A1 {
            sheet_name: self.sheet_name.clone(),
            reference: self.reference_iter.next()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn iter_no_sheet_name() {
        let a1 = range((0, 0), (0, 4));

        assert_eq!(
            a1.iter().map(|a| a.to_string()).collect::<Vec<_>>(),
            vec!["A1", "A2", "A3", "A4", "A5"]);
    }

    #[test]
    fn iter_with_sheet_name() {
        let a1 = range((0, 0), (0, 4)).with_sheet_name("Foo");

        assert_eq!(
            a1.iter().map(|a| a.to_string()).collect::<Vec<_>>(),
            vec!["Foo!A1", "Foo!A2", "Foo!A3", "Foo!A4", "Foo!A5"]);
    }
}

use crate::A1;
use std::fmt;

impl fmt::Display for A1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r = &self.reference;
        if let Some(sheet_name) = &self.sheet_name {
            write!(f, "{sheet_name}!{r}")
        } else {
            write!(f, "{r}")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn display() {
        let a1 = A1 {
            sheet_name: Some("Test1".to_string()),
            reference: RangeOrCell::Cell((1, 1).into()),
        };

        assert_eq!("Test1!B2", a1.to_string());
    }

    #[test]
    fn display_without_sheet_name() {
        let a1 = A1 {
            sheet_name: None,
            reference: RangeOrCell::Cell((0, 0).into()),
        };

        assert_eq!("A1", a1.to_string());
    }

    #[test]
    fn display_range() {
        let a1 = A1 {
            sheet_name: None,
            reference: RangeOrCell::ColumnRange {
                from: 1.into(),
                to: 5.into(),
            },
        };

        assert_eq!("B:F", a1.to_string());
    }
}

use crate::{A1, Error, RangeOrCell, Result};
use std::str;

fn parse_sheet_name(a1: &str) -> Result<(Option<String>, &str)> {
    if let Some((sheet_name, rest)) = a1.split_once('!') {
        Ok((Some(sheet_name.to_string()), rest))
    } else {
        Ok((None, a1))
    }
}

impl str::FromStr for A1 {
    type Err = Error;

    fn from_str(a1: &str) -> Result<Self> {
        let (sheet_name, rest) = parse_sheet_name(a1)?;
        let reference = RangeOrCell::from_str(rest)?;

        Ok(A1 { sheet_name, reference })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn from_str() {
        let a1 = A1 {
            sheet_name: None,
            reference: RangeOrCell::Cell((0, 0).into()),
        };

        assert_eq!(a1, A1::from_str("A1").unwrap());
    }

    #[test]
    fn from_str_sheet_name() {
        let a1 = A1 {
            sheet_name: Some("Foo".to_string()),
            reference: RangeOrCell::Cell((0, 0).into()),
        };

        assert_eq!(a1, A1::from_str("Foo!A1").unwrap());
    }
}

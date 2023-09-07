//! We allow converting from a more specific type (Position) to a more general one (A1) but it 
//! can't happen the other way around, so therefore we need to implement `Into` rather than
//! `From`
use crate::{A1, Address, Column, RangeOrCell, Row};

#[allow(clippy::from_over_into)]
impl Into<A1> for Address {
    fn into(self) -> A1 {
        A1 { sheet_name: None, reference: self.into() }
    }
}

#[allow(clippy::from_over_into)]
impl Into<Column> for Address {
    fn into(self) -> Column {
        self.column
    }
}

#[allow(clippy::from_over_into)]
impl Into<Row> for Address {
    fn into(self) -> Row {
        self.row
    }
}

#[allow(clippy::from_over_into)]
impl Into<RangeOrCell> for Address {
    fn into(self) -> RangeOrCell {
        RangeOrCell::Cell(self)
    }
}

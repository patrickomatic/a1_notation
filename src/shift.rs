//! # Shift
//!
//! A trait which allows for references to be shifted around.  They should always allocate and
//! return new references.  Since references are composed (absolute cells and ranges) this will
//! have to be implemented by all structs.
pub trait Shift {
    /// Shift the reference down by `rows`
    fn shift_down(&self, rows: usize) -> Self;

    /// Shift the reference left by `columns`
    fn shift_left(&self, columns: usize) -> Self;

    /// Shift the reference right by `columns`
    fn shift_right(&self, columns: usize) -> Self;

    /// Shift the reference up by `rows`
    fn shift_up(&self, rows: usize) -> Self;
}

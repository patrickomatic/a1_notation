//! # Error

#[derive(Clone, Debug)]
pub enum Error {
    /// # A1ParseError
    ///
    /// An error encountered by parsing a String into A1 format.
    ///
    /// * `bad_input` - The offending input that could not be parsed.
    /// * `message` - A relevant error message.
    A1ParseError { 
        bad_input: String,
        message: String,
    },
}

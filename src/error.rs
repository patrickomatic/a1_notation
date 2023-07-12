//! # Error

#[derive(Clone, Debug)]
pub enum Error {
    /// # A1BuilderError 
    /// 
    /// Thrown when the A1Builder is called in an invalid configuration.
    ///
    /// * `message` - The relevant error message.
    A1BuilderError(String),

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

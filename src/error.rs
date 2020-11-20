use thiserror::Error;

/// An error occurred while parsing an hexadecimal string to an id.
#[derive(Error, Debug)]
pub enum ParseError {
    /// The string representation of the id is not equal to 32 characters
    /// (after removing the dashes).
    #[error("Id length is expected to be 32 characters")]
    InvalidLength,
    /// One or many characters are invalid.
    #[error("Invalid chars")]
    InvalidChars,
    /// The provided validate function returned false.
    #[error("Invalid")]
    Invalid,
}

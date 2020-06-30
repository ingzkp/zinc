//!
//! The Zinc Schnorr signature tool error.
//!

use std::io;

use failure::Fail;

///
/// The Zinc Schnorr signature tool error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// The file input or output error.
    #[fail(display = "input-output error: {}", _0)]
    IO(io::Error),

    /// The hexadecimal string decoding error.
    #[fail(display = "hex decoding error: {}", _0)]
    Hex(hex::FromHexError),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::IO(error)
    }
}

impl From<hex::FromHexError> for Error {
    fn from(error: hex::FromHexError) -> Self {
        Self::Hex(error)
    }
}

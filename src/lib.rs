//!
//! The Jab compiler library.
//!

mod lexical;
mod syntax;

pub use self::lexical::TokenStream as LexicalStream;
pub use self::syntax::CircuitProgram;

use failure::Fail;
use serde_derive::Serialize;

#[derive(Debug, Fail, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Error {
    #[fail(display = "lexical: {}", _0)]
    Lexical(lexical::Error),
    #[fail(display = "syntax: {}", _0)]
    Syntax(syntax::Error),
}

pub type CircuitResult = Result<CircuitProgram, Error>;

pub fn compile(input: Vec<u8>) -> CircuitResult {
    syntax::parse(LexicalStream::new(input))
}

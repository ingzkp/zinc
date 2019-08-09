//!
//! The word lexical parser.
//!

use std::convert::TryFrom;

use crate::lexical::Identifier;
use crate::lexical::IdentifierError;
use crate::lexical::Lexeme;

pub enum State {
    Start,
    Continue,
}

pub fn parse(bytes: &[u8]) -> (usize, Lexeme) {
    let mut state = State::Start;
    let mut size = 0;

    while let Some(byte) = bytes.get(size).copied() {
        match state {
            State::Start => {
                if !Identifier::can_start_with(byte) {
                    break;
                }
                state = State::Continue;
            }
            State::Continue => {
                if !Identifier::can_contain_since_index_1(byte) {
                    break;
                }
            }
        }

        size += 1;
    }

    let lexeme = match Identifier::try_from(&bytes[..size]) {
        Ok(identifier) => Lexeme::Identifier(identifier),
        Err(IdentifierError::IsKeyword(keyword)) => Lexeme::Keyword(keyword),
    };
    (size, lexeme)
}

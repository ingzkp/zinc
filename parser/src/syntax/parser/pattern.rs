//!
//! The pattern parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Literal;
use crate::syntax::Pattern;
use crate::syntax::PatternBuilder;
use crate::Error;
use crate::Identifier;

#[derive(Default)]
pub struct Parser {
    builder: PatternBuilder,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<Pattern, Error> {
        match match initial.take() {
            Some(token) => token,
            None => stream.borrow_mut().next()?,
        } {
            Token {
                lexeme: Lexeme::Literal(literal),
                location,
            } => {
                self.builder.set_location(location);
                self.builder.set_literal(Literal::new(location, literal));
                Ok(self.builder.finish())
            }
            Token {
                lexeme: Lexeme::Identifier(identifier),
                location,
            } => {
                self.builder.set_location(location);
                self.builder
                    .set_binding(Identifier::new(location, identifier.name));
                Ok(self.builder.finish())
            }
            Token {
                lexeme: Lexeme::Symbol(Symbol::Underscore),
                location,
            } => {
                self.builder.set_location(location);
                self.builder.set_ignoring();
                Ok(self.builder.finish())
            }
            Token { lexeme, location } => Err(Error::Syntax(SyntaxError::Expected(
                location,
                vec!["{integer}", "{identifier}", "_"],
                lexeme,
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::Parser;
    use crate::lexical;
    use crate::lexical::BooleanLiteral;
    use crate::lexical::IntegerLiteral;
    use crate::lexical::Location;
    use crate::lexical::TokenStream;
    use crate::syntax::Identifier;
    use crate::syntax::Literal;
    use crate::syntax::Pattern;
    use crate::syntax::PatternVariant;

    #[test]
    fn ok_literal_boolean() {
        let input = "true";

        let expected = Ok(Pattern::new(
            Location::new(1, 1),
            PatternVariant::Literal(Literal::new(
                Location::new(1, 1),
                lexical::Literal::Boolean(BooleanLiteral::True),
            )),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_literal_integer() {
        let input = "42";

        let expected = Ok(Pattern::new(
            Location::new(1, 1),
            PatternVariant::Literal(Literal::new(
                Location::new(1, 1),
                lexical::Literal::Integer(IntegerLiteral::new_decimal("42".to_owned())),
            )),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_binding() {
        let input = "value";

        let expected = Ok(Pattern::new(
            Location::new(1, 1),
            PatternVariant::Binding(Identifier::new(Location::new(1, 1), "value".to_owned())),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_ignoring() {
        let input = "_";

        let expected = Ok(Pattern::new(Location::new(1, 1), PatternVariant::Ignoring));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }
}

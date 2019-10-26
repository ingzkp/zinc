//!
//! The mod statement parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::PathExpressionParser;
use crate::syntax::UseStatement;
use crate::syntax::UseStatementBuilder;
use crate::Error;

#[derive(Default)]
pub struct Parser {
    builder: UseStatementBuilder,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(UseStatement, Option<Token>), Error> {
        match match initial.take() {
            Some(token) => token,
            None => stream.borrow_mut().next()?,
        } {
            Token {
                lexeme: Lexeme::Keyword(Keyword::Use),
                location,
            } => {
                self.builder.set_location(location);
            }
            Token { lexeme, location } => {
                return Err(Error::Syntax(SyntaxError::Expected(
                    location,
                    vec!["use"],
                    lexeme,
                )));
            }
        }

        let (path, next) = PathExpressionParser::default().parse(stream, None)?;
        self.builder.set_path(path);
        Ok((self.builder.finish(), next))
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::Parser;
    use crate::lexical::Lexeme;
    use crate::lexical::Location;
    use crate::lexical::Symbol;
    use crate::lexical::Token;
    use crate::lexical::TokenStream;
    use crate::syntax::Expression;
    use crate::syntax::ExpressionElement;
    use crate::syntax::ExpressionObject;
    use crate::syntax::ExpressionOperand;
    use crate::syntax::ExpressionOperator;
    use crate::syntax::Identifier;
    use crate::syntax::UseStatement;

    #[test]
    fn ok() {
        let input = r#"use mega::ultra::namespace;"#;

        let expected = Ok((
            UseStatement::new(
                Location::new(1, 1),
                Expression::new(
                    Location::new(1, 5),
                    vec![
                        ExpressionElement::new(
                            Location::new(1, 5),
                            ExpressionObject::Operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::new(1, 5), "mega".to_owned()),
                            )),
                        ),
                        ExpressionElement::new(
                            Location::new(1, 11),
                            ExpressionObject::Operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::new(1, 11), "ultra".to_owned()),
                            )),
                        ),
                        ExpressionElement::new(
                            Location::new(1, 9),
                            ExpressionObject::Operator(ExpressionOperator::Path),
                        ),
                        ExpressionElement::new(
                            Location::new(1, 18),
                            ExpressionObject::Operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::new(1, 18), "namespace".to_owned()),
                            )),
                        ),
                        ExpressionElement::new(
                            Location::new(1, 16),
                            ExpressionObject::Operator(ExpressionOperator::Path),
                        ),
                    ],
                ),
            ),
            Some(Token::new(
                Lexeme::Symbol(Symbol::Semicolon),
                Location::new(1, 27),
            )),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }
}

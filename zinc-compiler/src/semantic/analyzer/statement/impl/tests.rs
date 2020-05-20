//!
//! The `impl` statement tests.
//!

#![cfg(test)]

use crate::error::Error;
use crate::lexical::token::location::Location;
use crate::semantic::analyzer::statement::error::Error as StatementError;
use crate::semantic::analyzer::statement::r#impl::error::Error as ImplStatementError;
use crate::semantic::error::Error as SemanticError;

#[test]
fn ok_structure_constructor() {
    let input = r#"
struct Data {
    value: u8,
}

impl Data {
    fn new(value: u8) -> Self {
        Self {
            value: value,
        }
    }
}

fn main() -> Data {
    Data::new(42)
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_enumeration_constructor() {
    let input = r#"
enum List {
    VALUE = 42,
}

impl List {
    fn default() -> Self {
        Self::VALUE
    }
}

fn main() -> List {
    List::default()
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn error_expected_namespace() {
    let input = r#"
type X = field;

impl X {
    fn impossible() {}
}

fn main() {}
"#;

    let expected = Err(Error::Semantic(SemanticError::Statement(
        StatementError::Impl(ImplStatementError::ExpectedStructureOrEnumeration {
            location: Location::new(4, 6),
            found: "X".to_owned(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
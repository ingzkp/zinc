//!
//! The semantic analyzer element error.
//!

use failure::Fail;

use crate::semantic::ConstantError;
use crate::semantic::PlaceError;
use crate::semantic::ValueError;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "place: {}", _0)]
    Place(PlaceError),
    #[fail(display = "value: {}", _0)]
    Value(ValueError),
    #[fail(display = "constant: {}", _0)]
    Constant(ConstantError),

    #[fail(
        display = "'=' operator expected a place expression as the first operand, but got '{}'",
        _0
    )]
    OperatorAssignmentFirstOperandExpectedPlace(String),
    #[fail(
        display = "'=' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorAssignmentSecondOperandExpectedEvaluable(String),

    #[fail(
        display = "'||' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorOrFirstOperandExpectedEvaluable(String),
    #[fail(
        display = "'||' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorOrSecondOperandExpectedEvaluable(String),

    #[fail(
        display = "'^^' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorXorFirstOperandExpectedEvaluable(String),
    #[fail(
        display = "'^^' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorXorSecondOperandExpectedEvaluable(String),

    #[fail(
        display = "'&&' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorAndFirstOperandExpectedEvaluable(String),
    #[fail(
        display = "'&&' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorAndSecondOperandExpectedEvaluable(String),

    #[fail(
        display = "'==' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorEqualsFirstOperandExpectedEvaluable(String),
    #[fail(
        display = "'==' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorEqualsSecondOperandExpectedEvaluable(String),

    #[fail(
        display = "'!=' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorNotEqualsFirstOperandExpectedEvaluable(String),
    #[fail(
        display = "'!=' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorNotEqualsSecondOperandExpectedEvaluable(String),

    #[fail(
        display = "'>=' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorGreaterEqualsFirstOperandExpectedEvaluable(String),
    #[fail(
        display = "'>=' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorGreaterEqualsSecondOperandExpectedEvaluable(String),

    #[fail(
        display = "'<=' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorLesserEqualsFirstOperandExpectedEvaluable(String),
    #[fail(
        display = "'<=' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorLesserEqualsSecondOperandExpectedEvaluable(String),

    #[fail(
        display = "'>' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorGreaterFirstOperandExpectedEvaluable(String),
    #[fail(
        display = "'>' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorGreaterSecondOperandExpectedEvaluable(String),

    #[fail(
        display = "'<' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorLesserFirstOperandExpectedEvaluable(String),
    #[fail(
        display = "'<' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorLesserSecondOperandExpectedEvaluable(String),

    #[fail(
        display = "'+' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorAdditionFirstOperandExpectedEvaluable(String),
    #[fail(
        display = "'+' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorAdditionSecondOperandExpectedEvaluable(String),

    #[fail(
        display = "'-' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorSubtractionFirstOperandExpectedEvaluable(String),
    #[fail(
        display = "'-' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorSubtractionSecondOperandExpectedEvaluable(String),

    #[fail(
        display = "'*' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorMultiplicationFirstOperandExpectedEvaluable(String),
    #[fail(
        display = "'*' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorMultiplicationSecondOperandExpectedEvaluable(String),

    #[fail(
        display = "'/' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorDivisionFirstOperandExpectedEvaluable(String),
    #[fail(
        display = "'/' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorDivisionSecondOperandExpectedEvaluable(String),

    #[fail(
        display = "'%' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorRemainderFirstOperandExpectedEvaluable(String),
    #[fail(
        display = "'%' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorRemainderSecondOperandExpectedEvaluable(String),

    #[fail(
        display = "'as' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorCastingFirstOperandExpectedEvaluable(String),
    #[fail(
        display = "'as' operator expected a type expression as the second operand, but got '{}'",
        _0
    )]
    OperatorCastingSecondOperandExpectedType(String),

    #[fail(
        display = "unary '-' operator expected an evaluable expression as the operand, but got '{}'",
        _0
    )]
    OperatorNegationExpectedEvaluable(String),
    #[fail(
        display = "'!' operator expected an evaluable expression as the operand, but got '{}'",
        _0
    )]
    OperatorNotExpectedEvaluable(String),

    #[fail(
        display = "'[]' operator expected a place expression as the first operand, but got '{}'",
        _0
    )]
    OperatorIndexFirstOperandExpectedPlace(String),
    #[fail(
        display = "'[]' operator expected a constant expression as the second operand, but got '{}'",
        _0
    )]
    OperatorIndexSecondOperandExpectedInteger(String),

    #[fail(
        display = "'.' operator expected a place expression as the first operand, but got '{}'",
        _0
    )]
    OperatorFieldFirstOperandExpectedPlace(String),
    #[fail(
        display = "'.' operator expected a member identifier as the second operand, but got '{}'",
        _0
    )]
    OperatorFieldSecondOperandExpectedMember(String),

    #[fail(
        display = "'::' operator expected a place expression as the first operand, but got '{}'",
        _0
    )]
    OperatorPathFirstOperandExpectedPlace(String),
    #[fail(
        display = "'::' operator expected a member string as the second operand, but got '{}'",
        _0
    )]
    OperatorPathSecondOperandExpectedMemberString(String),
}

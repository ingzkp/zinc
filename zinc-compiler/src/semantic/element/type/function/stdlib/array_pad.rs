//!
//! The semantic analyzer standard library `std::array::pad` function element.
//!

use std::fmt;
use std::ops::Deref;

use zinc_bytecode::builtins::BuiltinIdentifier;

use crate::semantic::element::constant::Constant;
use crate::semantic::element::r#type::function::error::Error;
use crate::semantic::element::r#type::function::stdlib::error::Error as StdlibError;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;

#[derive(Debug, Clone)]
pub struct Function {
    builtin_identifier: BuiltinIdentifier,
    identifier: &'static str,
}

impl Function {
    pub const ARGUMENT_INDEX_ARRAY: usize = 0;
    pub const ARGUMENT_INDEX_NEW_LENGTH: usize = 1;
    pub const ARGUMENT_INDEX_FILL_VALUE: usize = 2;
    pub const ARGUMENT_COUNT: usize = 3;

    pub fn new(builtin_identifier: BuiltinIdentifier) -> Self {
        Self {
            builtin_identifier,
            identifier: "pad",
        }
    }

    pub fn identifier(&self) -> &'static str {
        self.identifier
    }

    pub fn builtin_identifier(&self) -> BuiltinIdentifier {
        self.builtin_identifier
    }

    pub fn call(self, actual_elements: Vec<Element>) -> Result<Type, Error> {
        let mut actual_params = Vec::with_capacity(actual_elements.len());
        for (index, element) in actual_elements.into_iter().enumerate() {
            let (r#type, is_constant, number) = match element {
                Element::Value(value) => (value.r#type(), false, None),
                Element::Constant(Constant::Integer(integer)) => (
                    integer.r#type(),
                    true,
                    integer
                        .to_usize()
                        .map(Option::Some)
                        .map_err(|_error| {
                            StdlibError::array_new_length_invalid(integer.to_string())
                        })
                        .map_err(Error::StandardLibrary)?,
                ),
                Element::Constant(constant) => (constant.r#type(), true, None),
                element => {
                    return Err(Error::argument_not_evaluable(
                        self.identifier.to_owned(),
                        index + 1,
                        element.to_string(),
                    ))
                }
            };
            actual_params.push((r#type, is_constant, number));
        }

        let (input_array_type, input_array_size) =
            match actual_params.get(Self::ARGUMENT_INDEX_ARRAY) {
                Some((Type::Array { r#type, size }, _is_constant, _is_number))
                    if r#type.is_scalar() =>
                {
                    (r#type.deref().to_owned(), *size)
                }
                Some((r#type, _is_constant, _is_number)) => {
                    return Err(Error::argument_type(
                        self.identifier.to_owned(),
                        "array".to_owned(),
                        Self::ARGUMENT_INDEX_ARRAY + 1,
                        "[{scalar}; N]".to_owned(),
                        r#type.to_string(),
                    ))
                }
                None => {
                    return Err(Error::argument_count(
                        self.identifier.to_owned(),
                        Self::ARGUMENT_COUNT,
                        actual_params.len(),
                    ))
                }
            };

        let new_length = match actual_params.get(Self::ARGUMENT_INDEX_NEW_LENGTH) {
            Some((r#type, true, Some(number))) if r#type.is_scalar_unsigned() => *number,
            Some((r#type, true, _number)) => {
                return Err(Error::argument_type(
                    self.identifier.to_owned(),
                    "new_length".to_owned(),
                    Self::ARGUMENT_INDEX_NEW_LENGTH + 1,
                    "{unsigned integer}".to_owned(),
                    r#type.to_string(),
                ))
            }
            Some((r#type, false, _number)) => {
                return Err(Error::argument_constantness(
                    self.identifier.to_owned(),
                    "new_length".to_owned(),
                    Self::ARGUMENT_INDEX_NEW_LENGTH + 1,
                    r#type.to_string(),
                ))
            }
            None => {
                return Err(Error::argument_count(
                    self.identifier.to_owned(),
                    Self::ARGUMENT_COUNT,
                    actual_params.len(),
                ))
            }
        };

        match actual_params.get(Self::ARGUMENT_INDEX_FILL_VALUE) {
            Some((r#type, _is_constant, _number))
                if r#type.is_scalar() && r#type == &input_array_type => {}
            Some((r#type, _is_constant, _number)) => {
                return Err(Error::argument_type(
                    self.identifier.to_owned(),
                    "fill_value".to_owned(),
                    Self::ARGUMENT_INDEX_FILL_VALUE + 1,
                    input_array_type.to_string(),
                    r#type.to_string(),
                ))
            }
            None => {
                return Err(Error::argument_count(
                    self.identifier.to_owned(),
                    Self::ARGUMENT_COUNT,
                    actual_params.len(),
                ))
            }
        }

        if actual_params.len() > Self::ARGUMENT_COUNT {
            return Err(Error::argument_count(
                self.identifier.to_owned(),
                Self::ARGUMENT_COUNT,
                actual_params.len(),
            ));
        }

        if new_length < input_array_size {
            return Err(Error::StandardLibrary(
                StdlibError::array_padding_to_lesser_size(input_array_size, new_length),
            ));
        }

        Ok(Type::array(input_array_type, new_length))
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "fn std::array::{}(array: [T; N], new_length: M, fill_value: T) -> [T; M]",
            self.identifier,
        )
    }
}

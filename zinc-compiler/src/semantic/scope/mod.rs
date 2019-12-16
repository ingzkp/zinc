//!
//! The semantic analyzer scope.
//!

mod error;
mod item;

pub use self::error::Error;
pub use self::item::Item;
pub use self::item::Static as StaticItem;
pub use self::item::Variable as VariableItem;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::str;

use crate::semantic::Constant;
use crate::semantic::Error as SemanticError;
use crate::semantic::Place;
use crate::semantic::PlaceDescriptor;
use crate::semantic::Type;
use crate::semantic::Type as TypeItem;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Scope {
    parent: Option<Rc<RefCell<Self>>>,
    items: HashMap<String, Item>,
}

impl Scope {
    pub fn new(parent: Option<Rc<RefCell<Self>>>) -> Self {
        Self {
            parent,
            ..Default::default()
        }
    }

    pub fn declare_variable(
        &mut self,
        identifier: String,
        variable: VariableItem,
    ) -> Result<(), Error> {
        if self.is_item_declared(&identifier) {
            return Err(Error::ItemRedeclared(identifier));
        }
        self.items.insert(identifier, Item::Variable(variable));
        Ok(())
    }

    pub fn declare_constant(
        &mut self,
        identifier: String,
        constant: Constant,
    ) -> Result<(), Error> {
        if self.is_item_declared(&identifier) {
            return Err(Error::ItemRedeclared(identifier));
        }
        self.items.insert(identifier, Item::Constant(constant));
        Ok(())
    }

    pub fn declare_static(
        &mut self,
        identifier: String,
        r#static: StaticItem,
    ) -> Result<(), Error> {
        if self.is_item_declared(&identifier) {
            return Err(Error::ItemRedeclared(identifier));
        }
        self.items.insert(identifier, Item::Static(r#static));
        Ok(())
    }

    pub fn declare_type(&mut self, identifier: String, r#type: TypeItem) -> Result<(), Error> {
        if self.is_item_declared(&identifier) {
            return Err(Error::ItemRedeclared(identifier));
        }
        self.items.insert(identifier, Item::Type(r#type));
        Ok(())
    }

    pub fn declare_module(
        &mut self,
        identifier: String,
        scope: Rc<RefCell<Scope>>,
    ) -> Result<(), Error> {
        if self.is_item_declared(&identifier) {
            return Err(Error::ItemRedeclared(identifier));
        }
        self.items.insert(identifier, Item::Module(scope));
        Ok(())
    }

    pub fn resolve_place(
        mut scope: Rc<RefCell<Scope>>,
        place: &Place,
    ) -> Result<Item, SemanticError> {
        let mut result = Err(SemanticError::Scope(
            place.location,
            Error::ItemUndeclared(place.to_string()),
        ));
        for identifier in place.path.iter() {
            result = Ok(
                match Self::resolve_item(scope.clone(), &identifier.name)
                    .map_err(|error| SemanticError::Scope(identifier.location, error))?
                {
                    Item::Module(module) => {
                        scope = module.clone();
                        Item::Module(module)
                    }
                    Item::Type(Type::Enumeration {
                        identifier,
                        scope: enum_scope,
                    }) => {
                        scope = Rc::new(RefCell::new(enum_scope.clone()));
                        Item::Type(Type::Enumeration {
                            identifier,
                            scope: enum_scope,
                        })
                    }
                    Item::Variable(mut variable) => {
                        if place.is_static {
                            for descriptor in place.descriptors.iter() {
                                match (descriptor, &variable.r#type) {
                                    (
                                        PlaceDescriptor::ArrayIndexConstant(constant),
                                        Type::Array {
                                            r#type: array_element_type,
                                            size: array_size,
                                        },
                                    ) => {
                                        let array_size = *array_size;
                                        let array_index = constant.to_usize().map_err(|error| {
                                            SemanticError::InferenceConstant(place.location, error)
                                        })?;
                                        if array_index >= array_size {
                                            return Err(SemanticError::Scope(
                                                place.location,
                                                Error::ArrayIndexOutOfRange(
                                                    array_index,
                                                    variable.r#type.to_string(),
                                                ),
                                            ));
                                        }
                                        variable.address += array_index * array_element_type.size();
                                        variable.r#type = *array_element_type.to_owned();
                                    }
                                    (
                                        PlaceDescriptor::TupleField(tuple_field),
                                        Type::Tuple { types: tuple_types },
                                    ) => {
                                        let tuple_field = *tuple_field;
                                        if tuple_field >= tuple_types.len() {
                                            return Err(SemanticError::Scope(
                                                place.location,
                                                Error::TupleFieldDoesNotExist(
                                                    tuple_field,
                                                    variable.r#type.to_string(),
                                                ),
                                            ));
                                        }
                                        for _tuple_field_index in 0..tuple_field {
                                            variable.address += tuple_types[tuple_field].size();
                                        }
                                        variable.r#type = tuple_types[tuple_field].to_owned();
                                    }
                                    (
                                        PlaceDescriptor::StructureField(structure_field),
                                        Type::Structure { fields, .. },
                                    ) => {
                                        let mut found_type = None;
                                        for (field_name, field_type) in fields.iter() {
                                            if field_name == structure_field {
                                                found_type = Some(field_type);
                                                break;
                                            }
                                            variable.address += field_type.size();
                                        }
                                        match found_type.take() {
                                            Some(found_type) => {
                                                variable.r#type = found_type.to_owned()
                                            }
                                            None => {
                                                return Err(SemanticError::Scope(
                                                    place.location,
                                                    Error::StructureFieldDoesNotExist(
                                                        structure_field.to_owned(),
                                                        variable.r#type.to_string(),
                                                    ),
                                                ))
                                            }
                                        }
                                    }
                                    (descriptor, inner_type) => {
                                        return Err(SemanticError::Scope(
                                            place.location,
                                            Error::InvalidDescriptor(
                                                inner_type.to_string(),
                                                descriptor.to_owned(),
                                            ),
                                        ))
                                    }
                                }
                            }

                            Item::Variable(variable)
                        } else {
                            unimplemented!();
                        }
                    }
                    item => item,
                },
            );
        }
        result
    }

    pub fn resolve_item(scope: Rc<RefCell<Scope>>, identifier: &str) -> Result<Item, Error> {
        match scope.borrow().items.get(identifier) {
            Some(item) => Ok(item.to_owned()),
            None => match scope.borrow().parent {
                Some(ref parent) => Self::resolve_item(parent.to_owned(), identifier),
                None => Err(Error::ItemUndeclared(identifier.to_owned())),
            },
        }
    }

    pub fn is_item_declared(&self, identifier: &str) -> bool {
        if self.items.contains_key(identifier) {
            true
        } else {
            match self.parent {
                Some(ref parent) => parent.borrow().is_item_declared(identifier),
                None => false,
            }
        }
    }
}

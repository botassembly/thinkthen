//! Validated author declaration constructors use the ordinary six error kinds.
use super::Error;
use crate::core::declaration::{
    InputProperty, InputPropertyType, ObjectDeclaration, QuestionName, WordingVersion,
};

impl QuestionName {
    /// Validate a case-sensitive lowercase ASCII author name of at most 64 bytes.
    /// # Errors
    /// Returns Usage for an invalid name, without reading any file.
    pub fn new(value: &str) -> Result<Self, Error> {
        Self::validated(value).map_err(Error::refused)
    }
}
impl WordingVersion {
    /// Validate a wording version from 1 through 2147483647.
    /// # Errors
    /// Returns Usage outside the admitted range.
    pub fn new(value: u32) -> Result<Self, Error> {
        Self::validated(value).map_err(Error::refused)
    }
}
impl InputProperty {
    /// Declare one nonempty printable literal property name and its shape.
    /// # Errors
    /// Returns Usage for an empty name or a control character.
    pub fn new(name: &str, kind: InputPropertyType) -> Result<Self, Error> {
        Self::validated(name, kind).map_err(Error::refused)
    }
}
impl ObjectDeclaration {
    /// Declare ordered properties and distinct required names drawn from them.
    /// # Errors
    /// Returns Usage for duplicates or undeclared required names.
    pub fn new(properties: Vec<InputProperty>, required: Vec<String>) -> Result<Self, Error> {
        Self::validated(properties, required).map_err(Error::refused)
    }
}

//! Additive author/declaration descriptors; existing v1 layouts stay frozen.
use super::{OptionalStringV1, OptionalU64V1, StringV1, StringsV1};
/// C descriptor or borrowed view `InputPropertyV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct InputPropertyV1 {
    /// C field `name`.
    pub name: StringV1,
    /// C field `kind`.
    pub kind: u32,
}
/// C descriptor or borrowed view `InputPropertiesV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct InputPropertiesV1 {
    /// C field `data`.
    pub data: *const InputPropertyV1,
    /// C field `len`.
    pub len: usize,
}
/// C descriptor or borrowed view `InputDeclarationV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct InputDeclarationV1 {
    /// C field `kind`.
    pub kind: u32,
    /// C field `properties`.
    pub properties: InputPropertiesV1,
    /// C field `required`.
    pub required: StringsV1,
}
/// C descriptor or borrowed view `QuestionAuthorV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct QuestionAuthorV1 {
    /// C field `name`.
    pub name: OptionalStringV1,
    /// C field `wording_version`.
    pub wording_version: OptionalU64V1,
    /// C field `item_schema`.
    pub item_schema: InputDeclarationV1,
    /// C field `context_schema`.
    pub context_schema: InputDeclarationV1,
}

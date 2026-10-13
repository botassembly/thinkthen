//! Generated primitive ABI transport declarations. Do not edit.
#![allow(
    non_camel_case_types,
    missing_docs,
    reason = "native immutable ABI transport names"
)]
#![forbid(unsafe_code)]
/// C descriptor or borrowed view `StringV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct StringV1 {
    /// C field `data`.
    pub data: *const std::ffi::c_char,
    /// C field `len`.
    pub len: usize,
}
/// C descriptor or borrowed view `StringsV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct StringsV1 {
    /// C field `data`.
    pub data: *const StringV1,
    /// C field `len`.
    pub len: usize,
}
/// C descriptor or borrowed view `OptionalStringV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct OptionalStringV1 {
    /// C field `present`.
    pub present: std::ffi::c_int,
    /// C field `value`.
    pub value: StringV1,
}
/// C descriptor or borrowed view `OptionalU64V1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct OptionalU64V1 {
    /// C field `present`.
    pub present: std::ffi::c_int,
    /// C field `value`.
    pub value: u64,
}
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

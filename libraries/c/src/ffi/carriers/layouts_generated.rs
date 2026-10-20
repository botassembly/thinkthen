//! Generated primitive ABI transport declarations. Do not edit.
#![allow(
    non_camel_case_types,
    missing_docs,
    reason = "native immutable ABI transport names"
)]
#![forbid(unsafe_code)]
/// C descriptor or borrowed view `CountedString`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CountedString {
    /// C field `data`.
    pub data: *const std::ffi::c_char,
    /// C field `len`.
    pub len: usize,
}
/// C descriptor or borrowed view `Strings`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Strings {
    /// C field `data`.
    pub data: *const CountedString,
    /// C field `len`.
    pub len: usize,
}
/// C descriptor or borrowed view `OptionalString`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct OptionalString {
    /// C field `present`.
    pub present: std::ffi::c_int,
    /// C field `value`.
    pub value: CountedString,
}
/// C descriptor or borrowed view `OptionalU64`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct OptionalU64 {
    /// C field `present`.
    pub present: std::ffi::c_int,
    /// C field `value`.
    pub value: u64,
}
/// C descriptor or borrowed view `InputPropertyView`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct InputPropertyView {
    /// C field `name`.
    pub name: CountedString,
    /// C field `kind`.
    pub kind: u32,
}
/// C descriptor or borrowed view `InputProperties`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct InputProperties {
    /// C field `data`.
    pub data: *const InputPropertyView,
    /// C field `len`.
    pub len: usize,
}
/// C descriptor or borrowed view `InputDeclarationView`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct InputDeclarationView {
    /// C field `kind`.
    pub kind: u32,
    /// C field `properties`.
    pub properties: InputProperties,
    /// C field `required`.
    pub required: Strings,
}
/// C descriptor or borrowed view `QuestionAuthorInfo`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct QuestionAuthorInfo {
    /// C field `name`.
    pub name: OptionalString,
    /// C field `item_schema`.
    pub item_schema: InputDeclarationView,
    /// C field `context_schema`.
    pub context_schema: InputDeclarationView,
}

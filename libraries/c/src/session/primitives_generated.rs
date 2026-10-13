//! Generated primitive ABI transport declarations. Do not edit.
#![allow(
    non_camel_case_types,
    missing_docs,
    reason = "native immutable ABI transport names"
)]
#![forbid(unsafe_code)]
/// C descriptor or borrowed view `thinkthen_complete_utf8_v1`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_utf8_v1 {
    /// C field `data`.
    pub data: *const std::ffi::c_char,
    /// C field `len`.
    pub len: usize,
}
/// C descriptor or borrowed view `thinkthen_complete_extension_v1`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_extension_v1 {
    /// C field `name`.
    pub name: thinkthen_complete_utf8_v1,
    /// C field `json`.
    pub json: thinkthen_complete_utf8_v1,
    /// C field `value`.
    pub value: *const thinkthen_complete_json_v1,
}
/// C descriptor or borrowed view `thinkthen_complete_extensions_v1`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_extensions_v1 {
    /// C field `data`.
    pub data: *const thinkthen_complete_extension_v1,
    /// C field `len`.
    pub len: usize,
}
/// C descriptor or borrowed view `thinkthen_complete_json_entry_v1`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_json_entry_v1 {
    /// C field `name`.
    pub name: thinkthen_complete_utf8_v1,
    /// C field `value`.
    pub value: *const thinkthen_complete_json_v1,
}
/// C descriptor or borrowed view `thinkthen_complete_json_array_v1`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_json_array_v1 {
    /// C field `data`.
    pub data: *const *const thinkthen_complete_json_v1,
    /// C field `len`.
    pub len: usize,
}
/// C descriptor or borrowed view `thinkthen_complete_json_object_v1`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_json_object_v1 {
    /// C field `data`.
    pub data: *const thinkthen_complete_json_entry_v1,
    /// C field `len`.
    pub len: usize,
}
/// C descriptor or borrowed view `thinkthen_complete_json_data_v1`.
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_json_data_v1 {
    /// C field `boolean`.
    pub boolean: u32,
    /// C field `number`.
    pub number: thinkthen_complete_utf8_v1,
    /// C field `string`.
    pub string: thinkthen_complete_utf8_v1,
    /// C field `array`.
    pub array: thinkthen_complete_json_array_v1,
    /// C field `object`.
    pub object: thinkthen_complete_json_object_v1,
}
impl std::fmt::Debug for thinkthen_complete_json_data_v1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("JSON payload")
    }
}
/// C descriptor or borrowed view `thinkthen_complete_json_v1`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_json_v1 {
    /// C field `kind`.
    pub kind: u32,
    /// C field `data`.
    pub data: thinkthen_complete_json_data_v1,
}
pub const THINKTHEN_COMPLETE_JSON_NULL_V1: u32 = 1;
pub const THINKTHEN_COMPLETE_JSON_BOOLEAN_V1: u32 = 2;
pub const THINKTHEN_COMPLETE_JSON_NUMBER_V1: u32 = 3;
pub const THINKTHEN_COMPLETE_JSON_STRING_V1: u32 = 4;
pub const THINKTHEN_COMPLETE_JSON_ARRAY_V1: u32 = 5;
pub const THINKTHEN_COMPLETE_JSON_OBJECT_V1: u32 = 6;

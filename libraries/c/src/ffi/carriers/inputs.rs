//! Counted strings and optional values borrowed from owned author metadata.
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

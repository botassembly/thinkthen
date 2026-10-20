//! Canonical C values. Descriptor discriminators remain u32 so unknown host values are validated safely.
/// Version 0.2.0, the version of the library this header ships with.
/// Version 0.1.0 is the first release.
pub const THINKTHEN_VERSION_MAJOR: i32 = 0;
/// C value `THINKTHEN_VERSION_MINOR`.
pub const THINKTHEN_VERSION_MINOR: i32 = 2;
/// C value `THINKTHEN_VERSION_PATCH`.
pub const THINKTHEN_VERSION_PATCH: i32 = 0;
/// Exact C interface compatibility number. Compare with thinkthen_version at load.
/// Change this number whenever the C contract becomes incompatible.
pub const THINKTHEN_INTERFACE_NUMBER: u32 = 1;
/// Zero means success; codes 1..6 identify failures, never answers.
/// On failure every output stays unchanged; error_code, error_message,
/// error_retryable and error_facts_json describe the calling thread's failure.
pub const THINKTHEN_OK: i32 = 0;
/// the arguments/input broke the grammar; the rejected stage was not sent
pub const THINKTHEN_EUSAGE: i32 = 1;
/// the wire failed or refused
pub const THINKTHEN_EBACKEND: i32 = 2;
/// the caller's own budget ran out; no rows came
pub const THINKTHEN_EDEADLINE: i32 = 3;
/// a named local file, cache, or recording failed
pub const THINKTHEN_ELOCAL: i32 = 4;
/// the token fired; sent requests finished and no rows came
pub const THINKTHEN_ECANCELLED: i32 = 5;
/// the engine broke its own contract
pub const THINKTHEN_EDEFECT: i32 = 6;
/// _opts budgets are exact int64_t milliseconds from the call.
/// THINKTHEN_NO_DEADLINE (-1) sets none; zero returns EDEADLINE before sending.
/// Other negatives or values above 4294967295000 ms return EUSAGE before sending.
/// Positive budgets stop within one tick. Clamp elapsed computed budgets at
/// zero so an expired deadline never becomes the no-deadline sentinel.
pub const THINKTHEN_NO_DEADLINE: i64 = -1i64;
/// The three answers a yes-or-no question gives. UNSURE is the machine word;
/// the specification says "not sure" in prose.
pub const THINKTHEN_YES: i32 = 1;
/// C value `THINKTHEN_NO`.
pub const THINKTHEN_NO: i32 = 0;
/// C value `THINKTHEN_UNSURE`.
pub const THINKTHEN_UNSURE: i32 = 2;
/// Admitted native grammar values; descriptor storage remains uint32_t.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[allow(
    non_camel_case_types,
    reason = "preserve the public C enumerator names"
)]
pub enum InputDeclarationKind {
    /// C grammar value `THINKTHEN_DECLARATION_ABSENT`.
    THINKTHEN_DECLARATION_ABSENT = 0,
    /// C grammar value `THINKTHEN_DECLARATION_STRING`.
    THINKTHEN_DECLARATION_STRING = 1,
    /// C grammar value `THINKTHEN_DECLARATION_OBJECT`.
    THINKTHEN_DECLARATION_OBJECT = 2,
}
/// Admitted native grammar values; descriptor storage remains uint32_t.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[allow(
    non_camel_case_types,
    reason = "preserve the public C enumerator names"
)]
pub enum InputPropertyKind {
    /// C grammar value `THINKTHEN_PROPERTY_STRING`.
    THINKTHEN_PROPERTY_STRING = 1,
    /// C grammar value `THINKTHEN_PROPERTY_NUMBER`.
    THINKTHEN_PROPERTY_NUMBER = 2,
    /// C grammar value `THINKTHEN_PROPERTY_BOOLEAN`.
    THINKTHEN_PROPERTY_BOOLEAN = 3,
    /// C grammar value `THINKTHEN_PROPERTY_STRING_LIST`.
    THINKTHEN_PROPERTY_STRING_LIST = 4,
}
/// Admitted native grammar values; descriptor storage remains uint32_t.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[allow(
    non_camel_case_types,
    reason = "preserve the public C enumerator names"
)]
pub enum QuestionLoaderRole {
    /// C grammar value `THINKTHEN_LOAD_ATOMIC`.
    THINKTHEN_LOAD_ATOMIC = 1,
    /// C grammar value `THINKTHEN_LOAD_SET`.
    THINKTHEN_LOAD_SET = 2,
    /// C grammar value `THINKTHEN_LOAD_DYNAMIC_CHOOSE`.
    THINKTHEN_LOAD_DYNAMIC_CHOOSE = 3,
    /// C grammar value `THINKTHEN_LOAD_RECOGNIZE`.
    THINKTHEN_LOAD_RECOGNIZE = 4,
    /// C grammar value `THINKTHEN_LOAD_RELATE`.
    THINKTHEN_LOAD_RELATE = 5,
    /// C grammar value `THINKTHEN_LOAD_RANK`.
    THINKTHEN_LOAD_RANK = 6,
    /// C grammar value `THINKTHEN_LOAD_RANK_SET`.
    THINKTHEN_LOAD_RANK_SET = 7,
    /// C grammar value `THINKTHEN_LOAD_FIND`.
    THINKTHEN_LOAD_FIND = 8,
}

/// Validated uint32 descriptor spelling for `THINKTHEN_DECLARATION_STRING`.
pub(crate) const DECLARATION_STRING: u32 =
    InputDeclarationKind::THINKTHEN_DECLARATION_STRING as u32;

/// Validated uint32 descriptor spelling for `THINKTHEN_DECLARATION_OBJECT`.
pub(crate) const DECLARATION_OBJECT: u32 =
    InputDeclarationKind::THINKTHEN_DECLARATION_OBJECT as u32;

/// Validated uint32 descriptor spelling for `THINKTHEN_PROPERTY_STRING`.
pub(crate) const PROPERTY_STRING: u32 = InputPropertyKind::THINKTHEN_PROPERTY_STRING as u32;

/// Validated uint32 descriptor spelling for `THINKTHEN_PROPERTY_NUMBER`.
pub(crate) const PROPERTY_NUMBER: u32 = InputPropertyKind::THINKTHEN_PROPERTY_NUMBER as u32;

/// Validated uint32 descriptor spelling for `THINKTHEN_PROPERTY_BOOLEAN`.
pub(crate) const PROPERTY_BOOLEAN: u32 = InputPropertyKind::THINKTHEN_PROPERTY_BOOLEAN as u32;

/// Validated uint32 descriptor spelling for `THINKTHEN_PROPERTY_STRING_LIST`.
pub(crate) const PROPERTY_STRING_LIST: u32 =
    InputPropertyKind::THINKTHEN_PROPERTY_STRING_LIST as u32;

/// Validated uint32 descriptor spelling for `THINKTHEN_LOAD_ATOMIC`.
pub(crate) const LOAD_ATOMIC: u32 = QuestionLoaderRole::THINKTHEN_LOAD_ATOMIC as u32;

/// Validated uint32 descriptor spelling for `THINKTHEN_LOAD_SET`.
pub(crate) const LOAD_SET: u32 = QuestionLoaderRole::THINKTHEN_LOAD_SET as u32;

/// Validated uint32 descriptor spelling for `THINKTHEN_LOAD_DYNAMIC_CHOOSE`.
pub(crate) const LOAD_DYNAMIC_CHOOSE: u32 =
    QuestionLoaderRole::THINKTHEN_LOAD_DYNAMIC_CHOOSE as u32;

/// Validated uint32 descriptor spelling for `THINKTHEN_LOAD_RECOGNIZE`.
pub(crate) const LOAD_RECOGNIZE: u32 = QuestionLoaderRole::THINKTHEN_LOAD_RECOGNIZE as u32;

/// Validated uint32 descriptor spelling for `THINKTHEN_LOAD_RELATE`.
pub(crate) const LOAD_RELATE: u32 = QuestionLoaderRole::THINKTHEN_LOAD_RELATE as u32;

/// Validated uint32 descriptor spelling for `THINKTHEN_LOAD_RANK`.
pub(crate) const LOAD_RANK: u32 = QuestionLoaderRole::THINKTHEN_LOAD_RANK as u32;

/// Validated uint32 descriptor spelling for `THINKTHEN_LOAD_RANK_SET`.
pub(crate) const LOAD_RANK_SET: u32 = QuestionLoaderRole::THINKTHEN_LOAD_RANK_SET as u32;

/// Validated uint32 descriptor spelling for `THINKTHEN_LOAD_FIND`.
pub(crate) const LOAD_FIND: u32 = QuestionLoaderRole::THINKTHEN_LOAD_FIND as u32;

/// A counted COBOL scalar exceeds its documented representation capacity.
pub const THINKTHEN_COBOL_OVERFLOW: i32 = 1002;

//! Generated primitive ABI transport declarations. Do not edit.
#![allow(
    non_camel_case_types,
    missing_docs,
    reason = "native immutable ABI transport names"
)]
#![forbid(unsafe_code)]
/// C descriptor or borrowed view `thinkthen_error_details`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_error_details {
    /// C field `code`.
    pub code: std::ffi::c_int,
    /// C field `retryable`.
    pub retryable: std::ffi::c_int,
    /// C field `message`.
    pub message: thinkthen_complete_utf8,
    /// C field `facts`.
    pub facts: *const crate::session::views_generated::thinkthen_complete_immediate_facts,
}
/// C descriptor or borrowed view `thinkthen_complete_utf8`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_utf8 {
    /// C field `data`.
    pub data: *const std::ffi::c_char,
    /// C field `len`.
    pub len: usize,
}
/// C descriptor or borrowed view `thinkthen_complete_extension`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_extension {
    /// C field `name`.
    pub name: thinkthen_complete_utf8,
    /// C field `json`.
    pub json: thinkthen_complete_utf8,
    /// C field `value`.
    pub value: *const thinkthen_complete_json,
}
/// C descriptor or borrowed view `thinkthen_complete_extensions`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_extensions {
    /// C field `data`.
    pub data: *const thinkthen_complete_extension,
    /// C field `len`.
    pub len: usize,
}
/// C descriptor or borrowed view `thinkthen_complete_json_entry`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_json_entry {
    /// C field `name`.
    pub name: thinkthen_complete_utf8,
    /// C field `value`.
    pub value: *const thinkthen_complete_json,
}
/// C descriptor or borrowed view `thinkthen_complete_json_array`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_json_array {
    /// C field `data`.
    pub data: *const *const thinkthen_complete_json,
    /// C field `len`.
    pub len: usize,
}
/// C descriptor or borrowed view `thinkthen_complete_json_object`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_json_object {
    /// C field `data`.
    pub data: *const thinkthen_complete_json_entry,
    /// C field `len`.
    pub len: usize,
}
/// C descriptor or borrowed view `thinkthen_complete_json_data`.
#[repr(C)]
#[derive(Clone, Copy)]
pub union thinkthen_complete_json_data {
    /// C field `boolean`.
    pub boolean: u32,
    /// C field `number`.
    pub number: thinkthen_complete_utf8,
    /// C field `string`.
    pub string: thinkthen_complete_utf8,
    /// C field `array`.
    pub array: thinkthen_complete_json_array,
    /// C field `object`.
    pub object: thinkthen_complete_json_object,
}
impl std::fmt::Debug for thinkthen_complete_json_data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("JSON payload")
    }
}
/// C descriptor or borrowed view `thinkthen_complete_json`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct thinkthen_complete_json {
    /// C field `kind`.
    pub kind: u32,
    /// C field `data`.
    pub data: thinkthen_complete_json_data,
}
pub const THINKTHEN_COMPLETE_JSON_NULL: u32 = 1;
pub const THINKTHEN_COMPLETE_JSON_BOOLEAN: u32 = 2;
pub const THINKTHEN_COMPLETE_JSON_NUMBER: u32 = 3;
pub const THINKTHEN_COMPLETE_JSON_STRING: u32 = 4;
pub const THINKTHEN_COMPLETE_JSON_ARRAY: u32 = 5;
pub const THINKTHEN_COMPLETE_JSON_OBJECT: u32 = 6;
pub(crate) fn read_json(
    node: &super::Node,
    store: &mut super::Storage,
) -> Result<*const thinkthen_complete_json, thinkthen::ErrorKind> {
    let (kind, data) = match node.kind() {
        "null" => (
            THINKTHEN_COMPLETE_JSON_NULL,
            thinkthen_complete_json_data { boolean: 0 },
        ),
        "boolean" => (
            THINKTHEN_COMPLETE_JSON_BOOLEAN,
            thinkthen_complete_json_data {
                boolean: node.boolean()?,
            },
        ),
        "number" => (
            THINKTHEN_COMPLETE_JSON_NUMBER,
            thinkthen_complete_json_data {
                number: store.text(node.raw_text()),
            },
        ),
        "string" => (
            THINKTHEN_COMPLETE_JSON_STRING,
            thinkthen_complete_json_data {
                string: store.text(node.text()?),
            },
        ),
        "array" => {
            let mut copied = Vec::new();
            for value in node.array()? {
                copied.push(read_json(value, store)?);
            }
            let (data, len) = store.slice(copied);
            (
                THINKTHEN_COMPLETE_JSON_ARRAY,
                thinkthen_complete_json_data {
                    array: thinkthen_complete_json_array { data, len },
                },
            )
        }
        "object" => {
            let mut copied = Vec::new();
            for (name, value) in node.object()? {
                copied.push(thinkthen_complete_json_entry {
                    name: store.text(name),
                    value: read_json(value, store)?,
                });
            }
            let (data, len) = store.slice(copied);
            (
                THINKTHEN_COMPLETE_JSON_OBJECT,
                thinkthen_complete_json_data {
                    object: thinkthen_complete_json_object { data, len },
                },
            )
        }
        _ => return Err(thinkthen::ErrorKind::Defect),
    };
    Ok(store.hold(thinkthen_complete_json { kind, data }))
}

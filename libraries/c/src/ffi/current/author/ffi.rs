//! Additive author construction and explicit native named/reference loading.
#![allow(
    unsafe_code,
    reason = "counted descriptors and outputs obey the installed C header"
)]
#![deny(unsafe_op_in_unsafe_fn)]
use super::{question, read};
use crate::current::author::Author;
use crate::ffi::carriers::{InputDeclarationV1, QuestionAuthorV1, QuestionSpecV1, StringV1};
use crate::ffi::values as abi;
use crate::ffi::values::{
    THINKTHEN_FUNCTION_CHOOSE_V1 as CHOOSE, THINKTHEN_FUNCTION_FIND_V1 as FIND,
    THINKTHEN_FUNCTION_RANK_V1 as RANK, THINKTHEN_FUNCTION_RECOGNIZE_V1 as RECOGNIZE,
};
use crate::{
    Door,
    current::{self, QuestionHandle},
    failures::{DEFECT, Failure, OK, USAGE, guard},
};
use thinkthen::{
    InputDeclaration, InputProperty, InputPropertyType, ObjectDeclaration, QuestionName,
    WordingVersion,
};
unsafe fn declaration(value: InputDeclarationV1) -> Result<Option<InputDeclaration>, Failure> {
    // SAFETY: initialized counted arrays obey the same checked extent contract.
    unsafe {
        let properties = read::slice(value.properties.data, value.properties.len)?;
        let required = read::strings(value.required)?;
        if value.kind != CHOOSE && (!properties.is_empty() || !required.is_empty()) {
            return Err(Failure::usage(
                "only object declarations take properties/required",
            ));
        }
        match value.kind {
            abi::DECLARATION_ABSENT_V1 => Ok(None),
            abi::DECLARATION_STRING_V1 => Ok(Some(InputDeclaration::String)),
            abi::DECLARATION_OBJECT_V1 => {
                let properties = properties
                    .iter()
                    .map(|p| {
                        let kind = match p.kind {
                            abi::PROPERTY_STRING_V1 => InputPropertyType::String,
                            abi::PROPERTY_NUMBER_V1 => InputPropertyType::Number,
                            abi::PROPERTY_BOOLEAN_V1 => InputPropertyType::Boolean,
                            abi::PROPERTY_STRING_LIST_V1 => InputPropertyType::StringList,
                            _ => return Err(Failure::usage("invalid declaration property type")),
                        };
                        Ok(InputProperty::new(read::string(p.name)?, kind)?)
                    })
                    .collect::<Result<Vec<_>, Failure>>()?;
                Ok(Some(InputDeclaration::Object(ObjectDeclaration::new(
                    properties, required,
                )?)))
            }
            _ => Err(Failure::usage("invalid input declaration kind")),
        }
    }
}
unsafe fn author(value: Option<&QuestionAuthorV1>) -> Result<Author, Failure> {
    let Some(value) = value else {
        return Ok(Author::default());
    };
    // SAFETY: each active counted field follows the validated input extent contract.
    unsafe {
        Ok(Author {
            name: read::optional_string(value.name)?
                .map(|s| QuestionName::new(&s))
                .transpose()?,
            version: if read::flag(value.wording_version.present)? {
                Some(WordingVersion::new(
                    u32::try_from(value.wording_version.value)
                        .map_err(|_| Failure::usage("invalid wording version"))?,
                )?)
            } else {
                None
            },
            item: declaration(value.item_schema)?,
            context: declaration(value.context_schema)?,
        })
    }
}
pub(super) unsafe fn new(
    engine: *const Door,
    spec: *const QuestionSpecV1,
    metadata: *const QuestionAuthorV1,
    out: *mut *mut QuestionHandle,
) -> i32 {
    // SAFETY: existing constructor omits the additive recognition task.
    unsafe { new_with_task(engine, spec, metadata, None, out) }
}
pub(super) unsafe fn new_with_task(
    engine: *const Door,
    spec: *const QuestionSpecV1,
    metadata: *const QuestionAuthorV1,
    task: Option<crate::ffi::carriers::RecognitionTaskV1>,
    out: *mut *mut QuestionHandle,
) -> i32 {
    // SAFETY: descriptors are live counted storage; publication occurs only on success.
    unsafe {
        crate::ffi::typed(
            engine,
            |_| {
                read::required(out)?;
                let spec = read::reference(spec)?;
                if task.is_some() && spec.kind != RECOGNIZE {
                    return Err(Failure::usage(
                        "recognition task requires a recognition question",
                    ));
                }
                let author = author(metadata.as_ref())?;
                let json = question::build(spec, Some(&author), task)?;
                let mut q = if spec.kind == RANK && spec.members.len == 0 {
                    current::question::rank(json)
                } else if spec.kind == CHOOSE && spec.choices.len == 0 {
                    current::question::dynamic(json)
                } else {
                    current::parse(spec.kind, json)
                }?;
                if spec.kind == FIND && read::flag(spec.none)? {
                    let current::question::Native::Find(native) = &mut q.native else {
                        return Err(Failure::defect("native find constructor lost its kind"));
                    };
                    *native = native.clone().offering_none()?;
                }
                q.descriptor = Some(Box::new(question::descriptor(spec)?));
                Ok(q)
            },
            |_, value| {
                *out = Box::into_raw(Box::new(value));
                OK
            },
        )
    }
}
/// Construct through native grammar with additive author metadata.
/// # Safety
/// All pointers follow include/thinkthen.h's storage and lifetime contract.
/// Construct through the same native grammar, with separately counted author
/// metadata. author=NULL means no author metadata. All inputs are cloned.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_question_new_authored(
    engine: *const Door,
    spec: *const QuestionSpecV1,
    author: *const QuestionAuthorV1,
    out: *mut *mut QuestionHandle,
) -> std::ffi::c_int {
    // SAFETY: unchanged descriptors pass through the common guarded constructor.
    unsafe { new(engine, spec, author, out) }
}
/// Borrow the native author snapshot owned by this question.
/// # Safety
/// Question and output obey the header's lifetime/storage contract.
/// Borrow metadata owned by this immutable question until question_free.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_question_author(
    owner: *const QuestionHandle,
    out: *mut QuestionAuthorV1,
) -> std::ffi::c_int {
    guard(None, DEFECT, || {
        if out.is_null() {
            return USAGE;
        }
        // SAFETY: owner is NULL or live and immutable throughout this access.
        let Some(owner) = (unsafe { owner.as_ref() }) else {
            return USAGE;
        };
        // SAFETY: writable initialized output is published only on success.
        unsafe {
            *out = owner.author.view;
        }
        OK
    })
}
unsafe fn load(
    engine: *const Door,
    role: u32,
    value: StringV1,
    reference: bool,
    out: *mut *mut QuestionHandle,
) -> i32 {
    // SAFETY: counted UTF-8 and output follow the header's checked storage contract.
    unsafe {
        crate::ffi::typed(
            engine,
            |_| {
                read::required(out)?;
                current::question::named(role, read::string(value)?, reference)
            },
            |_, value| {
                *out = Box::into_raw(Box::new(value));
                OK
            },
        )
    }
}
/// Load a named question through the selected native role's loader.
/// # Safety
/// All pointers follow include/thinkthen.h's storage and lifetime contract.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_question_load_named(
    engine: *const Door,
    role: u32,
    name: StringV1,
    out: *mut *mut QuestionHandle,
) -> std::ffi::c_int {
    // SAFETY: unchanged descriptors pass through the common guarded loader.
    unsafe { load(engine, role, name, false, out) }
}
/// Load an explicit reference through the selected native role's loader.
/// # Safety
/// All pointers follow include/thinkthen.h's storage and lifetime contract.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_question_load_reference(
    engine: *const Door,
    role: u32,
    reference: StringV1,
    out: *mut *mut QuestionHandle,
) -> std::ffi::c_int {
    // SAFETY: unchanged descriptors pass through the common guarded loader.
    unsafe { load(engine, role, reference, true, out) }
}

// SAFETY: AuthorOwner is constructed only by AuthorOwner::new. Its closed
// builder stores cloned native metadata and boxed immutable bytes/property
// arrays. All view pointers address those allocations; no caller pointer,
// interior mutation, or mutable accessor survives construction. Moving the
// owner preserves allocation addresses, and shared reads cannot modify them.
unsafe impl Send for crate::current::author::AuthorOwner {}
// SAFETY: the same closed, immutable backing construction permits shared reads.
unsafe impl Sync for crate::current::author::AuthorOwner {}

/// Import counted saved-question grammar through an explicitly selected native parser.
/// # Safety
/// Counted bytes and output obey the header's storage contract.
/// Import saved question-file JSON through an explicit native grammar role.
/// This imports a question only; judgment calls and result fields remain typed.
/// Inline grammar failures return EUSAGE; no role guessing or parser fallback.
/// The immutable handle owns every byte independently of json's lifetime.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_question_parse(
    engine: *const Door,
    role: u32,
    json: StringV1,
    out: *mut *mut QuestionHandle,
) -> std::ffi::c_int {
    // SAFETY: common edge validates live engine/output and counted UTF-8 storage.
    unsafe {
        crate::ffi::typed(
            engine,
            |_| {
                read::required(out)?;
                current::question::parse_role(role, read::string(json)?)
            },
            |_, value| {
                *out = Box::into_raw(Box::new(value));
                OK
            },
        )
    }
}

//! Named selection copies each admitted typed snapshot.
#![allow(
    unsafe_code,
    reason = "typed exports borrow foreign builders and counted fields"
)]
use crate::{
    HostError, QuestionHandle, ToolboxHandle,
    exports::{OK, operation},
    memory::text,
};
use std::ptr;
/// Allocate an initially empty collection of named typed questions.
/// # Safety
/// Out and error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_toolbox_new(
    out: *mut *mut ToolboxHandle,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: exclusive outputs are writable until return. */
    unsafe {
        operation(error, || {
            out.write(Box::into_raw(Box::new(ToolboxHandle::default())));
            Ok(OK)
        })
    }
}
/// Insert a copied non-annotate question; duplicate names leave the toolbox unchanged.
/// # Safety
/// Live handles and counted name remain borrowed until return; error is empty.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_toolbox_insert(
    toolbox: *mut ToolboxHandle,
    name: *const u8,
    len: usize,
    question: *const QuestionHandle,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: only the collection is mutated; child and bytes are borrowed. */
    unsafe {
        operation(error, || {
            (*toolbox)
                .0
                .insert(text(name, len)?, (*question).0.build()?)?;
            Ok(OK)
        })
    }
}
/// Select an independent owned question that survives toolbox destruction.
/// # Safety
/// Handles and counted name are live; outputs are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_toolbox_select(
    toolbox: *const ToolboxHandle,
    name: *const u8,
    len: usize,
    out: *mut *mut QuestionHandle,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: live borrowed inputs and exclusive outputs remain valid until return. */
    unsafe {
        operation(error, || {
            out.write(ptr::null_mut());
            let question = (*toolbox).0.select(text(name, len)?)?;
            out.write(Box::into_raw(Box::new(QuestionHandle(question))));
            Ok(OK)
        })
    }
}
/// Release a toolbox; selected questions and opened calls remain independent.
/// # Safety
/// NULL or the unique live collection after all readers finish.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_toolbox_free(toolbox: *mut ToolboxHandle) {
    if !toolbox.is_null() {
        /* SAFETY: the caller transfers this unique allocation. */
        drop(unsafe { Box::from_raw(toolbox) });
    }
}

/// Admit canonical toolbox JSON without resolving files or executing entries.
/// # Safety
/// Counted UTF-8 and distinct empty output slots remain live until return.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_toolbox_from_json(
    data: *const u8,
    len: usize,
    out: *mut *mut ToolboxHandle,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the caller lends counted bytes and exclusive outputs. */
    unsafe {
        operation(error, || {
            out.write(ptr::null_mut());
            let toolbox = thinkthen::native::ToolboxBuilder::from_json(text(data, len)?)?;
            out.write(Box::into_raw(Box::new(ToolboxHandle(toolbox))));
            Ok(OK)
        })
    }
}
/// Select an owned snapshot of the requested function, with shared mismatch errors.
/// # Safety
/// Handles, counted name and distinct empty output slots are live until return.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_toolbox_select_as(
    toolbox: *const ToolboxHandle,
    name: *const u8,
    len: usize,
    function: u32,
    out: *mut *mut QuestionHandle,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the caller lends live handles, bytes and exclusive outputs. */
    unsafe {
        operation(error, || {
            out.write(ptr::null_mut());
            let function = thinkthen::FunctionName::from_abi(function)
                .ok_or_else(thinkthen::native::abi_error)?;
            let question = (*toolbox).0.select_as(text(name, len)?, function)?;
            out.write(Box::into_raw(Box::new(QuestionHandle(question))));
            Ok(OK)
        })
    }
}

/// Admit flat typed definitions from a native host constructor.
/// # Safety
/// Counted UTF-8 and distinct empty output slots remain live until return.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_toolbox_from_definitions_json(
    data: *const u8,
    len: usize,
    out: *mut *mut ToolboxHandle,
    error: *mut HostError,
) -> i32 {
    // SAFETY: the caller lends counted input and exclusive outputs.
    unsafe {
        operation(error, || {
            out.write(ptr::null_mut());
            let toolbox =
                thinkthen::native::ToolboxBuilder::from_definitions_json(text(data, len)?)?;
            out.write(Box::into_raw(Box::new(ToolboxHandle(toolbox))));
            Ok(OK)
        })
    }
}

/// Save the admitted named questions in the command's entries format.
/// # Safety
/// The handle is borrowed; out and error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_toolbox_to_json(
    toolbox: *const ToolboxHandle,
    out: *mut crate::Buffer,
    error: *mut HostError,
) -> i32 {
    // SAFETY: the caller lends a live handle and exclusive outputs.
    unsafe {
        operation(error, || {
            out.write(crate::Buffer::default());
            out.write(crate::Buffer::new((*toolbox).0.to_json()?));
            Ok(OK)
        })
    }
}

/// Export admitted flat definitions for host hydration, without host validation.
/// # Safety
/// The handle is borrowed; out and error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_toolbox_definitions_json(
    toolbox: *const ToolboxHandle,
    out: *mut crate::Buffer,
    error: *mut HostError,
) -> i32 {
    // SAFETY: the caller lends a live handle and exclusive outputs.
    unsafe {
        operation(error, || {
            out.write(crate::Buffer::default());
            out.write(crate::Buffer::new((*toolbox).0.definitions_json()?));
            Ok(OK)
        })
    }
}

/// Export one admitted question's canonical definition for host hydration.
/// # Safety
/// The handle is borrowed; out and error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_question_to_json(
    question: *const QuestionHandle,
    out: *mut crate::Buffer,
    error: *mut HostError,
) -> i32 {
    // SAFETY: the caller lends a live handle and exclusive outputs.
    unsafe {
        operation(error, || {
            out.write(crate::Buffer::default());
            let value = (*question).0.build()?;
            let json =
                serde_json::to_string(&value).map_err(|_| thinkthen::native::internal_error())?;
            out.write(crate::Buffer::new(json));
            Ok(OK)
        })
    }
}

/// Return the question's function discriminator without completing its builder.
/// # Safety
/// The question is live; out and error are distinct exclusive writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_question_function(
    question: *const QuestionHandle,
    out: *mut u32,
    error: *mut HostError,
) -> i32 {
    // SAFETY: the caller lends a live handle and exclusive outputs.
    unsafe {
        operation(error, || {
            out.write((*question).0.function().abi());
            Ok(OK)
        })
    }
}

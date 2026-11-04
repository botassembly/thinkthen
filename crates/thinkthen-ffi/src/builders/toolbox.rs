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
/// Make an empty toolbox of named questions.
///
/// Returns THINKTHEN_OK with *out set. The caller owns *out and frees it with
/// thinkthen_toolbox_free; an old handle in *out is not freed.
/// # Safety
/// Any thread may call this. out is writable. error is NULL or initialized with
/// THINKTHEN_ERROR_INIT. On a negative return it holds the code and message; passing it again
/// frees what it held, so a reused error leaks nothing. Free it with thinkthen_error_free after
/// its last use.
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
/// Add a copy of question to the toolbox under name.
///
/// Returns THINKTHEN_OK, or a negative code with the toolbox unchanged:
/// -THINKTHEN_CODE_USAGE_NAME when name is blank or already used,
/// -THINKTHEN_CODE_USAGE_QUESTION when question is incomplete or annotate, or
/// -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8.
/// # Safety
/// One thread at a time may call this on a toolbox; other toolboxs may be used on other
/// threads. question is live and no thread changes it during the call. name points to len
/// readable bytes, or to a NUL-terminated string when len is THINKTHEN_NUL_TERMINATED; it is
/// copied before return. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative
/// return it holds the code and message; passing it again frees what it held, so a reused error
/// leaks nothing. Free it with thinkthen_error_free after its last use.
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
                .insert(text(name, len)?, (*question).build()?)?;
            Ok(OK)
        })
    }
}
/// Copy the question named name out of the toolbox.
///
/// Returns THINKTHEN_OK with *out set, or a negative code with *out set to NULL:
/// -THINKTHEN_CODE_USAGE_NAME when no question has that name, or -THINKTHEN_CODE_USAGE_ABI when
/// the text is not UTF-8. The caller owns *out and frees it with thinkthen_question_free; it
/// outlives the toolbox, and an old handle in *out is not freed.
/// # Safety
/// Any thread may call this while no thread changes or frees toolbox. name points to len
/// readable bytes, or to a NUL-terminated string when len is THINKTHEN_NUL_TERMINATED; it is
/// copied before return. out is writable. error is NULL or initialized with
/// THINKTHEN_ERROR_INIT. On a negative return it holds the code and message; passing it again
/// frees what it held, so a reused error leaks nothing. Free it with thinkthen_error_free after
/// its last use.
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
            out.write(Box::into_raw(Box::new(QuestionHandle::new(question))));
            Ok(OK)
        })
    }
}
/// Free a toolbox.
///
/// Free a toolbox. NULL is ignored. Questions selected from it and calls opened from them keep
/// their own copies.
/// # Safety
/// Call this once, after every other use of toolbox has returned.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_toolbox_free(toolbox: *mut ToolboxHandle) {
    if !toolbox.is_null() {
        /* SAFETY: the caller transfers this unique allocation. */
        drop(unsafe { Box::from_raw(toolbox) });
    }
}

/// Read a toolbox from the command's entries JSON, without reading files or running anything.
///
/// Returns THINKTHEN_OK with *out set, or a negative code with *out set to NULL:
/// -THINKTHEN_CODE_LOCAL_TOOLBOX, -THINKTHEN_CODE_USAGE_NAME or -THINKTHEN_CODE_USAGE_QUESTION
/// for JSON that is not a valid toolbox, or -THINKTHEN_CODE_USAGE_ABI when the text is not
/// UTF-8. The caller owns *out and frees it with thinkthen_toolbox_free; an old handle in *out
/// is not freed.
/// # Safety
/// Any thread may call this. data points to len readable bytes, or to a NUL-terminated string
/// when len is THINKTHEN_NUL_TERMINATED; it is copied before return. out is writable. error is
/// NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
/// message; passing it again frees what it held, so a reused error leaks nothing. Free it with
/// thinkthen_error_free after its last use.
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
/// Copy the question named name out of the toolbox, checking that it asks function.
///
/// Returns THINKTHEN_OK with *out set, or a negative code with *out set to NULL:
/// -THINKTHEN_CODE_USAGE_FUNCTION for an unknown function, -THINKTHEN_CODE_USAGE_NAME when no
/// question has that name or it asks another function, or -THINKTHEN_CODE_USAGE_ABI when the
/// text is not UTF-8. The caller owns *out and frees it with thinkthen_question_free; it
/// outlives the toolbox, and an old handle in *out is not freed.
/// # Safety
/// Any thread may call this while no thread changes or frees toolbox. name points to len
/// readable bytes, or to a NUL-terminated string when len is THINKTHEN_NUL_TERMINATED; it is
/// copied before return. out is writable. error is NULL or initialized with
/// THINKTHEN_ERROR_INIT. On a negative return it holds the code and message; passing it again
/// frees what it held, so a reused error leaks nothing. Free it with thinkthen_error_free after
/// its last use.
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
                .ok_or_else(|| thinkthen::native::argument_error(thinkthen::Code::UsageFunction))?;
            let question = (*toolbox).0.select_as(text(name, len)?, function)?;
            out.write(Box::into_raw(Box::new(QuestionHandle::new(question))));
            Ok(OK)
        })
    }
}

/// Read a toolbox from flat question definitions, as thinkthen_toolbox_definitions_json writes
/// them.
///
/// Returns THINKTHEN_OK with *out set, or a negative code with *out set to NULL: a usage code
/// for definitions that are not valid, or -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8.
/// The caller owns *out and frees it with thinkthen_toolbox_free; an old handle in *out is not
/// freed.
/// # Safety
/// Any thread may call this. data points to len readable bytes, or to a NUL-terminated string
/// when len is THINKTHEN_NUL_TERMINATED; it is copied before return. out is writable. error is
/// NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
/// message; passing it again frees what it held, so a reused error leaks nothing. Free it with
/// thinkthen_error_free after its last use.
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

/// Write the toolbox in the command's entries JSON to *out.
///
/// Returns THINKTHEN_OK with *out written, or -THINKTHEN_CODE_INTERNAL_BUG with *out empty.
/// *out is a new buffer the caller owns; free it with thinkthen_buffer_free. The old contents
/// of *out are not freed, so free a buffer before passing it again.
/// # Safety
/// Any thread may call this while no thread changes or frees toolbox. out is writable. error is
/// NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
/// message; passing it again frees what it held, so a reused error leaks nothing. Free it with
/// thinkthen_error_free after its last use.
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

/// Write the toolbox as flat question definitions to *out.
///
/// Returns THINKTHEN_OK with *out written, or -THINKTHEN_CODE_INTERNAL_BUG with *out empty.
/// *out is a new buffer the caller owns; free it with thinkthen_buffer_free. The old contents
/// of *out are not freed, so free a buffer before passing it again.
/// # Safety
/// Any thread may call this while no thread changes or frees toolbox. out is writable. error is
/// NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
/// message; passing it again frees what it held, so a reused error leaks nothing. Free it with
/// thinkthen_error_free after its last use.
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

/// Write a complete question's JSON definition to *out.
///
/// Returns THINKTHEN_OK with *out written, or a negative code with *out empty:
/// -THINKTHEN_CODE_USAGE_QUESTION when the question is incomplete. *out is a new buffer the
/// caller owns; free it with thinkthen_buffer_free. The old contents of *out are not freed, so
/// free a buffer before passing it again.
/// # Safety
/// Any thread may call this while no thread changes or frees question. out is writable. error
/// is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
/// message; passing it again frees what it held, so a reused error leaks nothing. Free it with
/// thinkthen_error_free after its last use.
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
            let value = (*question).build()?;
            let json =
                serde_json::to_string(&value).map_err(|_| thinkthen::native::internal_error())?;
            out.write(crate::Buffer::new(json));
            Ok(OK)
        })
    }
}

/// Write the question's thinkthen_function to *out.
///
/// Returns THINKTHEN_OK with *out written. It works on an incomplete question too.
/// # Safety
/// Any thread may call this while no thread changes or frees question. out is writable. error
/// is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
/// message; passing it again frees what it held, so a reused error leaks nothing. Free it with
/// thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_question_function(
    question: *const QuestionHandle,
    out: *mut u32,
    error: *mut HostError,
) -> i32 {
    // SAFETY: the caller lends a live handle and exclusive outputs.
    unsafe {
        operation(error, || {
            out.write((*question).builder.function().abi());
            Ok(OK)
        })
    }
}

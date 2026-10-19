//! The synchronous native call owns execution; foreign callbacks remain on its caller.
#![allow(unsafe_code, reason = "the counted C boundary borrows foreign storage")]
#![deny(unsafe_op_in_unsafe_fn)]
use crate::failures::{self, Failure};
use crate::session::views::thinkthen_complete_utf8_v1 as Text;
use crate::EngineHandle;
use std::ffi::c_void;
use thinkthen::{CallOptions, RequestEnvironment, RequestFeed, RequestInput, RequestItem, Surface};

/// One independent complete Call in the existing canonical JSON grammar.
#[derive(Debug)]
pub struct CallResultHandle(String);
/// Independent safe failure details. Send or read on any thread; free after all readers.
#[derive(Debug)]
pub struct CallErrorHandle {
    failure: Failure,
    facts: Option<String>,
}
/// A caller-owned cancellation token. Keep alive until the call and all cancels return.
#[derive(Debug, Default)]
pub struct CallCancelHandle(thinkthen::CancelToken);
/// A producer returns ITEM and counted RequestItem JSON, END, or FAILURE.
/// Bytes remain readable until the next invocation or until run_call returns.
/// It runs synchronously on the run_call caller, and may reenter independent C operations.
/// Never unwind or throw across this callback; retain private host exceptions locally.
pub type CallProducerV1 = Option<unsafe extern "C" fn(*mut c_void, *mut Text) -> u32>;
/// Producer published one canonical RequestItem JSON value.
pub const THINKTHEN_CALL_PRODUCER_ITEM_V1: u32 = 0;
/// Producer ended normally; the output is ignored.
pub const THINKTHEN_CALL_PRODUCER_END_V1: u32 = 1;
/// Producer failed; no host exception text crosses this boundary.
pub const THINKTHEN_CALL_PRODUCER_FAILURE_V1: u32 = 2;

fn view(text: &str) -> Text {
    Text { data: text.as_ptr().cast(), len: text.len() }
}
unsafe fn read<'a>(text: Text) -> Result<&'a str, Failure> {
    // SAFETY: the foreign caller retains the counted readable extent.
    unsafe { super::text(text.data, text.len) }
}
unsafe fn execute(
    engine: *const EngineHandle,
    request: *const super::session::NamedRequestV1,
    producer: CallProducerV1,
    context: *mut c_void,
    cancel: *const CallCancelHandle,
) -> Result<CallResultHandle, Failure> {
    // SAFETY: every owner and counted view remains alive throughout the call.
    let engine = unsafe { engine.as_ref() }.ok_or(thinkthen::ErrorKind::Usage)?;
    // SAFETY: shared canonical component decoding copies the foreign input.
    let decoded = unsafe { super::session::named_request(request) }?;
    // SAFETY: the caller retains the component record and function bytes.
    let function = unsafe { read(request.as_ref().ok_or(thinkthen::ErrorKind::Usage)?.function) }?;
    let request = decoded;
    let arguments = request.call.arguments().clone();
    let mut controls = CallOptions::new().surface(Surface::C);
    // SAFETY: the optional token outlives this blocking invocation.
    if let Some(cancel) = unsafe { cancel.as_ref() } { controls = controls.cancel(&cancel.0); }
    let feed = producer.map(|callback| {
        let name = match &arguments.input { RequestInput::Feed { name } => name.clone(), _ => String::new() };
        let mut finished = false;
        RequestFeed::new(name, std::iter::from_fn(move || {
            if finished { return None; }
            let mut item = Text { data: std::ptr::null(), len: 0 };
            // SAFETY: callback and context belong to this caller and live through run_call.
            match unsafe { callback(context, &mut item) } {
                THINKTHEN_CALL_PRODUCER_END_V1 => { finished = true; None }
                THINKTHEN_CALL_PRODUCER_ITEM_V1 => {
                    // SAFETY: the producer retains these bytes until its next invocation.
                    Some(unsafe { read(item) }.map_err(|_| thinkthen::Error::usage("invalid producer UTF-8"))
                        .and_then(RequestItem::from_json))
                }
                _ => { finished = true; Some(Err(thinkthen::Error::local("session producer raised an exception"))) }
            }
        }))
    });
    let result = engine.0.engine.call(function, arguments, RequestEnvironment { controls, feed })?;
    serde_json::to_string(&result).map(CallResultHandle).map_err(|_| Failure::defect("a call result could not be serialized"))
}

/// Run one named call to completion using canonical counted question/input/options JSON.
/// Producer is optional and supplies the declared feed. Its callback runs only on this
/// calling thread. Async hosts dedicate one worker to this operation and cancel through
/// the token from another thread. Rust owns all admission, scheduling and stop rules.
/// On success publish only result; on failure publish only an independent error.
/// The other output remains unchanged. No legacy per-thread error slot is changed.
/// Both output slots are required and must be distinct; invalid slots return EUSAGE
/// without publishing. Every returned owner survives engine destruction and thread exit.
/// # Safety
/// engine and optional cancel are live for the whole call. request and its counted
/// UTF-8 components remain readable until return. Output slots are aligned writable
/// distinct storage. producer/context obey CallProducerV1 and outlive the call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_run_call(
    engine: *const EngineHandle,
    request: *const super::session::NamedRequestV1,
    producer: CallProducerV1,
    context: *mut c_void,
    cancel: *const CallCancelHandle,
    result: *mut *mut CallResultHandle,
    error: *mut *mut CallErrorHandle,
) -> i32 {
    failures::guard(None, failures::DEFECT, || {
        if result.is_null() || error.is_null() || result.cast::<()>() == error.cast::<()>() { return failures::USAGE; }
        let outcome = failures::owned_call(|| {
            // SAFETY: inputs retain the blocking export's documented lifetime.
            unsafe { execute(engine, request, producer, context, cancel) }
        });
        match outcome {
            Ok(value) => {
                // SAFETY: the checked slot receives one independent allocation.
                unsafe { *result = Box::into_raw(Box::new(value)); }
                failures::OK
            }
            Err(failure) => {
                let code = failure.code();
                let facts = failure.owned_facts();
                // SAFETY: the checked slot receives one independent allocation.
                unsafe { *error = Box::into_raw(Box::new(CallErrorHandle { failure, facts })); }
                code
            }
        }
    })
}

/// Allocate an independent cancellation token; NULL only if the boundary panics.
#[unsafe(no_mangle)]
pub extern "C" fn thinkthen_call_cancel_new() -> *mut CallCancelHandle {
    failures::guard(None, std::ptr::null_mut(), || Box::into_raw(Box::default()))
}
/// Cancel from any thread. NULL is ignored.
/// # Safety
/// token is NULL or live until every concurrent call/cancel returns.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_call_cancel(token: *const CallCancelHandle) {
    failures::guard(None, (), || {
        // SAFETY: caller retains the nullable token.
        if let Some(token) = unsafe { token.as_ref() } { token.0.cancel(); }
    })
}
/// Borrow complete canonical Call JSON until result_free. NULL returns empty text.
/// # Safety
/// owner is NULL or live for every reader of the returned text.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_call_result_json(owner: *const CallResultHandle) -> Text {
    failures::guard(None, view(""), || {
        // SAFETY: caller retains the immutable owner.
        unsafe { owner.as_ref() }.map_or(view(""), |owner| view(&owner.0))
    })
}
/// Read the error kind as the existing status code. NULL returns EUSAGE.
/// # Safety
/// owner is NULL or live throughout this read.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_call_error_code(owner: *const CallErrorHandle) -> i32 {
    failures::guard(None, failures::DEFECT, || {
        // SAFETY: caller retains the immutable owner.
        unsafe { owner.as_ref() }.map_or(failures::USAGE, |owner| owner.failure.code())
    })
}
/// Read retryability from any thread; NULL returns false.
/// # Safety
/// owner is NULL or live throughout this read.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_call_error_retryable(owner: *const CallErrorHandle) -> i32 {
    failures::guard(None, 0, || {
        // SAFETY: caller retains the immutable owner.
        unsafe { owner.as_ref() }.map_or(0, |owner| i32::from(owner.failure.retry_signal()))
    })
}
/// Borrow safe counted error text until error_free. NULL returns empty text.
/// # Safety
/// owner is NULL or live for every reader of the returned text.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_call_error_message(owner: *const CallErrorHandle) -> Text {
    failures::guard(None, view(""), || {
        // SAFETY: caller retains the immutable owner.
        unsafe { owner.as_ref() }.map_or(view(""), |owner| view(owner.failure.message_text()))
    })
}
/// Borrow existing canonical Facts JSON until error_free; absent facts return empty text.
/// # Safety
/// owner is NULL or live for every reader of the returned text.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_call_error_facts_json(owner: *const CallErrorHandle) -> Text {
    failures::guard(None, view(""), || {
        // SAFETY: caller retains the immutable owner.
        unsafe { owner.as_ref() }.and_then(|owner| owner.facts.as_deref()).map_or(view(""), view)
    })
}

/// Free this independent owner; NULL is ignored.
/// # Safety
/// owner is NULL or unfreed, after all concurrent operations and borrowed reads finish.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_call_result_free(owner: *mut CallResultHandle) {
    failures::guard(None, (), || {
        if !owner.is_null() {
            // SAFETY: the owner came from this allocator and has no remaining borrowers.
            drop(unsafe { Box::from_raw(owner) });
        }
    })
}

/// Free this independent owner; NULL is ignored.
/// # Safety
/// owner is NULL or unfreed, after all concurrent operations and borrowed reads finish.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_call_error_free(owner: *mut CallErrorHandle) {
    failures::guard(None, (), || {
        if !owner.is_null() {
            // SAFETY: the owner came from this allocator and has no remaining borrowers.
            drop(unsafe { Box::from_raw(owner) });
        }
    })
}

/// Free this independent owner; NULL is ignored.
/// # Safety
/// owner is NULL or unfreed, after all concurrent operations and borrowed reads finish.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_call_cancel_free(owner: *mut CallCancelHandle) {
    failures::guard(None, (), || {
        if !owner.is_null() {
            // SAFETY: the owner came from this allocator and has no remaining borrowers.
            drop(unsafe { Box::from_raw(owner) });
        }
    })
}

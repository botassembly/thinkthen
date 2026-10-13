//! Counted typed request carriers reuse the native grammar and construction.
#![allow(
    unsafe_code,
    reason = "the caller owns the aligned counted carrier tree"
)]
#![deny(unsafe_op_in_unsafe_fn)]
#[path = "operations_generated.rs"]
mod operations;
use crate::EngineHandle;
use crate::failures::{self, Failure};
use crate::session::SessionHandle;
use crate::session::views::thinkthen_complete_utf8_v1;
pub use operations::*;
use serde::Serialize;
use serde::ser::{SerializeMap, SerializeSeq};
use serde_json::value::RawValue;
use thinkthen::{ErrorKind, Request, RequestReaderFailure, Surface};

/// Counted records and tagged alternatives from the canonical request graph.
#[path = "carriers_generated.rs"]
pub mod carriers;

#[derive(Debug)]
enum CarrierError {
    Invalid,
    Overflow,
}
impl From<CarrierError> for Failure {
    fn from(error: CarrierError) -> Self {
        match error {
            CarrierError::Invalid => ErrorKind::Usage.into(),
            CarrierError::Overflow => Self::carrier_overflow(),
        }
    }
}

#[derive(Debug)]
enum Value {
    Boolean(bool),
    Text(String),
    Number(serde_json::Number),
    Raw(Box<RawValue>),
    Array(Vec<Self>),
    Object(Vec<(String, Self)>),
}
impl Value {
    fn literal(text: &str) -> Result<Self, CarrierError> {
        serde_json::from_str(text)
            .map(Self::Raw)
            .map_err(|_| CarrierError::Invalid)
    }
    fn json(&self) -> Result<String, Failure> {
        serde_json::to_string(self).map_err(|_| ErrorKind::Usage.into())
    }
}
impl Serialize for Value {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Boolean(value) => value.serialize(serializer),
            Self::Text(value) => value.serialize(serializer),
            Self::Number(value) => value.serialize(serializer),
            Self::Raw(value) => value.serialize(serializer),
            Self::Array(values) => {
                let mut sequence = serializer.serialize_seq(Some(values.len()))?;
                for value in values {
                    sequence.serialize_element(value)?;
                }
                sequence.end()
            }
            Self::Object(values) => {
                let mut object = serializer.serialize_map(Some(values.len()))?;
                for (key, value) in values {
                    object.serialize_entry(key, value)?;
                }
                object.end()
            }
        }
    }
}
struct Context {
    depth: usize,
    scalar_capacity: usize,
}
impl Context {
    fn new(surface: Surface) -> Self {
        Self {
            depth: 0,
            scalar_capacity: if surface == Surface::Cobol {
                8192
            } else {
                usize::MAX
            },
        }
    }
    fn enter<T>(
        &mut self,
        read: impl FnOnce(&mut Self) -> Result<T, CarrierError>,
    ) -> Result<T, CarrierError> {
        if self.depth == 512 {
            return Err(CarrierError::Invalid);
        }
        self.depth += 1;
        let result = read(self);
        self.depth -= 1;
        result
    }
    unsafe fn text(&self, value: thinkthen_complete_utf8_v1) -> Result<String, CarrierError> {
        if value.len > self.scalar_capacity {
            return Err(CarrierError::Overflow);
        }
        // SAFETY: each view carries the caller's live counted UTF-8 extent.
        unsafe { super::text(value.data, value.len) }
            .map(str::to_owned)
            .map_err(|_| CarrierError::Invalid)
    }
}
unsafe fn borrow<'a, T>(pointer: *const T) -> Result<&'a T, CarrierError> {
    // SAFETY: the caller retains this aligned record throughout conversion.
    unsafe { pointer.as_ref() }.ok_or(CarrierError::Invalid)
}
unsafe fn extent<'a, T>(pointer: *const T, len: usize) -> Result<&'a [T], CarrierError> {
    if len == 0 {
        return Ok(&[]);
    }
    if pointer.is_null() || len > isize::MAX as usize / std::mem::size_of::<T>().max(1) {
        return Err(CarrierError::Invalid);
    }
    // SAFETY: the validated counted extent is readable and belongs to the caller.
    Ok(unsafe { std::slice::from_raw_parts(pointer, len) })
}

unsafe fn optional<T>(
    name: &str,
    pointer: *const T,
    context: &mut Context,
    fields: &mut Vec<(String, Value)>,
    read: unsafe fn(*const T, &mut Context) -> Result<Value, CarrierError>,
) -> Result<(), CarrierError> {
    if !pointer.is_null() {
        // SAFETY: present fields inherit the caller's live carrier-tree obligations.
        fields.push((name.into(), unsafe { read(pointer, context)? }));
    }
    Ok(())
}

unsafe fn surface(value: thinkthen_complete_utf8_v1) -> Result<Surface, ErrorKind> {
    // SAFETY: the surface is a counted live UTF-8 view under the entry contract.
    unsafe { super::text(value.data, value.len) }
        .map_err(|_| ErrorKind::Usage)?
        .parse()
        .map_err(|_| ErrorKind::Usage)
}

/// Construct a named request from counted typed argument records through Rust.
/// The carrier tree copies strings, arrays, maps and explicit null alternatives;
/// no pointer survives return. Rust alone admits questions, controls and evidence.
/// COBOL retains its 8192-byte capacity per scalar, including image/base64 text;
/// overflow returns THINKTHEN_COBOL_OVERFLOW before constructing a session.
/// All output, error-slot and engine lifetime rules match the named constructor.
/// # Safety
/// engine is live and out is writable, nonnull. function and surface are counted
/// UTF-8 views. arguments and its entire finite carrier tree remain aligned and
/// readable throughout the call. Each pointer/length covers its declared element
/// type and fits Rust slices. Required pointers are nonnull; optional NULL means
/// missing. A tagged null arm means explicit null. Inputs may be freed on return.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_session_new_typed_with_surface(
    engine: *const EngineHandle,
    function: thinkthen_complete_utf8_v1,
    arguments: *const carriers::thinkthen_request_arguments_v1,
    surface_name: thinkthen_complete_utf8_v1,
    out: *mut *mut SessionHandle,
) -> std::ffi::c_int {
    // SAFETY: the caller retains its engine, carrier tree and writable output.
    unsafe {
        super::session::start_session(
            engine,
            out,
            || surface(surface_name),
            || {
                let mut context = Context::new(surface(surface_name)?);
                let Value::Object(values) =
                    carriers::read_thinkthen_request_arguments_v1(arguments, &mut context)?
                else {
                    return Err(ErrorKind::Usage.into());
                };
                let mut question = None;
                let mut input = None;
                let mut options = "{}".to_owned();
                for (name, value) in values {
                    match name.as_str() {
                        "question" => question = Some(value.json()?),
                        "input" => input = Some(value.json()?),
                        "options" => options = value.json()?,
                        _ => return Err(ErrorKind::Defect.into()),
                    }
                }
                Request::named_from_json(
                    &context.text(function)?,
                    &question.ok_or(ErrorKind::Usage)?,
                    &input.ok_or(ErrorKind::Usage)?,
                    &options,
                )
                .map_err(super::session::decode_failure)
            },
        )
    }
}

unsafe fn push_typed(
    session: &SessionHandle,
    descriptor: *const carriers::thinkthen_request_session_descriptor_v1,
    blocking: bool,
) -> Result<u32, Failure> {
    let decode = || {
        let mut context = Context::new(session.1);
        // SAFETY: native admission invokes conversion only with intake room;
        // the caller retains the descriptor throughout this operation.
        let json = unsafe {
            carriers::read_thinkthen_request_session_descriptor_v1(descriptor, &mut context)
        }?
        .json()?;
        thinkthen::RequestSessionDescriptor::from_json(&json).map_err(Failure::from)
    };
    let state = if blocking {
        session.0.push_with(decode)?
    } else {
        session.0.try_push_with(decode)?
    };
    Ok(super::session::push_status(state))
}

/// Finish input with a counted typed failure; NULL means EOF.
/// Rust preserves the safe diagnostic after the completed prefix and final facts.
/// # Safety
/// session is live. A nonnull failure satisfies the finite aligned carrier-tree
/// obligations of the typed constructor. No foreign pointer survives return.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_session_finish_typed(
    session: *const SessionHandle,
    failure: *const carriers::thinkthen_request_reader_failure_v1,
) -> std::ffi::c_int {
    failures::session_native_call(|| {
        // SAFETY: the caller retains the live session and optional failure tree.
        let session = unsafe { session.as_ref() }.ok_or(ErrorKind::Usage)?;
        let failure = if failure.is_null() {
            None
        } else {
            let mut context = Context::new(session.1);
            let json = unsafe {
                carriers::read_thinkthen_request_reader_failure_v1(failure, &mut context)
            }?
            .json()?;
            Some(RequestReaderFailure::from_json(&json).map_err(Failure::from)?)
        };
        session.0.finish(failure).map_err(Failure::from)
    })
}

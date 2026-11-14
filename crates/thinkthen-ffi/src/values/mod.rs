//! Typed complex projections own decoded values, never borrowed row allocations.
#![allow(
    unsafe_code,
    reason = "projections read supported native prefixes and counted owned buffers"
)]
mod answers;
mod detail;
mod json;
mod metadata;
pub(crate) mod views;
use crate::{HostRow, memory::live};
pub use answers::*;
pub use detail::*;
pub use json::*;
pub use metadata::*;
use std::any::Any;
use thinkthen::{
    Error, FunctionName, native,
    results::{AnnotationEntry, AnyRow, Function, Outcome, Row},
};
pub use views::*;
#[derive(Debug, Clone)]
pub(crate) enum Owned {
    Row(AnyRow),
    Entry(AnnotationEntry),
}
/// A copy of a row's answer, read with the thinkthen_value_* functions. Views it writes
/// borrow from it. Free it with thinkthen_value_free.
#[derive(Debug, Clone)]
pub struct ValueHandle(pub(crate) Owned);
impl ValueHandle {
    /// Own one engine row directly, as the one-call functions return it.
    pub(crate) fn from_any(row: AnyRow) -> Self {
        Self(Owned::Row(row))
    }
    pub(crate) unsafe fn from_row(row: *const HostRow) -> Result<Self, Error> {
        // SAFETY: the caller lends an initialized row and its owned payload.
        unsafe { Ok(crate::row::data(row)?.value.clone()) }
    }
    pub(crate) fn function(&self) -> Result<FunctionName, Error> {
        Ok(match &self.0 {
            Owned::Row(row) => row.function(),
            Owned::Entry(entry) => match entry {
                AnnotationEntry::Decide(_) => FunctionName::Decide,
                AnnotationEntry::Choose(_) => FunctionName::Choose,
                AnnotationEntry::Tag(_) => FunctionName::Tag,
                AnnotationEntry::Score(_) => FunctionName::Score,
                AnnotationEntry::Find(_) => FunctionName::Find,
                AnnotationEntry::Recognize(_) => FunctionName::Recognize,
                AnnotationEntry::Relate(_) => FunctionName::Relate,
                _ => return Err(native::abi_error()),
            },
        })
    }
    pub(crate) fn typed<F: Function>(&self) -> Result<&Outcome<F::Answer, F::Detail>, Error> {
        if self.function()? != F::NAME {
            return Err(native::argument_error(thinkthen::Code::UsageFunction));
        }
        match &self.0 {
            Owned::Row(row) => {
                let value: &dyn Any = match row {
                    AnyRow::Decide(row) => row,
                    AnyRow::Choose(row) => row,
                    AnyRow::Tag(row) => row,
                    AnyRow::Score(row) => row,
                    AnyRow::Filter(row) => row,
                    AnyRow::Rank(row) => row,
                    AnyRow::Find(row) => row,
                    AnyRow::Recognize(row) => row,
                    AnyRow::Relate(row) => row,
                    AnyRow::Annotate(row) => row,
                    _ => return Err(native::abi_error()),
                };
                value
                    .downcast_ref::<Row<F>>()
                    .map(Row::outcome)
                    .ok_or_else(native::abi_error)
            }
            Owned::Entry(entry) => {
                let value: &dyn Any = match entry {
                    AnnotationEntry::Decide(value) => value,
                    AnnotationEntry::Choose(value) => value,
                    AnnotationEntry::Tag(value) => value,
                    AnnotationEntry::Score(value) => value,
                    AnnotationEntry::Find(value) => value,
                    AnnotationEntry::Recognize(value) => value,
                    AnnotationEntry::Relate(value) => value,
                    _ => return Err(native::abi_error()),
                };
                value.downcast_ref().ok_or_else(native::abi_error)
            }
        }
    }
    pub(crate) fn answer<F: Function>(&self) -> Result<&F::Answer, Error> {
        match self.typed::<F>()? {
            Outcome::Answered { answer, .. } => Ok(answer),
            Outcome::Failed { error } => Err(error.clone()),
            _ => Err(native::abi_error()),
        }
    }
    pub(crate) fn detail<F: Function>(&self) -> Result<Option<&F::Detail>, Error> {
        match self.typed::<F>()? {
            Outcome::Answered { detail, .. } | Outcome::Unsure { detail, .. } => {
                Ok(detail.as_ref())
            }
            _ => Ok(None),
        }
    }
    pub(crate) fn status(&self) -> Result<thinkthen::results::Status, Error> {
        Ok(match &self.0 {
            Owned::Row(row) => row.status(),
            Owned::Entry(entry) => {
                macro_rules! status{($($variant:ident),*)=>{match entry{$(AnnotationEntry::$variant(value)=>value.status(),)*_=>return Err(native::abi_error())}};}
                status!(Decide, Choose, Tag, Score, Find, Recognize, Relate)
            }
        })
    }
    pub(crate) fn probability(&self) -> Result<f64, Error> {
        fn read<A, D>(outcome: &Outcome<A, D>) -> f64 {
            match outcome {
                Outcome::Answered { probability, .. } => {
                    probability.map_or(f64::NAN, thinkthen::Probability::get)
                }
                Outcome::Unsure { probability, .. } => probability.get(),
                _ => f64::NAN,
            }
        }
        Ok(match &self.0 {
            Owned::Row(row) => row
                .probability()
                .map_or(f64::NAN, thinkthen::Probability::get),
            Owned::Entry(entry) => {
                macro_rules! probability{($($variant:ident),*)=>{match entry{$(AnnotationEntry::$variant(value)=>read(value),)*_=>return Err(native::abi_error())}};}
                probability!(Decide, Choose, Tag, Score, Find, Recognize, Relate)
            }
        })
    }
    pub(crate) fn error(&self) -> Result<Option<&Error>, Error> {
        Ok(match &self.0 {
            Owned::Row(row) => row.error(),
            Owned::Entry(entry) => {
                fn failed<A, D>(outcome: &Outcome<A, D>) -> Option<&Error> {
                    match outcome {
                        Outcome::Failed { error } => Some(error),
                        _ => None,
                    }
                }
                macro_rules! error{($($variant:ident),*)=>{match entry{$(AnnotationEntry::$variant(value)=>failed(value),)*_=>return Err(native::abi_error())}};}
                error!(Decide, Choose, Tag, Score, Find, Recognize, Relate)
            }
        })
    }
}

pub(crate) unsafe fn project<T>(
    out: *mut T,
    error: *mut crate::HostError,
    read: impl FnOnce() -> Result<T, Error>,
) -> i32 {
    /* SAFETY: each export lends exclusive writable output slots until return. */
    unsafe {
        crate::exports::operation(error, || {
            out.write(read()?);
            Ok(crate::THINKTHEN_OK)
        })
    }
}

// Answer access is distinct from metadata/detail inspection: unsure never writes an answer.
pub(crate) unsafe fn answer_project<T>(
    value: *const ValueHandle,
    out: *mut T,
    error: *mut crate::HostError,
    functions: &[thinkthen::FunctionName],
    read: impl FnOnce() -> Result<T, Error>,
) -> i32 {
    /* SAFETY: the live value is borrowed and output slots are exclusive. */
    unsafe {
        crate::exports::operation(error, || {
            if !functions.contains(&live(value)?.function()?) {
                return Err(thinkthen::native::argument_error(
                    thinkthen::Code::UsageFunction,
                ));
            }
            if live(value)?.status()? == thinkthen::results::Status::Unsure {
                return Ok(crate::THINKTHEN_UNSURE);
            }
            if let Some(error) = live(value)?.error()? {
                return Err(error.clone());
            }
            out.write(read()?);
            Ok(crate::THINKTHEN_OK)
        })
    }
}

//! Typed complex projections own decoded values, never borrowed row allocations.
#![allow(
    unsafe_code,
    reason = "projections read supported native prefixes and counted owned buffers"
)]
mod answers;
mod detail;
mod metadata;
mod views;
use crate::{
    HostRow,
    memory::{row_size, text},
};
pub use answers::*;
pub use detail::*;
pub use metadata::*;
use std::any::Any;
use thinkthen::{
    Error, FunctionName, native,
    results::{AnnotationEntry, AnyRow, Function, Outcome, Row},
};
pub use views::*;
#[derive(Debug)]
enum Owned {
    Row(AnyRow),
    Entry(AnnotationEntry),
}
/// An independent typed snapshot; every returned view borrows this owner.
#[derive(Debug)]
pub struct ValueHandle(Owned, Option<crate::original::Original>);
impl ValueHandle {
    unsafe fn from_row(row: *const HostRow) -> Result<Self, Error> {
        /* SAFETY: only the supported prefix is read after size validation. */
        unsafe {
            row_size(row.cast_mut())?;
        }
        /* SAFETY: this row and its unique buffers remain live through this copy. */
        let row = unsafe { &*row };
        let mut value = serde_json::Map::new();
        value.insert(
            "function".into(),
            serde_json::to_value(
                FunctionName::from_abi(row.function).ok_or_else(native::abi_error)?,
            )
            .map_err(|_| native::abi_error())?,
        );
        value.insert(
            "status".into(),
            serde_json::to_value(
                thinkthen::results::Status::from_abi(row.status).ok_or_else(native::abi_error)?,
            )
            .map_err(|_| native::abi_error())?,
        );
        value.insert("position".into(), row.position.into());
        if !row.probability.is_nan() {
            value.insert("probability".into(), row.probability.into());
        }
        for (name, buffer) in [
            ("id", &row.id),
            ("answer", &row.answer),
            ("detail", &row.detail),
            ("source", &row.source),
            ("error", &row.error),
        ] {
            if buffer.len != 0 {
                /* SAFETY: each counted buffer belongs to the live row. */
                let data = unsafe { text(buffer.data, buffer.len) }?;
                value.insert(
                    name.into(),
                    serde_json::from_str(data).map_err(|_| native::abi_error())?,
                );
            }
        }
        serde_json::from_value(serde_json::Value::Object(value))
            .map(|value| {
                Self(
                    Owned::Row(value),
                    if row.function == thinkthen::FunctionName::Filter.abi() {
                        unsafe { row.source.original().cloned() }
                    } else {
                        None
                    },
                )
            })
            .map_err(|_| native::abi_error())
    }
    fn function(&self) -> Result<FunctionName, Error> {
        Ok(match &self.0 {
            Owned::Row(row) => row.function(),
            Owned::Entry(entry) => match entry {
                AnnotationEntry::Decide(_) => FunctionName::Decide,
                AnnotationEntry::Choose(_) => FunctionName::Choose,
                AnnotationEntry::Tag(_) => FunctionName::Tag,
                AnnotationEntry::Score(_) => FunctionName::Score,
                AnnotationEntry::Filter(_) => FunctionName::Filter,
                AnnotationEntry::Find(_) => FunctionName::Find,
                AnnotationEntry::Recognize(_) => FunctionName::Recognize,
                AnnotationEntry::Relate(_) => FunctionName::Relate,
                _ => return Err(native::abi_error()),
            },
        })
    }
    fn typed<F: Function>(&self) -> Result<&Outcome<F::Answer, F::Detail>, Error> {
        if self.function()? != F::NAME {
            return Err(native::abi_error());
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
                    AnnotationEntry::Filter(value) => value,
                    AnnotationEntry::Find(value) => value,
                    AnnotationEntry::Recognize(value) => value,
                    AnnotationEntry::Relate(value) => value,
                    _ => return Err(native::abi_error()),
                };
                value.downcast_ref().ok_or_else(native::abi_error)
            }
        }
    }
    fn answer<F: Function>(&self) -> Result<&F::Answer, Error> {
        match self.typed::<F>()? {
            Outcome::Answered { answer, .. } => Ok(answer),
            _ => Err(native::abi_error()),
        }
    }
    fn detail<F: Function>(&self) -> Result<Option<&F::Detail>, Error> {
        match self.typed::<F>()? {
            Outcome::Answered { detail, .. } | Outcome::Unsure { detail, .. } => {
                Ok(detail.as_ref())
            }
            _ => Ok(None),
        }
    }
    fn status(&self) -> Result<thinkthen::results::Status, Error> {
        Ok(match &self.0 {
            Owned::Row(row) => row.status(),
            Owned::Entry(entry) => {
                macro_rules! status{($($variant:ident),*)=>{match entry{$(AnnotationEntry::$variant(value)=>value.status(),)*_=>return Err(native::abi_error())}};}
                status!(Decide, Choose, Tag, Score, Filter, Find, Recognize, Relate)
            }
        })
    }
    fn error(&self) -> Result<Option<&Error>, Error> {
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
                error!(Decide, Choose, Tag, Score, Filter, Find, Recognize, Relate)
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
            Ok(crate::exports::OK)
        })
    }
}

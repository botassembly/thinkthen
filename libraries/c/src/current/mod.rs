//! Owned native author snapshots for parsed questions.
pub(crate) mod author;
pub(crate) mod question;
use crate::ffi::carriers::{OptionalStringV1, StringV1, StringsV1};
use std::fmt;

/// Immutable question metadata, independent of every constructor buffer.
pub struct QuestionHandle {
    pub(crate) author: Box<author::AuthorOwner>,
    pub(crate) definition: crate::session::views::ValueHandle,
}
impl fmt::Debug for QuestionHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("QuestionHandle").finish_non_exhaustive()
    }
}

pub(crate) use crate::session::views::Storage;
impl Storage {
    pub(crate) fn string(&mut self, value: &str) -> StringV1 {
        let value = self.text(value);
        StringV1 {
            data: value.data,
            len: value.len,
        }
    }
    pub(crate) fn optional_string(&mut self, value: Option<&str>) -> OptionalStringV1 {
        value.map_or_else(OptionalStringV1::default, |value| OptionalStringV1 {
            present: 1,
            value: self.string(value),
        })
    }
    pub(crate) fn strings(&mut self, strings: &[String]) -> StringsV1 {
        let strings = strings.iter().map(|string| self.string(string)).collect();
        let (data, len) = self.slice(strings);
        StringsV1 { data, len }
    }
}

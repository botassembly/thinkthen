//! Owned native author snapshots for parsed questions.
pub(crate) mod author;
pub(crate) mod question;
use crate::ffi::carriers::{OptionalStringV1, StringV1, StringsV1};
use std::fmt;

/// Immutable question metadata, independent of every constructor buffer.
#[derive(Clone)]
pub struct QuestionHandle {
    pub(crate) author: Box<author::AuthorOwner>,
}
impl fmt::Debug for QuestionHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("QuestionHandle").finish_non_exhaustive()
    }
}

/// Owned view allocations. Moving the owner cannot move their bytes.
#[derive(Default)]
pub(crate) struct Storage(Vec<Box<dyn std::any::Any>>);
impl Storage {
    pub(crate) fn array<T: 'static>(&mut self, values: Vec<T>) -> (*const T, usize) {
        if values.is_empty() {
            return (std::ptr::null(), 0);
        }
        let values = values.into_boxed_slice();
        let pair = (values.as_ptr(), values.len());
        self.0.push(Box::new(values));
        pair
    }
    pub(crate) fn string(&mut self, value: &str) -> StringV1 {
        let (data, len) = self.array(value.as_bytes().to_vec());
        StringV1 {
            data: data.cast(),
            len,
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
        let (data, len) = self.array(strings);
        StringsV1 { data, len }
    }
}

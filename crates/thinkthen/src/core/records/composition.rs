//! Shared typed record composition, over the ordinary record and pointer reader.
use super::{Held, Reading, Record, RecordError, found};
use crate::core::{Json, Pointer};

impl Record {
    pub(crate) fn context_text(&self, pointer: &Pointer) -> Result<&str, RecordError> {
        let Held::Json(value) = &self.0 else {
            return Err(RecordError::TextHasNoMembers);
        };
        found(pointer, value)?
            .as_str()
            .ok_or_else(|| RecordError::ContextText(pointer.as_str().to_owned()))
    }

    pub(crate) fn text(&self) -> Option<&str> {
        match &self.0 {
            Held::Text(text) => Some(text),
            Held::Json(_) => None,
        }
    }

    pub(crate) fn json(&self) -> Option<&Json> {
        match &self.0 {
            Held::Text(_) => None,
            Held::Json(value) => Some(value),
        }
    }
}

impl Reading {
    pub(crate) fn has_fields(&self) -> bool {
        !self.fields.is_empty()
    }
}

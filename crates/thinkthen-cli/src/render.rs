//! Compact command output with a cause that withholds the serialized document.

use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct RenderError;

impl fmt::Display for RenderError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("the document could not be written as JSON")
    }
}

impl std::error::Error for RenderError {}

pub(crate) fn json_line<T: serde::Serialize>(document: &T) -> Result<String, RenderError> {
    serde_json::to_string(document).map_err(|_| RenderError)
}

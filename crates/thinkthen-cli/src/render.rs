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

/// Quote a caller selector inside an existing command sentence.
pub(crate) fn safe_key(key: &str) -> String {
    let encoded = serde_json::to_string(key).unwrap_or_else(|_| "\"<unprintable>\"".to_owned());
    encoded
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .unwrap_or("<unprintable>")
        .to_owned()
}

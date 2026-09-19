//! One compact JSON line, which is how every document leaves the tool.

use serde::Serialize;
use thiserror::Error;

/// Why a document could not be written as JSON.
///
/// The error names no cause. A cause a JSON writer gives quotes the value it
/// stopped on, and the document being written holds the evidence.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("the document could not be written as JSON")]
pub struct RenderError;

/// Write one document as the compact line standard output carries.
///
/// # Errors
///
/// Returns [`RenderError`] when the document cannot be written as JSON.
pub fn json_line<T: Serialize>(document: &T) -> Result<String, RenderError> {
    serde_json::to_string(document).map_err(|_| RenderError)
}

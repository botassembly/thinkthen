//! The value `--by POINTER` groups each result line under: a field of its input.

use std::collections::BTreeMap;

use crate::core::json::Json;
use crate::core::measure::{MeasureError, record_id};
use crate::core::pointer::Pointer;

/// The group value of each numbered line: the string or integer at the pointer inside its input.
///
/// # Errors
///
/// Returns [`MeasureError::NoGroup`] for the first line with no input, or with
/// a value at the pointer that is missing or neither a string nor an integer.
pub(crate) fn values(
    lines: &[(usize, Json)],
    pointer: &Pointer,
) -> Result<BTreeMap<usize, String>, MeasureError> {
    lines
        .iter()
        .map(|(line, row)| {
            row.member("input")
                .and_then(|input| pointer.resolve(input))
                .and_then(record_id)
                .map(|value| (*line, value))
                .ok_or(MeasureError::NoGroup(*line))
        })
        .collect()
}

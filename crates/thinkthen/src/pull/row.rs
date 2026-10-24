//! Host rows retain detail as unparsed JSON text.

use crate::{Error, ErrorKind, Record, Row, SdkCall};
use serde_json::Value;

/// String index convention selected once per call.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OffsetUnit {
    /// Count UTF-8 bytes.
    Utf8,
    /// Count UTF-16 code units.
    Utf16,
    /// Count Unicode scalar values.
    CodePoints,
}

/// One row ready for a host, with detail kept outside its ordinary JSON parse.
#[derive(Debug)]
pub struct PullRow {
    /// Row fields except detail, encoded as JSON.
    pub json: String,
    /// Detail encoded separately, parsed only when the host caller asks for it.
    pub detail: Option<String>,
    /// Input position, starting at one, for retaining host object identity.
    pub position: usize,
    /// Whether the filter selected this record.
    pub kept: bool,
    failure: Option<Error>,
}

impl PullRow {
    /// The original outcome failure, including a failed named annotation.
    #[must_use]
    pub const fn failure(&self) -> Option<&Error> {
        self.failure.as_ref()
    }

    pub(super) fn new(
        call: &SdkCall,
        record: &Record,
        row: Row,
        position: usize,
        offsets: OffsetUnit,
    ) -> Result<Self, Error> {
        let kept = crate::passes(row.outcome());
        let failure = row.outcome().failure().cloned();
        let mut value = serde_json::to_value(row).map_err(encoding)?;
        if call.is_annotate() {
            if let Some(entries) = value.get_mut("answer").and_then(Value::as_object_mut) {
                for (name, outcome) in entries {
                    if let Ok(text) = call.evidence(record, Some(name)) {
                        translate(outcome, &text, offsets);
                    }
                }
            }
        } else if let Ok(text) = call.evidence(record, None) {
            if let Some(answer) = value.get_mut("answer") {
                translate(answer, &text, offsets);
            }
            if let Some(detail) = value.get_mut("detail") {
                translate(detail, &text, offsets);
            }
        }
        let detail = value
            .as_object_mut()
            .and_then(|fields| fields.remove("detail"))
            .map(|detail| detail.to_string());
        Ok(Self {
            json: value.to_string(),
            detail,
            position,
            kept,
            failure,
        })
    }
}

fn encoding(error: serde_json::Error) -> Error {
    Error::new(ErrorKind::Io, error.to_string())
}

fn translate(value: &mut Value, text: &str, unit: OffsetUnit) {
    match value {
        Value::Array(items) => items
            .iter_mut()
            .for_each(|item| translate(item, text, unit)),
        Value::Object(fields) => {
            if fields.contains_key("start") && fields.contains_key("end") {
                for key in ["start", "end"] {
                    if let Some(offset) = fields.get(key).and_then(Value::as_u64) {
                        let index: usize = text
                            .chars()
                            .take(usize::try_from(offset).unwrap_or(usize::MAX))
                            .map(|c| match unit {
                                OffsetUnit::Utf8 => c.len_utf8(),
                                OffsetUnit::Utf16 => c.len_utf16(),
                                OffsetUnit::CodePoints => 1,
                            })
                            .sum();
                        fields.insert(key.into(), Value::from(index));
                    }
                }
            }
            for item in fields.values_mut() {
                translate(item, text, unit);
            }
        }
        _ => {}
    }
}

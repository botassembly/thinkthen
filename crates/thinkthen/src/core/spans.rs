//! Shared translation applies the selected span convention to answers and detail.
use super::{Answer, Detail, Outcome};
super::abi::discriminators! {
    /// String index convention selected once per call.
    #[non_exhaustive]
    #[derive(Debug,Clone,Copy,PartialEq,Eq,serde::Serialize,serde::Deserialize)]
    #[serde(rename_all="snake_case")]
    pub enum OffsetUnit {
        /// Zero-based UTF-8 bytes with an exclusive end.
        Utf8=0,
        /// Zero-based UTF-16 units with an exclusive end.
        Utf16=1,
        /// Zero-based Unicode scalar values with an exclusive end.
        CodePoints=2,
        /// One-based Unicode scalar values with an inclusive end, as R uses.
        CodePointsInclusive=3,
    }
}
pub(crate) fn span(text: &str, start: usize, end: usize, unit: OffsetUnit) -> (usize, usize) {
    if unit == OffsetUnit::CodePointsInclusive {
        return (start.saturating_add(1), end);
    }
    let offset = |n| {
        text.chars()
            .take(n)
            .map(|c| match unit {
                OffsetUnit::Utf8 => c.len_utf8(),
                OffsetUnit::Utf16 => c.len_utf16(),
                OffsetUnit::CodePoints | OffsetUnit::CodePointsInclusive => 1,
            })
            .sum()
    };
    (offset(start), offset(end))
}
fn endpoint(value: &mut super::RelationName, text: &str, unit: OffsetUnit) {
    (value.start, value.end) = span(text, value.start, value.end, unit);
}
fn relations(values: &mut [super::Relation], text: &str, unit: OffsetUnit) {
    for value in values {
        endpoint(&mut value.source, text, unit);
        endpoint(&mut value.target, text, unit);
    }
}
fn detail(value: &mut Detail, text: &str, unit: OffsetUnit) {
    match value {
        Detail::Recognize(value) => {
            for piece in &mut value.pieces {
                (piece.start, piece.end) = span(text, piece.start, piece.end, unit);
            }
            for name in &mut value.names {
                (name.start, name.end) = span(text, name.start, name.end, unit);
            }
        }
        Detail::Relate(value) => {
            for name in &mut value.names {
                (name.start, name.end) = span(text, name.start, name.end, unit);
            }
            relations(&mut value.pairs, text, unit);
        }
        _ => {}
    }
}
pub(crate) fn outcome(value: &mut Outcome, text: &str, unit: OffsetUnit) {
    match value {
        Outcome::Answered {
            answer,
            detail: details,
            ..
        } => {
            match answer {
                Answer::Found(Some(found)) => {
                    (found.start, found.end) = span(text, found.start, found.end, unit)
                }
                Answer::Names(values) => {
                    for value in values {
                        (value.start, value.end) = span(text, value.start, value.end, unit);
                    }
                }
                Answer::Relations(values) => relations(values, text, unit),
                _ => {}
            }
            if let Some(value) = details {
                detail(value, text, unit);
            }
        }
        Outcome::Unsure {
            detail: Some(value),
            ..
        } => detail(value, text, unit),
        _ => {}
    }
}

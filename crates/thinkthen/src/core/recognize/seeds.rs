//! Caller proposals use the same exact piece boundaries as decoded proposals.
use super::Piece;
use crate::core::{Json, Pointer, RecognizeSpec, Record};
use serde::{Deserialize, Serialize};

/// An unconfirmed proposal in zero-based Unicode scalar `[start, end)` offsets.
/// Both edges must exactly match recognition piece edges; admission never rounds.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RecognitionSeedSpan {
    /// Inclusive scalar offset into the actual recognition evidence.
    pub start: usize,
    /// Exclusive scalar offset into the actual recognition evidence.
    pub end: usize,
    /// Optional proposed declared kind; it never restricts classification.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(test, schemars(with = "String"))]
    pub kind: Option<String>,
}
impl std::fmt::Debug for RecognitionSeedSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RecognitionSeedSpan(<withheld>)")
    }
}
fn present<'de, D: serde::Deserializer<'de>>(de: D) -> Result<Option<String>, D::Error> {
    String::deserialize(de).map(Some)
}

pub(crate) fn seed_stretches(
    spec: &RecognizeSpec,
    pieces: &[Piece],
) -> Result<Vec<(usize, usize)>, &'static str> {
    let mut stretches = Vec::new();
    for seed in &spec.seed_spans {
        if seed
            .kind
            .as_ref()
            .is_some_and(|kind| !spec.kinds.iter().any(|(name, _)| name == kind))
        {
            return Err("recognition seed kind must match a declared kind");
        }
        let first = pieces.iter().position(|piece| piece.start == seed.start);
        let last = pieces.iter().position(|piece| piece.end == seed.end);
        let (Some(first), Some(last)) = (first, last) else {
            return Err("recognition seed edges must exactly match piece edges");
        };
        if seed.start >= seed.end || first > last {
            return Err("recognition seed requires a nonempty span within the evidence");
        }
        stretches.push((first, last));
    }
    stretches.sort_unstable();
    stretches.dedup();
    Ok(stretches)
}

pub(crate) fn selected_seeds(
    record: &Record,
    pointer: &Pointer,
) -> Result<Option<Vec<RecognitionSeedSpan>>, &'static str> {
    let value = record
        .json()
        .ok_or("recognition seed pointer requires a JSON record")?;
    let Some(value) = pointer
        .select_optional(value)
        .map_err(|_| "recognition seed pointer traverses an invalid container")?
    else {
        return Ok(None);
    };
    let Json::Array(_) = value else {
        return Err("recognition seed spans must be an array");
    };
    let text =
        crate::core::json_line(value).map_err(|_| "recognition seed spans could not be decoded")?;
    serde_json::from_str(&text).map(Some).map_err(
        |_| "recognition seed spans require integer start/end and an optional declared kind",
    )
}

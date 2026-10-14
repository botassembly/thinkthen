//! Preserve the legacy C ranked-record schema without a library result carrier.

use serde::Serialize;

/// One ranked record: its zero-based input place, its text and its
/// probability of yes, as `{"index", "record", "probability"}`.
#[derive(Clone, Copy, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "rankedRecord"))]
pub(crate) struct RankedRow<'a> {
    index: usize,
    record: &'a str,
    probability: f64,
}

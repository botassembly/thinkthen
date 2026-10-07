//! Shared typed recognition values and distributions for native and CLI results.

use serde::Serialize;

use crate::core::{NameOdds, PieceOdds, RecognizedName, RelationEdge};

/// The names one text holds, and the edges between them when rules were given.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "recognize"))]
pub(crate) struct RecognizedValue {
    pub(crate) entities: Vec<RecognizedName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) relations: Option<Vec<RelationEdge<RecognizedName>>>,
}

/// Every probability behind one text's names, as `--details` keeps them.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[cfg_attr(test, schemars(rename = "recognizeAnswer"))]
pub(crate) struct RecognitionOdds {
    pub(crate) pieces: Vec<PieceOdds>,
    pub(crate) names: Vec<NameOdds>,
    pub(crate) pairs: Vec<PairOdds>,
}

/// One asked pair's probability.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "pairOdds"))]
pub(crate) struct PairOdds {
    pub(crate) relation: String,
    pub(crate) source: Place,
    pub(crate) target: Place,
    pub(crate) probability: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "place"))]
pub(crate) struct Place {
    pub(crate) start: usize,
    pub(crate) end: usize,
}

impl Place {
    pub(crate) const fn of(name: &RecognizedName) -> Self {
        Self {
            start: name.start,
            end: name.end,
        }
    }
}

//! Shared typed recognition values and distributions for native and CLI results.

use serde::Serialize;

use crate::core::{NameOdds, PieceOdds, RecognizedName, RelationEdge};

/// The names one text holds, and the edges between them when rules were given.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "recognize"))]
#[serde(untagged)]
pub(crate) enum RecognizedValue {
    Whole {
        entities: Vec<RecognizedName>,
        #[serde(skip_serializing_if = "Option::is_none")]
        relations: Option<Vec<RelationEdge<RecognizedName>>>,
    },
    BoundaryOnly {
        mode: BoundaryMode,
        proposals: Vec<BoundaryProposal>,
    },
}

/// The sole tag for a boundary proposal value.
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub(crate) enum BoundaryMode {
    BoundaryOnly,
}

/// An unclassified decoded stretch with its valid-path probability.
#[derive(Clone, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub(crate) struct BoundaryProposal {
    pub(crate) text: String,
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) length: usize,
    pub(crate) probability: f64,
}
impl std::fmt::Debug for BoundaryProposal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BoundaryProposal")
            .field("text", &crate::core::text::Withheld(self.text.len()))
            .field("start", &self.start)
            .field("end", &self.end)
            .field("probability", &self.probability)
            .finish()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(untagged)]
pub(crate) enum RecognitionOdds {
    Whole(WholeRecognitionOdds),
    BoundaryOnly(BoundaryOdds),
}
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub(crate) struct BoundaryOdds {
    pub(crate) pieces: Vec<PieceOdds>,
    pub(crate) proposals: Vec<BoundaryProposal>,
}

/// Every probability behind one text's names, as `--details` keeps them.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[cfg_attr(test, schemars(rename = "recognizeAnswer"))]
pub(crate) struct WholeRecognitionOdds {
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

impl BoundaryProposal {
    pub(crate) fn decoded(
        text: &str,
        pieces: &[crate::core::Piece],
        rows: &[crate::core::TagRow],
        found: &[(usize, usize)],
    ) -> Vec<Self> {
        let spans = crate::core::recognize::SpanOdds::new(rows);
        let mut proposals = found
            .iter()
            .filter_map(|&(first, last)| {
                let first_piece = pieces.get(first)?;
                let last_piece = pieces.get(last)?;
                Some(Self {
                    text: text
                        .get(first_piece.byte_start..last_piece.byte_end)?
                        .to_owned(),
                    start: first_piece.start,
                    end: last_piece.end,
                    length: last_piece.end - first_piece.start,
                    probability: crate::core::recognize::strength(1.0, spans.span(first, last)),
                })
            })
            .collect::<Vec<_>>();
        proposals.sort_by_key(|proposal| (proposal.start, proposal.end));
        proposals
    }
}

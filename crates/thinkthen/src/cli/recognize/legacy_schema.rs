//! Result/1 retains its original required odds while accepting added proposals.

use crate::core::{BoundaryOdds, NameOdds, PairOdds, PieceOdds, RecognitionProposal};

#[derive(schemars::JsonSchema)]
#[schemars(rename = "legacyRecognitionOdds")]
#[serde(untagged)]
#[expect(
    dead_code,
    reason = "only the legacy schema uses this typed projection"
)]
pub(super) enum RecognitionOdds {
    Whole(WholeOdds),
    BoundaryOnly(BoundaryOdds),
}

#[derive(schemars::JsonSchema)]
#[schemars(rename = "legacyRecognizeAnswer")]
#[expect(
    dead_code,
    reason = "only the legacy schema uses this typed projection"
)]
pub(super) struct WholeOdds {
    pieces: Vec<PieceOdds>,
    names: Vec<NameOdds>,
    #[serde(default)]
    proposals: Vec<RecognitionProposal>,
    pairs: Vec<PairOdds>,
}

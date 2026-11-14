//! Complete borrowed recognition detail in C's counted-array form.
use crate::values::{ProbabilityView, SpanView};
/// One input piece and its boundary-tag probabilities, borrowed from a row.
#[repr(C)]
#[derive(Debug)]
pub struct PieceDetailView {
    /// The original piece and its offsets in the call's host units.
    pub span: SpanView,
    /// Every boundary-tag probability, including unselected tags.
    pub tags: *const ProbabilityView,
    /// The number of tags.
    pub tag_count: usize,
}
/// One proposed name and its complete kind probabilities, borrowed from a row.
#[repr(C)]
#[derive(Debug)]
pub struct KindDetailView {
    /// The proposed name and its offsets in the call's host units.
    pub span: SpanView,
    /// Every kind probability, including unselected kinds.
    pub kinds: *const ProbabilityView,
    /// The number of kinds.
    pub kind_count: usize,
}
/// Complete recognition detail. Missing detail has both counts zero in every state.
#[repr(C)]
#[derive(Debug)]
pub struct RecognitionDetailView {
    /// The original pieces and their boundary-tag distributions.
    pub pieces: *const PieceDetailView,
    /// The number of pieces.
    pub piece_count: usize,
    /// The proposed names and their kind distributions.
    pub names: *const KindDetailView,
    /// The number of proposed names.
    pub name_count: usize,
}

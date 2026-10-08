//! Canonical reviewed C layouts; see include/thinkthen.h.
use super::{
    ContentV1, OptionalDoubleV1, OptionalSizeV1, OptionalStringV1, QuestionViewV1, RuleV1,
    StringV1, StringsV1,
};
/// C union `DecideValueDataV1`; the parent discriminator selects its active arm.
#[repr(C)]
#[derive(Clone, Copy)]
pub union DecideValueDataV1 {
    /// Active arm selected by the parent discriminator.
    pub boolean: std::ffi::c_int,
    /// C field `authored`.
    pub authored: ContentV1,
}

impl std::fmt::Debug for DecideValueDataV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DecideValueDataV1").finish_non_exhaustive()
    }
}
/// BOOLEAN is used only for ordinary un-authored true/false decisions.
/// AUTHORED preserves the chosen user meaning even if it spells a Boolean;
/// successful uncertainty always uses NULL. No JSON decode for that case.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct DecideValueV1 {
    /// C field `kind`.
    pub kind: u32,
    /// C field `data`.
    pub data: DecideValueDataV1,
}
/// C descriptor or borrowed view `ProbabilityV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ProbabilityV1 {
    /// C field `name`.
    pub name: StringV1,
    /// C field `probability`.
    pub probability: f64,
}
/// C descriptor or borrowed view `ProbabilitiesV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ProbabilitiesV1 {
    /// C field `data`.
    pub data: *const ProbabilityV1,
    /// C field `len`.
    pub len: usize,
}
/// C descriptor or borrowed view `OptionalProbabilitiesV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct OptionalProbabilitiesV1 {
    /// C field `present`.
    pub present: std::ffi::c_int,
    /// C field `value`.
    pub value: ProbabilitiesV1,
}
/// C descriptor or borrowed view `NamedAnswerV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct NamedAnswerV1 {
    /// C field `pick`.
    pub pick: StringV1,
    /// C field `probabilities`.
    pub probabilities: ProbabilitiesV1,
    /// C field `confidence`.
    pub confidence: OptionalDoubleV1,
}
/// C descriptor or borrowed view `ScoreAnswerV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ScoreAnswerV1 {
    /// C field `level`.
    pub level: StringV1,
    /// C field `probabilities`.
    pub probabilities: ProbabilitiesV1,
    /// C field `confidence`.
    pub confidence: OptionalDoubleV1,
}
/// C union `AnswerDataV1`; the parent discriminator selects its active arm.
#[repr(C)]
#[derive(Clone, Copy)]
pub union AnswerDataV1 {
    /// Active arm selected by the parent discriminator.
    pub probability: f64,
    /// C field `choice`.
    pub choice: NamedAnswerV1,
    /// C field `tag`.
    pub tag: ProbabilitiesV1,
    /// C field `score`.
    pub score: ScoreAnswerV1,
    /// C field `find`.
    pub find: NamedAnswerV1,
}

impl std::fmt::Debug for AnswerDataV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnswerDataV1").finish_non_exhaustive()
    }
}
/// C descriptor or borrowed view `AnswerV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct AnswerV1 {
    /// C field `kind`.
    pub kind: u32,
    /// C field `data`.
    pub data: AnswerDataV1,
}
/// C descriptor or borrowed view `OptionalAnswerV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct OptionalAnswerV1 {
    /// C field `present`.
    pub present: std::ffi::c_int,
    /// C field `value`.
    pub value: AnswerV1,
}
/// C descriptor or borrowed view `LocationV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct LocationV1 {
    /// C field `file`.
    pub file: OptionalStringV1,
    /// C field `first_line`.
    pub first_line: OptionalSizeV1,
    /// C field `last_line`.
    pub last_line: OptionalSizeV1,
}
/// C descriptor or borrowed view `OptionalLocationV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct OptionalLocationV1 {
    /// C field `present`.
    pub present: std::ffi::c_int,
    /// C field `value`.
    pub value: LocationV1,
}
/// C union `MemberValueDataV1`; the parent discriminator selects its active arm.
#[repr(C)]
#[derive(Clone, Copy)]
pub union MemberValueDataV1 {
    /// Active arm selected by the parent discriminator.
    pub decide: DecideValueV1,
    /// C field `choose`.
    pub choose: OptionalStringV1,
    /// C field `tag`.
    pub tag: StringsV1,
    /// C field `score`.
    pub score: f64,
}

impl std::fmt::Debug for MemberValueDataV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemberValueDataV1").finish_non_exhaustive()
    }
}
/// Annotate admits these four kinds. The discriminator uses FUNCTION_*.
/// A null decide/choose is a successful typed value, not member failure.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct MemberValueV1 {
    /// C field `kind`.
    pub kind: u32,
    /// C field `data`.
    pub data: MemberValueDataV1,
}
/// C descriptor or borrowed view `MemberFailureV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct MemberFailureV1 {
    /// C field `failure_id`.
    pub failure_id: StringV1,
    /// C field `cause`.
    pub cause: u32,
}
/// C descriptor or borrowed view `MemberSuccessV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct MemberSuccessV1 {
    /// C field `answer_id`.
    pub answer_id: StringV1,
    /// C field `value`.
    pub value: MemberValueV1,
    /// C field `answer`.
    pub answer: AnswerV1,
    /// C field `threshold`.
    pub threshold: RuleV1,
}
/// C union `MemberDataV1`; the parent discriminator selects its active arm.
#[repr(C)]
#[derive(Clone, Copy)]
pub union MemberDataV1 {
    /// Active arm selected by the parent discriminator.
    pub success: MemberSuccessV1,
    /// C field `failure`.
    pub failure: MemberFailureV1,
}

impl std::fmt::Debug for MemberDataV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemberDataV1").finish_non_exhaustive()
    }
}
/// C descriptor or borrowed view `MemberV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct MemberV1 {
    /// C field `name`.
    pub name: StringV1,
    /// C field `request`.
    pub request: StringV1,
    /// C field `question`.
    pub question: QuestionViewV1,
    /// C field `state`.
    pub state: u32,
    /// C field `data`.
    pub data: MemberDataV1,
}
/// C descriptor or borrowed view `MembersV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct MembersV1 {
    /// C field `data`.
    pub data: *const MemberV1,
    /// C field `len`.
    pub len: usize,
}
/// C descriptor or borrowed view `EntityV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct EntityV1 {
    /// C field `text`.
    pub text: StringV1,
    /// C field `start`.
    pub start: usize,
    /// C field `end`.
    pub end: usize,
    /// C field `length`.
    pub length: usize,
    /// C field `kind`.
    pub kind: StringV1,
    /// C field `strength`.
    pub strength: f64,
}
/// C descriptor or borrowed view `EntitiesV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct EntitiesV1 {
    /// C field `data`.
    pub data: *const EntityV1,
    /// C field `len`.
    pub len: usize,
}
/// C descriptor or borrowed view `EntityEdgeV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct EntityEdgeV1 {
    /// C field `relation`.
    pub relation: StringV1,
    /// C field `source`.
    pub source: EntityV1,
    /// C field `target`.
    pub target: EntityV1,
    /// C field `probability`.
    pub probability: f64,
    /// C field `either`.
    pub either: std::ffi::c_int,
}
/// C descriptor or borrowed view `EntityEdgesV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct EntityEdgesV1 {
    /// C field `data`.
    pub data: *const EntityEdgeV1,
    /// C field `len`.
    pub len: usize,
}
/// C descriptor or borrowed view `OptionalEntityEdgesV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct OptionalEntityEdgesV1 {
    /// C field `present`.
    pub present: std::ffi::c_int,
    /// C field `value`.
    pub value: EntityEdgesV1,
}
/// C descriptor or borrowed view `PlaceV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct PlaceV1 {
    /// C field `start`.
    pub start: usize,
    /// C field `end`.
    pub end: usize,
}
/// C descriptor or borrowed view `PieceV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct PieceV1 {
    /// C field `start`.
    pub start: usize,
    /// C field `end`.
    pub end: usize,
    /// C field `tags`.
    pub tags: ProbabilitiesV1,
}
/// C descriptor or borrowed view `PiecesV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct PiecesV1 {
    /// C field `data`.
    pub data: *const PieceV1,
    /// C field `len`.
    pub len: usize,
}
/// C descriptor or borrowed view `NameV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct NameV1 {
    /// C field `start`.
    pub start: usize,
    /// C field `end`.
    pub end: usize,
    /// C field `kinds`.
    pub kinds: OptionalProbabilitiesV1,
    /// C field `edges`.
    pub edges: OptionalProbabilitiesV1,
}
/// C descriptor or borrowed view `NamesV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct NamesV1 {
    /// C field `data`.
    pub data: *const NameV1,
    /// C field `len`.
    pub len: usize,
}
/// C descriptor or borrowed view `PairV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct PairV1 {
    /// C field `relation`.
    pub relation: StringV1,
    /// C field `source`.
    pub source: PlaceV1,
    /// C field `target`.
    pub target: PlaceV1,
    /// C field `probability`.
    pub probability: f64,
}
/// C descriptor or borrowed view `PairsV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct PairsV1 {
    /// C field `data`.
    pub data: *const PairV1,
    /// C field `len`.
    pub len: usize,
}
/// C descriptor or borrowed view `RecognizeValueV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct RecognizeValueV1 {
    /// C field `entities`.
    pub entities: EntitiesV1,
    /// C field `relations`.
    pub relations: OptionalEntityEdgesV1,
}
/// C descriptor or borrowed view `RecognizeAnswerV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct RecognizeAnswerV1 {
    /// C field `pieces`.
    pub pieces: PiecesV1,
    /// C field `names`.
    pub names: NamesV1,
    /// C field `pairs`.
    pub pairs: PairsV1,
}
/// C descriptor or borrowed view `EndpointV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct EndpointV1 {
    /// C field `name`.
    pub name: StringV1,
    /// C field `kind`.
    pub kind: StringV1,
}
/// C descriptor or borrowed view `OptionalEndpointV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct OptionalEndpointV1 {
    /// C field `present`.
    pub present: std::ffi::c_int,
    /// C field `value`.
    pub value: EndpointV1,
}
/// C descriptor or borrowed view `EdgeV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct EdgeV1 {
    /// C field `relation`.
    pub relation: StringV1,
    /// C field `source`.
    pub source: EndpointV1,
    /// C field `target`.
    pub target: EndpointV1,
    /// C field `probability`.
    pub probability: f64,
    /// C field `either`.
    pub either: std::ffi::c_int,
}
/// C descriptor or borrowed view `EdgesV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct EdgesV1 {
    /// C field `data`.
    pub data: *const EdgeV1,
    /// C field `len`.
    pub len: usize,
}
/// C descriptor or borrowed view `RelationSuccessV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct RelationSuccessV1 {
    /// C field `answer_id`.
    pub answer_id: StringV1,
    /// C field `probability`.
    pub probability: f64,
    /// C field `accepted`.
    pub accepted: std::ffi::c_int,
}
/// C union `RelationAnswerDataV1`; the parent discriminator selects its active arm.
#[repr(C)]
#[derive(Clone, Copy)]
pub union RelationAnswerDataV1 {
    /// Active arm selected by the parent discriminator.
    pub success: RelationSuccessV1,
    /// C field `failure`.
    pub failure: MemberFailureV1,
}

impl std::fmt::Debug for RelationAnswerDataV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RelationAnswerDataV1")
            .finish_non_exhaustive()
    }
}
/// C descriptor or borrowed view `RelationAnswerV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct RelationAnswerV1 {
    /// C field `relation`.
    pub relation: StringV1,
    /// C field `reads`.
    pub reads: StringV1,
    /// C field `method`.
    pub method: u32,
    /// C field `direction`.
    pub direction: u32,
    /// C field `source`.
    pub source: EndpointV1,
    /// C field `target`.
    pub target: OptionalEndpointV1,
    /// C field `request`.
    pub request: StringV1,
    /// C field `state`.
    pub state: u32,
    /// C field `data`.
    pub data: RelationAnswerDataV1,
}
/// C descriptor or borrowed view `RelationAnswersV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct RelationAnswersV1 {
    /// C field `data`.
    pub data: *const RelationAnswerV1,
    /// C field `len`.
    pub len: usize,
}

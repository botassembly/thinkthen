//! Canonical reviewed C layouts; see include/thinkthen.h.
use super::{
    DecideValueV1, EdgesV1, MemberFailureV1, MemberValueV1, MembersV1, MetaV1, OptionalAnswerV1,
    OptionalAttemptsV1, OptionalContentV1, OptionalDiscriminatorV1, OptionalDoubleV1,
    OptionalErrorV1, OptionalFactsV1, OptionalImageViewsV1, OptionalLocationV1, OptionalMetaV1,
    OptionalQuestionV1, OptionalRuleV1, OptionalSizeV1, OptionalStringV1, OptionalUsageV1,
    ProbabilitiesV1, QuestionSourcesV1, RecognizeAnswerV1, RecognizeValueV1, RelationAnswersV1,
    StringV1, StringsV1,
};
/// C descriptor or borrowed view `RowV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct RowV1 {
    /// C field `answer_id`.
    pub answer_id: StringV1,
    /// C field `input`.
    pub input: OptionalContentV1,
    /// C field `question`.
    pub question: OptionalQuestionV1,
    /// C field `answer`.
    pub answer: OptionalAnswerV1,
    /// C field `threshold`.
    pub threshold: OptionalRuleV1,
    /// C field `position`.
    pub position: OptionalLocationV1,
    /// C field `input_file`.
    pub input_file: OptionalStringV1,
    /// C field `meta`.
    pub meta: MetaV1,
    /// C field `images`.
    pub images: OptionalImageViewsV1,
}
/// C descriptor or borrowed view `DecideViewV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct DecideViewV1 {
    /// C field `common`.
    pub common: RowV1,
    /// C field `value`.
    pub value: DecideValueV1,
}
/// C descriptor or borrowed view `ChooseViewV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ChooseViewV1 {
    /// C field `common`.
    pub common: RowV1,
    /// C field `value`.
    pub value: OptionalStringV1,
}
/// C descriptor or borrowed view `TagViewV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct TagViewV1 {
    /// C field `common`.
    pub common: RowV1,
    /// C field `value`.
    pub value: StringsV1,
}
/// C descriptor or borrowed view `ScoreViewV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ScoreViewV1 {
    /// C field `common`.
    pub common: RowV1,
    /// C field `value`.
    pub value: f64,
}
/// C descriptor or borrowed view `FilterViewV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct FilterViewV1 {
    /// C field `common`.
    pub common: RowV1,
    /// C field `value`.
    pub value: std::ffi::c_int,
}
/// C descriptor or borrowed view `RankViewV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct RankViewV1 {
    /// C field `common`.
    pub common: RowV1,
    /// C field `value`.
    pub value: OptionalSizeV1,
    /// C field `question_name`.
    pub question_name: OptionalStringV1,
}
/// C descriptor or borrowed view `FindViewV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct FindViewV1 {
    /// C field `common`.
    pub common: RowV1,
    /// C field `value`.
    pub value: OptionalContentV1,
    /// C field `index`.
    pub index: OptionalSizeV1,
}
/// C descriptor or borrowed view `AnnotateViewV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct AnnotateViewV1 {
    /// C field `common`.
    pub common: RowV1,
    /// C field `answers`.
    pub answers: MembersV1,
}
/// C descriptor or borrowed view `RecognizeViewV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct RecognizeViewV1 {
    /// C field `common`.
    pub common: RowV1,
    /// C field `value`.
    pub value: RecognizeValueV1,
    /// C field `answer`.
    pub answer: RecognizeAnswerV1,
}
/// C descriptor or borrowed view `RelateViewV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct RelateViewV1 {
    /// C field `common`.
    pub common: RowV1,
    /// C field `value`.
    pub value: EdgesV1,
    /// C field `questions`.
    pub questions: RelationAnswersV1,
}
/// C union `ObservedProbabilitiesDataV1`; the parent discriminator selects its active arm.
#[repr(C)]
#[derive(Clone, Copy)]
pub union ObservedProbabilitiesDataV1 {
    /// Active arm selected by the parent discriminator.
    pub yes: f64,
    /// C field `named`.
    pub named: ProbabilitiesV1,
}

impl std::fmt::Debug for ObservedProbabilitiesDataV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ObservedProbabilitiesDataV1")
            .finish_non_exhaustive()
    }
}
/// C descriptor or borrowed view `ObservedProbabilitiesV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ObservedProbabilitiesV1 {
    /// C field `kind`.
    pub kind: u32,
    /// C field `data`.
    pub data: ObservedProbabilitiesDataV1,
}
/// C descriptor or borrowed view `ObservationSuccessV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ObservationSuccessV1 {
    /// C field `answer_id`.
    pub answer_id: StringV1,
    /// C field `observation_id`.
    pub observation_id: StringV1,
    /// C field `value`.
    pub value: MemberValueV1,
    /// C field `probabilities`.
    pub probabilities: ObservedProbabilitiesV1,
    /// C field `confidence`.
    pub confidence: OptionalDoubleV1,
}
/// C union `QuestionObservationDataV1`; the parent discriminator selects its active arm.
#[repr(C)]
#[derive(Clone, Copy)]
pub union QuestionObservationDataV1 {
    /// Active arm selected by the parent discriminator.
    pub success: ObservationSuccessV1,
    /// C field `failure`.
    pub failure: MemberFailureV1,
}

impl std::fmt::Debug for QuestionObservationDataV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QuestionObservationDataV1")
            .finish_non_exhaustive()
    }
}
/// C descriptor or borrowed view `QuestionObservationV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct QuestionObservationV1 {
    /// C field `index`.
    pub index: usize,
    /// C field `member`.
    pub member: OptionalStringV1,
    /// C field `stage`.
    pub stage: OptionalDiscriminatorV1,
    /// C field `position`.
    pub position: usize,
    /// C field `question_sha256`.
    pub question_sha256: StringV1,
    /// C field `model`.
    pub model: StringV1,
    /// C field `url`.
    pub url: StringV1,
    /// C field `requests`.
    pub requests: StringsV1,
    /// C field `requests_sent`.
    pub requests_sent: u64,
    /// C field `cached`.
    pub cached: std::ffi::c_int,
    /// C field `failed_questions`.
    pub failed_questions: usize,
    /// C field `usage`.
    pub usage: OptionalUsageV1,
    /// C field `question_sources`.
    pub question_sources: QuestionSourcesV1,
    /// C field `state`.
    pub state: u32,
    /// C field `data`.
    pub data: QuestionObservationDataV1,
}
/// C union `RowObservationDataV1`; the parent discriminator selects its active arm.
#[repr(C)]
#[derive(Clone, Copy)]
pub union RowObservationDataV1 {
    /// Active arm selected by the parent discriminator.
    pub decide: DecideViewV1,
    /// C field `choose`.
    pub choose: ChooseViewV1,
    /// C field `tag`.
    pub tag: TagViewV1,
    /// C field `score`.
    pub score: ScoreViewV1,
    /// C field `filter`.
    pub filter: FilterViewV1,
    /// C field `rank`.
    pub rank: RankViewV1,
    /// C field `find`.
    pub find: FindViewV1,
    /// C field `annotate`.
    pub annotate: AnnotateViewV1,
    /// C field `recognize`.
    pub recognize: RecognizeViewV1,
    /// C field `relate`.
    pub relate: RelateViewV1,
}

impl std::fmt::Debug for RowObservationDataV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RowObservationDataV1")
            .finish_non_exhaustive()
    }
}
/// C descriptor or borrowed view `RowObservationV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct RowObservationV1 {
    /// C field `index`.
    pub index: usize,
    /// C field `function`.
    pub function: u32,
    /// C field `data`.
    pub data: RowObservationDataV1,
}
/// C union `ObservationDataV1`; the parent discriminator selects its active arm.
#[repr(C)]
#[derive(Clone, Copy)]
pub union ObservationDataV1 {
    /// Active arm selected by the parent discriminator.
    pub question: QuestionObservationV1,
    /// C field `row`.
    pub row: RowObservationV1,
}

impl std::fmt::Debug for ObservationDataV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ObservationDataV1").finish_non_exhaustive()
    }
}
/// C descriptor or borrowed view `ObservationV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ObservationV1 {
    /// C field `kind`.
    pub kind: u32,
    /// C field `data`.
    pub data: ObservationDataV1,
}
/// C descriptor or borrowed view `SummaryV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SummaryV1 {
    /// C field `state`.
    pub state: u32,
    /// C field `schema`.
    pub schema: StringV1,
    /// C field `answer_id`.
    pub answer_id: OptionalStringV1,
    /// C field `function`.
    pub function: OptionalDiscriminatorV1,
    /// C field `count`.
    pub count: usize,
    /// C field `observation_count`.
    pub observation_count: usize,
    /// C field `meta`.
    pub meta: OptionalMetaV1,
    /// C field `facts`.
    pub facts: OptionalFactsV1,
    /// C field `attempts`.
    pub attempts: OptionalAttemptsV1,
    /// C field `error`.
    pub error: OptionalErrorV1,
}
// Additive detail views preserve the canonical v1 layouts above.
/// Additive complete detail accessors; all pointers borrow result ownership.
/// The singular question-observation observation_id is NULL/zero-length for
/// aggregate questions; details.observations retains every actual identity.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReportedUsageV1 {
    /// C field `present`.
    pub present: std::ffi::c_int,
    /// C field `input_tokens`.
    pub input_tokens: super::OptionalU64V1,
    /// C field `output_tokens`.
    pub output_tokens: super::OptionalU64V1,
}
/// C descriptor or borrowed view `SourceDetailV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SourceDetailV1 {
    /// C field `origin`.
    pub origin: u32,
    /// C field `answered_by`.
    pub answered_by: StringV1,
    /// C field `batch_size`.
    pub batch_size: OptionalSizeV1,
}
/// C descriptor or borrowed view `SourceDetailsV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SourceDetailsV1 {
    /// C field `data`.
    pub data: *const SourceDetailV1,
    /// C field `len`.
    pub len: usize,
}
/// C descriptor or borrowed view `InputViewV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct InputViewV1 {
    /// C field `original`.
    pub original: OptionalContentV1,
    /// C field `position`.
    pub position: OptionalLocationV1,
    /// C field `images`.
    pub images: OptionalImageViewsV1,
}
/// C descriptor or borrowed view `InputViewsV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct InputViewsV1 {
    /// C field `data`.
    pub data: *const InputViewV1,
    /// C field `len`.
    pub len: usize,
}
/// C descriptor or borrowed view `DetailsV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct DetailsV1 {
    /// C field `question`.
    pub question: OptionalQuestionV1,
    /// C field `threshold`.
    pub threshold: OptionalRuleV1,
    /// C field `raw_pick`.
    pub raw_pick: OptionalStringV1,
    /// C field `usage`.
    pub usage: ReportedUsageV1,
    /// C field `question_sources`.
    pub question_sources: SourceDetailsV1,
    /// C field `observations`.
    pub observations: super::ObservationIdentitiesV1,
    /// C field `inputs`.
    pub inputs: InputViewsV1,
}

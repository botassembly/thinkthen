//! Canonical reviewed C layouts; see include/thinkthen.h.
use super::{
    DecideValueV1, EdgesV1, MemberFailureV1, MemberValueV1, MembersV1, MetaV1, OptionalAnswerV1,
    OptionalAttemptsV1, OptionalContentV1, OptionalDiscriminatorV1, OptionalDoubleV1,
    OptionalErrorV1, OptionalFactsV1, OptionalImageViewsV1, OptionalLocationV1, OptionalMetaV1,
    OptionalQuestionV1, OptionalRuleV1, OptionalSizeV1, OptionalStringV1, OptionalUsageV1,
    ProbabilitiesV1, QuestionSourcesV1, RecognizeAnswerV1, RecognizeValueV1, RelationAnswersV1,
    StringV1, StringsV1,
};
layout!(RowV1 {
    answer_id: StringV1,
    input: OptionalContentV1,
    question: OptionalQuestionV1,
    answer: OptionalAnswerV1,
    threshold: OptionalRuleV1,
    position: OptionalLocationV1,
    input_file: OptionalStringV1,
    meta: MetaV1,
    images: OptionalImageViewsV1,
});
layout!(DecideViewV1 {
    common: RowV1,
    value: DecideValueV1,
});
layout!(ChooseViewV1 {
    common: RowV1,
    value: OptionalStringV1,
});
layout!(TagViewV1 {
    common: RowV1,
    value: StringsV1,
});
layout!(ScoreViewV1 {
    common: RowV1,
    value: f64,
});
layout!(FilterViewV1 {
    common: RowV1,
    value: i32,
});
layout!(RankViewV1 {
    common: RowV1,
    value: OptionalSizeV1,
    question_name: OptionalStringV1,
});
layout!(FindViewV1 {
    common: RowV1,
    value: OptionalContentV1,
    index: OptionalSizeV1,
});
layout!(AnnotateViewV1 {
    common: RowV1,
    answers: MembersV1,
});
layout!(RecognizeViewV1 {
    common: RowV1,
    value: RecognizeValueV1,
    answer: RecognizeAnswerV1,
});
layout!(RelateViewV1 {
    common: RowV1,
    value: EdgesV1,
    questions: RelationAnswersV1,
});
union_layout!(ObservedProbabilitiesDataV1 {
    yes: f64,
    named: ProbabilitiesV1,
});
layout!(ObservedProbabilitiesV1 {
    kind: u32,
    data: ObservedProbabilitiesDataV1,
});
layout!(ObservationSuccessV1 {
    answer_id: StringV1,
    observation_id: StringV1,
    value: MemberValueV1,
    probabilities: ObservedProbabilitiesV1,
    confidence: OptionalDoubleV1,
});
union_layout!(QuestionObservationDataV1 {
    success: ObservationSuccessV1,
    failure: MemberFailureV1,
});
layout!(QuestionObservationV1 {
    index: usize,
    member: OptionalStringV1,
    stage: OptionalDiscriminatorV1,
    position: usize,
    question_sha256: StringV1,
    model: StringV1,
    url: StringV1,
    requests: StringsV1,
    requests_sent: u64,
    cached: i32,
    failed_questions: usize,
    usage: OptionalUsageV1,
    question_sources: QuestionSourcesV1,
    state: u32,
    data: QuestionObservationDataV1,
});
union_layout!(RowObservationDataV1 {
    decide: DecideViewV1,
    choose: ChooseViewV1,
    tag: TagViewV1,
    score: ScoreViewV1,
    filter: FilterViewV1,
    rank: RankViewV1,
    find: FindViewV1,
    annotate: AnnotateViewV1,
    recognize: RecognizeViewV1,
    relate: RelateViewV1,
});
layout!(RowObservationV1 {
    index: usize,
    function: u32,
    data: RowObservationDataV1,
});
union_layout!(ObservationDataV1 {
    question: QuestionObservationV1,
    row: RowObservationV1,
});
layout!(ObservationV1 {
    kind: u32,
    data: ObservationDataV1,
});
layout!(SummaryV1 {
    state: u32,
    schema: StringV1,
    answer_id: OptionalStringV1,
    function: OptionalDiscriminatorV1,
    count: usize,
    observation_count: usize,
    meta: OptionalMetaV1,
    facts: OptionalFactsV1,
    attempts: OptionalAttemptsV1,
    error: OptionalErrorV1,
});
// Additive detail views preserve the canonical v1 layouts above.
layout!(ReportedUsageV1 {
    present: i32,
    input_tokens: super::OptionalU64V1,
    output_tokens: super::OptionalU64V1,
});
layout!(SourceDetailV1 {
    origin: u32,
    answered_by: StringV1,
    batch_size: OptionalSizeV1,
});
layout!(SourceDetailsV1 { data: *const SourceDetailV1, len: usize });
layout!(InputViewV1 {
    original: OptionalContentV1,
    position: OptionalLocationV1,
    images: OptionalImageViewsV1,
});
layout!(InputViewsV1 { data: *const InputViewV1, len: usize });
layout!(DetailsV1 {
    question: OptionalQuestionV1,
    threshold: OptionalRuleV1,
    raw_pick: OptionalStringV1,
    usage: ReportedUsageV1,
    question_sources: SourceDetailsV1,
    observations: super::ObservationIdentitiesV1,
    inputs: InputViewsV1,
});

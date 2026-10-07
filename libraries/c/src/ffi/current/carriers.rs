//! Private result/1 projections retained for native integration regressions.
use crate::ffi::carriers::layout;
pub(crate) use crate::ffi::carriers::{
    EdgesV1, EntitiesV1, EntityEdgesV1, OptionalContentV1, OptionalDoubleV1, OptionalLocationV1,
    OptionalSizeV1, OptionalStringV1, OptionalU16V1, OptionalU64V1, ProbabilitiesV1, StringV1,
    StringsV1,
};
layout!(pub(crate) CurrentValueV1 {
    kind: u32,
    decide_kind: u32,
    boolean: i32,
    authored: OptionalContentV1,
    choice: OptionalStringV1,
    tags: StringsV1,
    score: f64
});
layout!(pub(crate) CurrentFactsV1 {
    cache_answers: u64,
    estimated_cost_usd: OptionalStringV1,
    input_tokens: OptionalU64V1,
    model: OptionalStringV1,
    output_tokens: OptionalU64V1,
    records: u64,
    requests_sent: u64,
    seconds: f64
});
layout!(pub(crate) CurrentMetaV1 {
    question_sha256: StringV1,
    model: StringV1,
    url: StringV1,
    requests: StringsV1,
    requests_sent: u64,
    cached: i32,
    usage_present: i32,
    input_tokens: u64,
    output_tokens: u64,
    tuned_for: OptionalStringV1,
    running: OptionalStringV1
});
layout!(pub(crate) CurrentRowV1 {
    input: OptionalContentV1,
    position: OptionalLocationV1,
    index: OptionalSizeV1,
    probability: OptionalDoubleV1,
    question_name: OptionalStringV1
});
layout!(pub(crate) CurrentAtomicV1 {
    common: CurrentRowV1,
    value: CurrentValueV1,
    yes_probability: OptionalDoubleV1,
    probabilities: ProbabilitiesV1,
    nearest: OptionalStringV1,
    confidence: OptionalDoubleV1,
    meta: CurrentMetaV1
});
layout!(pub(crate) CurrentMemberV1 {
    name: StringV1,
    state: u32,
    value: CurrentValueV1,
    failure: u32
});
layout!(pub(crate) CurrentMembersV1 { data: *const CurrentMemberV1, len: usize });
layout!(pub(crate) CurrentAnnotationV1 {
    common: CurrentRowV1,
    members: CurrentMembersV1
});
layout!(pub(crate) CurrentRecognitionV1 {
    common: CurrentRowV1,
    entities: EntitiesV1,
    relations_present: i32,
    relations: EntityEdgesV1
});
layout!(pub(crate) CurrentRowsV1 { data: *const CurrentRowV1, len: usize });
layout!(pub(crate) CurrentRelationsV1 {
    edges: EdgesV1,
    inputs: CurrentRowsV1
});
layout!(pub(crate) CurrentQuestionV1 {
    index: usize,
    member: OptionalStringV1,
    stage: OptionalStringV1,
    position: usize,
    meta: CurrentMetaV1,
    state: u32,
    value: CurrentValueV1,
    failure: u32,
    yes_probability: OptionalDoubleV1,
    probabilities: ProbabilitiesV1,
    confidence: OptionalDoubleV1,
    failed_questions: usize
});
layout!(pub(crate) CurrentAttemptV1 {
    ordinal: u64,
    request_sha256: StringV1,
    wall_ms: u64,
    outcome: u32,
    status: OptionalU16V1,
    server_ms: OptionalU64V1,
    request_id: OptionalStringV1
});
layout!(pub(crate) CurrentSummaryV1 {
    function: u32,
    count: usize,
    question_count: usize,
    facts: CurrentFactsV1,
    attempts_present: i32,
    attempt_count: usize
});

layout!(pub(crate) CurrentCandidateV1 {
    common: CurrentRowV1,
    probability: f64
});
layout!(pub(crate) CurrentCandidatesV1 { data: *const CurrentCandidateV1, len: usize });
layout!(pub(crate) CurrentFindV1 {
    selected: CurrentRowV1,
    candidates: CurrentCandidatesV1
});

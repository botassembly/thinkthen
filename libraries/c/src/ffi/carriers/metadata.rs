//! Canonical reviewed C layouts; see include/thinkthen.h.
use super::{
    OptionalDiscriminatorV1, OptionalSizeV1, OptionalStringV1, OptionalU16V1, OptionalU64V1,
    StringV1, StringsV1,
};
layout!(UsageV1 {
    input_tokens: u64,
    output_tokens: u64,
});
layout!(OptionalUsageV1 {
    present: i32,
    value: UsageV1,
});
layout!(QuestionSourceV1 {
    origin: u32,
    answered_by: StringV1,
});
layout!(QuestionSourcesV1 {
    data: *const QuestionSourceV1,
    len: usize,
});
union_layout!(ObservationIdentityDataV1 {
    observation_id: StringV1,
    failure_id: StringV1,
});
layout!(ObservationIdentityV1 {
    kind: u32,
    data: ObservationIdentityDataV1,
});
layout!(ObservationIdentitiesV1 {
    data: *const ObservationIdentityV1,
    len: usize,
});
layout!(ProfileWarningV1 {
    tuned_for: StringV1,
    running: StringV1,
});
layout!(OptionalProfileWarningV1 {
    present: i32,
    value: ProfileWarningV1,
});
layout!(BatchV1 {
    kind: u32,
    records: usize,
});
layout!(OptionalBatchV1 {
    present: i32,
    value: BatchV1,
});
layout!(BatchWarningV1 {
    tuned_for: BatchV1,
    running: BatchV1,
});
layout!(OptionalBatchWarningV1 {
    present: i32,
    value: BatchWarningV1,
});
layout!(AttemptV1 {
    ordinal: u64,
    request_sha256: StringV1,
    wall_ms: u64,
    outcome: u32,
    sdk_request_id: StringV1,
    status: OptionalU16V1,
    server_ms: OptionalU64V1,
    request_id: OptionalStringV1,
});
layout!(AttemptsV1 {
    data: *const AttemptV1,
    len: usize,
});
layout!(OptionalAttemptsV1 {
    present: i32,
    value: AttemptsV1,
});
layout!(MetaV1 {
    tool: StringV1,
    question_sha256: OptionalStringV1,
    questions_sha256: OptionalStringV1,
    url: StringV1,
    model: StringV1,
    usage: OptionalUsageV1,
    requests_sent: u64,
    cached: i32,
    requests: StringsV1,
    failed_questions: usize,
    profile_warning: OptionalProfileWarningV1,
    batch_setting: OptionalBatchV1,
    batch_warning: OptionalBatchWarningV1,
    context_sha256: OptionalStringV1,
    attempts: OptionalAttemptsV1,
    origin: OptionalDiscriminatorV1,
    question_sources: QuestionSourcesV1,
    observations: ObservationIdentitiesV1,
    answered_by: OptionalStringV1,
});
layout!(OptionalMetaV1 {
    present: i32,
    value: MetaV1,
});
layout!(FactsV1 {
    call_id: StringV1,
    cache_answers: u64,
    estimated_cost_usd: OptionalStringV1,
    input_tokens: OptionalU64V1,
    model: OptionalStringV1,
    output_tokens: OptionalU64V1,
    records: u64,
    requests_sent: u64,
    seconds: f64,
    command_ms: OptionalU64V1,
});
layout!(OptionalFactsV1 {
    present: i32,
    value: FactsV1,
});
layout!(StoppedV1 {
    at: OptionalSizeV1,
    cause: u32,
    status: OptionalU16V1,
    retryable: i32,
});
layout!(OptionalStoppedV1 {
    present: i32,
    value: StoppedV1,
});
layout!(ErrorV1 {
    code: i32,
    message: StringV1,
    retryable: i32,
    stopped: OptionalStoppedV1,
});
layout!(OptionalErrorV1 {
    present: i32,
    value: ErrorV1,
});

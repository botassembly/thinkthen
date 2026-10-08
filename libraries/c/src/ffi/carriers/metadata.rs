//! Canonical reviewed C layouts; see include/thinkthen.h.
use super::{
    OptionalDiscriminatorV1, OptionalSizeV1, OptionalStringV1, OptionalU16V1, OptionalU64V1,
    StringV1, StringsV1,
};
/// C descriptor or borrowed view `UsageV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct UsageV1 {
    /// C field `input_tokens`.
    pub input_tokens: u64,
    /// C field `output_tokens`.
    pub output_tokens: u64,
}
/// C descriptor or borrowed view `OptionalUsageV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct OptionalUsageV1 {
    /// C field `present`.
    pub present: std::ffi::c_int,
    /// C field `value`.
    pub value: UsageV1,
}
/// reserved, never emitted in 0.2
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct QuestionSourceV1 {
    /// C field `origin`.
    pub origin: u32,
    /// C field `answered_by`.
    pub answered_by: StringV1,
}
/// C descriptor or borrowed view `QuestionSourcesV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct QuestionSourcesV1 {
    /// C field `data`.
    pub data: *const QuestionSourceV1,
    /// C field `len`.
    pub len: usize,
}
/// C union `ObservationIdentityDataV1`; the parent discriminator selects its active arm.
#[repr(C)]
#[derive(Clone, Copy)]
pub union ObservationIdentityDataV1 {
    /// Active arm selected by the parent discriminator.
    pub observation_id: StringV1,
    /// C field `failure_id`.
    pub failure_id: StringV1,
}

impl std::fmt::Debug for ObservationIdentityDataV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ObservationIdentityDataV1")
            .finish_non_exhaustive()
    }
}
/// C descriptor or borrowed view `ObservationIdentityV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ObservationIdentityV1 {
    /// C field `kind`.
    pub kind: u32,
    /// C field `data`.
    pub data: ObservationIdentityDataV1,
}
/// C descriptor or borrowed view `ObservationIdentitiesV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ObservationIdentitiesV1 {
    /// C field `data`.
    pub data: *const ObservationIdentityV1,
    /// C field `len`.
    pub len: usize,
}
/// C descriptor or borrowed view `ProfileWarningV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ProfileWarningV1 {
    /// C field `tuned_for`.
    pub tuned_for: StringV1,
    /// C field `running`.
    pub running: StringV1,
}
/// C descriptor or borrowed view `OptionalProfileWarningV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct OptionalProfileWarningV1 {
    /// C field `present`.
    pub present: std::ffi::c_int,
    /// C field `value`.
    pub value: ProfileWarningV1,
}
/// C descriptor or borrowed view `BatchV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct BatchV1 {
    /// C field `kind`.
    pub kind: u32,
    /// C field `records`.
    pub records: usize,
}
/// C descriptor or borrowed view `OptionalBatchV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct OptionalBatchV1 {
    /// C field `present`.
    pub present: std::ffi::c_int,
    /// C field `value`.
    pub value: BatchV1,
}
/// C descriptor or borrowed view `BatchWarningV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct BatchWarningV1 {
    /// C field `tuned_for`.
    pub tuned_for: BatchV1,
    /// C field `running`.
    pub running: BatchV1,
}
/// C descriptor or borrowed view `OptionalBatchWarningV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct OptionalBatchWarningV1 {
    /// C field `present`.
    pub present: std::ffi::c_int,
    /// C field `value`.
    pub value: BatchWarningV1,
}
/// C descriptor or borrowed view `AttemptV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct AttemptV1 {
    /// C field `ordinal`.
    pub ordinal: u64,
    /// C field `request_sha256`.
    pub request_sha256: StringV1,
    /// C field `wall_ms`.
    pub wall_ms: u64,
    /// C field `outcome`.
    pub outcome: u32,
    /// C field `sdk_request_id`.
    pub sdk_request_id: StringV1,
    /// C field `status`.
    pub status: OptionalU16V1,
    /// C field `server_ms`.
    pub server_ms: OptionalU64V1,
    /// C field `request_id`.
    pub request_id: OptionalStringV1,
}
/// C descriptor or borrowed view `AttemptsV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct AttemptsV1 {
    /// C field `data`.
    pub data: *const AttemptV1,
    /// C field `len`.
    pub len: usize,
}
/// C descriptor or borrowed view `OptionalAttemptsV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct OptionalAttemptsV1 {
    /// C field `present`.
    pub present: std::ffi::c_int,
    /// C field `value`.
    pub value: AttemptsV1,
}
/// C descriptor or borrowed view `MetaV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct MetaV1 {
    /// C field `tool`.
    pub tool: StringV1,
    /// C field `question_sha256`.
    pub question_sha256: OptionalStringV1,
    /// C field `questions_sha256`.
    pub questions_sha256: OptionalStringV1,
    /// C field `url`.
    pub url: StringV1,
    /// C field `model`.
    pub model: StringV1,
    /// C field `usage`.
    pub usage: OptionalUsageV1,
    /// C field `requests_sent`.
    pub requests_sent: u64,
    /// C field `cached`.
    pub cached: std::ffi::c_int,
    /// C field `requests`.
    pub requests: StringsV1,
    /// C field `failed_questions`.
    pub failed_questions: usize,
    /// C field `profile_warning`.
    pub profile_warning: OptionalProfileWarningV1,
    /// C field `batch_setting`.
    pub batch_setting: OptionalBatchV1,
    /// C field `batch_warning`.
    pub batch_warning: OptionalBatchWarningV1,
    /// C field `context_sha256`.
    pub context_sha256: OptionalStringV1,
    /// C field `attempts`.
    pub attempts: OptionalAttemptsV1,
    /// C field `origin`.
    pub origin: OptionalDiscriminatorV1,
    /// C field `question_sources`.
    pub question_sources: QuestionSourcesV1,
    /// C field `observations`.
    pub observations: ObservationIdentitiesV1,
    /// C field `answered_by`.
    pub answered_by: OptionalStringV1,
}
/// C descriptor or borrowed view `OptionalMetaV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct OptionalMetaV1 {
    /// C field `present`.
    pub present: std::ffi::c_int,
    /// C field `value`.
    pub value: MetaV1,
}
/// origin.present=0 represents required JSON null, not omitted provenance.
/// requests, question_sources, observations have the same length/order.
/// Exactly one question digest is present: plural for annotate, singular else.
/// No proxy field exists: 0.2 cannot activate it or emit proxy metadata.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct FactsV1 {
    /// C field `call_id`.
    pub call_id: StringV1,
    /// C field `cache_answers`.
    pub cache_answers: u64,
    /// C field `estimated_cost_usd`.
    pub estimated_cost_usd: OptionalStringV1,
    /// C field `input_tokens`.
    pub input_tokens: OptionalU64V1,
    /// C field `model`.
    pub model: OptionalStringV1,
    /// C field `output_tokens`.
    pub output_tokens: OptionalU64V1,
    /// C field `records`.
    pub records: u64,
    /// C field `requests_sent`.
    pub requests_sent: u64,
    /// C field `seconds`.
    pub seconds: f64,
    /// C field `command_ms`.
    pub command_ms: OptionalU64V1,
}
/// C descriptor or borrowed view `OptionalFactsV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct OptionalFactsV1 {
    /// C field `present`.
    pub present: std::ffi::c_int,
    /// C field `value`.
    pub value: FactsV1,
}
/// C descriptor or borrowed view `StoppedV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct StoppedV1 {
    /// C field `at`.
    pub at: OptionalSizeV1,
    /// C field `cause`.
    pub cause: u32,
    /// C field `status`.
    pub status: OptionalU16V1,
    /// C field `retryable`.
    pub retryable: std::ffi::c_int,
}
/// C descriptor or borrowed view `OptionalStoppedV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct OptionalStoppedV1 {
    /// C field `present`.
    pub present: std::ffi::c_int,
    /// C field `value`.
    pub value: StoppedV1,
}
/// C descriptor or borrowed view `ErrorV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ErrorV1 {
    /// C field `code`.
    pub code: std::ffi::c_int,
    /// C field `message`.
    pub message: StringV1,
    /// C field `retryable`.
    pub retryable: std::ffi::c_int,
    /// C field `stopped`.
    pub stopped: OptionalStoppedV1,
}
/// C descriptor or borrowed view `OptionalErrorV1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct OptionalErrorV1 {
    /// C field `present`.
    pub present: std::ffi::c_int,
    /// C field `value`.
    pub value: ErrorV1,
}

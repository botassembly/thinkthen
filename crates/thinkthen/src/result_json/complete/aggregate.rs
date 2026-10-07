//! Actual ordered stage observations and aggregate metadata from the one engine.
use crate::core::{self, Meta, RequestMeta};
use crate::engine::facade;

pub(crate) struct Totals {
    pub(crate) model: Option<core::ModelName>,
    pub(crate) usage: Option<core::Usage>,
    pub(crate) reported: Option<core::ReportedUsage>,
    pub(crate) cached: bool,
    pub(crate) sent: u64,
    pub(crate) requests: Vec<String>,
    pub(crate) failed: usize,
}
pub(crate) fn meta(
    engine: &facade::Engine,
    digest: String,
    totals: Totals,
    context_sha256: Option<String>,
    attempts: Option<Vec<core::AttemptObservation>>,
    profile: Option<&core::ProfileName>,
) -> Meta {
    Meta::new(
        env!("CARGO_PKG_VERSION"),
        digest,
        engine.backend().url().clone(),
        totals
            .model
            .unwrap_or_else(|| engine.backend().model().clone()),
        totals.usage,
        RequestMeta::new(totals.cached, totals.sent, totals.requests)
            .with_failed_questions(totals.failed)
            .with_profile_warning(core::ProfileWarning::between(
                profile,
                engine.profile().map(core::BackendProfile::name),
            ))
            .with_context_sha256(context_sha256),
    )
    .with_reported_usage(totals.reported)
    .with_captured_attempts(attempts)
}

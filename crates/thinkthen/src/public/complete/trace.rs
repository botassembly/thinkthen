//! Actual ordered stage observations and aggregate metadata from the one engine.
use crate::core::{self, Meta, RequestMeta};
use crate::engine::facade;
use crate::public::CallOptions;

pub(super) struct Totals {
    pub(super) model: Option<core::ModelName>,
    pub(super) usage: Option<core::Usage>,
    pub(super) reported: Option<core::ReportedUsage>,
    pub(super) cached: bool,
    pub(super) sent: u64,
    pub(super) requests: Vec<String>,
    pub(super) failed: usize,
}
pub(super) fn meta(
    engine: &facade::Engine,
    digest: String,
    totals: Totals,
    options: &CallOptions<'_>,
    attempts: Option<Vec<core::AttemptObservation>>,
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
            .with_context_sha256(
                options
                    .context_text()
                    .filter(|text| !text.is_empty())
                    .map(|text| core::bytes_sha256(text.as_bytes())),
            ),
    )
    .with_reported_usage(totals.reported)
    .with_captured_attempts(attempts)
}

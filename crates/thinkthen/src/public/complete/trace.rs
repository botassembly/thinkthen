//! Call-option context delegates to shared aggregate metadata assembly.
use crate::core::{self, Meta};
use crate::engine::facade;
use crate::public::CallOptions;
pub(super) use crate::result_json::complete::Totals;
pub(super) fn meta(
    engine: &facade::Engine,
    digest: String,
    totals: Totals,
    options: &CallOptions<'_>,
    attempts: Option<Vec<core::AttemptObservation>>,
    profile: Option<&core::ProfileName>,
) -> Meta {
    crate::result_json::complete::aggregate_meta(
        engine,
        digest,
        totals,
        options
            .context_text()
            .filter(|text| !text.is_empty())
            .map(|text| core::bytes_sha256(text.as_bytes())),
        attempts,
        profile,
    )
}

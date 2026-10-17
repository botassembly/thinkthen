//! The detailed document one judgment prints. The command's `--details` row
//! and the library's `Details::to_json` both write it here, so their bytes agree.

use crate::core::{
    Backend, BatchSetting, BatchWarning, Meta, ProfileName, ProfileWarning, RequestMeta,
};
use crate::engine::facade::Judgment;
pub(crate) mod bounded;
pub(crate) mod complete;

/// What a detailed row names beside the judgment: the backend's address and the
/// calibration profile the run compares.
#[derive(Clone)]
pub(crate) struct Run<'a> {
    pub(crate) backend: &'a Backend,
    pub(crate) tuned_for: Option<&'a ProfileName>,
    pub(crate) warning: Option<ProfileWarning>,
    pub(crate) batch_setting: Option<BatchSetting>,
    pub(crate) batch_warning: Option<BatchWarning>,
    pub(crate) context_sha256: Option<String>,
}

fn atomic_metadata(run: Run<'_>, judged: &Judgment, requests: Vec<String>, digest: &str) -> Meta {
    let answered = &judged.answered;
    Meta::new(
        env!("CARGO_PKG_VERSION"),
        digest.to_owned(),
        run.backend.url().clone(),
        answered.reply.model().clone(),
        answered.reply.usage(),
        RequestMeta::new(answered.replayed, answered.requests_sent, requests)
            .with_profile_warning(run.warning)
            .with_batch_setting(run.batch_setting)
            .with_batch_warning(run.batch_warning)
            .with_context_sha256(run.context_sha256),
    )
    .with_reported_usage(answered.reply.reported_usage())
}

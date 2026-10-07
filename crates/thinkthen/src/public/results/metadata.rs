//! Borrowed truthful metadata; consumers never need to decode result JSON.
use crate::core;
use crate::public::{AttemptObservation, BatchSetting, Origin, ReportedUsage, ResultIdentity};
use std::fmt;

/// Metadata for one concrete complete result, borrowed from its owner.
#[derive(Clone, Copy)]
pub struct ResultMetadata<'a> {
    pub(crate) fields: core::MetadataFields<'a>,
    pub(crate) identity: &'a ResultIdentity,
}
impl ResultMetadata<'_> {
    /// Compiled engine version line.
    #[must_use]
    pub const fn tool(&self) -> &str {
        self.fields.tool
    }
    /// Normalized question digest, absent on annotation sets.
    #[must_use]
    pub const fn question_sha256(&self) -> Option<&str> {
        if self.fields.plural {
            None
        } else {
            Some(self.fields.digest)
        }
    }
    /// Normalized ordered question-set digest, only on annotations.
    #[must_use]
    pub const fn questions_sha256(&self) -> Option<&str> {
        if self.fields.plural {
            Some(self.fields.digest)
        } else {
            None
        }
    }
    /// The one resolved posting address.
    #[must_use]
    pub const fn url(&self) -> &str {
        self.fields.url
    }
    /// Historical compatibility model, distinct from actual answered_by.
    #[must_use]
    pub const fn model(&self) -> &str {
        self.fields.model
    }
    /// Independently observed token dimensions; absence remains unknown.
    #[must_use]
    pub const fn usage(&self) -> Option<ReportedUsage> {
        self.fields.usage
    }
    /// This result's retained even share of sends, including retries.
    #[must_use]
    pub const fn requests_sent(&self) -> u64 {
        self.fields.sent
    }
    /// Saved keys in logical wire-question order, aligned with sources/observations.
    #[must_use]
    pub const fn requests(&self) -> &[String] {
        self.fields.requests
    }
    /// Recoverably failed logical members.
    #[must_use]
    pub const fn failed_questions(&self) -> usize {
        self.fields.failed
    }
    /// True only when every actual constituent came from cache or replay.
    #[must_use]
    pub fn cached(&self) -> bool {
        !self.identity.question_sources().is_empty()
            && self
                .identity
                .question_sources()
                .iter()
                .all(|source| matches!(source.origin(), Origin::Cache | Origin::Replay))
    }
    /// Actual constituent provenance and persistent/failure identities.
    #[must_use]
    pub const fn identity(&self) -> &ResultIdentity {
        self.identity
    }
    /// Saved versus effective profile mismatch, when both were named.
    #[must_use]
    pub fn profile_warning(&self) -> Option<ProfileMismatch<'_>> {
        self.fields.profile.map(ProfileMismatch)
    }
    /// The explicit resolved packing setting, outside cache identity.
    #[must_use]
    pub fn batch_setting(&self) -> Option<BatchSetting> {
        self.fields.batch.and_then(batch)
    }
    /// Saved versus effective packing setting mismatch.
    #[must_use]
    pub fn batch_warning(&self) -> Option<BatchMismatch<'_>> {
        self.fields.batch_warning.map(BatchMismatch)
    }
    /// Hash of the exact separate effective context, if present.
    #[must_use]
    pub const fn context_sha256(&self) -> Option<&str> {
        self.fields.context
    }
    /// Requested attempts; cache/replay zero sends retain Some(empty).
    #[must_use]
    pub const fn attempts(&self) -> Option<&[AttemptObservation]> {
        self.fields.attempts
    }
}
/// A named profile mismatch, borrowed from its result.
#[derive(Clone, Copy, Debug)]
pub struct ProfileMismatch<'a>(&'a core::ProfileWarning);
impl ProfileMismatch<'_> {
    /// Profile under which the question was tuned.
    #[must_use]
    pub fn tuned_for(&self) -> &str {
        self.0.tuned_for()
    }
    /// Effective profile of this run.
    #[must_use]
    pub fn running(&self) -> &str {
        self.0.running()
    }
}
/// A packing mismatch, borrowed from its result.
#[derive(Clone, Copy, Debug)]
pub struct BatchMismatch<'a>(&'a core::BatchWarning);
impl BatchMismatch<'_> {
    /// Saved packing setting.
    #[must_use]
    pub fn tuned_for(&self) -> Option<BatchSetting> {
        batch(self.0.tuned_for())
    }
    /// Effective packing setting.
    #[must_use]
    pub fn running(&self) -> Option<BatchSetting> {
        batch(self.0.running())
    }
}
fn batch(value: core::BatchSetting) -> Option<BatchSetting> {
    match value {
        core::BatchSetting::Max(_) => Some(BatchSetting::Max),
        core::BatchSetting::Records(value) => {
            std::num::NonZeroUsize::new(value).map(BatchSetting::Records)
        }
    }
}
impl fmt::Debug for ResultMetadata<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ResultMetadata")
            .field("requests_sent", &self.fields.sent)
            .field("identity", &self.identity)
            .finish_non_exhaustive()
    }
}

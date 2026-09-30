//! One bounded answered batch member, retained only until its ordered row leaves.

use serde::Serialize;
use std::fmt;

use crate::core::{self, BatchWarning, Framing, ProfileWarning, Reading};
use crate::engine::facade;
use crate::public::error::Error;
use crate::public::question::Question;
use crate::result_json::{Run, decision_with_requests};

use super::{Details, Written};

impl fmt::Debug for Details {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Details")
            .field("value", &self.value)
            .field("probabilities", &self.probabilities)
            .field("nearest", &self.nearest)
            .field("model", &self.model)
            .field("question_sha256", &self.question_sha256)
            .field("profile_warning", &self.profile_warning)
            .field("requests", &self.requests)
            .field("requests_sent", &self.requests_sent)
            .field("cached", &self.cached)
            .field("usage", &self.usage)
            .field("confidence", &self.confidence)
            .field("url", &"<withheld>")
            .field("json", &self.json)
            .field("scalar_json", &self.scalar_json)
            .finish()
    }
}

pub(crate) struct Member {
    judged: facade::Judgment,
    requests: Vec<String>,
}

impl Member {
    /// One pipeline answer's receipt: its judgment and question keys.
    pub(crate) fn new(judged: facade::Judgment, requests: Vec<String>) -> Self {
        Self { judged, requests }
    }
}

impl Details {
    /// The same detailed result as a scalar SQL value, without a record input.
    #[doc(hidden)]
    #[must_use]
    pub fn to_scalar_json(&self) -> String {
        self.scalar_json.as_ref().unwrap_or(&self.json).text()
    }

    #[allow(
        clippy::too_many_arguments,
        reason = "a record detail needs its original input and selected run metadata"
    )]
    pub(crate) fn of_member<T: Serialize>(
        member: Member,
        input: &T,
        question: &Question,
        backend: &core::Backend,
        profile: Option<&core::BackendProfile>,
        setting: core::Setting,
        context_sha256: Option<&str>,
    ) -> Result<Self, Error> {
        let original = serde_json::to_vec(input)
            .map_err(|_| Error::usage("a record cannot be written as JSON"))?;
        let reading = Reading::new(Framing::Jsonl, Vec::new())
            .map_err(|_| Error::defect("JSON record reading was refused"))?;
        let input = reading.record(&original).map_err(Error::refused)?;
        Self::of_batch_member(
            member,
            Some(input),
            question,
            backend,
            profile,
            setting,
            context_sha256,
        )
    }

    #[allow(
        clippy::too_many_arguments,
        reason = "the native scalar shares the member's saved run metadata"
    )]
    pub(crate) fn of_native_member(
        member: Member,
        question: &Question,
        backend: &core::Backend,
        profile: Option<&core::BackendProfile>,
        setting: core::Setting,
        context_sha256: Option<&str>,
    ) -> Result<Self, Error> {
        Self::of_batch_member(
            member,
            None,
            question,
            backend,
            profile,
            setting,
            context_sha256,
        )
    }

    #[allow(
        clippy::too_many_arguments,
        reason = "one batch member carries its saved run metadata"
    )]
    fn of_batch_member(
        member: Member,
        input: Option<core::Record>,
        question: &Question,
        backend: &core::Backend,
        profile: Option<&core::BackendProfile>,
        setting: core::Setting,
        context_sha256: Option<&str>,
    ) -> Result<Self, Error> {
        let mut details = Self::of(
            &member.judged,
            question,
            backend,
            profile,
            member.requests.clone(),
        )?;
        let file_batch = question
            .batch
            .as_ref()
            .and_then(core::Setting::of_json)
            .or_else(|| {
                question
                    .threshold
                    .map(|_| core::Setting::Records(std::num::NonZeroUsize::MIN))
            });
        let run = Run {
            backend,
            tuned_for: question.profile.as_ref(),
            warning: ProfileWarning::between(
                question.profile.as_ref(),
                profile.map(core::BackendProfile::name),
            ),
            batch_setting: Some(setting.into()),
            batch_warning: file_batch.and_then(|saved| BatchWarning::between(saved, setting)),
            context_sha256: context_sha256.map(str::to_owned),
        };
        let scalar_json = input
            .as_ref()
            .map(|_| {
                decision_with_requests(
                    run.clone(),
                    &member.judged,
                    question.core.clone(),
                    question.threshold,
                    member.judged.value.clone(),
                    None,
                    member.requests.clone(),
                )
                .map_err(|_| Error::defect("a scalar result could not be written as JSON"))
            })
            .transpose()?;
        let json = decision_with_requests(
            run,
            &member.judged,
            question.core.clone(),
            question.threshold,
            member.judged.value.clone(),
            input,
            member.requests.clone(),
        )
        .map_err(|_| Error::defect("a result could not be written as JSON"))?;
        details.json = Written(json);
        details.scalar_json = scalar_json.map(Written);
        Ok(details)
    }
}

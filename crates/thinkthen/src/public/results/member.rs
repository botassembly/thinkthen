//! One bounded answered batch member, retained only until its ordered row leaves.

use std::fmt;

use crate::core::{self, BatchWarning, ProfileWarning};
use crate::engine::facade;
use crate::public::error::Error;
use crate::public::question::Question;
use crate::result_json::Run;

use super::Details;

pub(crate) fn batch_warning(question: &Question, setting: core::Setting) -> Option<BatchWarning> {
    question
        .authored_threshold
        .then(|| {
            let saved = question
                .batch
                .as_ref()
                .and_then(core::Setting::of_json)
                .unwrap_or(core::Setting::Records(std::num::NonZeroUsize::MIN));
            BatchWarning::between(saved, setting)
        })
        .flatten()
}

impl fmt::Debug for Details {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Details")
            .field("value", &"<withheld>")
            .field("probabilities", &"<withheld>")
            .field("nearest", &"<withheld>")
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

    pub(crate) fn of_native_member(
        member: Member,
        question: &Question,
        backend: &core::Backend,
        profile: Option<&core::BackendProfile>,
        setting: core::Setting,
        context_sha256: Option<&str>,
    ) -> Result<Self, Error> {
        let run = Run {
            backend,
            tuned_for: question.profile.as_ref(),
            warning: ProfileWarning::between(
                question.profile.as_ref(),
                profile.map(core::BackendProfile::name),
            ),
            batch_setting: Some(setting.into()),
            batch_warning: batch_warning(question, setting),
            context_sha256: context_sha256.map(str::to_owned),
        };
        let function =
            crate::RequestFunction::for_definition(&question.clone().into()).input_function();
        let canonical = crate::result_json::complete::atomic(
            run,
            &member.judged,
            crate::public::complete::spec(function, question, member.judged.value.clone(), 0),
            member.requests,
            None,
            None,
        )
        .map_err(|_| Error::defect("a complete result could not be constructed"))?;
        Self::of_complete::<()>(&canonical, None)
    }
}

//! One bounded answered batch member, retained only until its ordered row leaves.

//! Stays: The retained private scalar projection must preserve original serialization, probabilities and batch metadata without changing canonical result identity.
use std::fmt;

use crate::core::{self, BatchWarning};
#[cfg(test)]
use crate::public::error::Error;
use crate::public::question::Question;

use super::Details;
#[cfg(test)]
use super::{Written, complete, judgment, usage};
#[cfg(test)]
use serde::Serialize;

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

impl Details {
    /// The same detailed result as a scalar SQL value, without a record input.
    #[doc(hidden)]
    #[must_use]
    pub fn to_scalar_json(&self) -> String {
        self.scalar_json.as_ref().unwrap_or(&self.json).text()
    }
}

#[cfg(test)]
impl Details {
    pub(in crate::public) fn of_complete<T: Serialize>(
        canonical: &core::CompleteAtomic,
        original: Option<&T>,
    ) -> Result<Self, Error> {
        let fields = canonical.metadata();
        let scalar = Written::of(&canonical.scalar_details(None))?;
        let json = match original {
            Some(original) => {
                // Match native member materialization without reapplying reader byte limits.
                let source = serde_json::to_string(original)
                    .map_err(|_| Error::usage("a record cannot be written as JSON"))?;
                let record =
                    core::Record::from_json(core::Json::parse(&source).map_err(Error::refused)?);
                Written::of(&canonical.scalar_details(Some(record)))?
            }
            None => scalar.clone(),
        };
        Ok(Self {
            sources: canonical.identity.question_sources().to_vec(),
            observations: canonical.identity.observations().to_vec(),
            value: judgment(canonical.value()),
            probabilities: complete::probabilities(canonical.answer()),
            nearest: canonical.answer().level().map(str::to_owned),
            model: fields.model.to_owned(),
            question_sha256: fields.digest.to_owned(),
            profile_warning: fields.profile.cloned(),
            requests: fields.requests.to_vec(),
            requests_sent: fields.sent,
            cached: canonical.cached(),
            usage: canonical.usage().map(usage),
            reported_usage: canonical.reported_usage(),
            confidence: canonical.answer().confidence().map(|p| p.as_f64()),
            url: fields.url.to_owned(),
            json,
            scalar_json: original.map(|_| scalar),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::public::{CallOptions, CompleteRecord, Engine, ErrorKind, Question};
    use conformance_backend::{Canned, Listener};

    #[test]
    fn actual_cached_flag_survives_absent_complete_sources_and_refused_originals_stay_safe() {
        let listener = Listener::answering(|_| {
            Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}}}"#)
        })
        .unwrap();
        let folder = std::env::temp_dir().join(format!("details-cache-{}", std::process::id()));
        std::fs::create_dir(&folder).unwrap();
        let engine = Engine::builder()
            .base_url(listener.base())
            .unwrap()
            .model("fixed")
            .unwrap()
            .api_key("projection-private")
            .unwrap()
            .cache_at(&folder)
            .unwrap()
            .max_retries(0)
            .build()
            .unwrap();
        let question = Question::decide("Fits?").unwrap().cut();
        engine
            .decide_input_complete_with(
                &question,
                &crate::QuestionInput::Text("Original.".into()),
                CallOptions::new(),
            )
            .unwrap();
        let call = engine
            .decide_input_complete_with(
                &question,
                &crate::QuestionInput::Text("Original.".into()),
                CallOptions::new(),
            )
            .unwrap();
        let mut result = call.value().clone();
        result.canonical.identity =
            core::ResultIdentity::resolved(result.answer_id().clone(), None, vec![], vec![], None);
        let details = Details::of_complete::<()>(&result.canonical, None).unwrap();
        assert!(details.cached());
        assert!(result.identity().question_sources().is_empty());
        assert!(details.to_json().contains(r#""cached":true"#));
        struct Refuses;
        impl Serialize for Refuses {
            fn serialize<S: serde::Serializer>(&self, _: S) -> Result<S::Ok, S::Error> {
                Err(serde::ser::Error::custom("sensitive original"))
            }
        }
        let row = CompleteRecord {
            original: Refuses,
            ordinal: 0,
            result,
        };
        let error = Details::of_complete(&row.result.canonical, Some(&row.original)).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Usage);
        assert_eq!(error.to_string(), "a record cannot be written as JSON");
        assert_eq!(listener.count(), 1);
        drop(engine);
        std::fs::remove_dir_all(folder).unwrap();
    }
}

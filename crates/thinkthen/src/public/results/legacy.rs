//! Compatibility projections reuse retained native result/1 documents.
use super::{CompleteRecord, Details, Written, complete, judgment, usage};
use crate::{core, public::Error};
use serde::Serialize;

impl Details {
    fn of_complete<T: Serialize>(
        canonical: &core::CompleteAtomic,
        original: Option<&T>,
    ) -> Result<Self, Error> {
        let fields = canonical.metadata();
        let scalar = Written::of(&canonical.legacy)?;
        let json = match original {
            Some(original) => {
                // Match native member materialization without reapplying reader byte limits.
                let source = serde_json::to_string(original)
                    .map_err(|_| Error::usage("a record cannot be written as JSON"))?;
                let record =
                    core::Record::from_json(core::Json::parse(&source).map_err(Error::refused)?);
                Written::of(&canonical.legacy.clone().with_input(record))?
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
            cached: canonical.legacy.cached(),
            usage: canonical.usage().map(usage),
            reported_usage: canonical.reported_usage(),
            confidence: canonical.answer().confidence().map(|p| p.as_f64()),
            url: fields.url.to_owned(),
            json,
            scalar_json: original.map(|_| scalar),
        })
    }
}

macro_rules! legacy {
    ($($result:ident),+ $(,)?) => { $(
        impl super::$result {
            /// Project the retained native reading as an input-free result/1 Details.
            /// Batch and context metadata retain the actual execution values.
            /// # Errors
            /// Returns Defect if the native carrier cannot be serialized.
            pub fn legacy_details(&self) -> Result<Details, Error> {
                Details::of_complete::<()>(&self.canonical, None)
            }
        }
        impl<T: Serialize> CompleteRecord<T, super::$result> {
            /// Project result/1 Details with the original and an input-free scalar document.
            /// Neither projection repeats admission or executes another judgment.
            /// # Errors
            /// Retains native member errors for an unserializable original, and Defect
            /// for an internal carrier serialization failure.
            pub fn legacy_details(&self) -> Result<Details, Error> {
                Details::of_complete(&self.result.canonical, Some(&self.original))
            }
        }
    )+ };
}
legacy!(
    CompleteDecision,
    CompleteChoice,
    CompleteTags,
    CompleteScore
);

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
        let folder = std::env::temp_dir().join(format!("legacy-cache-{}", std::process::id()));
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
            .decide_complete_with(&question, "Original.", CallOptions::new())
            .unwrap();
        let call = engine
            .decide_complete_with(&question, "Original.", CallOptions::new())
            .unwrap();
        let mut result = call.value().clone();
        result.canonical.identity =
            core::ResultIdentity::resolved(result.answer_id().clone(), None, vec![], vec![], None);
        let details = result.legacy_details().unwrap();
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
        let error = row.legacy_details().unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Usage);
        assert_eq!(error.to_string(), "a record cannot be written as JSON");
        assert_eq!(listener.count(), 1);
        drop(engine);
        std::fs::remove_dir_all(folder).unwrap();
    }
}

//! Bounded eager details for a native host that retains recoverable SQL rows.

use crate::core::{self, AnswerOutcome};
use crate::engine::error::Kind as EngineKind;
use crate::engine::pipeline::{Failed, Flow};
use crate::public::annotated::{FailureCause, cause};
use crate::public::asking::{Decided, Decisions, Miss, Text, packed};
use crate::public::engine::{DetailQuestion, Engine, evidence, only};
use crate::public::error::{Error, ErrorKind};
use crate::public::options::{CallOptions, Stop};
use crate::public::pull::{self, Row};
use crate::public::question::{Kind, Question};
use crate::public::results::{Call, Details};

/// One SQL member's answer or safe, recoverable failure.
#[doc(hidden)]
#[derive(Debug)]
#[non_exhaustive]
#[allow(
    clippy::large_enum_variant,
    reason = "ADR 0094 approved Answered(Details) as the public variant shape"
)]
pub enum RecoverableDetails {
    /// Full details from the one request that answered this member.
    Answered(Details),
    /// The member failed without stopping its request's other members.
    Failed {
        /// The safe public failure class.
        kind: ErrorKind,
        /// Whether a later independent call might succeed.
        retryable: bool,
        /// Why a logical answer failed; absent for a request failure.
        cause: Option<FailureCause>,
    },
}

fn spent() -> RecoverableDetails {
    RecoverableDetails::Failed {
        kind: ErrorKind::Usage,
        retryable: false,
        cause: None,
    }
}

fn failed(error: &Error) -> RecoverableDetails {
    RecoverableDetails::Failed {
        kind: error.kind(),
        retryable: error.retryable(),
        cause: None,
    }
}

fn selected(
    question: &Question,
    options: &CallOptions<'_>,
    engine: &Engine,
) -> Result<core::Setting, Error> {
    if let Some(typed) = options.batch_setting() {
        return Ok(typed.into());
    }
    if let Some(engine) = engine.batch {
        return Ok(engine);
    }
    let Some(file) = question.batch.as_ref() else {
        return Ok(core::Setting::Max);
    };
    core::Setting::of_json(file)
        .ok_or_else(|| Error::usage("batch takes max or a whole number of at least 1"))
}

/// What each native row needs to become its details.
struct Native<'a> {
    question: &'a Question,
    backend: &'a core::Backend,
    profile: Option<&'a core::BackendProfile>,
    setting: core::Setting,
    context_sha256: Option<&'a str>,
}

impl Native<'_> {
    fn answered(&self, decided: &Decided) -> Result<RecoverableDetails, Error> {
        let detail = Details::of_native_member(
            decided.member(self.question)?,
            self.question,
            self.backend,
            self.profile,
            self.setting,
            self.context_sha256,
        )?;
        Ok(RecoverableDetails::Answered(detail))
    }
}

/// One native row, or the stop that ends the vector: a fatal error, or a
/// spent budget that leaves every later row spent.
enum Taken {
    Row(Box<RecoverableDetails>),
    Spent,
    Fatal(Error),
}

fn take(native: &Native<'_>, row: Row<Decisions>) -> Taken {
    let one = |row| Taken::Row(Box::new(row));
    match row {
        Ok(decided) => native.answered(&decided).map_or_else(Taken::Fatal, one),
        Err(Failed::Asker(Miss::Failed(decided))) => match decided.outcome {
            AnswerOutcome::Failed(failure) => one(RecoverableDetails::Failed {
                kind: ErrorKind::Backend,
                retryable: false,
                cause: Some(cause(core::FailedValue::new(failure).cause())),
            }),
            AnswerOutcome::Answered(_) => Taken::Fatal(Error::defect("an answered row failed")),
        },
        Err(Failed::Asker(Miss::Refused(error))) => one(failed(&error)),
        Err(Failed::Pack { error, .. }) => one(failed(&packed(error))),
        Err(Failed::Engine { error, .. }) if error.spent() => Taken::Spent,
        Err(Failed::Engine { error, .. })
            if !matches!(
                error.kind(),
                EngineKind::Cancelled | EngineKind::Deadline | EngineKind::Defect
            ) =>
        {
            one(failed(&Error::from(error)))
        }
        Err(Failed::Engine { error, .. } | Failed::Stopped(error)) => {
            Taken::Fatal(Error::from(error))
        }
    }
}

impl Engine {
    /// Answer one bounded native vector while retaining safe per-member failures.
    /// Fatal cancellation, deadline and defect stop admission and join workers.
    ///
    /// # Errors
    /// Returns a fatal call error or a pre-send validation error.
    #[doc(hidden)]
    pub fn details_many_recoverable_with<Q: DetailQuestion + ?Sized>(
        &self,
        question: &Q,
        texts: &[String],
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<RecoverableDetails>>, Error> {
        let question = question.question();
        only(
            question,
            &[
                Kind::Decide,
                Kind::Banded,
                Kind::Choose,
                Kind::Tag,
                Kind::Score,
            ],
            "details_many_recoverable_with",
        )?;
        let setting = selected(question, &options, self)?;
        let context_sha256 = options
            .context_text()
            .map(|text| core::bytes_sha256(text.as_bytes()));
        let context = options.context_text().map(evidence).transpose()?;
        let stop = Stop::begin(options)?.with_prices(self.prices);
        let engine = self.asking(question)?;
        let asker = Decisions::new(&engine, question, context.clone());
        let packing = pull::packing(setting, context.is_some(), true);
        let native = Native {
            question,
            backend: engine.backend(),
            profile: self.profile.as_ref(),
            setting,
            context_sha256: context_sha256.as_deref(),
        };
        let count = texts.len();
        let inputs = texts
            .iter()
            .enumerate()
            .map(|(at, text)| Text {
                at,
                text: text.clone(),
            })
            .collect();
        stop.run_call(count, |cancel| {
            let mut results = Vec::with_capacity(count);
            let mut ended = None;
            let host = pull::eager(inputs, |row| match take(&native, row) {
                Taken::Row(row) => {
                    results.push(*row);
                    Flow::Continue
                }
                Taken::Spent => {
                    ended = Some(Ok(()));
                    Flow::Stop
                }
                Taken::Fatal(error) => {
                    ended = Some(Err(error));
                    Flow::Stop
                }
            });
            engine
                .ask_all(&asker, packing, host, cancel)
                .map_err(Error::from)?;
            if let Some(Err(error)) = ended {
                return Err(error);
            }
            results.resize_with(count, spent);
            Ok(results)
        })
    }
}

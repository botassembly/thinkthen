//! Bounded eager details for a native host that retains recoverable SQL rows.

use std::collections::VecDeque;

use crate::core::{self, AnswerOutcome, Batcher};
use crate::engine::facade;
use crate::public::annotated::{FailureCause, cause};
use crate::public::engine::{DetailQuestion, Engine, evidence, only};
use crate::public::error::{Error, ErrorKind};
use crate::public::options::{CallOptions, Stop};
use crate::public::question::Kind;
use crate::public::results::{Call, Details, Member, ParentReceipt};

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

struct Planned {
    batch: core::Batch,
    texts: Vec<String>,
}

enum Work {
    Planned(Planned),
    Failed(RecoverableDetails),
}

struct NativeOutcome {
    rows: Vec<RecoverableDetails>,
    denied: bool,
}

fn denied(error: &crate::engine::error::Error) -> bool {
    matches!(
        error,
        crate::engine::error::Error::EstimatedInput(_)
            | crate::engine::error::Error::SendBudgetFirst
            | crate::engine::error::Error::SendBudgetAdditional
            | crate::engine::error::Error::SendBudgetRetry(_)
    )
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

fn recoverable(kind: ErrorKind) -> bool {
    matches!(
        kind,
        ErrorKind::Usage | ErrorKind::Local | ErrorKind::Backend
    )
}

fn original_pairs(
    texts: &[String],
    question: &crate::public::question::Question,
) -> Result<Vec<(core::BatchRecord, core::Question)>, crate::engine::error::Error> {
    let mut pairs = Vec::with_capacity(texts.len());
    for text in texts {
        let evidence = core::Evidence::new(text.clone())
            .map_err(|_| crate::engine::error::Error::Defect("a planned record changed"))?;
        pairs.push((
            core::BatchRecord {
                evidence,
                value: core::Json::String(text.clone()),
            },
            question.core.clone(),
        ));
    }
    Ok(pairs)
}

fn selected(
    question: &crate::public::question::Question,
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

fn plan(
    engine: &facade::Engine,
    question: &crate::public::question::Question,
    texts: Vec<String>,
    setting: core::Setting,
    context: Option<&core::Evidence>,
) -> Result<Vec<Work>, Error> {
    let mut planner = Batcher::new(
        engine.backend().clone(),
        engine.profile().cloned(),
        question.core.clone(),
        setting,
        context.cloned(),
    )
    .map_err(Error::refused)?;
    let mut pending = VecDeque::new();
    let mut work = Vec::new();
    for text in texts {
        let Ok(evidence) = evidence(&text) else {
            if let Some(batch) = planner.pause().map_err(Error::refused)? {
                let texts = pending.drain(..batch.outcomes.len()).collect();
                work.push(Work::Planned(Planned { batch, texts }));
            }
            work.push(Work::Failed(RecoverableDetails::Failed {
                kind: ErrorKind::Usage,
                retryable: false,
                cause: None,
            }));
            continue;
        };
        let record = core::BatchRecord {
            evidence,
            value: core::Json::String(text.clone()),
        };
        pending.push_back(text);
        let mut closed = Vec::new();
        let added = planner.push_with_question(record, question.core.clone(), &mut closed);
        for batch in closed {
            let texts = pending.drain(..batch.outcomes.len()).collect();
            work.push(Work::Planned(Planned { batch, texts }));
        }
        if added.is_err() {
            pending.pop_back();
            if let Some(batch) = planner.pause().map_err(Error::refused)? {
                let texts = pending.drain(..batch.outcomes.len()).collect();
                work.push(Work::Planned(Planned { batch, texts }));
            }
            work.push(Work::Failed(RecoverableDetails::Failed {
                kind: ErrorKind::Usage,
                retryable: false,
                cause: None,
            }));
        }
    }
    if let Some(batch) = planner.finish().map_err(Error::refused)? {
        let texts = pending.drain(..batch.outcomes.len()).collect();
        work.push(Work::Planned(Planned { batch, texts }));
    }
    if !pending.is_empty() {
        return Err(Error::defect("a native batch lost its records"));
    }
    Ok(work)
}

#[allow(
    clippy::too_many_arguments,
    reason = "one native member retains its saved question and run metadata"
)]
fn read(
    engine: &facade::Engine,
    question: &crate::public::question::Question,
    profile: Option<&core::BackendProfile>,
    setting: core::Setting,
    context_sha256: Option<&str>,
    batch: &core::Batch,
    result: Result<crate::engine::prepared_request::Answered, crate::engine::error::Error>,
    texts: &[String],
    parent: Option<(&facade::SplitParent, usize)>,
) -> Result<NativeOutcome, crate::engine::error::Error> {
    let answered = match result {
        Ok(answered) => answered,
        Err(error) => {
            if matches!(
                error.kind(),
                crate::engine::error::Kind::Cancelled
                    | crate::engine::error::Kind::Deadline
                    | crate::engine::error::Kind::Defect
            ) {
                return Err(error);
            }
            if denied(&error) {
                return Ok(NativeOutcome {
                    rows: (0..texts.len()).map(|_| spent()).collect(),
                    denied: true,
                });
            }
            let error = Error::from(error);
            debug_assert!(recoverable(error.kind()));
            return Ok(NativeOutcome {
                rows: (0..texts.len()).map(|_| failed(&error)).collect(),
                denied: false,
            });
        }
    };
    let mut rows = Vec::with_capacity(texts.len());
    for (position, (&place, _text)) in batch.outcomes.iter().zip(texts).enumerate() {
        match answered.reply.outcomes().get(place) {
            Some(AnswerOutcome::Answered(answer)) => {
                let (value, outcome) = answer.read(question.threshold);
                let receipt = parent.map(|(parent, offset)| ParentReceipt {
                    digest: &parent.digest,
                    sent: parent.sent,
                    total: parent.total,
                    offset,
                    closed: parent.closed,
                });
                let member = Member::from_batch(
                    batch,
                    &answered,
                    answer,
                    value,
                    outcome,
                    position,
                    setting,
                    context_sha256.is_some(),
                    receipt,
                )
                .map_err(|_| {
                    crate::engine::error::Error::Defect(
                        "a native member receipt could not be written",
                    )
                })?;
                let detail = Details::of_native_member(
                    member,
                    question,
                    engine.backend(),
                    profile,
                    setting,
                    context_sha256,
                )
                .map_err(|_| {
                    crate::engine::error::Error::Defect(
                        "a native member detail could not be written",
                    )
                })?;
                rows.push(RecoverableDetails::Answered(detail));
            }
            Some(AnswerOutcome::Failed(failed)) => rows.push(RecoverableDetails::Failed {
                kind: ErrorKind::Backend,
                retryable: false,
                cause: Some(cause(core::FailedValue::new(*failed).cause())),
            }),
            None => {
                return Err(crate::engine::error::Error::Defect(
                    "a batch lost its member outcome",
                ));
            }
        }
    }
    Ok(NativeOutcome {
        rows,
        denied: false,
    })
}

struct NativeRun<'a> {
    engine: &'a facade::Engine,
    question: &'a crate::public::question::Question,
    profile: Option<&'a core::BackendProfile>,
    setting: core::Setting,
    context: Option<&'a core::Evidence>,
    context_sha256: Option<&'a str>,
}

impl NativeRun<'_> {
    fn read(
        &self,
        batch: &core::Batch,
        answered: Result<crate::engine::prepared_request::Answered, crate::engine::error::Error>,
        texts: &[String],
        parent: Option<(&facade::SplitParent, usize)>,
    ) -> Result<NativeOutcome, crate::engine::error::Error> {
        read(
            self.engine,
            self.question,
            self.profile,
            self.setting,
            self.context_sha256,
            batch,
            answered,
            texts,
            parent,
        )
    }

    fn answer(
        &self,
        work: Work,
        cancel: &crate::engine::Cancel<'_>,
    ) -> Result<NativeOutcome, crate::engine::error::Error> {
        match work {
            Work::Failed(failed) => Ok(NativeOutcome {
                rows: vec![failed],
                denied: false,
            }),
            Work::Planned(work) => self.answer_planned(work, cancel),
        }
    }

    fn answer_planned(
        &self,
        work: Planned,
        cancel: &crate::engine::Cancel<'_>,
    ) -> Result<NativeOutcome, crate::engine::error::Error> {
        let sent = self.engine.ask_record_batch_with_one_split(
            &work.batch,
            || original_pairs(&work.texts, self.question),
            self.context,
            cancel,
            |_, result| {
                if result.as_ref().err().is_some_and(denied) {
                    facade::SplitDecision::Stop
                } else {
                    facade::SplitDecision::SendRight
                }
            },
        )?;
        self.decode(&work, sent)
    }

    fn decode(
        &self,
        work: &Planned,
        sent: facade::OneSplit,
    ) -> Result<NativeOutcome, crate::engine::error::Error> {
        match sent {
            facade::OneSplit::Single(answered) => {
                self.read(&work.batch, answered, &work.texts, None)
            }
            facade::OneSplit::Halved {
                parent,
                left,
                right,
            } => {
                let (left_batch, left_answer) = *left;
                let count = left_batch.outcomes.len();
                let left_texts =
                    work.texts
                        .get(..count)
                        .ok_or(crate::engine::error::Error::Defect(
                            "a split lost its left records",
                        ))?;
                let right_texts =
                    work.texts
                        .get(count..)
                        .ok_or(crate::engine::error::Error::Defect(
                            "a split lost its right records",
                        ))?;
                let mut outcome =
                    self.read(&left_batch, left_answer, left_texts, Some((&parent, 0)))?;
                if let Some(right) = right {
                    let (right_batch, right_answer) = *right;
                    let right = self.read(
                        &right_batch,
                        right_answer,
                        right_texts,
                        Some((&parent, count)),
                    )?;
                    outcome.denied |= right.denied;
                    outcome.rows.extend(right.rows);
                } else if outcome.denied {
                    outcome.rows.extend((0..right_texts.len()).map(|_| spent()));
                } else {
                    return Err(crate::engine::error::Error::Defect(
                        "a native split lost its right result",
                    ));
                }
                Ok(outcome)
            }
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
        let count = texts.len();
        let work = plan(&engine, question, texts.to_vec(), setting, context.as_ref())?;
        let run = NativeRun {
            engine: &engine,
            question,
            profile: self.profile.as_ref(),
            setting,
            context: context.as_ref(),
            context_sha256: context_sha256.as_deref(),
        };
        stop.run_call(count, |cancel| {
            let mut results = Vec::with_capacity(count);
            let mut stopped = false;
            engine.ask_batches_recoverable(
                cancel,
                work,
                &|work| run.answer(work, cancel),
                |outcome| outcome.denied,
                |outcome| {
                    stopped |= outcome.denied;
                    results.extend(outcome.rows);
                    Ok::<(), Error>(())
                },
            )?;
            if stopped {
                results.resize_with(count, spent);
            }
            if results.len() != count {
                return Err(Error::defect("a native batch lost its result rows"));
            }
            Ok(results)
        })
    }
}

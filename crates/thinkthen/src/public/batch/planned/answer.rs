//! One planned request, its optional split, and ordered row packets.

use std::sync::atomic::{AtomicU64, Ordering};

use super::{Packet, Work};
use crate::core::{self, AnswerOutcome, BatchRecord};
use crate::engine::facade::{self, Completed};
use crate::public::error::Error;
use crate::public::results::Member;
use crate::public::results::ObservedQuestion;
use crate::public::results::ParentReceipt;

struct ParentAttempt {
    digest: String,
    sent: u64,
    total: usize,
    offset: usize,
    closed: core::batch::Closed,
}

impl ParentAttempt {
    fn add_to(&self, detail: &mut ObservedQuestion, position: usize) -> Result<(), Error> {
        if self.sent == 0 {
            return Ok(());
        }
        detail.requests.insert(0, self.digest.clone());
        detail.requests_sent = detail
            .requests_sent
            .checked_add(core::share(self.sent, self.total, self.offset + position))
            .ok_or_else(|| Error::defect("a batch request share overflowed"))?;
        Ok(())
    }
}

struct Observation<'a> {
    enabled: bool,
    details: bool,
    setting: core::Setting,
    context: bool,
    parent: Option<&'a ParentAttempt>,
}

impl Observation<'_> {
    fn add_to(&self, detail: &mut ObservedQuestion, position: usize) -> Result<(), Error> {
        self.parent
            .map_or(Ok(()), |parent| parent.add_to(detail, position))
    }

    fn member(
        &self,
        batch: &core::Batch,
        answered: &crate::engine::prepared_request::Answered,
        answer: &core::Answer,
        read: &(core::Value, core::Outcome),
        position: usize,
    ) -> Result<Option<Member>, Error> {
        if !self.details {
            return Ok(None);
        }
        let parent = self.parent.map(|parent| ParentReceipt {
            digest: &parent.digest,
            sent: parent.sent,
            total: parent.total,
            offset: parent.offset,
            closed: parent.closed,
        });
        Member::from_batch(
            batch,
            answered,
            answer,
            read.0.clone(),
            read.1,
            position,
            self.setting,
            self.context,
            parent,
        )
        .map(Some)
    }
}

#[allow(
    clippy::too_many_arguments,
    clippy::too_many_lines,
    reason = "one worker needs the selected question, call controls and bounded detail flag"
)]
pub(super) fn answer(
    engine: &facade::Engine,
    work: &Work,
    question: &core::Question,
    threshold: Option<core::Threshold>,
    tuned_for: Option<&core::ProfileName>,
    context: Option<&core::Evidence>,
    cancel: &crate::engine::Cancel<'static>,
    observing: bool,
    details: bool,
    setting: core::Setting,
) -> Result<Completed<Vec<Packet>, Error>, Error> {
    let attempted = AtomicU64::new(0);
    match engine.ask_batch_with_attempts(
        &work.batch,
        cancel,
        (observing || details).then_some(&attempted),
    ) {
        Ok(answered) => rows(
            engine,
            &work.batch,
            answered,
            threshold,
            tuned_for,
            Observation {
                enabled: observing,
                details,
                setting,
                context: context.is_some(),
                parent: None,
            },
        ),
        Err(error) if error.too_large() && work.texts.len() > 1 => {
            let parent = ParentAttempt {
                digest: work.batch.digest.as_str().to_owned(),
                sent: attempted.load(Ordering::Relaxed),
                total: work.batch.outcomes.len(),
                offset: 0,
                closed: work.batch.closed,
            };
            let records = work
                .texts
                .iter()
                .map(|text| Ok((record(text)?, question.clone())))
                .collect::<Result<Vec<_>, Error>>()?;
            let [left, right] = core::batch::halves_with_questions(
                engine.backend(),
                engine.profile(),
                context,
                records,
            )
            .map_err(Error::refused)?;
            let right_offset = left.outcomes.len();
            let left = rows(
                engine,
                &left,
                engine.ask_batch(&left, cancel).map_err(Error::from)?,
                threshold,
                tuned_for,
                Observation {
                    enabled: observing,
                    details,
                    setting,
                    context: context.is_some(),
                    parent: Some(&parent),
                },
            )?;
            if left.stop.is_some() {
                return Ok(left);
            }
            if let Some(stop) = cancel.stop() {
                return Ok(Completed {
                    stop: Some(stop.into()),
                    ..left
                });
            }
            let right = match engine.ask_batch(&right, cancel) {
                Ok(answered) => rows(
                    engine,
                    &right,
                    answered,
                    threshold,
                    tuned_for,
                    Observation {
                        enabled: observing,
                        details,
                        setting,
                        context: context.is_some(),
                        parent: Some(&ParentAttempt {
                            offset: right_offset,
                            ..parent
                        }),
                    },
                ),
                Err(error) => Err(Error::from(error)),
            };
            match right {
                Ok(right) => Ok(Completed {
                    value: left.value.into_iter().chain(right.value).collect(),
                    records: left.records + right.records,
                    replayed: left.replayed + right.replayed,
                    partial_failure: left.partial_failure || right.partial_failure,
                    stop: right.stop,
                }),
                Err(error) => Ok(Completed {
                    stop: Some(error),
                    ..left
                }),
            }
        }
        Err(error) => Err(error.into()),
    }
}

fn rows(
    engine: &facade::Engine,
    batch: &core::Batch,
    answered: crate::engine::prepared_request::Answered,
    threshold: Option<core::Threshold>,
    tuned_for: Option<&core::ProfileName>,
    observation: Observation<'_>,
) -> Result<Completed<Vec<Packet>, Error>, Error> {
    if batch.outcomes.len() != batch.questions.len() {
        return Err(Error::defect("a batch lost its row outcomes"));
    }
    let mut values = Vec::with_capacity(batch.outcomes.len());
    let mut stop = None;
    let mut completed = 0;
    for (position, &place) in batch.outcomes.iter().enumerate() {
        let outcome = answered.reply.outcomes().get(place);
        let detail = if observation.enabled {
            match (batch.row_questions.get(position), outcome) {
                (Some(question), Some(outcome)) => {
                    let mut detail = ObservedQuestion::from_reply(
                        question,
                        threshold,
                        tuned_for,
                        engine.backend(),
                        outcome,
                        &answered.reply,
                        answered.request.as_str(),
                        answered.requests_sent,
                        answered.replayed,
                        batch.outcomes.len(),
                        position,
                    )?;
                    observation.add_to(&mut detail, position)?;
                    Some(detail)
                }
                _ => return Err(Error::defect("a batch lost a question detail")),
            }
        } else {
            None
        };
        match outcome {
            Some(AnswerOutcome::Answered(answer)) => {
                let read = answer.read(threshold);
                let member = observation.member(batch, &answered, answer, &read, position)?;
                values.push(Packet {
                    value: Some((read.0, answer.yes().unwrap_or_default())),
                    detail,
                    member,
                });
                completed += 1;
            }
            _ => {
                values.push(Packet {
                    value: None,
                    detail,
                    member: None,
                });
                stop = Some(Error::of(
                    crate::public::error::ErrorKind::Backend,
                    "a backend question failed in a batch",
                ));
                break;
            }
        }
    }
    Ok(Completed {
        records: completed,
        replayed: if answered.replayed { completed } else { 0 },
        value: values,
        partial_failure: false,
        stop,
    })
}

pub(super) fn record(text: &str) -> Result<BatchRecord, Error> {
    Ok(BatchRecord {
        evidence: core::Evidence::new(text.to_owned())
            .map_err(|_| Error::usage("evidence is text, not white space"))?,
        value: core::Json::String(text.to_owned()),
    })
}

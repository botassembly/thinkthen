//! Complete one judged record and render its requested view.

use super::{Judging, RowContext, asked_of};
use crate::core::{Outcome, Question, Reading, Record, RecordValue, Value, json_line};
use crate::engine::facade::Judgment;
use crate::failure::Failure;
use crate::judge::Keeping;
use crate::result_json::{Run, decision_with_batch};
use crate::schedule::Judged;

impl Judging<'_> {
    pub(super) fn finish_row(
        &self,
        reading: &Reading,
        record: Record,
        arrived: Option<&[u8]>,
    ) -> Result<Judged, Failure> {
        let sending = asked_of(reading, record, &self.asks)?;
        let judged = self
            .engine
            .judge(
                &sending.question,
                self.threshold,
                sending.evidence,
                self.environment.cancel(),
            )
            .map_err(|error| {
                Failure::from(error).with_replay_document(&sending.question, self.streams)
            })?;
        self.row_of(
            reading,
            sending.record,
            sending.question,
            &judged,
            RowContext {
                arrived,
                batch: None,
            },
        )
    }

    /// Build the line one answered record prints. Both paths share it.
    pub(super) fn row_of(
        &self,
        reading: &Reading,
        record: Record,
        question: Question,
        judged: &Judgment,
        context: RowContext<'_>,
    ) -> Result<Judged, Failure> {
        let (outcome, replayed) = (judged.outcome, judged.answered.replayed);
        let probability = judged.answer.yes();
        let printed =
            if self.view.details && (self.keeping != Keeping::Passing || outcome == Outcome::Yes) {
                // `rank` orders and never selects, so a ranked row carries no
                // value. A value here would be a cut at 0.5 that nobody named.
                let shown = if self.keeping == Keeping::Ordered {
                    Value::YesNo(None)
                } else {
                    judged.value.clone()
                };
                let run = Run {
                    backend: self.engine.backend(),
                    tuned_for: self.tuned_for_profile(),
                    warning: self.mismatch.warning(),
                    batch_warning: self.mismatch.batch_warning(),
                    context_sha256: self
                        .context
                        .as_ref()
                        .map(|context| context.digest().to_owned()),
                };
                let input = self.streams.then_some(record);
                Some(decision_with_batch(
                    run,
                    judged,
                    question,
                    self.threshold,
                    shown,
                    input,
                    context.batch,
                )?)
            } else if self.keeping == Keeping::Passing && outcome != Outcome::Yes {
                None
            } else if self.keeping.streams_only() {
                Some(match context.arrived {
                    Some(bytes) => reading.as_it_arrived(bytes)?.to_owned(),
                    None => json_line(&record)?,
                })
            } else if self.view.raw {
                // One line stands for one record, so an unresolved record prints
                // an empty line. On one document it prints nothing at all.
                match judged.value.label() {
                    Some(label) => Some(label.to_owned()),
                    None if self.streams => Some(String::new()),
                    None => None,
                }
            } else if self.view.quiet {
                None
            } else if self.streams {
                Some(json_line(&RecordValue::new(record, judged.value.clone()))?)
            } else {
                Some(json_line(&judged.value)?)
            };
        Ok(Judged {
            printed,
            outcome,
            replayed,
            probability,
            partial_failure: false,
            profile_mismatch: self.mismatch.notice(),
        })
    }

    fn tuned_for_profile(&self) -> Option<&crate::core::ProfileName> {
        self.mismatch.tuned_for()
    }
}

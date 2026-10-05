//! Complete one judged record and render its requested view.

use super::{Judging, RowContext};
use crate::core::{Outcome, Question, Reading, Record, RecordValue, Value, json_line};
use crate::engine::facade::Judgment;
use crate::failure::Failure;
use crate::judge::Keeping;
use crate::result_json::{Run, decision_row};
use crate::schedule::Judged;

impl Judging<'_> {
    /// Build the line one answered record prints.
    pub(super) fn row_of(
        &self,
        reading: &Reading,
        record: Record,
        question: Question,
        judged: &Judgment,
        context: RowContext<'_>,
    ) -> Result<Judged, Failure> {
        let mismatch = match &question {
            Question::Decide { text, .. }
                if matches!(self.asks, super::Asks::Set(_))
                    && text.as_json().as_str().is_none() =>
            {
                self.mismatch.clone().with_batch(
                    None,
                    crate::core::Setting::Records(std::num::NonZeroUsize::MIN),
                )
            }
            _ => self.mismatch.clone(),
        };
        let (outcome, replayed) = (judged.outcome, judged.answered.replayed);
        let order_value = match &judged.value {
            Value::Score(position) => Some(*position),
            _ => judged.answer.yes(),
        };
        let original = context
            .position
            .filter(|p| p.located)
            .map(|_| record.clone());
        let mut printed =
            if self.view.details && (self.keeping != Keeping::Passing || outcome == Outcome::Yes) {
                // Ordinary rank has no cut and therefore no yes/no value.
                // Graded rank keeps the score value that orders its records.
                let shown = if self.keeping == Keeping::Ordered
                    && !matches!(question, Question::Score { .. })
                {
                    Value::YesNo(None)
                } else {
                    judged.value.clone()
                };
                let run = Run {
                    backend: self.engine.backend(),
                    tuned_for: mismatch.tuned_for(),
                    warning: mismatch.warning(),
                    batch_setting: mismatch.batch_setting(),
                    batch_warning: mismatch.batch_warning(),
                    context_sha256: self
                        .context
                        .as_ref()
                        .map(|context| context.digest().to_owned()),
                };
                let input = self.streams.then_some(record);
                Some(decision_row(
                    run,
                    judged,
                    question,
                    self.threshold,
                    shown,
                    input,
                    context.requests,
                    context.attempts,
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
        if self.view.details {
            crate::cli::intake::locate(&mut printed, context.position)?;
        }
        self.source_output(original, &mut printed, context.position)?;
        if self.documents && !context.position.is_some_and(|p| p.located) {
            crate::cli::intake::document(&mut printed, context.position, self.view.details)?;
        }
        Ok(Judged {
            model: Some(judged.answered.reply.model().clone()),
            printed,
            position: context.position.cloned(),
            outcome,
            replayed,
            order_value,
            partial_failure: false,
            profile_mismatch: mismatch.notice(),
        })
    }
    fn source_output(
        &self,
        original: Option<Record>,
        printed: &mut Option<String>,
        position: Option<&crate::cli::intake::Position>,
    ) -> Result<(), Failure> {
        let original_value = original.as_ref().map(crate::core::json_line).transpose()?;
        if let (Some(position), Some(original)) = (position.filter(|p| p.located), original) {
            if self.view.details || (self.streams && !self.keeping.streams_only() && !self.view.raw)
            {
                crate::cli::intake::source_members(printed, Some(position))?;
            } else if let Some(line) = printed {
                let value = original_value
                    .as_deref()
                    .filter(|_| self.keeping.streams_only())
                    .unwrap_or(line);
                *line = crate::cli::intake::source_value(&original, value, position)?;
            }
        }
        Ok(())
    }
}

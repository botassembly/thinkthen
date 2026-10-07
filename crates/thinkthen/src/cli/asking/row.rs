//! Complete one judged record and render its requested view.

use super::{Judging, RowContext};
use crate::core::{Outcome, Question, Reading, Record, RecordValue, Value, json_line};
use crate::engine::facade::Judgment;
use crate::failure::Failure;
use crate::judge::Keeping;
use crate::result_json::Run;
use crate::schedule::Judged;
use serde::{Serialize, Serializer};

struct Occurrence<'a> {
    canonical: &'a crate::core::CompleteAtomic,
    index: usize,
    original: &'a Record,
}
impl Serialize for Occurrence<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.canonical
            .serialize_occurrence(Some(self.original), None, Some(self.index), serializer)
    }
}

impl Judging<'_> {
    /// Build the line one answered record prints.
    pub(super) fn row_of(
        &self,
        reading: &Reading,
        record: Record,
        question: Question,
        judged: &Judgment,
        mut context: RowContext<'_>,
    ) -> Result<Judged, Failure> {
        let mismatch = self.mismatch_of(&question);
        let (outcome, replayed) = (judged.outcome, judged.answered.replayed);
        let order_value = match &judged.value {
            Value::Score(position) => Some(*position),
            _ => judged.answer.yes(),
        };
        let rank = if self.view.details && self.keeping == Keeping::Ordered {
            Some(crate::schedule::rank::RankRow {
                value: crate::schedule::rank::RankValue::Single(Box::new(self.canonical(
                    record.clone(),
                    question.clone(),
                    judged,
                    &mismatch,
                    &mut context,
                )?)),
                record: context.record,
                documents: self.documents,
            })
        } else {
            None
        };
        let original = context
            .position
            .filter(|p| p.located && !self.text_view)
            .map(|_| record.clone());
        let mut printed = if rank.is_some() {
            None
        } else if self.view.details && (self.keeping != Keeping::Passing || outcome == Outcome::Yes)
        {
            Some(self.detailed(record, question, judged, &mismatch, &mut context)?)
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
        if !self.image_output(context.images, &mut printed, context.position)? {
            self.source_output(original, &mut printed, context.position)?;
        }
        if self.documents && !context.position.is_some_and(|p| p.located) {
            crate::cli::intake::document(&mut printed, context.position, self.view.details)?;
        }
        Ok(Judged {
            rank,
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
    fn detailed(
        &self,
        record: Record,
        question: Question,
        judged: &Judgment,
        mismatch: &crate::profile::Mismatch,
        context: &mut RowContext<'_>,
    ) -> Result<String, Failure> {
        let canonical = self.canonical(record.clone(), question, judged, mismatch, context)?;
        if self.keeping == Keeping::Passing {
            Ok(json_line(&Occurrence {
                canonical: &canonical,
                index: context.record,
                original: &record,
            })?)
        } else {
            Ok(json_line(&canonical)?)
        }
    }
    fn canonical(
        &self,
        record: Record,
        question: Question,
        judged: &Judgment,
        mismatch: &crate::profile::Mismatch,
        context: &mut RowContext<'_>,
    ) -> Result<crate::core::CompleteAtomic, Failure> {
        // Ordinary rank has no cut and therefore no yes/no value.
        // Graded rank keeps the score value that orders its records.
        let shown =
            if self.keeping == Keeping::Ordered && !matches!(question, Question::Score { .. }) {
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
            context_sha256: context.context_sha256.clone(),
        };
        let input = (self.streams || self.keeping == Keeping::Ordered).then_some(record);
        let requests = std::mem::take(&mut context.requests);
        let attempts = std::mem::take(&mut context.attempts);
        let function = if self.keeping == Keeping::Ordered {
            crate::core::image::InputFunction::Rank
        } else if self.keeping == Keeping::Passing {
            crate::core::image::InputFunction::Filter
        } else {
            match question {
                Question::Decide { .. } => crate::core::image::InputFunction::Decide,
                Question::Choose { .. } => crate::core::image::InputFunction::Choose,
                Question::Tag { .. } => crate::core::image::InputFunction::Tag,
                Question::Score { .. } => crate::core::image::InputFunction::Score,
            }
        };
        let mut canonical = crate::result_json::complete::atomic(
            run,
            judged,
            crate::result_json::complete::AtomicSpec {
                declarations: self.declarations.clone(),
                function,
                record: context.record,
                question,
                threshold: self.threshold,
                shown,
                rank_position: None,
            },
            requests,
            input,
            Some(attempts),
        )?;
        canonical.images = context.images.map(|images| {
            images
                .images()
                .iter()
                .map(|image| image.0.clone())
                .collect()
        });
        Ok(canonical)
    }
    fn mismatch_of(&self, question: &Question) -> crate::profile::Mismatch {
        match question {
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
        }
    }
    fn image_output(
        &self,
        images: Option<&crate::public::ImageEvidence>,
        printed: &mut Option<String>,
        position: Option<&crate::cli::intake::Position>,
    ) -> Result<bool, Failure> {
        let Some(images) = images else {
            return Ok(false);
        };
        if self.view.details {
            if !self.streams {
                crate::cli::intake::image_input(printed, images)?;
            }
            crate::cli::intake::source_members(printed, position)?;
        } else if let Some(position) = position.filter(|p| p.located) {
            *printed = printed
                .as_ref()
                .map(|value| crate::cli::intake::source_value(images, value, position))
                .transpose()?;
        }
        Ok(true)
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

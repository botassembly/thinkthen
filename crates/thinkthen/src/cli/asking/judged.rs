//! `decide`, `filter`, `rank`, `choose`, `tag` and `score` over the one
//! question pipeline, by ADR 0111 section 4. A record thread frames records
//! on each ask, and each row comes back in input order.

use std::process::ExitCode;

use super::{Asks, JudgingInput};
use crate::core::pack::{self, Ask, PackError};
use crate::core::{
    AnswerOutcome, BackendProfile, BatchError, Descriptions, Evidence, ModelName, Plan, Reading,
    Record, Setting, Url, quoted_plan, quoted_plan_of,
};
use crate::failure::Failure;
use crate::failure::context::Limits;
use crate::judge::Keeping;
use crate::schedule::{Output, Placed};

/// One framed record, and the bytes it arrived as when it arrived as a line.
pub(super) struct Held {
    pub(super) record: Record,
    pub(super) images: Option<crate::public::ImageEvidence>,
    pub(super) context: Option<crate::public::RecordContext>,
    pub(super) arrived: Option<Vec<u8>>,
    pub(super) at: usize,
    pub(super) ordinal: usize,
    pub(super) position: Option<crate::cli::intake::Position>,
}

pub(super) type Records = Box<dyn Iterator<Item = Result<Held, Placed>> + Send>;

/// Frame the input as records: table rows, lines, or one document.
pub(super) fn records(
    configuration: &JudgingInput<'_>,
    reading: &Reading,
    source: crate::cli::intake::Intake,
) -> Result<Records, Failure> {
    let reading = reading.clone();
    let context_field = configuration.context_field.clone();
    let context_schema = configuration.declarations.context_schema.clone();
    let declarations = configuration.declarations.clone();
    let streams = reading.streams();
    let limited = configuration.keeping == Keeping::Ordered && configuration.common.located();
    let mut remaining = crate::core::MAX_RECORD_BYTES;
    let mut source = source.enumerate();
    let mut stopped = false;
    Ok(Box::new(std::iter::from_fn(move || {
        if stopped {
            return None;
        }
        let (ordinal, item) = source.next()?;
        let held = item.and_then(|item| {
            let (record, arrived, images) = match item.data {
                crate::cli::intake::Data::Record(record) => (record, None, None),
                crate::cli::intake::Data::Images(images) => (
                    reading
                        .record(images.text().unwrap_or_default().as_bytes())
                        .map_err(|error| Placed::at(Failure::record(error, streams), item.at))?,
                    None,
                    Some(images),
                ),
                crate::cli::intake::Data::Bytes(bytes) => {
                    let record = reading
                        .record(&bytes)
                        .map_err(|error| Placed::at(Failure::record(error, streams), item.at))?;
                    (record, Some(bytes), None)
                }
            };
            if !streams
                && !reading.has_fields()
                && reading.declares_item()
                && let Some(images) = item.images.as_ref().or(images.as_ref())
            {
                declarations
                    .validate_item(&crate::public::QuestionInput::Images(images.clone()))
                    .map_err(|error| Placed::at(Failure::Image(error.to_string()), item.at))?;
            }
            let held = Held {
                context: super::context::record(
                    &record,
                    context_field.as_deref(),
                    context_schema.as_ref(),
                )
                .map_err(|error| Placed::at(error, item.at))?,
                record,
                images: item.images.or(images),
                arrived,
                at: item.at,
                ordinal,
                position: item.position,
            };
            if limited {
                super::reading::charge(&reading, &held, &mut remaining)?;
            }
            Ok(held)
        });
        stopped = held.is_err();
        Some(held)
    })))
}

/// What one record's plan needs besides the record.
pub(super) struct Planner<'a> {
    pub(super) asks: &'a Asks,
    pub(super) reading: &'a Reading,
    pub(super) asked: (ModelName, Descriptions),
    pub(super) context: Option<Evidence>,
    pub(super) profile: Option<&'a BackendProfile>,
    pub(super) limits: Limits,
    pub(super) route: crate::core::adapters::built_in::images::ImageRoute,
}

impl Planner<'_> {
    /// The quoted plan one record sends. A stream's record quotes the JSON
    /// value a batch has always quoted, and one document quotes its evidence.
    pub(super) fn plans(&self, held: &Held) -> Result<Vec<Plan>, Failure> {
        let context = self.context_for(held)?;
        if let Some(images) = &held.images {
            if self.reading.declares_item() && images.text().is_none() {
                return Err(Failure::Record(crate::core::RecordError::ItemSchema));
            }
            if self.reading.declares_item() && (self.reading.streams() || self.reading.has_fields())
            {
                self.reading.evidence(&held.record)?;
            }
            let mut state = images.state();
            if self.reading.streams() || self.reading.has_fields() {
                state.text = self.reading.batch_record(&held.record)?.value;
            }
            return crate::core::image::plan(
                self.asked.clone(),
                state,
                context.as_ref(),
                self.asks.questions(&held.record)?,
                self.profile,
                self.route,
            )
            .map(|plan| vec![plan])
            .map_err(|error| self.limits.refused(error, false));
        }
        let record = &held.record;
        self.asks
            .questions(record)?
            .into_iter()
            .map(|question| self.plan(record, question, context.as_ref()))
            .collect()
    }

    pub(super) fn context_for(&self, held: &Held) -> Result<Option<Evidence>, Failure> {
        match held.context.as_ref() {
            Some(context) => crate::public::RecordContext::resolved(Some(context), None)
                .map_err(|_| Failure::Usage("the per-item context does not match context_schema")),
            None => Ok(self.context.clone()),
        }
    }

    fn plan(
        &self,
        record: &Record,
        question: crate::core::Question,
        context: Option<&Evidence>,
    ) -> Result<Plan, Failure> {
        let planned = if self.reading.streams() {
            let batch = self.reading.batch_record(record)?;
            quoted_plan_of(
                self.asked.clone(),
                batch.evidence,
                &batch.value,
                context,
                vec![question],
                self.profile,
            )
        } else {
            quoted_plan(
                self.asked.clone(),
                self.reading.evidence(record)?,
                context,
                vec![question],
                self.profile,
            )
        };
        planned.map_err(|error| self.limits.refused(error, false))
    }

    /// The wire questions one record sends.
    pub(super) fn asks(
        &self,
        api: crate::core::adapters::ApiType,
        url: &Url,
        held: &Held,
    ) -> Result<Vec<Ask>, Failure> {
        let mut asks = Vec::new();
        for plan in self.plans(held)? {
            asks.extend(
                pack::asks_for(api, url, &plan).map_err(|error| super::encoded(&plan, error))?,
            );
        }
        Ok(asks)
    }

    /// The command's refusal of a question the packer cannot send.
    pub(super) fn refused(&self, error: PackError) -> Failure {
        match error {
            PackError::Profile(limit) => Failure::ProfileLimit(limit),
            PackError::Context {
                initial,
                kind,
                limit,
                actual,
            } => self.limits.refused(
                BatchError::ContextOverLimit {
                    kind,
                    limit,
                    actual,
                },
                initial,
            ),
        }
    }
}

/// Frame host input, then execute the retained native request.
pub(super) fn run(
    mut configuration: JudgingInput<'_>,
    reading: &Reading,
    source: crate::cli::intake::Intake,
    setting: Option<Setting>,
    output: &mut Output<'_>,
) -> Result<ExitCode, Failure> {
    let records = records(&configuration, reading, source)?;
    if configuration.common.dry_run {
        let context = configuration.context.as_ref().map(super::Context::evidence);
        let inputs = if !reading.streams() {
            Some(1)
        } else {
            setting.and_then(|setting| match setting {
                Setting::Records(most) => Some(most.get()),
                Setting::Max => None,
            })
        };
        return super::plan::packed(&configuration, reading, records, context, inputs, output);
    }
    let admitted = configuration.admitted.take().ok_or(Failure::Defect(
        "judgment execution has no admitted request",
    ))?;
    super::native::run(configuration, admitted, reading, records, setting, output)
}

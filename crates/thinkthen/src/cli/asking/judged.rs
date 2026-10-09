//! `decide`, `filter`, `rank`, `choose`, `tag` and `score` over the one
//! question pipeline, by ADR 0111 section 4. A record thread frames records
//! on each ask, and each row comes back in input order.

use std::process::ExitCode;

use super::JudgingInput;
use crate::core::{Reading, Record, Setting};
use crate::failure::Failure;
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

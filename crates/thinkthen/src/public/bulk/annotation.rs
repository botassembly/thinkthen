//! Grouped annotation of one input record.

use super::{Values, evidence};
use crate::core::{self, Plan};
use crate::engine::facade::{self, Completed};
use crate::public::error::Error;
use crate::public::results::{ObservedQuestion, Written};

/// Answer every question of the set for one record.
pub(super) fn annotated(
    engine: &facade::Engine,
    set: &core::QuestionSet,
    text: &str,
    cancel: &crate::engine::Cancel<'_>,
    observing: bool,
) -> Result<Completed<Values, Error>, Error> {
    let record = record(set, text)?;
    let parts = set
        .groups()
        .into_iter()
        .map(|places| Ok((set.group_evidence(&places, &record)?, places)))
        .collect::<Result<Vec<_>, core::PartError>>()
        .map_err(|error| match error {
            core::PartError::Record(error) => Error::usage(error.to_string()),
            core::PartError::Reading(_) => {
                Error::defect("a checked question set could not read its parts")
            }
        })?;
    let model = engine.backend().model();
    let plan = |places: &[usize]| {
        let part = parts
            .iter()
            .find(|(_, held)| held == places)
            .map(|(part, _)| part.clone())
            .ok_or(crate::engine::error::Error::Defect(
                "an annotate group has no part",
            ))?;
        let questions = places
            .iter()
            .filter_map(|place| set.questions().get(*place));
        Plan::new(
            part,
            model.clone(),
            questions.map(|named| named.question().clone()).collect(),
        )
        .map_err(|_| crate::engine::error::Error::Defect("an annotate group asks nothing"))
    };
    let annotation = engine.annotate(set, plan, cancel)?;
    let observed = if observing {
        let model = annotation
            .model
            .as_ref()
            .unwrap_or(engine.backend().model());
        annotation
            .details
            .iter()
            .zip(&annotation.receipts)
            .map(|((name, entry), receipt)| {
                Ok((
                    name.clone(),
                    ObservedQuestion::from_annotated(
                        entry,
                        set.profile(),
                        engine.backend(),
                        model,
                        (receipt.usage, receipt.requests_sent, receipt.replayed),
                    )?,
                ))
            })
            .collect::<Result<Vec<_>, Error>>()?
    } else {
        Vec::new()
    };
    let json = Written::of(&core::NamedValues::new(annotation.values.clone()))?;
    Ok(Completed::one(
        Values {
            values: annotation.values,
            json,
            observed,
        },
        annotation.replayed,
        annotation.failed_questions > 0,
    ))
}

/// One record as its groups read it. Only a part group parses the text, once,
/// as the command reads a whole document, so a root-only set sends it as given.
fn record(set: &core::QuestionSet, text: &str) -> Result<core::BatchRecord, Error> {
    let evidence = evidence(text)?;
    if set.first_part().is_none() {
        let value = core::Json::String(text.to_owned());
        return Ok(core::BatchRecord { evidence, value });
    }
    let usage = |error: core::RecordError| Error::usage(error.to_string());
    let reading = core::Reading::new(core::Framing::Document, Vec::new())
        .map_err(|_| Error::defect("a document reading takes no pointer"))?;
    let held = reading.annotation_record(text.as_bytes()).map_err(usage)?;
    let value = reading.batch_record(&held).map_err(usage)?.value;
    Ok(core::BatchRecord { evidence, value })
}

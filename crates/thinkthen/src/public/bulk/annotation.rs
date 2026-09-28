//! Grouped annotation of one input record.

use super::{Values, evidence};
use crate::core;
use crate::engine::facade::{self, Completed};
use crate::public::error::Error;
use crate::public::results::{ObservedQuestion, Written};

/// Render a row after all its request-aligned group fragments arrive.
pub(crate) fn rendered(
    set: &core::QuestionSet,
    engine: &facade::Engine,
    annotation: facade::Annotation,
    observing: bool,
) -> Result<Completed<Values, Error>, Error> {
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
                let mut detail = ObservedQuestion::from_annotated(
                    entry,
                    set.profile(),
                    engine.backend(),
                    model,
                    (receipt.usage, receipt.requests_sent, receipt.replayed),
                )?;
                if let Some(parent) = &receipt.parent_request {
                    detail.prepend_request(parent.clone());
                }
                Ok((name.clone(), detail))
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
pub(crate) fn record(set: &core::QuestionSet, text: &str) -> Result<core::BatchRecord, Error> {
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

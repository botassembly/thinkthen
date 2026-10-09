//! Rendering one annotated record from the engine's ordered assembly.

use crate::core::{NamedValues, Outcome, Record, RecordValue, json_line};
use crate::engine::facade::{GroupAnswer, assemble};
use crate::failure::Failure;
use crate::schedule::Judged;

use super::Judging;

pub(super) fn finish(
    judging: &Judging<'_>,
    record: Record,
    ordinal: usize,
    answered: Vec<GroupAnswer>,
) -> Result<Judged, Failure> {
    let context = judging.context_for(&record)?;
    let annotation = assemble(&judging.set, answered, judging.engine.backend().model())?;
    let replayed = annotation.replayed;
    let failed_questions = annotation.failed_questions;
    let printed = if judging.common.details {
        let complete = crate::result_json::complete::annotation(
            &judging.engine,
            &judging.set,
            annotation,
            crate::result_json::complete::AnnotationRow {
                input: record,
                record: ordinal,
                context_sha256: context
                    .as_ref()
                    .map(|context| {
                        context
                            .as_text()
                            .map(|text| crate::core::bytes_sha256(text.as_bytes()))
                    })
                    .transpose()?,
                attempts: true,
            },
        )?;
        json_line(&complete)?
    } else if judging.streams && !record.is_object() {
        json_line(&RecordValue::new(
            record,
            NamedValues::new(annotation.values),
        ))?
    } else {
        json_line(&record.annotated(annotation.values))?
    };
    Ok(Judged {
        model: None,
        printed: Some(printed),
        position: None,
        outcome: Outcome::Yes,
        replayed,
        order_value: None,
        partial_failure: failed_questions > 0,
        profile_mismatch: judging.mismatch.notice(),
    })
}

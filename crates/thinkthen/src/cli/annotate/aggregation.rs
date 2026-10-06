//! Rendering one annotated record from the engine's ordered assembly.

use crate::core::{
    AnnotateMeta, AnnotateResult, NamedValues, Outcome, Record, RecordValue, RequestMeta, json_line,
};
use crate::engine::facade::{GroupAnswer, assemble};
use crate::failure::Failure;
use crate::schedule::Judged;

use super::Judging;

pub(super) fn finish(
    judging: &Judging<'_>,
    record: Record,
    answered: Vec<GroupAnswer>,
) -> Result<Judged, Failure> {
    let annotation = assemble(&judging.set, answered, judging.engine.backend().model())?;
    let replayed = annotation.replayed;
    let failed_questions = annotation.failed_questions;
    let printed = if judging.common.details {
        let meta = AnnotateMeta::new(
            env!("CARGO_PKG_VERSION"),
            judging.set.sha256()?,
            judging.engine.backend().url().clone(),
            annotation
                .model
                .ok_or(Failure::Defect("no group reported a model"))?,
            annotation.usage,
            RequestMeta::new(replayed, annotation.requests_sent, annotation.requests)
                .with_failed_questions(failed_questions)
                .with_profile_warning(judging.mismatch.warning()),
        )
        .with_reported_usage(annotation.reported_usage);
        json_line(&AnnotateResult::new(
            record,
            annotation.values,
            annotation.details,
            meta,
        ))?
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

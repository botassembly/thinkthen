//! Whole-frame recognition through one Rust call per text.

use super::{AccountedFailure, Stop};
use crate::result::{Completed, Observations};
use crate::worker::Controls;
use std::time::{Duration, Instant};
use thinkthen::{CallOptions, RecognizedEntity};

/// The call's deadline, resolved once at call start into an instant.
pub(super) fn due(controls: &Controls) -> Option<Instant> {
    controls
        .deadline
        .filter(|millis| *millis >= 0)
        .map(|millis| Duration::from_millis(millis as u64))
        .and_then(|budget| Instant::now().checked_add(budget))
}

/// Recognize present rows in one native collection, sharing deadline and facts.
pub(super) fn each_complete(
    engine: &thinkthen::Engine,
    ask: &thinkthen::Recognize,
    texts: &[&str],
    options: CallOptions<'_>,
    due: Option<Instant>,
) -> Result<Completed<Vec<thinkthen::Recognized>>, Stop> {
    let options = due.map_or(options, |at| options.deadline_at(at));
    let observed = Observations::default();
    let collector = observed.clone();
    let observer = |event: thinkthen::RecordObservation<'_>| collector.push(event);
    let records = texts.iter().map(|text| thinkthen::RecordInput {
        examples: None,
        original: (*text).to_owned(),
        context: None,
        options: None,
    });
    let result = engine.recognize_records_complete_with(ask, records, options.observe(&observer));
    match result {
        Ok(call) => {
            let details = observed
                .snapshot()
                .map_err(|_| Stop::Unwritten(Box::new(call.facts().clone())))?;
            Ok(Completed {
                value: call
                    .value()
                    .iter()
                    .map(|row| row.result().value().clone())
                    .collect(),
                facts: call.facts().clone(),
                details,
            })
        }
        Err(error) if error.facts().is_some() => {
            let facts = error
                .facts()
                .cloned()
                .ok_or("the started recognition lost its facts")?;
            let details = observed
                .snapshot()
                .map_err(|_| Stop::Unwritten(Box::new(facts.clone())))?;
            Err(Stop::Accounted(Box::new(AccountedFailure {
                error,
                facts,
                details,
            })))
        }
        Err(error) => Err(Stop::Engine(error)),
    }
}

/// The existing names-only frame view of complete native recognition.
pub(super) fn each_named(
    engine: &thinkthen::Engine,
    ask: &thinkthen::Recognize,
    texts: &[&str],
    options: CallOptions<'_>,
    due: Option<Instant>,
) -> Result<Completed<Vec<Vec<RecognizedEntity>>>, Stop> {
    each_complete(engine, ask, texts, options, due)
        .map(|done| done.map(|rows| rows.iter().map(|row| row.entities().to_vec()).collect()))
}

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

fn indexed(mut detail: serde_json::Value, index: usize) -> serde_json::Value {
    if let Some(fields) = detail.as_object_mut() {
        fields.insert("index".to_owned(), serde_json::json!(index));
    }
    detail
}

/// Each text's complete result: one `recognize_with` call per text, all under one
/// deadline. `max_requests` caps each text's call.
pub(super) fn each_complete(
    engine: &thinkthen::Engine,
    ask: &thinkthen::Recognize,
    texts: &[&str],
    options: CallOptions<'_>,
    due: Option<Instant>,
) -> Result<Completed<Vec<thinkthen::Recognized>>, Stop> {
    if engine.estimate_reported_cost(0, 0).is_some() {
        return Err(Stop::from(
            "priced recognition collections require native aggregate pricing",
        ));
    }
    let options = due.map_or(options, |at| options.deadline_at(at));
    let tally = thinkthen::Tally::new();
    let mut counted = false;
    let mut details = Vec::new();
    let mut rows = Vec::with_capacity(texts.len());
    for (index, text) in texts.iter().enumerate() {
        let observed = Observations::default();
        let collector = observed.clone();
        let observer = |event: thinkthen::RecordObservation<'_>| collector.push(event);
        let started = tally.start();
        let result = engine.recognize_with(ask, text, options.observe(&observer));
        let facts = match &result {
            Ok(found) => Some(found.facts()),
            Err(error) => error.facts(),
        };
        let Ok(seen) = observed.snapshot() else {
            return Err(Stop::Unwritten(Box::new(tally.facts())));
        };
        details.extend(seen.into_iter().map(|detail| indexed(detail, index)));
        if let Some(facts) = facts {
            if let Err(error) = started.finish(facts) {
                return Err(Stop::Accounted(Box::new(AccountedFailure {
                    error,
                    facts: tally.facts(),
                    details,
                })));
            }
            counted = true;
        }
        match result {
            Ok(found) => rows.push(found.into_value()),
            Err(error) if counted => {
                return Err(Stop::Accounted(Box::new(AccountedFailure {
                    error,
                    facts: tally.facts(),
                    details,
                })));
            }
            Err(error) => return Err(Stop::Engine(error)),
        }
    }
    Ok(Completed {
        value: rows,
        facts: tally.facts(),
        details,
    })
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

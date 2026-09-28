//! Whole-frame recognition through one Rust call per text.

use super::{AccountedFailure, Stop};
use crate::result::{Completed, Observations, OwnedFacts};
use crate::worker::Controls;
use std::time::{Duration, Instant};
use thinkthen::{CallOptions, RecognizedEntity};

/// The call's deadline, resolved once at call start into an instant.
pub(super) fn due(controls: &Controls) -> Option<Instant> {
    controls
        .deadline
        .filter(|seconds| *seconds >= 0.0)
        .and_then(|seconds| Duration::try_from_secs_f64(seconds).ok())
        .and_then(|budget| Instant::now().checked_add(budget))
}

fn merge(facts: &mut Option<OwnedFacts>, next: &thinkthen::Facts) {
    if let Some(all) = facts {
        all.combine(next);
    } else {
        *facts = Some(next.into());
    }
}

fn indexed(mut detail: serde_json::Value, index: usize) -> serde_json::Value {
    if let Some(fields) = detail.as_object_mut() {
        fields.insert("index".to_owned(), serde_json::json!(index));
    }
    detail
}

/// Each text's names: one `recognize_with` call per text, all under one
/// deadline. `max_requests` caps each text's call.
pub(super) fn each_named(
    engine: &thinkthen::Engine,
    ask: &thinkthen::Recognize,
    texts: &[&str],
    options: CallOptions<'_>,
    due: Option<Instant>,
) -> Result<Completed<Vec<Vec<RecognizedEntity>>>, Stop> {
    let options = due.map_or(options, |at| options.deadline_at(at));
    let started = Instant::now();
    let mut facts: Option<OwnedFacts> = None;
    let mut details = Vec::new();
    let mut rows = Vec::with_capacity(texts.len());
    for (index, text) in texts.iter().enumerate() {
        let observed = Observations::default();
        let collector = observed.clone();
        let observer = |event: thinkthen::RecordObservation<'_>| collector.push(event);
        match engine.recognize_with(ask, text, options.observe(&observer)) {
            Ok(found) => {
                merge(&mut facts, found.facts());
                rows.push(found.value().entities().to_vec());
            }
            Err(error) => {
                if let Some(next) = error.facts() {
                    merge(&mut facts, next);
                }
                details.extend(
                    observed
                        .snapshot()
                        .into_iter()
                        .map(|detail| indexed(detail, index)),
                );
                return Err(match facts {
                    Some(mut facts) => {
                        facts.seconds = started.elapsed().as_secs_f64();
                        Stop::Accounted(Box::new(AccountedFailure {
                            error,
                            facts,
                            details,
                        }))
                    }
                    None => Stop::Engine(error),
                });
            }
        }
        details.extend(
            observed
                .snapshot()
                .into_iter()
                .map(|detail| indexed(detail, index)),
        );
    }
    let mut facts = facts.unwrap_or_else(OwnedFacts::empty);
    facts.seconds = started.elapsed().as_secs_f64();
    Ok(Completed {
        value: rows,
        facts,
        details,
    })
}

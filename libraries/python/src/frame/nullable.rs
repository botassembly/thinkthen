//! Keep the caller's row places while the engine sees only present text.

use serde_json::Value;
use thinkthen::CallOptions;

use super::{AccountedFailure, Stop};
use crate::result::{Completed, Observations};

fn original(details: Vec<Value>, positions: &[usize]) -> Result<Vec<Value>, Stop> {
    details
        .into_iter()
        .map(|mut detail| {
            let compact = detail
                .get("index")
                .and_then(Value::as_u64)
                .and_then(|one| usize::try_from(one).ok())
                .ok_or("the engine observed a row without an index")?;
            let at = positions
                .get(compact)
                .ok_or("the engine observed a row outside the input")?;
            let place = detail
                .as_object_mut()
                .and_then(|fields| fields.get_mut("index"))
                .ok_or("the engine observed a row without an index")?;
            *place = serde_json::json!(at);
            Ok(detail)
        })
        .collect::<Result<_, &str>>()
        .map_err(Stop::from)
}

/// One call over present rows. A missing input has no observation and sends
/// nothing; a valid not-sure or failed answer still has its own observation.
pub(super) fn present<T>(
    rows: &[Option<&str>],
    options: CallOptions<'_>,
    job: impl FnOnce(&[&str], &[Option<&str>], CallOptions<'_>) -> Result<Completed<T>, Stop>,
) -> Result<Completed<T>, Stop> {
    let positions: Vec<usize> = rows
        .iter()
        .enumerate()
        .filter_map(|(at, text)| text.map(|_| at))
        .collect();
    let texts: Vec<&str> = rows.iter().flatten().copied().collect();
    let observed = Observations::default();
    let collector = observed.clone();
    let observer = |event: thinkthen::RecordObservation<'_>| collector.push(event);
    let result = job(&texts, rows, options.observe(&observer));
    match result {
        Ok(mut done) => {
            let seen = observed.snapshot();
            let details = match seen {
                Err(_) => return Err(Stop::Unwritten(Box::new(done.facts))),
                Ok(seen) if done.details.is_empty() => seen,
                Ok(_) => done.details,
            };
            done.details = original(details, &positions)
                .map_err(|error| error.with_facts(done.facts.clone()))?;
            Ok(done)
        }
        Err(Stop::Engine(error)) if error.facts().is_some() => {
            let facts = error
                .facts()
                .cloned()
                .ok_or("the started call lost its facts")?;
            let Ok(seen) = observed.snapshot() else {
                return Err(Stop::Unwritten(Box::new(facts)));
            };
            let details = original(seen, &positions)?;
            Err(Stop::Accounted(Box::new(AccountedFailure {
                error,
                facts,
                details,
            })))
        }
        Err(Stop::Accounted(mut failed)) => {
            failed.details = original(failed.details, &positions)?;
            Err(Stop::Accounted(failed))
        }
        Err(error) => Err(error),
    }
}

/// Expand one result per sent text back to the input's exact row places.
pub(super) fn aligned<T>(
    rows: &[Option<&str>],
    done: Completed<Vec<T>>,
) -> Result<Completed<Vec<Option<T>>>, Stop> {
    let Completed {
        value,
        facts,
        details,
    } = done;
    let mut sent = value.into_iter();
    let mut aligned = Vec::with_capacity(rows.len());
    for row in rows {
        aligned.push(if row.is_some() {
            Some(
                sent.next()
                    .ok_or_else(|| Stop::after("the engine omitted an input row", facts.clone()))?,
            )
        } else {
            None
        });
    }
    if sent.next().is_some() {
        return Err(Stop::after("the engine added an input row", facts));
    }
    Ok(Completed {
        value: aligned,
        facts,
        details,
    })
}

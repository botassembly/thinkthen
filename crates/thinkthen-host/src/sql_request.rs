//! SQLite host arguments enter the native shared Request without serialization.
use crate::complete_native::{Inputs, Prepared, defect};
mod rows;
pub use rows::{project_carrier, project_outcome, project_value, typed_values};
use serde_json::{Value, json};
use std::sync::Mutex;
use thinkthen::{
    AdmittedRequest, CallOptions, Engine, Error, LoadedQuestion, RecordObservation, Request,
    RequestArguments, RequestDefinition, RequestEnvironment, RequestFeed, RequestInput,
    RequestOptions, RequestOutcome, RequestQuestion, RequestValue, Surface,
};

pub fn run(
    engine: &Engine,
    prepared: &Prepared,
    request: &AdmittedRequest,
    inputs: Inputs,
    options: CallOptions<'_>,
    surface: Surface,
) -> Value {
    let cancelled = thinkthen::CancelToken::new();
    if inputs.cancelled {
        cancelled.cancel();
    }
    let events = Mutex::new(Vec::new());
    let observer = |event: RecordObservation<'_>| {
        events
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .push(
                observation(&event, request.request().call.function())
                    .unwrap_or_else(|error| crate::complete_native::failure(&error)),
            );
    };
    let options = options
        .surface(surface)
        .attempts(inputs.attempts)
        .observe(&observer);
    let options = if inputs.cancelled {
        options.cancel(&cancelled)
    } else {
        options
    };
    let result = (|| {
        let feed = if matches!(
            request.request().call.arguments().input,
            RequestInput::Feed { .. }
        ) {
            let reading = request.record_reading()?;
            let records = inputs.deferred_records(prepared.reading().or(Some(&reading)));
            let feed = RequestFeed::from_records("sqlite", records);
            let feed = if matches!(prepared, Prepared::Filter(_)) {
                feed.with_all_filter_results()
            } else {
                feed
            };
            let feed = if inputs.image_inputs()? {
                feed.with_image_inputs()
            } else {
                feed
            };
            let feed = if inputs.incremental {
                feed
            } else {
                feed.eager()
            };
            Some(feed)
        } else {
            None
        };
        let outcome = engine.execute_request(
            request,
            RequestEnvironment {
                controls: options,
                feed,
            },
        )?;
        render(outcome)
    })();
    let native = result.unwrap_or_else(|error| crate::complete_native::failure(&error));
    let events = events
        .into_inner()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    crate::complete_native::carrier(native, Value::Array(events))
}

fn observation(
    event: &RecordObservation<'_>,
    function: thinkthen::RequestFunction,
) -> Result<Value, Error> {
    let mut value = crate::complete_native::observations::event(event)?;
    if matches!(event, RecordObservation::Question { .. }) {
        let packet = thinkthen::RequestSessionResult::Observation {
            function,
            value: event.to_owned(),
        };
        let native: Value = serde_json::from_str(&packet.to_json()?).map_err(|_| defect())?;
        if let Some(detail) = native.pointer("/value/detail") {
            crate::complete_native::put(&mut value, "detail", detail.clone())?;
        }
    }
    Ok(value)
}

pub fn definition(prepared: &Prepared) -> RequestDefinition {
    match prepared {
        Prepared::Atomic(q) => RequestDefinition::Atomic(q.clone()),
        Prepared::Dynamic(q) => RequestDefinition::DynamicChoose(q.clone()),
        Prepared::Filter(q) => q.clone().into(),
        Prepared::Find(q, reading) => (q.clone(), reading.clone()).into(),
        Prepared::Rank(q) => RequestDefinition::Rank(q.clone()),
        Prepared::RankSet(q) => q.clone().into(),
        Prepared::Annotate(q) => q.clone().into(),
        Prepared::Recognize(q, reading) => (q.clone(), reading.clone()).into(),
        Prepared::Relate(q) => q.clone().into(),
    }
}

pub fn admit(prepared: &Prepared) -> Result<AdmittedRequest, Error> {
    request(prepared, RequestOptions::default())?.admit()
}

pub fn request(prepared: &Prepared, options: RequestOptions) -> Result<Request, Error> {
    request_input(
        prepared,
        RequestInput::Feed {
            name: "sqlite".to_owned(),
            framing: Default::default(),
            reading: Default::default(),
            images: Vec::new(),
        },
        options,
    )
}

/// Build an explicit file request using the existing native source reader.
pub fn request_source(
    prepared: &Prepared,
    source: thinkthen::RequestSource,
    options: RequestOptions,
) -> Result<Request, Error> {
    request_input(prepared, RequestInput::Source { source }, options)
}

fn request_input(
    prepared: &Prepared,
    input: RequestInput,
    options: RequestOptions,
) -> Result<Request, Error> {
    let args = RequestArguments {
        question: RequestQuestion::Definition {
            value: definition(prepared),
        },
        input,
        options,
    };
    let function = match prepared {
        Prepared::Atomic(LoadedQuestion::Banded(_)) => thinkthen::RequestFunction::Decide,
        Prepared::Atomic(LoadedQuestion::Question(q)) => match q.kind() {
            thinkthen::QuestionKind::Decide => thinkthen::RequestFunction::Decide,
            thinkthen::QuestionKind::Choose => thinkthen::RequestFunction::Choose,
            thinkthen::QuestionKind::Tag => thinkthen::RequestFunction::Tag,
            thinkthen::QuestionKind::Score => thinkthen::RequestFunction::Score,
            _ => return Err(defect()),
        },
        Prepared::Dynamic(_) => thinkthen::RequestFunction::Choose,
        Prepared::Filter(_) => thinkthen::RequestFunction::Filter,
        Prepared::Rank(_) | Prepared::RankSet(_) => thinkthen::RequestFunction::Rank,
        Prepared::Find(_, _) => thinkthen::RequestFunction::Find,
        Prepared::Annotate(_) => thinkthen::RequestFunction::Annotate,
        Prepared::Recognize(_, _) => thinkthen::RequestFunction::Recognize,
        Prepared::Relate(_) => thinkthen::RequestFunction::Relate,
    };
    Ok(Request::for_function(function, args))
}
fn render(outcome: RequestOutcome) -> Result<Value, Error> {
    match outcome {
        RequestOutcome::Complete(call) => {
            let mut value = serde_json::to_value(&call).map_err(|_| defect())?;
            supplement(&mut value, call.value())?;
            Ok(value)
        }
        RequestOutcome::Failed { completed, error } => {
            let mut value = crate::complete_native::failure(&error);
            supplement(&mut value, &completed)?;
            crate::complete_native::put(
                &mut value,
                "completed",
                serde_json::to_value(completed).map_err(|_| defect())?,
            )?;
            Ok(value)
        }
    }
}
pub fn supplement(value: &mut Value, result: &RequestValue) -> Result<(), Error> {
    macro_rules! rows {
        ($rows:expr) => {
            crate::complete_native::put(
                value,
                "ordinals",
                json!($rows.iter().map(|row| row.ordinal()).collect::<Vec<_>>()),
            )
        };
    }
    match result {
        RequestValue::Decisions(v) => rows!(v),
        RequestValue::Choices(v) => rows!(v),
        RequestValue::Tags(v) => rows!(v),
        RequestValue::Scores(v) => rows!(v),
        RequestValue::Filtered(v) => rows!(v),
        RequestValue::Ranked(v) => rows!(v),
        RequestValue::SetRanked(v) => rows!(v),
        RequestValue::Annotations(v) => rows!(v),
        RequestValue::Recognized(v) => rows!(v),
        RequestValue::Found(v) => crate::complete_native::put(
            value,
            "selection",
            json!(match v.selection() {
                thinkthen::FindSelection::None => None,
                thinkthen::FindSelection::Unit(at) => Some(at),
            }),
        ),
        RequestValue::Related(_) => Ok(()),
    }
}

/// Execute SQL records with row recovery while retaining canonical call facts.
pub fn run_records_recovering<'a>(
    engine: &Engine,
    prepared: &Prepared,
    request: &AdmittedRequest,
    records: impl Iterator<Item = Result<thinkthen::RecordInput<thinkthen::QuestionInput>, Error>> + 'a,
    controls: CallOptions<'_>,
    surface: Surface,
) -> Result<Value, Error> {
    let name = match &request.request().call.arguments().input {
        RequestInput::Feed { name, .. } => name.as_str(),
        _ => "sql",
    };
    let feed = RequestFeed::from_records(name, records).eager();
    run_feed_recovering(engine, prepared, request, feed, controls, surface)
}

/// Execute a feed with descriptor-only image admission and native reader controls.
pub fn run_feed_recovering(
    engine: &Engine,
    prepared: &Prepared,
    request: &AdmittedRequest,
    feed: RequestFeed<'_>,
    controls: CallOptions<'_>,
    surface: Surface,
) -> Result<Value, Error> {
    recover(engine, prepared, request, Some(feed), controls, surface)
}

fn recover(
    engine: &Engine,
    prepared: &Prepared,
    request: &AdmittedRequest,
    feed: Option<RequestFeed<'_>>,
    controls: CallOptions<'_>,
    surface: Surface,
) -> Result<Value, Error> {
    let feed = feed.map(|feed| {
        if matches!(prepared, Prepared::Filter(_)) {
            feed.with_all_filter_results()
        } else {
            feed
        }
    });
    let events = Mutex::new(Vec::new());
    let observer = |event: RecordObservation<'_>| {
        events
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .push(
                observation(&event, request.request().call.function())
                    .unwrap_or_else(|error| crate::complete_native::failure(&error)),
            );
    };
    let recovered = Mutex::new(Vec::new());
    let sink = |row| {
        recovered
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .push(row)
    };
    let outcome = engine.execute_request_recovering_sink(
        request,
        RequestEnvironment {
            controls: controls.surface(surface).observe(&observer),
            feed,
        },
        &sink,
    );
    let mut carrier = match outcome {
        Ok(call) => json!({"native":{"facts":call.facts().complete()}}),
        Err(error) => failure_carrier(request.request().call.function(), error)?,
    };
    let facts = carrier
        .pointer("/native/facts")
        .cloned()
        .unwrap_or(Value::Null);
    let recovered = recovered
        .into_inner()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let events = events
        .into_inner()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let mut rows = Vec::new();
    for row in &recovered {
        let observed = events
            .iter()
            .filter(|event| {
                matches!(
                    request.request().call.function(),
                    thinkthen::RequestFunction::Rank
                        | thinkthen::RequestFunction::Find
                        | thinkthen::RequestFunction::Relate
                ) || event.get("index").is_none()
                    || event.get("index").and_then(Value::as_u64) == Some(row.ordinal() as u64)
            })
            .cloned()
            .collect::<Vec<_>>();
        match row.outcome() {
            Ok(result) => {
                let mut recovered = rows::complete(
                    request.request().call.function(),
                    row.ordinal(),
                    serde_json::to_value(result).map_err(|_| defect())?,
                    &facts,
                    &observed,
                );
                rows::typed_values(result, &mut recovered)?;
                rows.extend(recovered);
            }
            Err(error) => rows.push(rows::failed(
                row.ordinal(),
                serde_json::to_value(row)
                    .map_err(|_| defect())?
                    .get("failure")
                    .cloned()
                    .unwrap_or(
                        serde_json::to_value(error.complete().error()).map_err(|_| defect())?,
                    ),
                &facts,
                &observed,
            )),
        }
    }
    for row in &recovered {
        if let Ok(RequestValue::Found(found)) = row.outcome() {
            let selection = match found.selection() {
                thinkthen::FindSelection::None => None,
                thinkthen::FindSelection::Unit(index) => Some(index),
            };
            crate::complete_native::put(&mut carrier, "selection", json!(selection))?;
        }
    }
    if let Some(failed) = carrier.get("rows").and_then(Value::as_array) {
        rows.extend(failed.iter().cloned());
    }
    crate::complete_native::put(&mut carrier, "rows", json!(rows))?;
    crate::complete_native::put(&mut carrier, "observations", json!(events))?;
    Ok(carrier)
}

/// Decode SQL-owned readers only after admission, then use shared row recovery.
pub fn run_recovering(
    engine: &Engine,
    prepared: &Prepared,
    request: &AdmittedRequest,
    inputs: Inputs,
    controls: CallOptions<'_>,
    surface: Surface,
) -> Result<Value, Error> {
    let cancelled = thinkthen::CancelToken::new();
    if inputs.cancelled {
        cancelled.cancel();
    }
    let controls = controls.attempts(inputs.attempts);
    let controls = if inputs.cancelled {
        controls.cancel(&cancelled)
    } else {
        controls
    };
    if matches!(
        request.request().call.arguments().input,
        RequestInput::Source { .. }
    ) {
        return recover(engine, prepared, request, None, controls, surface);
    }
    let reading = request.record_reading()?;
    let name = match &request.request().call.arguments().input {
        RequestInput::Feed { name, .. } => name.as_str(),
        _ => "sql",
    };
    let feed = RequestFeed::from_records(
        name,
        inputs.deferred_records(prepared.reading().or(Some(&reading))),
    );
    let feed = if inputs.image_inputs()? {
        feed.with_image_inputs()
    } else {
        feed
    };
    let feed = if inputs.incremental {
        feed
    } else {
        feed.eager()
    };
    run_feed_recovering(engine, prepared, request, feed, controls, surface)
}

/// Retain one whole-call refusal with only metadata the actual error owns.
pub fn failure_carrier(function: thinkthen::RequestFunction, error: Error) -> Result<Value, Error> {
    if matches!(
        error.kind(),
        thinkthen::ErrorKind::Cancelled
            | thinkthen::ErrorKind::Deadline
            | thinkthen::ErrorKind::Defect
    ) {
        return Err(error);
    }
    let mut carrier = crate::complete_native::admission(&error);
    let rows = project_carrier(function, &carrier);
    crate::complete_native::put(&mut carrier, "rows", json!(rows))?;
    Ok(carrier)
}

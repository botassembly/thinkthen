//! Shared SQL request definitions, observations and row recovery.
use crate::complete_native::{Prepared, defect};
mod rows;
pub use rows::{project_carrier, typed_values};
use serde_json::{Value, json};
use std::sync::Mutex;
use thinkthen::{
    AdmittedRequest, CallOptions, Engine, Error, LoadedQuestion, RecordObservation, Request,
    RequestArguments, RequestDefinition, RequestEnvironment, RequestFeed, RequestInput,
    RequestOptions, RequestQuestion, RequestValue, Surface,
};

/// Serialize a borrowed SQL observation with its native question detail.
pub fn observation(
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

/// Translate a prepared SQL question into its native request definition.
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

/// Construct the shared SQL named-feed request with explicit call options.
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

/// Construct a native request for a prepared SQL definition and input.
pub fn request_input(
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

/// Execute an optional SQL feed with native row recovery and call facts.
pub fn recover(
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
        match row.outcome() {
            Ok(result) => {
                let mut recovered = rows::complete(
                    request.request().call.function(),
                    row.ordinal(),
                    serde_json::to_value(result).map_err(|_| defect())?,
                    &facts,
                    &events,
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
                &events,
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
    rows::observe_rows(request.request().call.function(), &mut rows, &events);
    crate::complete_native::put(&mut carrier, "rows", json!(rows))?;
    crate::complete_native::put(&mut carrier, "observations", json!(events))?;
    Ok(carrier)
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

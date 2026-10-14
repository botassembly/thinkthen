//! SQLite host arguments enter the native shared Request without serialization.
use crate::complete_native::{Inputs, Prepared, defect};
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
    let records = records.collect::<Vec<_>>();
    let image_inputs = records
        .iter()
        .filter_map(|row| row.as_ref().ok())
        .any(|row| matches!(row.original, thinkthen::QuestionInput::Images(_)));
    let feed = RequestFeed::from_records("sqlite", records.into_iter()).eager();
    let feed = if matches!(prepared, Prepared::Filter(_)) {
        feed.with_all_filter_results()
    } else {
        feed
    };
    let feed = if image_inputs {
        feed.with_image_inputs()
    } else {
        feed
    };
    let call = engine.execute_request_recovering(
        request,
        RequestEnvironment {
            controls: controls.surface(surface).observe(&observer),
            feed: Some(feed),
        },
    )?;
    let facts = serde_json::to_value(call.facts()).map_err(|_| defect())?;
    let events = events
        .into_inner()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let rows = call
        .value()
        .iter()
        .map(|row| {
            let observed = events
                .iter()
                .filter(|event| {
                    event.get("index").and_then(Value::as_u64) == Some(row.ordinal() as u64)
                })
                .cloned()
                .collect::<Vec<_>>();
            let (value, probability, result, failure) = match row.outcome() {
                Ok(result) => {
                    let result = serde_json::to_value(result).map_err(|_| defect())?;
                    let (value, probability) =
                        row_value(request.request().call.function(), &result);
                    let result = result
                        .as_array()
                        .and_then(|rows| rows.first())
                        .cloned()
                        .unwrap_or(result);
                    (value, probability, result, Value::Null)
                }
                Err(error) => (
                    Value::Null,
                    Value::Null,
                    Value::Null,
                    serde_json::to_value(error.complete().error()).map_err(|_| defect())?,
                ),
            };
            Ok(
                json!({"ordinal":row.ordinal(),"value":value,"probability":probability,
            "result":result,"failure":failure,"facts":facts,"observations":observed}),
            )
        })
        .collect::<Result<Vec<_>, Error>>()?;
    Ok(json!({"native":{"facts":facts},"rows":rows,"observations":events}))
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
    let reading = request.record_reading()?;
    run_records_recovering(
        engine,
        prepared,
        request,
        inputs.deferred_records(prepared.reading().or(Some(&reading))),
        controls,
        surface,
    )
}

fn row_value(function: thinkthen::RequestFunction, result: &Value) -> (Value, Value) {
    use thinkthen::RequestFunction as F;
    let records = result
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_else(|| std::slice::from_ref(result));
    let row = records.first().unwrap_or(&Value::Null);
    let answer = row.get("value").cloned().unwrap_or(Value::Null);
    let probability = match function {
        F::Decide | F::Filter => row
            .pointer("/answer/probability")
            .cloned()
            .unwrap_or(Value::Null),
        F::Choose | F::Find => row
            .pointer("/answer/pick")
            .and_then(Value::as_str)
            .and_then(|pick| {
                row.pointer("/answer/probabilities")
                    .and_then(|values| values.get(pick))
            })
            .cloned()
            .unwrap_or(Value::Null),
        F::Rank => Value::Array(
            records
                .iter()
                .map(|row| {
                    row.pointer("/answer/probability")
                        .cloned()
                        .unwrap_or(Value::Null)
                })
                .collect(),
        ),
        F::Recognize => answer
            .get("entities")
            .and_then(Value::as_array)
            .map(|entities| {
                Value::Array(
                    entities
                        .iter()
                        .map(|entity| entity.get("strength").cloned().unwrap_or(Value::Null))
                        .collect(),
                )
            })
            .unwrap_or(Value::Null),
        F::Relate => answer
            .as_array()
            .map(|edges| {
                Value::Array(
                    edges
                        .iter()
                        .map(|edge| edge.get("probability").cloned().unwrap_or(Value::Null))
                        .collect(),
                )
            })
            .unwrap_or(Value::Null),
        _ => Value::Null,
    };
    let value = match function {
        F::Rank => Value::Array(
            records
                .iter()
                .map(|row| row.get("index").cloned().unwrap_or(Value::Null))
                .collect(),
        ),
        F::Filter => {
            if answer == Value::Bool(true) {
                row.get("input").cloned().unwrap_or(Value::Null)
            } else {
                Value::Null
            }
        }
        F::Recognize => answer.get("entities").cloned().unwrap_or(Value::Null),
        _ => answer,
    };
    (value, probability)
}

/// Project an ordinary SQL call without changing its aggregate input semantics.
pub fn project_outcome(
    function: thinkthen::RequestFunction,
    outcome: RequestOutcome,
    observations: Vec<Value>,
) -> Result<Vec<Value>, Error> {
    let (result, facts, failure) = match outcome {
        RequestOutcome::Complete(call) => {
            let facts = serde_json::to_value(call.facts()).map_err(|_| defect())?;
            (call.into_value(), facts, None)
        }
        RequestOutcome::Failed { completed, error } => {
            if matches!(
                error.kind(),
                thinkthen::ErrorKind::Cancelled
                    | thinkthen::ErrorKind::Deadline
                    | thinkthen::ErrorKind::Defect
            ) {
                return Err(error);
            }
            let failed = crate::complete_native::failure(&error);
            (
                completed,
                failed.get("facts").cloned().unwrap_or(Value::Null),
                failed.get("error").cloned(),
            )
        }
    };
    let encoded = serde_json::to_value(&result).map_err(|_| defect())?;
    let results = encoded.as_array().cloned().unwrap_or_else(|| vec![encoded]);
    let mut rows = results
        .into_iter()
        .enumerate()
        .map(|(at, result)| {
            let ordinal = result
                .get("index")
                .and_then(Value::as_u64)
                .unwrap_or(at as u64);
            let (value, probability) = row_value(function, &result);
            json!({"ordinal":ordinal,"value":value,"probability":probability,
            "result":result,"failure":null,"facts":facts,"observations":observations})
        })
        .collect::<Vec<_>>();
    if let Some(failure) = failure {
        rows.push(json!({"ordinal":rows.len(),"value":null,"probability":null,
            "result":null,"failure":failure,"facts":facts,"observations":observations}));
    }
    Ok(rows)
}

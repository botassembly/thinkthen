//! SQLite owns input controls, cancellation and ordinary complete envelopes.
use super::inputs::Inputs;
use complete_native::{self, Prepared, defect};
use serde_json::{Value, json};
use std::sync::Mutex;
use thinkthen::{AdmittedRequest, CallOptions, Engine, Error, RecordObservation, Request, RequestEnvironment, RequestFeed, RequestInput, RequestOptions, RequestOutcome, RequestValue, Surface};
use thinkthen_host::sql_request::{observation, recover, run_feed_recovering};

pub(super) fn run(
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
                    .unwrap_or_else(|error| complete_native::failure(&error)),
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
    let native = result.unwrap_or_else(|error| complete_native::failure(&error));
    let events = events
        .into_inner()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    complete_native::carrier(native, Value::Array(events))
}

/// Build an explicit file request using the existing native source reader.
pub(super) fn request_source(
    prepared: &Prepared,
    source: thinkthen::RequestSource,
    options: RequestOptions,
) -> Result<Request, Error> {
    thinkthen_host::sql_request::request_input(prepared, RequestInput::Source { source }, options)
}

fn render(outcome: RequestOutcome) -> Result<Value, Error> {
    match outcome {
        RequestOutcome::Complete(call) => {
            let mut value = serde_json::to_value(&call).map_err(|_| defect())?;
            supplement(&mut value, call.value())?;
            Ok(value)
        }
        RequestOutcome::Failed { completed, error } => {
            let mut value = complete_native::failure(&error);
            supplement(&mut value, &completed)?;
            complete_native::put(
                &mut value,
                "completed",
                serde_json::to_value(completed).map_err(|_| defect())?,
            )?;
            Ok(value)
        }
    }
}
pub(super) fn supplement(value: &mut Value, result: &RequestValue) -> Result<(), Error> {
    macro_rules! rows {
        ($rows:expr) => {
            complete_native::put(
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
        RequestValue::Found(v) => complete_native::put(
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

/// Decode SQL-owned readers only after admission, then use shared row recovery.
pub(super) fn run_recovering(
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


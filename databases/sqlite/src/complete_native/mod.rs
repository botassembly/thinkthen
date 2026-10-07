//! Shared SQL edge: named native complete calls and isolated final envelopes.
use serde_json::{Value, json};
use std::sync::Mutex;
use thinkthen::{CallOptions, Engine, Error, ErrorKind, RecordObservation, Surface};
mod execute;
mod inputs;
mod observations;
mod questions;
mod reader_error;
pub(crate) mod settings;
pub(crate) use inputs::Inputs;
pub(crate) use questions::Prepared;
pub(crate) use settings::prepare;

pub(crate) fn usage(message: &str) -> Error {
    Error::new(ErrorKind::Usage, message)
}
pub(crate) fn defect() -> Error {
    Error::new(
        ErrorKind::Defect,
        "a native SQL complete envelope could not be encoded",
    )
}
pub(crate) fn failure(error: &Error) -> Value {
    serde_json::to_value(error.complete()).unwrap_or_else(|_| json!(defect().complete()))
}

pub(crate) fn run(
    engine: &Engine,
    prepared: &Prepared,
    inputs: Inputs,
    options: CallOptions<'_>,
    surface: Surface,
) -> Value {
    let cancelled = thinkthen::CancelToken::new();
    if inputs.cancelled {
        cancelled.cancel();
    }
    let options = if inputs.cancelled {
        options.cancel(&cancelled)
    } else {
        options
    };
    let events = Mutex::new(Vec::new());
    let observer = |event: RecordObservation<'_>| {
        let value = observations::event(&event).unwrap_or_else(|e| failure(&e));
        events
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .push(value);
    };
    let options = options
        .surface(surface)
        .attempts(inputs.attempts)
        .observe(&observer);
    let result = execute::run(engine, prepared, inputs, options);
    let document = match result {
        Ok(value) => value,
        Err(error) => failure(&error),
    };
    let recorded = Value::Array(
        events
            .into_inner()
            .unwrap_or_else(std::sync::PoisonError::into_inner),
    );
    carrier(document, recorded)
}

pub(super) fn put(value: &mut Value, key: &str, field: Value) -> Result<(), Error> {
    value
        .as_object_mut()
        .ok_or_else(defect)?
        .insert(key.to_owned(), field);
    Ok(())
}

/// Keep the generated strict native envelope intact, with SQL supplements alongside.
pub(crate) fn carrier(mut native: Value, observations: Value) -> Value {
    let mut output = serde_json::Map::new();
    output.insert("observations".to_owned(), observations);
    for key in ["ordinals", "selection", "completed"] {
        if let Some(value) = native.as_object_mut().and_then(|fields| fields.remove(key)) {
            output.insert(key.to_owned(), value);
        }
    }
    output.insert("native".to_owned(), native);
    Value::Object(output)
}
pub(crate) fn admission(error: &Error) -> Value {
    carrier(failure(error), json!([]))
}

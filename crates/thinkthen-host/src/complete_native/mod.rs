//! Shared SQL edge: named native complete calls and isolated final envelopes.
use serde_json::{Value, json};
use thinkthen::{Error, ErrorKind};
pub mod file_format;
mod inputs;
pub mod observations;
mod questions;
mod reader_error;
pub mod settings;
pub use inputs::Inputs;
pub use questions::Prepared;
pub use settings::prepare;

pub fn usage(message: &str) -> Error {
    Error::new(ErrorKind::Usage, message)
}
pub fn defect() -> Error {
    Error::new(
        ErrorKind::Defect,
        "a native SQL complete envelope could not be encoded",
    )
}
pub fn failure(error: &Error) -> Value {
    serde_json::to_value(error.complete()).unwrap_or_else(|_| json!(defect().complete()))
}

pub fn put(value: &mut Value, key: &str, field: Value) -> Result<(), Error> {
    value
        .as_object_mut()
        .ok_or_else(defect)?
        .insert(key.to_owned(), field);
    Ok(())
}

/// Keep the generated strict native envelope intact, with SQL supplements alongside.
pub fn carrier(mut native: Value, observations: Value) -> Value {
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
pub fn admission(error: &Error) -> Value {
    carrier(failure(error), json!([]))
}

//! One completed call, or one receipt, as the JSON text the R half reads.

use extendr_api::prelude::*;
use serde::Serialize;

use super::Crossed;
use super::account::{Completed, Snapshot};
use super::receipt::State;
use crate::defect;

/// A call's value or its packed error, with its facts and question events.
#[derive(Serialize)]
struct Envelope<'a, T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<&'a str>,
    #[serde(flatten)]
    snapshot: &'a Snapshot,
}

/// The envelope under the one name `.tt_call` looks for.
pub(crate) fn envelope<T: Serialize>(completed: Completed<T>) -> Crossed<List> {
    let (value, error) = match &completed.result {
        Ok(value) => (Some(value), None),
        Err(error) => (None, Some(error.as_str())),
    };
    let text = serde_json::to_string(&Envelope {
        value,
        error,
        snapshot: &completed.snapshot,
    })
    .map_err(|_| defect("a result could not be written as JSON"))?;
    Ok(list!(tt_envelope = text))
}

#[derive(Serialize)]
struct Receipt<'a> {
    state: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    kind: Option<&'a str>,
    #[serde(flatten)]
    snapshot: Option<&'a Snapshot>,
}

pub(crate) fn receipt(state: &State) -> Crossed<String> {
    let (state, kind, snapshot) = match state {
        State::Unused => ("unused", None, None),
        State::Claimed => ("claimed", None, None),
        State::Running => ("running", None, None),
        State::Terminal { kind, snapshot } => ("terminal", Some(kind.as_str()), Some(snapshot)),
    };
    serde_json::to_string(&Receipt {
        state,
        kind,
        snapshot,
    })
    .map_err(|_| defect("a receipt could not be written as JSON"))
}

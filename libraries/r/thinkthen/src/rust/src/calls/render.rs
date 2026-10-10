//! One completed call, or one receipt, as the JSON text the R half reads.

use serde::Serialize;

use super::Crossed;
use super::account::Snapshot;
use super::receipt::State;
use crate::defect;

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
        State::Terminal { kind, snapshot } => {
            ("terminal", Some(kind.as_str()), Some(snapshot.as_ref()))
        }
    };
    serde_json::to_string(&Receipt {
        state,
        kind,
        snapshot,
    })
    .map_err(|_| defect("a receipt could not be written as JSON"))
}

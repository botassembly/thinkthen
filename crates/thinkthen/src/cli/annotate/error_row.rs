//! The one safe row for a skipped annotate pointer miss.

use serde::Serialize;

use crate::core::{Outcome, json_line};
use crate::failure::Failure;
use crate::schedule::Judged;

#[derive(Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[cfg_attr(test, schemars(rename = "recordError", deny_unknown_fields))]
pub(crate) struct Row<'a> {
    schema: &'static str,
    at: usize,
    failure: Miss<'a>,
}

#[derive(Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "recordFailure"))]
struct Miss<'a> {
    kind: &'static str,
    cause: &'static str,
    pointer: &'a str,
}

pub(crate) fn missed(at: usize, pointer: &str) -> Result<Judged, Failure> {
    let printed = json_line(&Row {
        schema: "thinkthen.record-error/1",
        at,
        failure: Miss {
            kind: "usage",
            cause: "missing_pointer",
            pointer,
        },
    })
    .map_err(Failure::Render)?;
    Ok(Judged {
        model: None,
        printed: Some(printed),
        outcome: Outcome::Yes,
        replayed: false,
        order_value: None,
        partial_failure: false,
        profile_mismatch: None,
    })
}

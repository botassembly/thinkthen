//! Serialize actual native outcomes, retaining completed prefixes and final facts.
use crate::calls::Crossed;
use serde::Serialize;
use thinkthen::{CompleteError, CompleteFacts, RequestOutcome, RequestValue};

#[derive(Serialize)]
struct Packet<'a> {
    results: &'a RequestValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    facts: Option<CompleteFacts<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    failure: Option<CompleteError<'a>>,
}

pub(crate) fn written(outcome: &RequestOutcome) -> Crossed<String> {
    let packet = match outcome {
        RequestOutcome::Complete(call) => Packet {
            results: call.value(),
            facts: call.facts().complete(),
            failure: None,
        },
        RequestOutcome::Failed { completed, error } => Packet {
            results: completed,
            facts: error.facts().and_then(thinkthen::Facts::complete),
            failure: Some(error.complete()),
        },
    };
    serde_json::to_string(&packet)
        .map_err(|_| crate::defect("native request outcome could not be written"))
}

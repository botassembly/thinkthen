//! Four dynamic-question record arrays through the shared stopping Rust bridge.

use thinkthen::{CallOptions, DetailQuestion, Engine, Facts};

use crate::failures::Failure;

use super::{Request, bare, member, object};

pub(super) fn judgments(
    engine: &Engine,
    request: &Request,
    options: CallOptions<'_>,
    detailed: bool,
) -> Result<(String, Facts), Failure> {
    if request.envelope.contains_key("evidence") {
        return Err(Failure::usage(
            "one request takes evidence or records, not both",
        ));
    }
    let records = member(request, "records", |raw| {
        serde_json::from_str::<Vec<String>>(raw)
    })?;
    match thinkthen::Question::from_json(&object(&request.question)?)? {
        thinkthen::LoadedQuestion::Question(asked) => {
            collect(engine, &asked, records, options, detailed)
        }
        thinkthen::LoadedQuestion::Banded(asked) => {
            collect(engine, &asked, records, options, detailed)
        }
    }
}

fn collect<Q: DetailQuestion + ?Sized>(
    engine: &Engine,
    question: &Q,
    records: Vec<String>,
    options: CallOptions<'_>,
    detailed: bool,
) -> Result<(String, Facts), Failure> {
    let mut batch = engine.details_many_with(question, records, options);
    let rows = batch.by_ref().collect::<Result<Vec<_>, _>>()?;
    let values = rows
        .iter()
        .map(|row| {
            if detailed {
                Ok(row.value().to_json())
            } else {
                bare(row.value().value())
            }
        })
        .collect::<Result<Vec<_>, Failure>>()?;
    let facts = batch
        .facts()
        .cloned()
        .ok_or_else(|| Failure::defect("completed record details have no facts"))?;
    Ok((format!("[{}]", values.join(",")), facts))
}

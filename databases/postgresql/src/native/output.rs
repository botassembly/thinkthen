//! Native projections retain call facts on successful and failed outcomes.
use crate::call::{self, OrRaise as _};
use pgrx::prelude::*;
use thinkthen::{Error, Facts, RequestOutcome, RequestValue};

extension_sql!(
    r#"
CREATE TYPE thinkthen_name AS (text text, start bigint, end bigint, kind text, strength double precision);
CREATE TYPE thinkthen_edge AS (relation text, source_name text, source_kind text, target_name text, target_kind text, probability double precision, either boolean);
"#,
    name = "native_result_types"
);

pub(super) type FactsTuple = pgrx::composite_type!('static, "thinkthen_facts");
pub(super) type FailureTuple = pgrx::composite_type!('static, "thinkthen_error");
pub(super) type Names = Vec<pgrx::composite_type!('static, "thinkthen_name")>;
pub(super) type Edges = Vec<pgrx::composite_type!('static, "thinkthen_edge")>;

pub(super) fn tuple(name: &str) -> PgHeapTuple<'static, AllocatedByRust> {
    PgHeapTuple::new_composite_type(name)
        .map_err(|_| call::defect("native result type is unavailable"))
        .or_raise()
}
fn set<T: pgrx::IntoDatum>(tuple: &mut PgHeapTuple<'_, AllocatedByRust>, name: &str, value: T) {
    tuple
        .set_by_name(name, value)
        .map_err(|_| call::defect("native result field is unavailable"))
        .or_raise();
}
pub(super) fn index(value: usize) -> i64 {
    i64::try_from(value).unwrap_or(i64::MAX)
}

pub(super) fn facts(facts: &Facts) -> FactsTuple {
    let value = serde_json::to_value(facts.complete())
        .map_err(|_| call::defect("native facts could not be projected"))
        .or_raise();
    super::results_generated::convert("thinkthen_facts", &value).or_raise()
}
pub(super) fn failure(error: &Error) -> FailureTuple {
    let value = serde_json::to_value(error.complete().error())
        .map_err(|_| call::defect("native failure could not be projected"))
        .or_raise();
    super::results_generated::convert("thinkthen_error", &value).or_raise()
}
pub(super) fn reading(
    kind: &str,
    value: &serde_json::Value,
) -> PgHeapTuple<'static, AllocatedByRust> {
    super::results_generated::convert(kind, value).or_raise()
}

#[derive(Debug)]
pub(super) struct Projection {
    pub(super) value: Option<RequestValue>,
    pub(super) facts: Option<Facts>,
    pub(super) failure: Option<Error>,
    pub(super) observations: Vec<serde_json::Value>,
}

pub(super) fn outcome(result: Result<super::Execution, Error>) -> Projection {
    let (result, observations) = match result {
        Ok(execution) => (Ok(execution.outcome), execution.observations),
        Err(error) => (Err(error), Vec::new()),
    };
    let (value, facts, error) = match result {
        Ok(RequestOutcome::Complete(call)) => {
            let counts = call.facts().clone();
            (Some(call.into_value()), Some(counts), None)
        }
        Ok(RequestOutcome::Failed { completed, error }) => {
            (Some(completed), error.facts().cloned(), Some(error))
        }
        Err(error) => (None, error.facts().cloned(), Some(error)),
    };
    Projection {
        value,
        facts,
        failure: error,
        observations,
    }
}

pub(super) fn original(value: &thinkthen::QuestionInput) -> pgrx::datum::JsonB {
    match value {
        thinkthen::QuestionInput::Text(text) => {
            pgrx::datum::JsonB(serde_json::Value::String(text.clone()))
        }
        thinkthen::QuestionInput::Record(record) => pgrx::datum::JsonB(
            serde_json::to_value(record.original())
                .map_err(|_| call::defect("native original is invalid"))
                .or_raise(),
        ),
        thinkthen::QuestionInput::Images(_) => {
            call::raise(call::usage("image-only originals have no JSON value"))
        }
    }
}
pub(super) fn names(value: &thinkthen::Recognized) -> Names {
    value
        .entities()
        .iter()
        .map(|name| {
            let mut value = tuple("thinkthen_name");
            set(&mut value, "text", name.text());
            set(&mut value, "start", index(name.start()));
            set(&mut value, "end", index(name.end()));
            set(&mut value, "kind", name.kind());
            set(&mut value, "strength", name.strength());
            value
        })
        .collect()
}
pub(super) fn edges(edges: &[thinkthen::Edge]) -> Edges {
    edges
        .iter()
        .map(|edge| {
            let mut value = tuple("thinkthen_edge");
            set(&mut value, "relation", edge.relation());
            set(&mut value, "source_name", edge.source().name());
            set(&mut value, "source_kind", edge.source().kind());
            set(&mut value, "target_name", edge.target().name());
            set(&mut value, "target_kind", edge.target().kind());
            set(&mut value, "probability", edge.probability());
            set(&mut value, "either", edge.either());
            value
        })
        .collect()
}

pub(super) fn events(
    values: &[serde_json::Value],
) -> Vec<pgrx::composite_type!('static, "thinkthen_event")> {
    values
        .iter()
        .map(|value| {
            let value = value.get("value").unwrap_or(value);
            super::results_generated::convert("thinkthen_event", value).or_raise()
        })
        .collect()
}

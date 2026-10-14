//! Same-name ordered record overloads return typed answers and final facts.
use super::output;
use crate::call::{self, OrRaise as _};
use pgrx::datum::JsonB;
use pgrx::prelude::*;
use thinkthen::RequestValue;

crate::descriptions::describe! {
"Find an original from ordered native records; return its index, value, probability and final facts. Required NULL operands return no rows. Evidence files must be read by the client.";
[name = "thinkthen_find", parallel_restricted, requires = ["native_record_types", "native_reading_type", "native_result_types", "native_generated_types"]];
#[allow(clippy::type_complexity, reason = "pgrx reads SQL columns from the signature")]
fn find_records(
    question: Option<&str>,
    records: Option<Array<'_, pgrx::composite_type!("thinkthen_record")>>,
    reading: default!(Option<pgrx::composite_type!("thinkthen_reading")>, "NULL"),
) -> TableIterator<
    'static,
    (
        name!(index, Option<i64>),
        name!(value, Option<JsonB>),
        name!(probability, Option<f64>),
    name!(result, Option<pgrx::composite_type!('static, "thinkthen_found")>),
        name!(
            facts,
            Option<pgrx::composite_type!('static, "thinkthen_facts")>
        ),
        name!(
            failure,
            Option<pgrx::composite_type!('static, "thinkthen_error")>
        ),
        name!(observations, Vec<pgrx::composite_type!('static, "thinkthen_event")>),
    ),
> {
    call::guarded(|| {
        let (Some(question), Some(records)) = (question, records) else {
            return TableIterator::new(Vec::new());
        };
        let output::Projection { value, facts, failure, observations } =
            output::outcome(super::execute("find", question, records, reading));
        let (index, value, probability, reading) = match value {
            Some(RequestValue::Found(found)) => (
                match found.selection() {
                    thinkthen::FindSelection::Unit(at) => Some(output::index(at)),
                    thinkthen::FindSelection::None => None,
                },
                found.selected().map(output::original),
                found.confidence(),
                Some(output::reading("thinkthen_found", &serde_json::to_value(&found).map_err(|_| call::defect("native result could not be projected")).or_raise())),
            ),
            None => (None, None, None, None),
            _ => call::raise(crate::request::wrong_result()),
        };
        TableIterator::once((
            index,
            value,
            probability,
            reading,
            facts.as_ref().map(output::facts),
            failure.as_ref().map(output::failure),
            output::events(&observations),
        ))
    })
}
}

crate::descriptions::describe! {
"Relate ordered native entity records; return native named edges and final facts. Required NULL operands return no rows. Evidence files must be read by the client.";
[name = "thinkthen_relate", parallel_restricted, requires = ["native_record_types", "native_reading_type", "native_result_types", "native_generated_types"]];
#[allow(clippy::type_complexity, reason = "pgrx reads SQL columns from the signature")]
fn relate_records(
    question: Option<&str>,
    records: Option<Array<'_, pgrx::composite_type!("thinkthen_record")>>,
    reading: default!(Option<pgrx::composite_type!("thinkthen_reading")>, "NULL"),
) -> TableIterator<
    'static,
    (
        name!(
            value,
            Option<Vec<pgrx::composite_type!('static, "thinkthen_edge")>>
        ),
        name!(result, Option<pgrx::composite_type!('static, "thinkthen_related")>),
        name!(
            facts,
            Option<pgrx::composite_type!('static, "thinkthen_facts")>
        ),
        name!(
            failure,
            Option<pgrx::composite_type!('static, "thinkthen_error")>
        ),
        name!(observations, Vec<pgrx::composite_type!('static, "thinkthen_event")>),
    ),
> {
    call::guarded(|| {
        let (Some(question), Some(records)) = (question, records) else {
            return TableIterator::new(Vec::new());
        };
        let output::Projection { value, facts, failure, observations } =
            output::outcome(super::execute("relate", question, records, reading));
        let (value, reading) = match value {
            Some(RequestValue::Related(related)) => (Some(output::edges(related.result().value())),
                Some(output::reading("thinkthen_related", &serde_json::to_value(&related).map_err(|_| call::defect("native result could not be projected")).or_raise()))),
            None => (None, None),
            _ => call::raise(crate::request::wrong_result()),
        };
        TableIterator::once((
            value,
            reading,
            facts.as_ref().map(output::facts),
            failure.as_ref().map(output::failure),
            output::events(&observations),
        ))
    })
}
}

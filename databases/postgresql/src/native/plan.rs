//! Canonical native previews expose SQL counts without sending.
use crate::call::{self, OrRaise as _};
use pgrx::prelude::*;
use thinkthen::{Request, RequestEnvironment, RequestInput};

crate::descriptions::describe! {
"Plan ordered native SQL records using the same admission and packing as execution; return counts and token bounds without sending. Required NULL operands return no rows. Evidence files must be read by the client.";
[name = "thinkthen_plan", parallel_restricted, requires = ["native_record_types", "native_reading_type"]];
#[allow(clippy::type_complexity, reason = "pgrx reads SQL columns from the signature")]
fn plan_records(
    question: Option<&str>,
    records: Option<Array<'_, pgrx::composite_type!("thinkthen_record")>>,
    reading: default!(Option<pgrx::composite_type!("thinkthen_reading")>, "NULL"),
) -> TableIterator<'static, (
    name!(record_count, i64), name!(requests, i64), name!(estimated_bytes, i64),
    name!(input_tokens_lower, i64), name!(input_tokens_upper, i64),
    name!(upper_bound, bool),
)> {
    call::guarded(|| {
        let (Some(question), Some(records)) = (question, records) else {
            return TableIterator::new(Vec::new());
        };
        let settings = thinkthen::Settings::default();
        let verb = crate::forms::plan_verb_result(question, &settings).or_raise();
        let question = crate::forms::question_result(Some(question), None, verb, &settings).or_raise();
        let request = Request::for_definition(question.into(),
            RequestInput::Records { items: super::input::records(records).or_raise() },
            super::options::read(reading).or_raise(),
        ).admit().or_raise();
        let plan = call::run(call::read(), move |engine, controls| {
            engine.plan_request(&request, RequestEnvironment {
                controls: controls.surface(thinkthen::Surface::Postgresql), feed: None,
            })
        });
        let (lower, upper) = plan.estimated_input_tokens();
        TableIterator::once((super::output::index(plan.records()),
            super::output::index(plan.requests()), super::output::index(plan.estimated_bytes()),
            super::output::index(lower), super::output::index(upper), plan.upper_bound()))
    })
}
}

//! Canonical native previews expose SQL counts without sending.
use crate::call::{self, OrRaise as _};
use pgrx::prelude::*;
use thinkthen::RequestEnvironment;

crate::descriptions::describe! {
"Plan ordered native SQL records using the same admission and packing as execution; return complete typed counts, bytes, token bounds and the first body without sending. An optional named function selects a canonical request function; native unsupported functions refuse without sending. Required NULL operands return no rows. Evidence files must be read by the client.";
[name = "thinkthen_plan", parallel_restricted, requires = ["native_record_types", "native_reading_type", "native_generated_types"]];
#[allow(clippy::type_complexity, reason = "pgrx reads SQL columns from the signature")]
fn plan_records(
    question: Option<&str>,
    records: Option<Array<'_, pgrx::composite_type!("thinkthen_record")>>,
    reading: default!(Option<pgrx::composite_type!("thinkthen_reading")>, "NULL"),
    function: default!(Option<&str>, "NULL"),
) -> TableIterator<'static, (
    name!(record_count, i64), name!(requests, i64), name!(estimated_bytes, i64),
    name!(input_tokens_lower, i64), name!(input_tokens_upper, i64),
    name!(upper_bound, bool), name!(result, pgrx::composite_type!('static, "thinkthen_plan")),
)> {
    call::guarded(|| {
        let (Some(question), Some(records)) = (question, records) else {
            return TableIterator::new(Vec::new());
        };
        let settings = super::question_settings::read(reading.as_ref()).or_raise();
        let verb = if let Some(function) = function { function } else { match crate::forms::plan_verb_result(question, &settings).or_raise() {
            thinkthen::For::Decide => "decide", thinkthen::For::Choose => "choose",
            thinkthen::For::Score => "score", thinkthen::For::Tag => "tag",
            thinkthen::For::Find => "find", thinkthen::For::Rank => "rank",
        }};
        let context_schema = super::context_schema::read(reading.as_ref()).or_raise();
        let request = super::request(verb, question, super::options::read(reading).or_raise(), &settings).or_raise();
        let mut projection = request.record_reading().or_raise();
        if let Some(schema) = context_schema { projection = projection.with_context_schema(schema); }
        let request = super::composed_request(&request).or_raise();
        let records = super::input::deferred(records).or_raise();
        let images = records.iter().any(super::input::Record::has_images);
        let plan = call::run(call::read(), move |engine, controls| {
            let records = records.into_iter().map(|record| super::input::compose(record, &projection));
            let mut feed = thinkthen::RequestFeed::from_records("postgresql", records).eager();
            if images { feed = feed.with_image_inputs(); }
            engine.plan_request(&request, RequestEnvironment {
                controls: controls.surface(thinkthen::Surface::Postgresql), feed: Some(feed),
            })
        });
        let result = serde_json::to_value(&plan).map_err(|_| call::defect("native plan could not be projected")).or_raise();
        let result = super::results_generated::convert("thinkthen_plan", &result).or_raise();
        let (lower, upper) = plan.estimated_input_tokens();
        TableIterator::once((super::output::index(plan.records()),
            super::output::index(plan.requests()), super::output::index(plan.estimated_bytes()),
            super::output::index(lower), super::output::index(upper), plan.upper_bound(), result))
    })
}
}

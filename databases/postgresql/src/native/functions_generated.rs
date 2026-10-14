// Generated native PostgreSQL declarations; do not edit.
use super::output;
use crate::call;
use pgrx::datum::JsonB;
use pgrx::prelude::*;
use serde_json::Value;
crate::descriptions::describe! {
"Return full typed records with values, probabilities, failures, facts and observations. Recoverable failures retain their rows; cancellation, deadline and defects raise. Required NULL operands return no rows. Evidence files must be read by the client.";
[name = "thinkthen_decide_details", parallel_restricted, requires = ["native_record_types", "native_reading_type", "native_result_types", "native_generated_types"]];
#[allow(clippy::type_complexity, clippy::too_many_arguments, reason = "pgrx reads named SQL arguments and columns from the signature")]
fn decide_records(
    question: Option<&str>, records: Option<Array<'_, pgrx::composite_type!("thinkthen_record")>>,
                    reading: default!(Option<pgrx::composite_type!("thinkthen_reading")>, "NULL"),
) -> TableIterator<'static, (name!(ordinal, Option<i64>), name!(value, Option<bool>),
            name!(probability, Option<f64>),
            name!(result, Option<pgrx::composite_type!('static, "thinkthen_decision")>),
            name!(failure, Option<pgrx::composite_type!('static, "thinkthen_error")>),
            name!(facts, Option<pgrx::composite_type!('static, "thinkthen_facts")>),
            name!(observations, crate::ffi::EventArray))> {
    call::guarded(|| {
        let (Some(question), Some(records)) = (question, records) else {
            return TableIterator::new(Vec::new());
        };
        let rows = super::row_values(super::execute("decide", question, records, reading));
        TableIterator::new(rows.into_iter().map(|row| {
            let value = row.get("value");
            (row.get("ordinal").and_then(Value::as_i64), value.and_then(Value::as_bool), row.get("probability").and_then(Value::as_f64),
                output::optional_reading("thinkthen_decision", row.get("result")),
                output::optional_reading("thinkthen_error", row.get("failure")),
                output::optional_reading("thinkthen_facts", row.get("facts")),
                output::events(row.get("observations").and_then(Value::as_array).map_or(&[], Vec::as_slice)))
        }).collect::<Vec<_>>())
    })
}
}

crate::descriptions::describe! {
"Return full typed records with values, probabilities, failures, facts and observations. Recoverable failures retain their rows; cancellation, deadline and defects raise. Required NULL operands return no rows. Evidence files must be read by the client.";
[name = "thinkthen_decide_details", parallel_restricted, requires = ["native_record_types", "native_reading_type", "native_result_types", "native_generated_types"]];
#[allow(clippy::type_complexity, clippy::too_many_arguments, reason = "pgrx reads named SQL arguments and columns from the signature")]
fn decide_plain(
    question: Option<&str>, input: Option<&str>,
                    settings: default!(Option<crate::ffi::RawJson>, "NULL"),
                    threshold: default!(Option<&str>, "NULL"),
                    context: default!(Option<&str>, "NULL"),
                    model: default!(Option<&str>, "NULL"),
                    batch: default!(Option<&str>, "NULL"),
                    deadline_ms: default!(Option<i64>, "NULL"),
) -> TableIterator<'static, (name!(ordinal, Option<i64>), name!(value, Option<bool>),
            name!(probability, Option<f64>),
            name!(result, Option<pgrx::composite_type!('static, "thinkthen_decision")>),
            name!(failure, Option<pgrx::composite_type!('static, "thinkthen_error")>),
            name!(facts, Option<pgrx::composite_type!('static, "thinkthen_facts")>),
            name!(observations, crate::ffi::EventArray))> {
    call::guarded(|| {
        let (Some(question), Some(input)) = (question, input) else {
            return TableIterator::new(Vec::new());
        };
        let rows = super::row_values(super::execute_plain("decide", question, input, None, settings, crate::forms::Named::new(threshold, context, model, batch, deadline_ms)));
        TableIterator::new(rows.into_iter().map(|row| {
            let value = row.get("value");
            (row.get("ordinal").and_then(Value::as_i64), value.and_then(Value::as_bool), row.get("probability").and_then(Value::as_f64),
                output::optional_reading("thinkthen_decision", row.get("result")),
                output::optional_reading("thinkthen_error", row.get("failure")),
                output::optional_reading("thinkthen_facts", row.get("facts")),
                output::events(row.get("observations").and_then(Value::as_array).map_or(&[], Vec::as_slice)))
        }).collect::<Vec<_>>())
    })
}
}

crate::descriptions::describe! {
"Return full typed records with values, probabilities, failures, facts and observations. Recoverable failures retain their rows; cancellation, deadline and defects raise. Required NULL operands return no rows. Evidence files must be read by the client.";
[name = "thinkthen_choose_details", parallel_restricted, requires = ["native_record_types", "native_reading_type", "native_result_types", "native_generated_types"]];
#[allow(clippy::type_complexity, clippy::too_many_arguments, reason = "pgrx reads named SQL arguments and columns from the signature")]
fn choose_records(
    question: Option<&str>, records: Option<Array<'_, pgrx::composite_type!("thinkthen_record")>>,
                    reading: default!(Option<pgrx::composite_type!("thinkthen_reading")>, "NULL"),
) -> TableIterator<'static, (name!(ordinal, Option<i64>), name!(value, Option<String>),
            name!(probability, Option<f64>),
            name!(result, Option<pgrx::composite_type!('static, "thinkthen_choice")>),
            name!(failure, Option<pgrx::composite_type!('static, "thinkthen_error")>),
            name!(facts, Option<pgrx::composite_type!('static, "thinkthen_facts")>),
            name!(observations, crate::ffi::EventArray))> {
    call::guarded(|| {
        let (Some(question), Some(records)) = (question, records) else {
            return TableIterator::new(Vec::new());
        };
        let rows = super::row_values(super::execute("choose", question, records, reading));
        TableIterator::new(rows.into_iter().map(|row| {
            let value = row.get("value");
            (row.get("ordinal").and_then(Value::as_i64), value.and_then(Value::as_str).map(str::to_owned), row.get("probability").and_then(Value::as_f64),
                output::optional_reading("thinkthen_choice", row.get("result")),
                output::optional_reading("thinkthen_error", row.get("failure")),
                output::optional_reading("thinkthen_facts", row.get("facts")),
                output::events(row.get("observations").and_then(Value::as_array).map_or(&[], Vec::as_slice)))
        }).collect::<Vec<_>>())
    })
}
}

crate::descriptions::describe! {
"Return full typed records with values, probabilities, failures, facts and observations. Recoverable failures retain their rows; cancellation, deadline and defects raise. Required NULL operands return no rows. Evidence files must be read by the client.";
[name = "thinkthen_choose_details", parallel_restricted, requires = ["native_record_types", "native_reading_type", "native_result_types", "native_generated_types"]];
#[allow(clippy::type_complexity, clippy::too_many_arguments, reason = "pgrx reads named SQL arguments and columns from the signature")]
fn choose_plain(
    question: Option<&str>, input: Option<&str>, members: default!(Option<Array<'_, &str>>, "NULL"),
                    settings: default!(Option<crate::ffi::RawJson>, "NULL"),
                    threshold: default!(Option<&str>, "NULL"),
                    context: default!(Option<&str>, "NULL"),
                    model: default!(Option<&str>, "NULL"),
                    batch: default!(Option<&str>, "NULL"),
                    deadline_ms: default!(Option<i64>, "NULL"),
) -> TableIterator<'static, (name!(ordinal, Option<i64>), name!(value, Option<String>),
            name!(probability, Option<f64>),
            name!(result, Option<pgrx::composite_type!('static, "thinkthen_choice")>),
            name!(failure, Option<pgrx::composite_type!('static, "thinkthen_error")>),
            name!(facts, Option<pgrx::composite_type!('static, "thinkthen_facts")>),
            name!(observations, crate::ffi::EventArray))> {
    call::guarded(|| {
        let (Some(question), Some(input)) = (question, input) else {
            return TableIterator::new(Vec::new());
        };
        let rows = super::row_values(super::execute_plain("choose", question, input, members, settings, crate::forms::Named::new(threshold, context, model, batch, deadline_ms)));
        TableIterator::new(rows.into_iter().map(|row| {
            let value = row.get("value");
            (row.get("ordinal").and_then(Value::as_i64), value.and_then(Value::as_str).map(str::to_owned), row.get("probability").and_then(Value::as_f64),
                output::optional_reading("thinkthen_choice", row.get("result")),
                output::optional_reading("thinkthen_error", row.get("failure")),
                output::optional_reading("thinkthen_facts", row.get("facts")),
                output::events(row.get("observations").and_then(Value::as_array).map_or(&[], Vec::as_slice)))
        }).collect::<Vec<_>>())
    })
}
}

crate::descriptions::describe! {
"Return full typed records with values, probabilities, failures, facts and observations. Recoverable failures retain their rows; cancellation, deadline and defects raise. Required NULL operands return no rows. Evidence files must be read by the client.";
[name = "thinkthen_tag_details", parallel_restricted, requires = ["native_record_types", "native_reading_type", "native_result_types", "native_generated_types"]];
#[allow(clippy::type_complexity, clippy::too_many_arguments, reason = "pgrx reads named SQL arguments and columns from the signature")]
fn tag_records(
    question: Option<&str>, records: Option<Array<'_, pgrx::composite_type!("thinkthen_record")>>,
                    reading: default!(Option<pgrx::composite_type!("thinkthen_reading")>, "NULL"),
) -> TableIterator<'static, (name!(ordinal, Option<i64>), name!(value, Option<Vec<String>>),
            name!(probability, Option<f64>),
            name!(result, Option<pgrx::composite_type!('static, "thinkthen_tags")>),
            name!(failure, Option<pgrx::composite_type!('static, "thinkthen_error")>),
            name!(facts, Option<pgrx::composite_type!('static, "thinkthen_facts")>),
            name!(observations, crate::ffi::EventArray))> {
    call::guarded(|| {
        let (Some(question), Some(records)) = (question, records) else {
            return TableIterator::new(Vec::new());
        };
        let rows = super::row_values(super::execute("tag", question, records, reading));
        TableIterator::new(rows.into_iter().map(|row| {
            let value = row.get("value");
            (row.get("ordinal").and_then(Value::as_i64), output::strings(value), row.get("probability").and_then(Value::as_f64),
                output::optional_reading("thinkthen_tags", row.get("result")),
                output::optional_reading("thinkthen_error", row.get("failure")),
                output::optional_reading("thinkthen_facts", row.get("facts")),
                output::events(row.get("observations").and_then(Value::as_array).map_or(&[], Vec::as_slice)))
        }).collect::<Vec<_>>())
    })
}
}

crate::descriptions::describe! {
"Return full typed records with values, probabilities, failures, facts and observations. Recoverable failures retain their rows; cancellation, deadline and defects raise. Required NULL operands return no rows. Evidence files must be read by the client.";
[name = "thinkthen_tag_details", parallel_restricted, requires = ["native_record_types", "native_reading_type", "native_result_types", "native_generated_types"]];
#[allow(clippy::type_complexity, clippy::too_many_arguments, reason = "pgrx reads named SQL arguments and columns from the signature")]
fn tag_plain(
    question: Option<&str>, input: Option<&str>, members: default!(Option<Array<'_, &str>>, "NULL"),
                    settings: default!(Option<crate::ffi::RawJson>, "NULL"),
                    threshold: default!(Option<&str>, "NULL"),
                    context: default!(Option<&str>, "NULL"),
                    model: default!(Option<&str>, "NULL"),
                    batch: default!(Option<&str>, "NULL"),
                    deadline_ms: default!(Option<i64>, "NULL"),
) -> TableIterator<'static, (name!(ordinal, Option<i64>), name!(value, Option<Vec<String>>),
            name!(probability, Option<f64>),
            name!(result, Option<pgrx::composite_type!('static, "thinkthen_tags")>),
            name!(failure, Option<pgrx::composite_type!('static, "thinkthen_error")>),
            name!(facts, Option<pgrx::composite_type!('static, "thinkthen_facts")>),
            name!(observations, crate::ffi::EventArray))> {
    call::guarded(|| {
        let (Some(question), Some(input)) = (question, input) else {
            return TableIterator::new(Vec::new());
        };
        let rows = super::row_values(super::execute_plain("tag", question, input, members, settings, crate::forms::Named::new(threshold, context, model, batch, deadline_ms)));
        TableIterator::new(rows.into_iter().map(|row| {
            let value = row.get("value");
            (row.get("ordinal").and_then(Value::as_i64), output::strings(value), row.get("probability").and_then(Value::as_f64),
                output::optional_reading("thinkthen_tags", row.get("result")),
                output::optional_reading("thinkthen_error", row.get("failure")),
                output::optional_reading("thinkthen_facts", row.get("facts")),
                output::events(row.get("observations").and_then(Value::as_array).map_or(&[], Vec::as_slice)))
        }).collect::<Vec<_>>())
    })
}
}

crate::descriptions::describe! {
"Return full typed records with values, probabilities, failures, facts and observations. Recoverable failures retain their rows; cancellation, deadline and defects raise. Required NULL operands return no rows. Evidence files must be read by the client.";
[name = "thinkthen_score_details", parallel_restricted, requires = ["native_record_types", "native_reading_type", "native_result_types", "native_generated_types"]];
#[allow(clippy::type_complexity, clippy::too_many_arguments, reason = "pgrx reads named SQL arguments and columns from the signature")]
fn score_records(
    question: Option<&str>, records: Option<Array<'_, pgrx::composite_type!("thinkthen_record")>>,
                    reading: default!(Option<pgrx::composite_type!("thinkthen_reading")>, "NULL"),
) -> TableIterator<'static, (name!(ordinal, Option<i64>), name!(value, Option<f64>),
            name!(probability, Option<f64>),
            name!(result, Option<pgrx::composite_type!('static, "thinkthen_score")>),
            name!(failure, Option<pgrx::composite_type!('static, "thinkthen_error")>),
            name!(facts, Option<pgrx::composite_type!('static, "thinkthen_facts")>),
            name!(observations, crate::ffi::EventArray))> {
    call::guarded(|| {
        let (Some(question), Some(records)) = (question, records) else {
            return TableIterator::new(Vec::new());
        };
        let rows = super::row_values(super::execute("score", question, records, reading));
        TableIterator::new(rows.into_iter().map(|row| {
            let value = row.get("value");
            (row.get("ordinal").and_then(Value::as_i64), value.and_then(Value::as_f64), row.get("probability").and_then(Value::as_f64),
                output::optional_reading("thinkthen_score", row.get("result")),
                output::optional_reading("thinkthen_error", row.get("failure")),
                output::optional_reading("thinkthen_facts", row.get("facts")),
                output::events(row.get("observations").and_then(Value::as_array).map_or(&[], Vec::as_slice)))
        }).collect::<Vec<_>>())
    })
}
}

crate::descriptions::describe! {
"Return full typed records with values, probabilities, failures, facts and observations. Recoverable failures retain their rows; cancellation, deadline and defects raise. Required NULL operands return no rows. Evidence files must be read by the client.";
[name = "thinkthen_score_details", parallel_restricted, requires = ["native_record_types", "native_reading_type", "native_result_types", "native_generated_types"]];
#[allow(clippy::type_complexity, clippy::too_many_arguments, reason = "pgrx reads named SQL arguments and columns from the signature")]
fn score_plain(
    question: Option<&str>, input: Option<&str>, members: default!(Option<Array<'_, &str>>, "NULL"),
                    settings: default!(Option<crate::ffi::RawJson>, "NULL"),
                    threshold: default!(Option<&str>, "NULL"),
                    context: default!(Option<&str>, "NULL"),
                    model: default!(Option<&str>, "NULL"),
                    batch: default!(Option<&str>, "NULL"),
                    deadline_ms: default!(Option<i64>, "NULL"),
) -> TableIterator<'static, (name!(ordinal, Option<i64>), name!(value, Option<f64>),
            name!(probability, Option<f64>),
            name!(result, Option<pgrx::composite_type!('static, "thinkthen_score")>),
            name!(failure, Option<pgrx::composite_type!('static, "thinkthen_error")>),
            name!(facts, Option<pgrx::composite_type!('static, "thinkthen_facts")>),
            name!(observations, crate::ffi::EventArray))> {
    call::guarded(|| {
        let (Some(question), Some(input)) = (question, input) else {
            return TableIterator::new(Vec::new());
        };
        let rows = super::row_values(super::execute_plain("score", question, input, members, settings, crate::forms::Named::new(threshold, context, model, batch, deadline_ms)));
        TableIterator::new(rows.into_iter().map(|row| {
            let value = row.get("value");
            (row.get("ordinal").and_then(Value::as_i64), value.and_then(Value::as_f64), row.get("probability").and_then(Value::as_f64),
                output::optional_reading("thinkthen_score", row.get("result")),
                output::optional_reading("thinkthen_error", row.get("failure")),
                output::optional_reading("thinkthen_facts", row.get("facts")),
                output::events(row.get("observations").and_then(Value::as_array).map_or(&[], Vec::as_slice)))
        }).collect::<Vec<_>>())
    })
}
}

crate::descriptions::describe! {
"Return full typed records with values, probabilities, failures, facts and observations. Recoverable failures retain their rows; cancellation, deadline and defects raise. Required NULL operands return no rows. Evidence files must be read by the client.";
[name = "thinkthen_filter_details", parallel_restricted, requires = ["native_record_types", "native_reading_type", "native_result_types", "native_generated_types"]];
#[allow(clippy::type_complexity, clippy::too_many_arguments, reason = "pgrx reads named SQL arguments and columns from the signature")]
fn filter_records(
    question: Option<&str>, records: Option<Array<'_, pgrx::composite_type!("thinkthen_record")>>,
                    reading: default!(Option<pgrx::composite_type!("thinkthen_reading")>, "NULL"),
) -> TableIterator<'static, (name!(ordinal, Option<i64>), name!(value, Option<JsonB>),
            name!(probability, Option<f64>),
            name!(result, Option<pgrx::composite_type!('static, "thinkthen_filtered")>),
            name!(failure, Option<pgrx::composite_type!('static, "thinkthen_error")>),
            name!(facts, Option<pgrx::composite_type!('static, "thinkthen_facts")>),
            name!(observations, crate::ffi::EventArray))> {
    call::guarded(|| {
        let (Some(question), Some(records)) = (question, records) else {
            return TableIterator::new(Vec::new());
        };
        let rows = super::row_values(super::execute("filter", question, records, reading));
        TableIterator::new(rows.into_iter().map(|row| {
            let value = row.get("value");
            (row.get("ordinal").and_then(Value::as_i64), value.filter(|v| !v.is_null()).cloned().map(JsonB), row.get("probability").and_then(Value::as_f64),
                output::optional_reading("thinkthen_filtered", row.get("result")),
                output::optional_reading("thinkthen_error", row.get("failure")),
                output::optional_reading("thinkthen_facts", row.get("facts")),
                output::events(row.get("observations").and_then(Value::as_array).map_or(&[], Vec::as_slice)))
        }).collect::<Vec<_>>())
    })
}
}

crate::descriptions::describe! {
"Return full typed records with values, probabilities, failures, facts and observations. Recoverable failures retain their rows; cancellation, deadline and defects raise. Required NULL operands return no rows. Evidence files must be read by the client.";
[name = "thinkthen_filter_details", parallel_restricted, requires = ["native_record_types", "native_reading_type", "native_result_types", "native_generated_types"]];
#[allow(clippy::type_complexity, clippy::too_many_arguments, reason = "pgrx reads named SQL arguments and columns from the signature")]
fn filter_plain(
    question: Option<&str>, input: Option<JsonB>, settings: default!(Option<crate::ffi::RawJson>, "NULL"),
) -> TableIterator<'static, (name!(ordinal, Option<i64>), name!(value, Option<JsonB>),
            name!(probability, Option<f64>),
            name!(result, Option<pgrx::composite_type!('static, "thinkthen_filtered")>),
            name!(failure, Option<pgrx::composite_type!('static, "thinkthen_error")>),
            name!(facts, Option<pgrx::composite_type!('static, "thinkthen_facts")>),
            name!(observations, crate::ffi::EventArray))> {
    call::guarded(|| {
        let (Some(question), Some(input)) = (question, input) else {
            return TableIterator::new(Vec::new());
        };
        let rows = super::row_values(super::plain::keyed("filter", question, input, settings));
        TableIterator::new(rows.into_iter().map(|row| {
            let value = row.get("value");
            (row.get("ordinal").and_then(Value::as_i64), value.filter(|v| !v.is_null()).cloned().map(JsonB), row.get("probability").and_then(Value::as_f64),
                output::optional_reading("thinkthen_filtered", row.get("result")),
                output::optional_reading("thinkthen_error", row.get("failure")),
                output::optional_reading("thinkthen_facts", row.get("facts")),
                output::events(row.get("observations").and_then(Value::as_array).map_or(&[], Vec::as_slice)))
        }).collect::<Vec<_>>())
    })
}
}

crate::descriptions::describe! {
"Return full typed records with values, probabilities, failures, facts and observations. Recoverable failures retain their rows; cancellation, deadline and defects raise. Required NULL operands return no rows. Evidence files must be read by the client.";
[name = "thinkthen_rank_details", parallel_restricted, requires = ["native_record_types", "native_reading_type", "native_result_types", "native_generated_types"]];
#[allow(clippy::type_complexity, clippy::too_many_arguments, reason = "pgrx reads named SQL arguments and columns from the signature")]
fn rank_records(
    question: Option<&str>, records: Option<Array<'_, pgrx::composite_type!("thinkthen_record")>>,
                    reading: default!(Option<pgrx::composite_type!("thinkthen_reading")>, "NULL"),
) -> TableIterator<'static, (name!(ordinal, Option<i64>), name!(value, Option<Vec<i64>>),
            name!(probability, Option<Vec<f64>>),
            name!(result, Option<pgrx::composite_type!('static, "thinkthen_ranked")>),
            name!(failure, Option<pgrx::composite_type!('static, "thinkthen_error")>),
            name!(facts, Option<pgrx::composite_type!('static, "thinkthen_facts")>),
            name!(observations, crate::ffi::EventArray))> {
    call::guarded(|| {
        let (Some(question), Some(records)) = (question, records) else {
            return TableIterator::new(Vec::new());
        };
        let rows = super::row_values(super::execute("rank", question, records, reading));
        TableIterator::new(rows.into_iter().map(|row| {
            let value = row.get("value");
            (row.get("ordinal").and_then(Value::as_i64), output::indexes(value), output::numbers(row.get("probability")),
                output::optional_reading("thinkthen_ranked", row.get("result")),
                output::optional_reading("thinkthen_error", row.get("failure")),
                output::optional_reading("thinkthen_facts", row.get("facts")),
                output::events(row.get("observations").and_then(Value::as_array).map_or(&[], Vec::as_slice)))
        }).collect::<Vec<_>>())
    })
}
}

crate::descriptions::describe! {
"Return full typed records with values, probabilities, failures, facts and observations. Recoverable failures retain their rows; cancellation, deadline and defects raise. Required NULL operands return no rows. Evidence files must be read by the client.";
[name = "thinkthen_rank_details", parallel_restricted, requires = ["native_record_types", "native_reading_type", "native_result_types", "native_generated_types"]];
#[allow(clippy::type_complexity, clippy::too_many_arguments, reason = "pgrx reads named SQL arguments and columns from the signature")]
fn rank_plain(
    question: Option<&str>, input: Option<JsonB>, settings: default!(Option<crate::ffi::RawJson>, "NULL"),
) -> TableIterator<'static, (name!(ordinal, Option<i64>), name!(value, Option<Vec<i64>>),
            name!(probability, Option<Vec<f64>>),
            name!(result, Option<pgrx::composite_type!('static, "thinkthen_ranked")>),
            name!(failure, Option<pgrx::composite_type!('static, "thinkthen_error")>),
            name!(facts, Option<pgrx::composite_type!('static, "thinkthen_facts")>),
            name!(observations, crate::ffi::EventArray))> {
    call::guarded(|| {
        let (Some(question), Some(input)) = (question, input) else {
            return TableIterator::new(Vec::new());
        };
        let rows = super::row_values(super::plain::keyed("rank", question, input, settings));
        TableIterator::new(rows.into_iter().map(|row| {
            let value = row.get("value");
            (row.get("ordinal").and_then(Value::as_i64), output::indexes(value), output::numbers(row.get("probability")),
                output::optional_reading("thinkthen_ranked", row.get("result")),
                output::optional_reading("thinkthen_error", row.get("failure")),
                output::optional_reading("thinkthen_facts", row.get("facts")),
                output::events(row.get("observations").and_then(Value::as_array).map_or(&[], Vec::as_slice)))
        }).collect::<Vec<_>>())
    })
}
}

crate::descriptions::describe! {
"Return full typed records with values, probabilities, failures, facts and observations. Recoverable failures retain their rows; cancellation, deadline and defects raise. Required NULL operands return no rows. Evidence files must be read by the client.";
[name = "thinkthen_annotate_details", parallel_restricted, requires = ["native_record_types", "native_reading_type", "native_result_types", "native_generated_types"]];
#[allow(clippy::type_complexity, clippy::too_many_arguments, reason = "pgrx reads named SQL arguments and columns from the signature")]
fn annotate_records(
    question: Option<&str>, records: Option<Array<'_, pgrx::composite_type!("thinkthen_record")>>,
                    reading: default!(Option<pgrx::composite_type!("thinkthen_reading")>, "NULL"),
) -> TableIterator<'static, (name!(ordinal, Option<i64>), name!(value, Option<JsonB>),
            name!(probability, Option<f64>),
            name!(result, Option<pgrx::composite_type!('static, "thinkthen_annotation")>),
            name!(failure, Option<pgrx::composite_type!('static, "thinkthen_error")>),
            name!(facts, Option<pgrx::composite_type!('static, "thinkthen_facts")>),
            name!(observations, crate::ffi::EventArray))> {
    call::guarded(|| {
        let (Some(question), Some(records)) = (question, records) else {
            return TableIterator::new(Vec::new());
        };
        let rows = super::row_values(super::execute("annotate", question, records, reading));
        TableIterator::new(rows.into_iter().map(|row| {
            let value = row.get("value");
            (row.get("ordinal").and_then(Value::as_i64), value.filter(|v| !v.is_null()).cloned().map(JsonB), row.get("probability").and_then(Value::as_f64),
                output::optional_reading("thinkthen_annotation", row.get("result")),
                output::optional_reading("thinkthen_error", row.get("failure")),
                output::optional_reading("thinkthen_facts", row.get("facts")),
                output::events(row.get("observations").and_then(Value::as_array).map_or(&[], Vec::as_slice)))
        }).collect::<Vec<_>>())
    })
}
}

crate::descriptions::describe! {
"Return full typed records with values, probabilities, failures, facts and observations. Recoverable failures retain their rows; cancellation, deadline and defects raise. Required NULL operands return no rows. Evidence files must be read by the client.";
[name = "thinkthen_annotate_details", parallel_restricted, requires = ["native_record_types", "native_reading_type", "native_result_types", "native_generated_types"]];
#[allow(clippy::type_complexity, clippy::too_many_arguments, reason = "pgrx reads named SQL arguments and columns from the signature")]
fn annotate_plain(
    question: Option<&str>, input: Option<&str>,
                    settings: default!(Option<crate::ffi::RawJson>, "NULL"),
                    threshold: default!(Option<&str>, "NULL"),
                    context: default!(Option<&str>, "NULL"),
                    model: default!(Option<&str>, "NULL"),
                    batch: default!(Option<&str>, "NULL"),
                    deadline_ms: default!(Option<i64>, "NULL"),
) -> TableIterator<'static, (name!(ordinal, Option<i64>), name!(value, Option<JsonB>),
            name!(probability, Option<f64>),
            name!(result, Option<pgrx::composite_type!('static, "thinkthen_annotation")>),
            name!(failure, Option<pgrx::composite_type!('static, "thinkthen_error")>),
            name!(facts, Option<pgrx::composite_type!('static, "thinkthen_facts")>),
            name!(observations, crate::ffi::EventArray))> {
    call::guarded(|| {
        let (Some(question), Some(input)) = (question, input) else {
            return TableIterator::new(Vec::new());
        };
        let rows = super::row_values(super::execute_plain("annotate", question, input, None, settings, crate::forms::Named::new(threshold, context, model, batch, deadline_ms)));
        TableIterator::new(rows.into_iter().map(|row| {
            let value = row.get("value");
            (row.get("ordinal").and_then(Value::as_i64), value.filter(|v| !v.is_null()).cloned().map(JsonB), row.get("probability").and_then(Value::as_f64),
                output::optional_reading("thinkthen_annotation", row.get("result")),
                output::optional_reading("thinkthen_error", row.get("failure")),
                output::optional_reading("thinkthen_facts", row.get("facts")),
                output::events(row.get("observations").and_then(Value::as_array).map_or(&[], Vec::as_slice)))
        }).collect::<Vec<_>>())
    })
}
}

crate::descriptions::describe! {
"Return full typed records with values, probabilities, failures, facts and observations. Recoverable failures retain their rows; cancellation, deadline and defects raise. Required NULL operands return no rows. Evidence files must be read by the client.";
[name = "thinkthen_recognize_details", parallel_restricted, requires = ["native_record_types", "native_reading_type", "native_result_types", "native_generated_types"]];
#[allow(clippy::type_complexity, clippy::too_many_arguments, reason = "pgrx reads named SQL arguments and columns from the signature")]
fn recognize_records(
    question: Option<&str>, records: Option<Array<'_, pgrx::composite_type!("thinkthen_record")>>,
                    reading: default!(Option<pgrx::composite_type!("thinkthen_reading")>, "NULL"),
) -> TableIterator<'static, (name!(ordinal, Option<i64>), name!(value, Option<crate::ffi::NameArray>),
            name!(probability, Option<Vec<f64>>),
            name!(result, Option<pgrx::composite_type!('static, "thinkthen_recognized")>),
            name!(failure, Option<pgrx::composite_type!('static, "thinkthen_error")>),
            name!(facts, Option<pgrx::composite_type!('static, "thinkthen_facts")>),
            name!(observations, crate::ffi::EventArray))> {
    call::guarded(|| {
        let (Some(question), Some(records)) = (question, records) else {
            return TableIterator::new(Vec::new());
        };
        let rows = super::row_values(super::execute("recognize", question, records, reading));
        TableIterator::new(rows.into_iter().map(|row| {
            let value = row.get("value");
            (row.get("ordinal").and_then(Value::as_i64), value.map(output::name_values), output::numbers(row.get("probability")),
                output::optional_reading("thinkthen_recognized", row.get("result")),
                output::optional_reading("thinkthen_error", row.get("failure")),
                output::optional_reading("thinkthen_facts", row.get("facts")),
                output::events(row.get("observations").and_then(Value::as_array).map_or(&[], Vec::as_slice)))
        }).collect::<Vec<_>>())
    })
}
}

crate::descriptions::describe! {
"Return full typed records with values, probabilities, failures, facts and observations. Recoverable failures retain their rows; cancellation, deadline and defects raise. Required NULL operands return no rows. Evidence files must be read by the client.";
[name = "thinkthen_recognize_details", parallel_restricted, requires = ["native_record_types", "native_reading_type", "native_result_types", "native_generated_types"]];
#[allow(clippy::type_complexity, clippy::too_many_arguments, reason = "pgrx reads named SQL arguments and columns from the signature")]
fn recognize_plain(
    question: Option<&str>, input: Option<&str>,
) -> TableIterator<'static, (name!(ordinal, Option<i64>), name!(value, Option<crate::ffi::NameArray>),
            name!(probability, Option<Vec<f64>>),
            name!(result, Option<pgrx::composite_type!('static, "thinkthen_recognized")>),
            name!(failure, Option<pgrx::composite_type!('static, "thinkthen_error")>),
            name!(facts, Option<pgrx::composite_type!('static, "thinkthen_facts")>),
            name!(observations, crate::ffi::EventArray))> {
    call::guarded(|| {
        let (Some(question), Some(input)) = (question, input) else {
            return TableIterator::new(Vec::new());
        };
        let rows = super::row_values(super::execute_plain("recognize", input, question, None, None, Default::default()));
        TableIterator::new(rows.into_iter().map(|row| {
            let value = row.get("value");
            (row.get("ordinal").and_then(Value::as_i64), value.map(output::name_values), output::numbers(row.get("probability")),
                output::optional_reading("thinkthen_recognized", row.get("result")),
                output::optional_reading("thinkthen_error", row.get("failure")),
                output::optional_reading("thinkthen_facts", row.get("facts")),
                output::events(row.get("observations").and_then(Value::as_array).map_or(&[], Vec::as_slice)))
        }).collect::<Vec<_>>())
    })
}
}

crate::descriptions::describe! {
"Return full typed records with values, probabilities, failures, facts and observations. Recoverable failures retain their rows; cancellation, deadline and defects raise. Required NULL operands return no rows. Evidence files must be read by the client.";
[name = "thinkthen_recognize_details", parallel_restricted, requires = ["native_record_types", "native_reading_type", "native_result_types", "native_generated_types"]];
#[allow(clippy::type_complexity, clippy::too_many_arguments, reason = "pgrx reads named SQL arguments and columns from the signature")]
fn recognize_kinds(
    question: Option<&str>, input: Option<Array<'_, &str>>,
) -> TableIterator<'static, (name!(ordinal, Option<i64>), name!(value, Option<crate::ffi::NameArray>),
            name!(probability, Option<Vec<f64>>),
            name!(result, Option<pgrx::composite_type!('static, "thinkthen_recognized")>),
            name!(failure, Option<pgrx::composite_type!('static, "thinkthen_error")>),
            name!(facts, Option<pgrx::composite_type!('static, "thinkthen_facts")>),
            name!(observations, crate::ffi::EventArray))> {
    call::guarded(|| {
        let (Some(question), Some(input)) = (question, input) else {
            return TableIterator::new(Vec::new());
        };
        let rows = super::row_values(super::plain::recognize(question, input));
        TableIterator::new(rows.into_iter().map(|row| {
            let value = row.get("value");
            (row.get("ordinal").and_then(Value::as_i64), value.map(output::name_values), output::numbers(row.get("probability")),
                output::optional_reading("thinkthen_recognized", row.get("result")),
                output::optional_reading("thinkthen_error", row.get("failure")),
                output::optional_reading("thinkthen_facts", row.get("facts")),
                output::events(row.get("observations").and_then(Value::as_array).map_or(&[], Vec::as_slice)))
        }).collect::<Vec<_>>())
    })
}
}

crate::descriptions::describe! {
"Return full typed records with values, probabilities, failures, facts and observations. Recoverable failures retain their rows; cancellation, deadline and defects raise. Required NULL operands return no rows. Evidence files must be read by the client.";
[name = "thinkthen_find_details", parallel_restricted, requires = ["native_record_types", "native_reading_type", "native_result_types", "native_generated_types"]];
#[allow(clippy::type_complexity, clippy::too_many_arguments, reason = "pgrx reads named SQL arguments and columns from the signature")]
fn find_records(
    question: Option<&str>, records: Option<Array<'_, pgrx::composite_type!("thinkthen_record")>>,
                    reading: default!(Option<pgrx::composite_type!("thinkthen_reading")>, "NULL"),
) -> TableIterator<'static, (name!(ordinal, Option<i64>), name!(value, Option<JsonB>),
            name!(probability, Option<f64>),
            name!(result, Option<pgrx::composite_type!('static, "thinkthen_found")>),
            name!(failure, Option<pgrx::composite_type!('static, "thinkthen_error")>),
            name!(facts, Option<pgrx::composite_type!('static, "thinkthen_facts")>),
            name!(observations, crate::ffi::EventArray))> {
    call::guarded(|| {
        let (Some(question), Some(records)) = (question, records) else {
            return TableIterator::new(Vec::new());
        };
        let rows = super::row_values(super::execute("find", question, records, reading));
        TableIterator::new(rows.into_iter().map(|row| {
            let value = row.get("value");
            (row.get("ordinal").and_then(Value::as_i64), value.filter(|v| !v.is_null()).cloned().map(JsonB), row.get("probability").and_then(Value::as_f64),
                output::optional_reading("thinkthen_found", row.get("result")),
                output::optional_reading("thinkthen_error", row.get("failure")),
                output::optional_reading("thinkthen_facts", row.get("facts")),
                output::events(row.get("observations").and_then(Value::as_array).map_or(&[], Vec::as_slice)))
        }).collect::<Vec<_>>())
    })
}
}

crate::descriptions::describe! {
"Return full typed records with values, probabilities, failures, facts and observations. Recoverable failures retain their rows; cancellation, deadline and defects raise. Required NULL operands return no rows. Evidence files must be read by the client.";
[name = "thinkthen_find_details", parallel_restricted, requires = ["native_record_types", "native_reading_type", "native_result_types", "native_generated_types"]];
#[allow(clippy::type_complexity, clippy::too_many_arguments, reason = "pgrx reads named SQL arguments and columns from the signature")]
fn find_plain(
    question: Option<&str>, input: Option<Array<'_, &str>>, settings: default!(Option<crate::ffi::RawJson>, "NULL"),
) -> TableIterator<'static, (name!(ordinal, Option<i64>), name!(value, Option<JsonB>),
            name!(probability, Option<f64>),
            name!(result, Option<pgrx::composite_type!('static, "thinkthen_found")>),
            name!(failure, Option<pgrx::composite_type!('static, "thinkthen_error")>),
            name!(facts, Option<pgrx::composite_type!('static, "thinkthen_facts")>),
            name!(observations, crate::ffi::EventArray))> {
    call::guarded(|| {
        let (Some(question), Some(input)) = (question, input) else {
            return TableIterator::new(Vec::new());
        };
        let rows = super::row_values(super::plain::find(question, input, settings));
        TableIterator::new(rows.into_iter().map(|row| {
            let value = row.get("value");
            (row.get("ordinal").and_then(Value::as_i64), value.filter(|v| !v.is_null()).cloned().map(JsonB), row.get("probability").and_then(Value::as_f64),
                output::optional_reading("thinkthen_found", row.get("result")),
                output::optional_reading("thinkthen_error", row.get("failure")),
                output::optional_reading("thinkthen_facts", row.get("facts")),
                output::events(row.get("observations").and_then(Value::as_array).map_or(&[], Vec::as_slice)))
        }).collect::<Vec<_>>())
    })
}
}

crate::descriptions::describe! {
"Return full typed records with values, probabilities, failures, facts and observations. Recoverable failures retain their rows; cancellation, deadline and defects raise. Required NULL operands return no rows. Evidence files must be read by the client.";
[name = "thinkthen_relate_details", parallel_restricted, requires = ["native_record_types", "native_reading_type", "native_result_types", "native_generated_types"]];
#[allow(clippy::type_complexity, clippy::too_many_arguments, reason = "pgrx reads named SQL arguments and columns from the signature")]
fn relate_records(
    question: Option<&str>, records: Option<Array<'_, pgrx::composite_type!("thinkthen_record")>>,
                    reading: default!(Option<pgrx::composite_type!("thinkthen_reading")>, "NULL"),
) -> TableIterator<'static, (name!(ordinal, Option<i64>), name!(value, Option<crate::ffi::EdgeArray>),
            name!(probability, Option<Vec<f64>>),
            name!(result, Option<pgrx::composite_type!('static, "thinkthen_related")>),
            name!(failure, Option<pgrx::composite_type!('static, "thinkthen_error")>),
            name!(facts, Option<pgrx::composite_type!('static, "thinkthen_facts")>),
            name!(observations, crate::ffi::EventArray))> {
    call::guarded(|| {
        let (Some(question), Some(records)) = (question, records) else {
            return TableIterator::new(Vec::new());
        };
        let rows = super::row_values(super::execute("relate", question, records, reading));
        TableIterator::new(rows.into_iter().map(|row| {
            let value = row.get("value");
            (row.get("ordinal").and_then(Value::as_i64), value.map(output::edge_values), output::numbers(row.get("probability")),
                output::optional_reading("thinkthen_related", row.get("result")),
                output::optional_reading("thinkthen_error", row.get("failure")),
                output::optional_reading("thinkthen_facts", row.get("facts")),
                output::events(row.get("observations").and_then(Value::as_array).map_or(&[], Vec::as_slice)))
        }).collect::<Vec<_>>())
    })
}
}

crate::descriptions::describe! {
"Return full typed records with values, probabilities, failures, facts and observations. Recoverable failures retain their rows; cancellation, deadline and defects raise. Required NULL operands return no rows. Evidence files must be read by the client.";
[name = "thinkthen_relate_details", parallel_restricted, requires = ["native_record_types", "native_reading_type", "native_result_types", "native_generated_types"]];
#[allow(clippy::type_complexity, clippy::too_many_arguments, reason = "pgrx reads named SQL arguments and columns from the signature")]
fn relate_plain(
    question: Option<&str>, input: Option<&str>,
) -> TableIterator<'static, (name!(ordinal, Option<i64>), name!(value, Option<crate::ffi::EdgeArray>),
            name!(probability, Option<Vec<f64>>),
            name!(result, Option<pgrx::composite_type!('static, "thinkthen_related")>),
            name!(failure, Option<pgrx::composite_type!('static, "thinkthen_error")>),
            name!(facts, Option<pgrx::composite_type!('static, "thinkthen_facts")>),
            name!(observations, crate::ffi::EventArray))> {
    call::guarded(|| {
        let (Some(question), Some(input)) = (question, input) else {
            return TableIterator::new(Vec::new());
        };
        let rows = super::row_values(super::plain::relate_spec(question, input));
        TableIterator::new(rows.into_iter().map(|row| {
            let value = row.get("value");
            (row.get("ordinal").and_then(Value::as_i64), value.map(output::edge_values), output::numbers(row.get("probability")),
                output::optional_reading("thinkthen_related", row.get("result")),
                output::optional_reading("thinkthen_error", row.get("failure")),
                output::optional_reading("thinkthen_facts", row.get("facts")),
                output::events(row.get("observations").and_then(Value::as_array).map_or(&[], Vec::as_slice)))
        }).collect::<Vec<_>>())
    })
}
}

crate::descriptions::describe! {
"Return full typed records with values, probabilities, failures, facts and observations. Recoverable failures retain their rows; cancellation, deadline and defects raise. Required NULL operands return no rows. Evidence files must be read by the client.";
[name = "thinkthen_relate_details", parallel_restricted, requires = ["native_record_types", "native_reading_type", "native_result_types", "native_generated_types"]];
#[allow(clippy::type_complexity, clippy::too_many_arguments, reason = "pgrx reads named SQL arguments and columns from the signature")]
fn relate_rules(
    question: Option<&str>, input: Option<Array<'_, &str>>,
) -> TableIterator<'static, (name!(ordinal, Option<i64>), name!(value, Option<crate::ffi::EdgeArray>),
            name!(probability, Option<Vec<f64>>),
            name!(result, Option<pgrx::composite_type!('static, "thinkthen_related")>),
            name!(failure, Option<pgrx::composite_type!('static, "thinkthen_error")>),
            name!(facts, Option<pgrx::composite_type!('static, "thinkthen_facts")>),
            name!(observations, crate::ffi::EventArray))> {
    call::guarded(|| {
        let (Some(question), Some(input)) = (question, input) else {
            return TableIterator::new(Vec::new());
        };
        let rows = super::row_values(super::plain::relate(question, input));
        TableIterator::new(rows.into_iter().map(|row| {
            let value = row.get("value");
            (row.get("ordinal").and_then(Value::as_i64), value.map(output::edge_values), output::numbers(row.get("probability")),
                output::optional_reading("thinkthen_related", row.get("result")),
                output::optional_reading("thinkthen_error", row.get("failure")),
                output::optional_reading("thinkthen_facts", row.get("facts")),
                output::events(row.get("observations").and_then(Value::as_array).map_or(&[], Vec::as_slice)))
        }).collect::<Vec<_>>())
    })
}
}

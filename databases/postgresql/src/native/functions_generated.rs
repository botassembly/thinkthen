// Generated native PostgreSQL declarations; do not edit.
use super::output;
use crate::call::{self, OrRaise as _};
use pgrx::datum::JsonB;
use pgrx::prelude::*;
use thinkthen::RequestValue;
crate::descriptions::describe! {
"Read ordered native SQL records with typed controls; return typed answers, full result readings, final facts, failures and observations. Required NULL operands return no rows. Evidence files must be read by the client.";
[name = "thinkthen_decide", parallel_restricted, requires = ["native_record_types", "native_reading_type", "native_result_types", "native_generated_types"]];
#[allow(clippy::type_complexity, reason = "pgrx reads SQL columns from the signature")]
fn decide_records(
    question: Option<&str>,
    records: Option<Array<'_, pgrx::composite_type!("thinkthen_record")>>,
    reading: default!(Option<pgrx::composite_type!("thinkthen_reading")>, "NULL"),
) -> TableIterator<'static, (
    name!(ordinal, Option<i64>), name!(value, Option<bool>),
    name!(result, Option<pgrx::composite_type!('static, "thinkthen_decision")>),
    name!(facts, Option<pgrx::composite_type!('static, "thinkthen_facts")>),
    name!(failure, Option<pgrx::composite_type!('static, "thinkthen_error")>),
    name!(observations, crate::ffi::EventArray),
)> {
    call::guarded(|| {
        let (Some(question), Some(records)) = (question, records) else {
            return TableIterator::new(Vec::new());
        };
        let output::Projection { value, facts, failure, observations } = output::outcome(
            super::execute("decide", question, records, reading),
        );
        let mut rows = match value {
            Some(RequestValue::Decisions(rows)) => rows.into_iter().map(|row| {
                let reading = output::reading("thinkthen_decision", &serde_json::to_value(&row)
                    .map_err(|_| call::defect("native result could not be projected")).or_raise());
                (Some(output::index(row.ordinal())), crate::answer_value(row.result().value()), Some(reading),
                    facts.as_ref().map(output::facts), None, output::events(&observations))
            }).collect::<Vec<_>>(),

            None => Vec::new(),
            _ => call::raise(crate::request::wrong_result()),
        };
        if failure.is_some() || rows.is_empty() {
            rows.push((None, None, None, facts.as_ref().map(output::facts),
                failure.as_ref().map(output::failure), output::events(&observations)));
        }
        TableIterator::new(rows)
    })
}
}

crate::descriptions::describe! {
"Read ordered native SQL records with typed controls; return typed answers, full result readings, final facts, failures and observations. Required NULL operands return no rows. Evidence files must be read by the client.";
[name = "thinkthen_choose", parallel_restricted, requires = ["native_record_types", "native_reading_type", "native_result_types", "native_generated_types"]];
#[allow(clippy::type_complexity, reason = "pgrx reads SQL columns from the signature")]
fn choose_records(
    question: Option<&str>,
    records: Option<Array<'_, pgrx::composite_type!("thinkthen_record")>>,
    reading: default!(Option<pgrx::composite_type!("thinkthen_reading")>, "NULL"),
) -> TableIterator<'static, (
    name!(ordinal, Option<i64>), name!(value, Option<String>),
    name!(result, Option<pgrx::composite_type!('static, "thinkthen_choice")>),
    name!(facts, Option<pgrx::composite_type!('static, "thinkthen_facts")>),
    name!(failure, Option<pgrx::composite_type!('static, "thinkthen_error")>),
    name!(observations, crate::ffi::EventArray),
)> {
    call::guarded(|| {
        let (Some(question), Some(records)) = (question, records) else {
            return TableIterator::new(Vec::new());
        };
        let output::Projection { value, facts, failure, observations } = output::outcome(
            super::execute("choose", question, records, reading),
        );
        let mut rows = match value {
            Some(RequestValue::Choices(rows)) => rows.into_iter().map(|row| {
                let reading = output::reading("thinkthen_choice", &serde_json::to_value(&row)
                    .map_err(|_| call::defect("native result could not be projected")).or_raise());
                (Some(output::index(row.ordinal())), row.result().value().map(str::to_owned), Some(reading),
                    facts.as_ref().map(output::facts), None, output::events(&observations))
            }).collect::<Vec<_>>(),

            None => Vec::new(),
            _ => call::raise(crate::request::wrong_result()),
        };
        if failure.is_some() || rows.is_empty() {
            rows.push((None, None, None, facts.as_ref().map(output::facts),
                failure.as_ref().map(output::failure), output::events(&observations)));
        }
        TableIterator::new(rows)
    })
}
}

crate::descriptions::describe! {
"Read ordered native SQL records with typed controls; return typed answers, full result readings, final facts, failures and observations. Required NULL operands return no rows. Evidence files must be read by the client.";
[name = "thinkthen_tag", parallel_restricted, requires = ["native_record_types", "native_reading_type", "native_result_types", "native_generated_types"]];
#[allow(clippy::type_complexity, reason = "pgrx reads SQL columns from the signature")]
fn tag_records(
    question: Option<&str>,
    records: Option<Array<'_, pgrx::composite_type!("thinkthen_record")>>,
    reading: default!(Option<pgrx::composite_type!("thinkthen_reading")>, "NULL"),
) -> TableIterator<'static, (
    name!(ordinal, Option<i64>), name!(value, Option<Vec<String>>),
    name!(result, Option<pgrx::composite_type!('static, "thinkthen_tags")>),
    name!(facts, Option<pgrx::composite_type!('static, "thinkthen_facts")>),
    name!(failure, Option<pgrx::composite_type!('static, "thinkthen_error")>),
    name!(observations, crate::ffi::EventArray),
)> {
    call::guarded(|| {
        let (Some(question), Some(records)) = (question, records) else {
            return TableIterator::new(Vec::new());
        };
        let output::Projection { value, facts, failure, observations } = output::outcome(
            super::execute("tag", question, records, reading),
        );
        let mut rows = match value {
            Some(RequestValue::Tags(rows)) => rows.into_iter().map(|row| {
                let reading = output::reading("thinkthen_tags", &serde_json::to_value(&row)
                    .map_err(|_| call::defect("native result could not be projected")).or_raise());
                (Some(output::index(row.ordinal())), Some(row.result().value().to_vec()), Some(reading),
                    facts.as_ref().map(output::facts), None, output::events(&observations))
            }).collect::<Vec<_>>(),

            None => Vec::new(),
            _ => call::raise(crate::request::wrong_result()),
        };
        if failure.is_some() || rows.is_empty() {
            rows.push((None, None, None, facts.as_ref().map(output::facts),
                failure.as_ref().map(output::failure), output::events(&observations)));
        }
        TableIterator::new(rows)
    })
}
}

crate::descriptions::describe! {
"Read ordered native SQL records with typed controls; return typed answers, full result readings, final facts, failures and observations. Required NULL operands return no rows. Evidence files must be read by the client.";
[name = "thinkthen_score", parallel_restricted, requires = ["native_record_types", "native_reading_type", "native_result_types", "native_generated_types"]];
#[allow(clippy::type_complexity, reason = "pgrx reads SQL columns from the signature")]
fn score_records(
    question: Option<&str>,
    records: Option<Array<'_, pgrx::composite_type!("thinkthen_record")>>,
    reading: default!(Option<pgrx::composite_type!("thinkthen_reading")>, "NULL"),
) -> TableIterator<'static, (
    name!(ordinal, Option<i64>), name!(value, Option<f64>),
    name!(result, Option<pgrx::composite_type!('static, "thinkthen_score")>),
    name!(facts, Option<pgrx::composite_type!('static, "thinkthen_facts")>),
    name!(failure, Option<pgrx::composite_type!('static, "thinkthen_error")>),
    name!(observations, crate::ffi::EventArray),
)> {
    call::guarded(|| {
        let (Some(question), Some(records)) = (question, records) else {
            return TableIterator::new(Vec::new());
        };
        let output::Projection { value, facts, failure, observations } = output::outcome(
            super::execute("score", question, records, reading),
        );
        let mut rows = match value {
            Some(RequestValue::Scores(rows)) => rows.into_iter().map(|row| {
                let reading = output::reading("thinkthen_score", &serde_json::to_value(&row)
                    .map_err(|_| call::defect("native result could not be projected")).or_raise());
                (Some(output::index(row.ordinal())), Some(row.result().value()), Some(reading),
                    facts.as_ref().map(output::facts), None, output::events(&observations))
            }).collect::<Vec<_>>(),

            None => Vec::new(),
            _ => call::raise(crate::request::wrong_result()),
        };
        if failure.is_some() || rows.is_empty() {
            rows.push((None, None, None, facts.as_ref().map(output::facts),
                failure.as_ref().map(output::failure), output::events(&observations)));
        }
        TableIterator::new(rows)
    })
}
}

crate::descriptions::describe! {
"Read ordered native SQL records with typed controls; return typed answers, full result readings, final facts, failures and observations. Required NULL operands return no rows. Evidence files must be read by the client.";
[name = "thinkthen_filter", parallel_restricted, requires = ["native_record_types", "native_reading_type", "native_result_types", "native_generated_types"]];
#[allow(clippy::type_complexity, reason = "pgrx reads SQL columns from the signature")]
fn filter_records(
    question: Option<&str>,
    records: Option<Array<'_, pgrx::composite_type!("thinkthen_record")>>,
    reading: default!(Option<pgrx::composite_type!("thinkthen_reading")>, "NULL"),
) -> TableIterator<'static, (
    name!(ordinal, Option<i64>), name!(value, Option<JsonB>),
    name!(result, Option<pgrx::composite_type!('static, "thinkthen_filtered")>),
    name!(facts, Option<pgrx::composite_type!('static, "thinkthen_facts")>),
    name!(failure, Option<pgrx::composite_type!('static, "thinkthen_error")>),
    name!(observations, crate::ffi::EventArray),
)> {
    call::guarded(|| {
        let (Some(question), Some(records)) = (question, records) else {
            return TableIterator::new(Vec::new());
        };
        let output::Projection { value, facts, failure, observations } = output::outcome(
            super::execute("filter", question, records, reading),
        );
        let mut rows = match value {
            Some(RequestValue::Filtered(rows)) => rows.into_iter().map(|row| {
                let reading = output::reading("thinkthen_filtered", &serde_json::to_value(&row)
                    .map_err(|_| call::defect("native result could not be projected")).or_raise());
                (Some(output::index(row.ordinal())), row.result().value().then(|| output::original(row.original())), Some(reading),
                    facts.as_ref().map(output::facts), None, output::events(&observations))
            }).collect::<Vec<_>>(),

            None => Vec::new(),
            _ => call::raise(crate::request::wrong_result()),
        };
        if failure.is_some() || rows.is_empty() {
            rows.push((None, None, None, facts.as_ref().map(output::facts),
                failure.as_ref().map(output::failure), output::events(&observations)));
        }
        TableIterator::new(rows)
    })
}
}

crate::descriptions::describe! {
"Read ordered native SQL records with typed controls; return typed answers, full result readings, final facts, failures and observations. Required NULL operands return no rows. Evidence files must be read by the client.";
[name = "thinkthen_rank", parallel_restricted, requires = ["native_record_types", "native_reading_type", "native_result_types", "native_generated_types"]];
#[allow(clippy::type_complexity, reason = "pgrx reads SQL columns from the signature")]
fn rank_records(
    question: Option<&str>,
    records: Option<Array<'_, pgrx::composite_type!("thinkthen_record")>>,
    reading: default!(Option<pgrx::composite_type!("thinkthen_reading")>, "NULL"),
) -> TableIterator<'static, (
    name!(ordinal, Option<i64>), name!(value, Option<i64>),
    name!(result, Option<pgrx::composite_type!('static, "thinkthen_ranked")>),
    name!(facts, Option<pgrx::composite_type!('static, "thinkthen_facts")>),
    name!(failure, Option<pgrx::composite_type!('static, "thinkthen_error")>),
    name!(observations, crate::ffi::EventArray),
)> {
    call::guarded(|| {
        let (Some(question), Some(records)) = (question, records) else {
            return TableIterator::new(Vec::new());
        };
        let output::Projection { value, facts, failure, observations } = output::outcome(
            super::execute("rank", question, records, reading),
        );
        let mut rows = match value {
            Some(RequestValue::Ranked(rows)) => rows.into_iter().map(|row| {
                let reading = output::reading("thinkthen_ranked", &serde_json::to_value(&row)
                    .map_err(|_| call::defect("native result could not be projected")).or_raise());
                (Some(output::index(row.ordinal())), Some(output::index(row.ordinal())), Some(reading),
                    facts.as_ref().map(output::facts), None, output::events(&observations))
            }).collect::<Vec<_>>(),
            Some(RequestValue::SetRanked(rows)) => rows.into_iter().map(|row| {
                let reading = output::reading("thinkthen_ranked", &serde_json::to_value(&row)
                    .map_err(|_| call::defect("native result could not be projected")).or_raise());
                (Some(output::index(row.ordinal())), Some(output::index(row.ordinal())), Some(reading),
                    facts.as_ref().map(output::facts), None, output::events(&observations))
            }).collect::<Vec<_>>(),
            None => Vec::new(),
            _ => call::raise(crate::request::wrong_result()),
        };
        if failure.is_some() || rows.is_empty() {
            rows.push((None, None, None, facts.as_ref().map(output::facts),
                failure.as_ref().map(output::failure), output::events(&observations)));
        }
        TableIterator::new(rows)
    })
}
}

crate::descriptions::describe! {
"Read ordered native SQL records with typed controls; return typed answers, full result readings, final facts, failures and observations. Required NULL operands return no rows. Evidence files must be read by the client.";
[name = "thinkthen_annotate", parallel_restricted, requires = ["native_record_types", "native_reading_type", "native_result_types", "native_generated_types"]];
#[allow(clippy::type_complexity, reason = "pgrx reads SQL columns from the signature")]
fn annotate_records(
    question: Option<&str>,
    records: Option<Array<'_, pgrx::composite_type!("thinkthen_record")>>,
    reading: default!(Option<pgrx::composite_type!("thinkthen_reading")>, "NULL"),
) -> TableIterator<'static, (
    name!(ordinal, Option<i64>), name!(value, Option<JsonB>),
    name!(result, Option<pgrx::composite_type!('static, "thinkthen_annotation")>),
    name!(facts, Option<pgrx::composite_type!('static, "thinkthen_facts")>),
    name!(failure, Option<pgrx::composite_type!('static, "thinkthen_error")>),
    name!(observations, crate::ffi::EventArray),
)> {
    call::guarded(|| {
        let (Some(question), Some(records)) = (question, records) else {
            return TableIterator::new(Vec::new());
        };
        let output::Projection { value, facts, failure, observations } = output::outcome(
            super::execute("annotate", question, records, reading),
        );
        let mut rows = match value {
            Some(RequestValue::Annotations(rows)) => rows.into_iter().map(|row| {
                let reading = output::reading("thinkthen_annotation", &serde_json::to_value(&row)
                    .map_err(|_| call::defect("native result could not be projected")).or_raise());
                (Some(output::index(row.ordinal())), Some(crate::jsonb(&row.result().value_json().or_raise())), Some(reading),
                    facts.as_ref().map(output::facts), None, output::events(&observations))
            }).collect::<Vec<_>>(),

            None => Vec::new(),
            _ => call::raise(crate::request::wrong_result()),
        };
        if failure.is_some() || rows.is_empty() {
            rows.push((None, None, None, facts.as_ref().map(output::facts),
                failure.as_ref().map(output::failure), output::events(&observations)));
        }
        TableIterator::new(rows)
    })
}
}

crate::descriptions::describe! {
"Read ordered native SQL records with typed controls; return typed answers, full result readings, final facts, failures and observations. Required NULL operands return no rows. Evidence files must be read by the client.";
[name = "thinkthen_recognize", parallel_restricted, requires = ["native_record_types", "native_reading_type", "native_result_types", "native_generated_types"]];
#[allow(clippy::type_complexity, reason = "pgrx reads SQL columns from the signature")]
fn recognize_records(
    question: Option<&str>,
    records: Option<Array<'_, pgrx::composite_type!("thinkthen_record")>>,
    reading: default!(Option<pgrx::composite_type!("thinkthen_reading")>, "NULL"),
) -> TableIterator<'static, (
    name!(ordinal, Option<i64>), name!(value, Option<crate::ffi::NameArray>),
    name!(result, Option<pgrx::composite_type!('static, "thinkthen_recognized")>),
    name!(facts, Option<pgrx::composite_type!('static, "thinkthen_facts")>),
    name!(failure, Option<pgrx::composite_type!('static, "thinkthen_error")>),
    name!(observations, crate::ffi::EventArray),
)> {
    call::guarded(|| {
        let (Some(question), Some(records)) = (question, records) else {
            return TableIterator::new(Vec::new());
        };
        let output::Projection { value, facts, failure, observations } = output::outcome(
            super::execute("recognize", question, records, reading),
        );
        let mut rows = match value {
            Some(RequestValue::Recognized(rows)) => rows.into_iter().map(|row| {
                let reading = output::reading("thinkthen_recognized", &serde_json::to_value(&row)
                    .map_err(|_| call::defect("native result could not be projected")).or_raise());
                (Some(output::index(row.ordinal())), Some(output::names(row.result().value())), Some(reading),
                    facts.as_ref().map(output::facts), None, output::events(&observations))
            }).collect::<Vec<_>>(),

            None => Vec::new(),
            _ => call::raise(crate::request::wrong_result()),
        };
        if failure.is_some() || rows.is_empty() {
            rows.push((None, None, None, facts.as_ref().map(output::facts),
                failure.as_ref().map(output::failure), output::events(&observations)));
        }
        TableIterator::new(rows)
    })
}
}

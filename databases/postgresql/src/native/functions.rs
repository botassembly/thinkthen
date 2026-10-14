//! Same-name SQL record overloads project native answers once.
use super::output;
use crate::call::{self, OrRaise as _};
use pgrx::datum::JsonB;
use pgrx::prelude::*;
use thinkthen::RequestValue;

macro_rules! records {
    ($name:ident, $sql:literal, $verb:literal, $type:ty, $variant:ident, $result_type:literal, $row:ident, $answer:expr, $keep:expr) => {
        crate::descriptions::describe! {
        "Read ordered native SQL records with typed controls; return typed answers, final count-only facts and failures. Required NULL operands return no rows. Evidence files must be read by the client.";
        [name = $sql, parallel_restricted, requires = ["native_record_types", "native_reading_type", "native_result_types", "native_generated_types"]];
        #[allow(clippy::type_complexity, reason = "pgrx reads SQL columns from the signature")]
        fn $name(
            question: Option<&str>,
            records: Option<Array<'_, pgrx::composite_type!("thinkthen_record")>>,
            reading: default!(Option<pgrx::composite_type!("thinkthen_reading")>, "NULL"),
        ) -> TableIterator<'static, (
            name!(ordinal, Option<i64>),
            name!(value, Option<$type>),
            name!(reading, Option<pgrx::composite_type!('static,  $result_type)>),
            name!(facts, Option<pgrx::composite_type!('static, "thinkthen_facts")>),
            name!(failure, Option<pgrx::composite_type!('static, "thinkthen_error")>),
            name!(observations, Vec<pgrx::composite_type!('static, "thinkthen_event")>),
        )> {
            call::guarded(|| {
                let (Some(question), Some(records)) = (question, records) else {
                    return TableIterator::new(Vec::new());
                };
                let output::Projection { value, facts, failure, observations } = output::outcome(
                    super::execute($verb, question, records, reading),
                );
                let mut rows = match value {
                    Some(RequestValue::$variant(rows)) => rows.into_iter().filter($keep)
                        .map(|$row| (
                            Some(output::index($row.ordinal())), $answer,
                            Some(output::reading($result_type, &serde_json::to_value(&$row)
                                .map_err(|_| call::defect("native result could not be projected")).or_raise())),
                            facts.as_ref().map(output::facts), None, output::events(&observations),
                        )).collect::<Vec<_>>(),
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
    };
}

records!(
    decide_records,
    "thinkthen_decide",
    "decide",
    bool,
    Decisions,
    "thinkthen_decision",
    row,
    crate::answer_value(row.result().value()),
    |_| true
);
records!(
    choose_records,
    "thinkthen_choose",
    "choose",
    String,
    Choices,
    "thinkthen_choice",
    row,
    row.result().value().map(str::to_owned),
    |_| true
);
records!(
    tag_records,
    "thinkthen_tag",
    "tag",
    Vec<String>,
    Tags,
    "thinkthen_tags",
    row,
    Some(row.result().value().to_vec()),
    |_| true
);
records!(
    score_records,
    "thinkthen_score",
    "score",
    f64,
    Scores,
    "thinkthen_score",
    row,
    Some(row.result().value()),
    |_| true
);
records!(
    filter_records,
    "thinkthen_filter",
    "filter",
    JsonB,
    Filtered,
    "thinkthen_filtered",
    row,
    row.result()
        .value()
        .then(|| output::original(row.original())),
    |_| true
);
records!(
    rank_records,
    "thinkthen_rank",
    "rank",
    i64,
    Ranked,
    "thinkthen_ranked",
    row,
    Some(output::index(row.result().value())),
    |_| true
);
records!(
    annotate_records,
    "thinkthen_annotate",
    "annotate",
    JsonB,
    Annotations,
    "thinkthen_annotation",
    row,
    Some(crate::jsonb(&row.result().value_json().or_raise())),
    |_| true
);
records!(
    recognize_records,
    "thinkthen_recognize",
    "recognize",
    Vec<pgrx::composite_type!('static, "thinkthen_name")>,
    Recognized,
    "thinkthen_recognized",
    row,
    Some(output::names(row.result().value())),
    |_| true
);

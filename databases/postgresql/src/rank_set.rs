//! Keyed native turns rank; no probability comparison between members.
use pgrx::datum::JsonB;
use pgrx::prelude::*;
use thinkthen::RankSet;

use crate::call::{self, OrRaise as _};
use crate::ffi::RawJson;
use crate::files::Given;
use crate::forms::{self, Named};

crate::descriptions::describe! {
["Rank keyed jsonb text records with an authored ordered decide-question set, returning key, rank, selecting probability, question name and count facts. Set text may use privileged/confined @files. ", crate::descriptions::REQUIRED_TABLE, "NULL settings uses defaults. Failures raise SQL errors; evidence files must be read by the client."].concat();
[name = "thinkthen_rank_set", parallel_restricted];
#[allow(clippy::type_complexity, reason = "pgrx reads the named SQL tuple")]
fn rank_set(
    questions: Option<&str>,
    input: Option<JsonB>,
    settings: default!(Option<RawJson>, "NULL"),
) -> TableIterator<
    'static,
    (
        name!(key, String),
        name!(rank, i64),
        name!(probability, f64),
        name!(question_name, String),
        name!(facts, JsonB),
    ),
> {
    call::guarded(|| {
        let (Some(questions), Some(input)) = (questions, input) else {
            return TableIterator::new(Vec::new());
        };
        let (_, call) = forms::controls(settings.as_ref(), Named::default());
        if let Some(raw) = &settings {
            let fields: serde_json::Value = serde_json::from_str(&raw.0)
                .map_err(|_| call::usage("settings is one JSON object"))
                .or_raise();
            if let Some(key) = fields.as_object().and_then(|fields| {
                fields
                    .keys()
                    .find(|key| !matches!(key.as_str(), "batch" | "context" | "deadline_ms"))
            }) {
                call::raise(call::usage(format!(
                    "the settings key `{key}` does not belong to this verb"
                )));
            }
        }
        let set = Given::read(
            Some(questions),
            "rank question set",
            call::file_directory().as_deref(),
        )
        .and_then(|given| given.parse(RankSet::from_json))
        .or_raise();
        let records = forms::keyed(Some(input));
        call.within(records.len());
        if records.is_empty() {
            return TableIterator::new(Vec::new());
        }
        let (keys, texts): (Vec<_>, Vec<_>) = records.into_iter().unzip();
        let rows = call::run(call, move |engine, options| {
            let ranked = crate::request::run(
                engine,
                thinkthen::RequestFunction::Rank,
                set.into(),
                texts.into_iter().map(crate::request::text).collect(),
                options,
            )?;
            let facts = serde_json::to_value(ranked.facts())
                .map_err(|_| call::defect("rank facts could not be encoded"))?;
            let thinkthen::RequestValue::SetRanked(ranked_rows) = ranked.value() else {
                return Err(crate::request::wrong_result());
            };
            ranked_rows
                .iter()
                .zip(1_i64..)
                .map(|(row, place)| {
                    let key = keys
                        .get(row.ordinal())
                        .ok_or_else(|| call::defect("a ranked row lost its record"))?;
                    Ok((
                        key.clone(),
                        place,
                        match row.result().result().probabilities() {
                            thinkthen::Probabilities::YesNo { yes } => yes,
                            _ => return Err(crate::request::wrong_result()),
                        },
                        row.result().question_name().to_owned(),
                        facts.clone(),
                    ))
                })
                .collect::<Result<Vec<_>, thinkthen::Error>>()
        });
        TableIterator::new(
            rows.into_iter()
                .map(|(key, rank, probability, name, facts)| {
                    (key, rank, probability, name, JsonB(facts))
                }),
        )
    })
}
}

//! One typed SQL reading maps to canonical request controls.
use super::{field, number};
use crate::call;
use pgrx::prelude::*;
use thinkthen::{Error, RequestBatch, RequestOptions, RequestThreshold};

extension_sql!(
    r#"
CREATE TYPE thinkthen_context_property AS (name text, kind text);
CREATE TYPE thinkthen_context_schema AS (kind text, properties thinkthen_context_property[], required text[]);
CREATE TYPE thinkthen_reading AS (
    context_schema thinkthen_context_schema,
    options thinkthen_option[], levels thinkthen_option[], labels thinkthen_option[],
    model text, mode text, snippet_pieces bigint, relation_threshold text,
    threshold text, context text, boundary_context text, kind_edge_context text,
    relation_context text, field text[], context_field text, options_field text,
    seed_spans thinkthen_seed[], examples thinkthen_example[], seed_spans_field text,
    examples_field text, batch text, deadline_ms bigint, max_requests_total bigint,
    attempts boolean, none boolean, top bigint, files_only boolean, incremental boolean, cancelled boolean
);
"#,
    name = "native_reading_type",
    requires = ["native_record_types"]
);

pub(super) fn read(
    tuple: Option<pgrx::composite_type!("thinkthen_reading")>,
) -> Result<RequestOptions, Error> {
    let Some(tuple) = tuple else {
        return Ok(RequestOptions::default());
    };
    let threshold =
        |name| field::<String>(&tuple, name).map(|value| value.map(RequestThreshold::Rule));
    let strings = |name| {
        field::<Array<'_, &str>>(&tuple, name)?
            .map(|values| {
                values
                    .iter()
                    .map(|value| {
                        value
                            .map(str::to_owned)
                            .ok_or_else(|| call::usage(format!("{name} list contains NULL")))
                    })
                    .collect::<Result<Vec<_>, Error>>()
            })
            .transpose()
    };
    let mode = field::<String>(&tuple, "mode")?
        .map(|value| match value.as_str() {
            "whole" => Ok(thinkthen::RecognitionMode::Whole),
            "boundary_only" => Ok(thinkthen::RecognitionMode::BoundaryOnly),
            _ => Err(call::usage("recognition mode is whole or boundary_only")),
        })
        .transpose()?;
    let batch = field::<String>(&tuple, "batch")?
        .map(|value| {
            if value == "max" {
                Ok(RequestBatch::Named(value))
            } else {
                value
                    .parse()
                    .map(RequestBatch::Count)
                    .map_err(|_| call::usage("batch is a positive count or max"))
            }
        })
        .transpose()?;
    Ok(RequestOptions {
        model: field(&tuple, "model")?,
        mode,
        snippet_pieces: field::<i64>(&tuple, "snippet_pieces")?
            .map(|value| {
                u32::try_from(value).map_err(|_| call::usage("snippet pieces is an unsigned count"))
            })
            .transpose()?,
        relation_threshold: threshold("relation_threshold")?,
        threshold: threshold("threshold")?,
        context: field(&tuple, "context")?,
        stage_context: Some(thinkthen::RecognitionStageContext {
            boundary: field(&tuple, "boundary_context")?,
            kind_edge: field(&tuple, "kind_edge_context")?,
            relation: field(&tuple, "relation_context")?,
        })
        .filter(|value| !value.is_empty()),
        field: strings("field")?,
        context_field: field(&tuple, "context_field")?,
        options_field: field(&tuple, "options_field")?,
        seed_spans: super::input::seeds(field(&tuple, "seed_spans")?)?,
        examples: super::input::examples(field(&tuple, "examples")?)?,
        seed_spans_field: field(&tuple, "seed_spans_field")?,
        examples_field: field(&tuple, "examples_field")?,
        batch,
        deadline_ms: field(&tuple, "deadline_ms")?,
        max_requests_total: field::<i64>(&tuple, "max_requests_total")?
            .map(|value| {
                u64::try_from(value).map_err(|_| call::usage("request total cannot be negative"))
            })
            .transpose()?,
        attempts: field(&tuple, "attempts")?.unwrap_or(false),
        none: field(&tuple, "none")?.unwrap_or(false),
        top: field::<i64>(&tuple, "top")?.map(number).transpose()?,
        files_only: field(&tuple, "files_only")?.unwrap_or(false),
    })
}

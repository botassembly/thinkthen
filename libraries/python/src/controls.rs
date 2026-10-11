//! Convert explicit caller controls; native admission owns their meaning.
use crate::{inputs::decode, usage};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use thinkthen::RequestOptions;
pub(crate) fn options(py: Python<'_>, value: &Bound<'_, PyAny>) -> PyResult<RequestOptions> {
    let values = value.cast::<PyDict>()?;
    let mut options = RequestOptions::default();
    for (key, value) in values.iter() {
        match key.extract::<String>()?.as_str() {
            "model" => options.model = Some(value.extract()?),
            "context" => options.context = Some(value.extract()?),
            "field" => options.field = Some(value.extract()?),
            "context_field" => options.context_field = Some(value.extract()?),
            "options_field" => options.options_field = Some(value.extract()?),
            "seed_spans_field" => options.seed_spans_field = Some(value.extract()?),
            "examples_field" => options.examples_field = Some(value.extract()?),
            "snippet_pieces" => options.snippet_pieces = Some(value.extract()?),
            "deadline_ms" => options.deadline_ms = Some(value.extract()?),
            "max_requests_total" => options.max_requests_total = Some(value.extract()?),
            "top" => options.top = Some(value.extract()?),
            "attempts" => options.attempts = value.extract()?,
            "details" => options.details = value.extract()?,
            "none" => options.none = value.extract()?,
            "files_only" => options.files_only = value.extract()?,
            "mode" => options.mode = Some(decode(py, &value)?),
            "threshold" => options.threshold = Some(decode(py, &value)?),
            "relation_threshold" => options.relation_threshold = Some(decode(py, &value)?),
            "batch" => options.batch = Some(decode(py, &value)?),
            "stage_context" => options.stage_context = Some(decode(py, &value)?),
            "seed_spans" => options.seed_spans = Some(decode(py, &value)?),
            "examples" => options.examples = Some(decode(py, &value)?),
            "instructions" | "entity_definition" | "descriptions" => {
                return Err(usage(
                    py,
                    "recognize task keywords take an inline declaration",
                ));
            }
            _ => return Err(usage(py, "invalid canonical request")),
        }
    }
    Ok(options)
}

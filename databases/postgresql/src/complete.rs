//! Native complete SQL calls keep file privilege checks on the backend thread.
use crate::{call, files};
use pgrx::datum::Json;
use pgrx::prelude::*;
use thinkthen::Surface;

#[path = "../../sqlite/src/complete/request.rs"]
mod request;

fn invoke(
    verb: &'static str,
    question: Option<&str>,
    inputs: Option<&str>,
    settings: Option<&str>,
) -> Option<Json> {
    let (question, inputs) = (question?, inputs?);
    let prepared = (|| {
        let saved = if question.starts_with('@') {
            let (reference, source) =
                files::read_resolved_question(question, call::file_directory().as_deref())?;
            let parsed = crate::complete_native::settings::parse_file(verb, &source, &reference)?;
            Some((parsed, source))
        } else {
            None
        };
        let settings = thinkthen::Settings::parse(settings.unwrap_or("{}"))
            .map_err(|e| call::usage(e.to_string()))?;
        let prepared = match saved {
            Some((parsed, source)) => {
                crate::complete_native::settings::prepare_file(verb, &source, &settings, parsed)?
            }
            None => crate::complete_native::prepare(verb, question, &settings)?,
        };
        // Retain PostgreSQL's client-reader restriction before enabling deferred native inputs.
        crate::complete_native::Inputs::parse(inputs, false)?;
        let inputs = crate::complete_native::Inputs::parse_request(inputs)?;
        let request = request::admit(&prepared)?;
        let call = call::read_result()?.with_settings(&settings)?;
        Ok::<_, thinkthen::Error>((prepared, inputs, request, call))
    })();
    let result = match prepared {
        Err(error) => crate::complete_native::admission(&error),
        Ok((prepared, inputs, request, call)) => {
            match call::run_result(call, move |engine, options| {
                Ok(request::run(
                    engine,
                    &prepared,
                    &request,
                    inputs,
                    options,
                    Surface::Postgresql,
                ))
            }) {
                Ok(value) => value,
                Err(error) => crate::complete_native::admission(&error),
            }
        }
    };
    Some(Json(result))
}

#[pg_extern(parallel_restricted)]
fn thinkthen_decide_complete(
    question: Option<&str>,
    inputs: Option<&str>,
    settings: default!(Option<&str>, "NULL"),
) -> Option<Json> {
    call::guarded(|| invoke("decide", question, inputs, settings))
}

#[pg_extern(parallel_restricted)]
fn thinkthen_choose_complete(
    question: Option<&str>,
    inputs: Option<&str>,
    settings: default!(Option<&str>, "NULL"),
) -> Option<Json> {
    call::guarded(|| invoke("choose", question, inputs, settings))
}

#[pg_extern(parallel_restricted)]
fn thinkthen_tag_complete(
    question: Option<&str>,
    inputs: Option<&str>,
    settings: default!(Option<&str>, "NULL"),
) -> Option<Json> {
    call::guarded(|| invoke("tag", question, inputs, settings))
}

#[pg_extern(parallel_restricted)]
fn thinkthen_score_complete(
    question: Option<&str>,
    inputs: Option<&str>,
    settings: default!(Option<&str>, "NULL"),
) -> Option<Json> {
    call::guarded(|| invoke("score", question, inputs, settings))
}

#[pg_extern(parallel_restricted)]
fn thinkthen_filter_complete(
    question: Option<&str>,
    inputs: Option<&str>,
    settings: default!(Option<&str>, "NULL"),
) -> Option<Json> {
    call::guarded(|| invoke("filter", question, inputs, settings))
}

#[pg_extern(parallel_restricted)]
fn thinkthen_rank_complete(
    question: Option<&str>,
    inputs: Option<&str>,
    settings: default!(Option<&str>, "NULL"),
) -> Option<Json> {
    call::guarded(|| invoke("rank", question, inputs, settings))
}

#[pg_extern(parallel_restricted)]
fn thinkthen_find_complete(
    question: Option<&str>,
    inputs: Option<&str>,
    settings: default!(Option<&str>, "NULL"),
) -> Option<Json> {
    call::guarded(|| invoke("find", question, inputs, settings))
}

#[pg_extern(parallel_restricted)]
fn thinkthen_annotate_complete(
    question: Option<&str>,
    inputs: Option<&str>,
    settings: default!(Option<&str>, "NULL"),
) -> Option<Json> {
    call::guarded(|| invoke("annotate", question, inputs, settings))
}

#[pg_extern(parallel_restricted)]
fn thinkthen_recognize_complete(
    question: Option<&str>,
    inputs: Option<&str>,
    settings: default!(Option<&str>, "NULL"),
) -> Option<Json> {
    call::guarded(|| invoke("recognize", question, inputs, settings))
}

#[pg_extern(parallel_restricted)]
fn thinkthen_relate_complete(
    question: Option<&str>,
    inputs: Option<&str>,
    settings: default!(Option<&str>, "NULL"),
) -> Option<Json> {
    call::guarded(|| invoke("relate", question, inputs, settings))
}

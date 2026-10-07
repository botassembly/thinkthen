//! Native complete SQL calls keep file privilege checks on the backend thread.
use crate::{call, files};
use pgrx::datum::Json;
use pgrx::prelude::*;
use thinkthen::Surface;

fn invoke(
    verb: &'static str,
    question: Option<&str>,
    inputs: Option<&str>,
    settings: Option<&str>,
) -> Option<Json> {
    call::guarded(|| {
        let (question, inputs) = (question?, inputs?);
        let prepared = (|| {
            let source = match question.strip_prefix('@') {
                Some(path) => {
                    files::read_named("question", path, call::file_directory().as_deref())?
                }
                None => question.to_owned(),
            };
            let settings = thinkthen::Settings::parse(settings.unwrap_or("{}"))
                .map_err(|e| call::usage(e.to_string()))?;
            let prepared = if question.starts_with('@') {
                crate::complete_native::settings::prepare_file(verb, &source, &settings)?
            } else {
                crate::complete_native::prepare(verb, &source, &settings)?
            };
            let inputs = crate::complete_native::Inputs::parse(inputs, false)?;
            let call = call::read_result()?.with_settings(&settings)?;
            Ok::<_, thinkthen::Error>((prepared, inputs, call))
        })();
        let result = match prepared {
            Err(error) => crate::complete_native::failure(&error),
            Ok((prepared, inputs, call)) => match call::run_result(call, move |engine, options| {
                Ok(crate::complete_native::run(
                    engine,
                    &prepared,
                    inputs,
                    options,
                    Surface::Postgresql,
                ))
            }) {
                Ok(value) => value,
                Err(error) => crate::complete_native::failure(&error),
            },
        };
        Some(Json(result))
    })
}

#[pg_extern(parallel_restricted)]
fn thinkthen_decide_complete(
    question: Option<&str>,
    inputs: Option<&str>,
    settings: default!(Option<&str>, "NULL"),
) -> Option<Json> {
    invoke("decide", question, inputs, settings)
}

#[pg_extern(parallel_restricted)]
fn thinkthen_choose_complete(
    question: Option<&str>,
    inputs: Option<&str>,
    settings: default!(Option<&str>, "NULL"),
) -> Option<Json> {
    invoke("choose", question, inputs, settings)
}

#[pg_extern(parallel_restricted)]
fn thinkthen_tag_complete(
    question: Option<&str>,
    inputs: Option<&str>,
    settings: default!(Option<&str>, "NULL"),
) -> Option<Json> {
    invoke("tag", question, inputs, settings)
}

#[pg_extern(parallel_restricted)]
fn thinkthen_score_complete(
    question: Option<&str>,
    inputs: Option<&str>,
    settings: default!(Option<&str>, "NULL"),
) -> Option<Json> {
    invoke("score", question, inputs, settings)
}

#[pg_extern(parallel_restricted)]
fn thinkthen_filter_complete(
    question: Option<&str>,
    inputs: Option<&str>,
    settings: default!(Option<&str>, "NULL"),
) -> Option<Json> {
    invoke("filter", question, inputs, settings)
}

#[pg_extern(parallel_restricted)]
fn thinkthen_rank_complete(
    question: Option<&str>,
    inputs: Option<&str>,
    settings: default!(Option<&str>, "NULL"),
) -> Option<Json> {
    invoke("rank", question, inputs, settings)
}

#[pg_extern(parallel_restricted)]
fn thinkthen_find_complete(
    question: Option<&str>,
    inputs: Option<&str>,
    settings: default!(Option<&str>, "NULL"),
) -> Option<Json> {
    invoke("find", question, inputs, settings)
}

#[pg_extern(parallel_restricted)]
fn thinkthen_annotate_complete(
    question: Option<&str>,
    inputs: Option<&str>,
    settings: default!(Option<&str>, "NULL"),
) -> Option<Json> {
    invoke("annotate", question, inputs, settings)
}

#[pg_extern(parallel_restricted)]
fn thinkthen_recognize_complete(
    question: Option<&str>,
    inputs: Option<&str>,
    settings: default!(Option<&str>, "NULL"),
) -> Option<Json> {
    invoke("recognize", question, inputs, settings)
}

#[pg_extern(parallel_restricted)]
fn thinkthen_relate_complete(
    question: Option<&str>,
    inputs: Option<&str>,
    settings: default!(Option<&str>, "NULL"),
) -> Option<Json> {
    invoke("relate", question, inputs, settings)
}

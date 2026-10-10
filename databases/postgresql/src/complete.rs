//! Native complete SQL calls keep file privilege checks on the backend thread.
use crate::{call, files};
use pgrx::datum::{Array, Json};
use pgrx::prelude::*;
use thinkthen::Surface;

use thinkthen_host::sql_request as request;

fn invoke(
    verb: &'static str,
    question: Option<&str>,
    inputs: Option<&str>,
    settings: Option<&str>,
) -> Option<Json> {
    let (question, inputs) = (question?, inputs?);
    execute(verb, question, settings, || {
        // Retain PostgreSQL's client-reader restriction for text descriptors.
        crate::complete_native::Inputs::parse(inputs, false)?;
        crate::complete_native::Inputs::parse_request(inputs)
    })
}

fn execute(
    verb: &'static str,
    question: &str,
    settings: Option<&str>,
    inputs: impl FnOnce() -> Result<crate::complete_native::Inputs, thinkthen::Error>,
) -> Option<Json> {
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
        let inputs = inputs()?;
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

crate::descriptions::describe! {
["Run native decide ", crate::descriptions::COMPLETE_TEXT].concat();
[parallel_restricted];
fn thinkthen_decide_complete(
    question: Option<&str>,
    inputs: Option<&str>,
    settings: default!(Option<&str>, "NULL"),
) -> Option<Json> {
    call::guarded(|| invoke("decide", question, inputs, settings))
}
}

crate::descriptions::describe! {
["Run native choose ", crate::descriptions::COMPLETE_TEXT].concat();
[parallel_restricted];
fn thinkthen_choose_complete(
    question: Option<&str>,
    inputs: Option<&str>,
    settings: default!(Option<&str>, "NULL"),
) -> Option<Json> {
    call::guarded(|| invoke("choose", question, inputs, settings))
}
}

crate::descriptions::describe! {
["Run native tag ", crate::descriptions::COMPLETE_TEXT].concat();
[parallel_restricted];
fn thinkthen_tag_complete(
    question: Option<&str>,
    inputs: Option<&str>,
    settings: default!(Option<&str>, "NULL"),
) -> Option<Json> {
    call::guarded(|| invoke("tag", question, inputs, settings))
}
}

crate::descriptions::describe! {
["Run native score ", crate::descriptions::COMPLETE_TEXT].concat();
[parallel_restricted];
fn thinkthen_score_complete(
    question: Option<&str>,
    inputs: Option<&str>,
    settings: default!(Option<&str>, "NULL"),
) -> Option<Json> {
    call::guarded(|| invoke("score", question, inputs, settings))
}
}

crate::descriptions::describe! {
["Run native filter ", crate::descriptions::COMPLETE_TEXT].concat();
[parallel_restricted];
fn thinkthen_filter_complete(
    question: Option<&str>,
    inputs: Option<&str>,
    settings: default!(Option<&str>, "NULL"),
) -> Option<Json> {
    call::guarded(|| invoke("filter", question, inputs, settings))
}
}

crate::descriptions::describe! {
["Run native rank ", crate::descriptions::COMPLETE_TEXT].concat();
[parallel_restricted];
fn thinkthen_rank_complete(
    question: Option<&str>,
    inputs: Option<&str>,
    settings: default!(Option<&str>, "NULL"),
) -> Option<Json> {
    call::guarded(|| invoke("rank", question, inputs, settings))
}
}

crate::descriptions::describe! {
["Run native find ", crate::descriptions::COMPLETE_TEXT].concat();
[parallel_restricted];
fn thinkthen_find_complete(
    question: Option<&str>,
    inputs: Option<&str>,
    settings: default!(Option<&str>, "NULL"),
) -> Option<Json> {
    call::guarded(|| invoke("find", question, inputs, settings))
}
}

crate::descriptions::describe! {
["Run native annotate ", crate::descriptions::COMPLETE_TEXT].concat();
[parallel_restricted];
fn thinkthen_annotate_complete(
    question: Option<&str>,
    inputs: Option<&str>,
    settings: default!(Option<&str>, "NULL"),
) -> Option<Json> {
    call::guarded(|| invoke("annotate", question, inputs, settings))
}
}

crate::descriptions::describe! {
["Run native recognize ", crate::descriptions::COMPLETE_TEXT].concat();
[parallel_restricted];
fn thinkthen_recognize_complete(
    question: Option<&str>,
    inputs: Option<&str>,
    settings: default!(Option<&str>, "NULL"),
) -> Option<Json> {
    call::guarded(|| invoke("recognize", question, inputs, settings))
}
}

crate::descriptions::describe! {
["Run native relate ", crate::descriptions::COMPLETE_TEXT].concat();
[parallel_restricted];
fn thinkthen_relate_complete(
    question: Option<&str>,
    inputs: Option<&str>,
    settings: default!(Option<&str>, "NULL"),
) -> Option<Json> {
    call::guarded(|| invoke("relate", question, inputs, settings))
}
}

crate::descriptions::describe! {
"Run native decide on one ordered native bytea image collection and return the complete result/2 envelope as PostgreSQL json, including final facts and started failures. NULL question or image collection returns NULL before inspecting partners; NULL settings uses defaults. Questions support native catalog references and privileged/confined @files. Evidence and image files must be read by the client. Admission and backend failures return error envelopes.";
[parallel_restricted, requires = ["image_value_type"]];
/// Complete result for one ordered native bytea image collection.
fn thinkthen_decide_images_complete(
    question: Option<&str>,
    images: Option<Array<'_, pgrx::composite_type!("thinkthen_image_value")>>,
    settings: default!(Option<&str>, "NULL"),
) -> Option<Json> {
    call::guarded(|| {
        let (question, images) = (question?, images?);
        execute("decide", question, settings, || {
            crate::images::input(images, None).map(crate::complete_native::Inputs::from_record)
        })
    })
}
}

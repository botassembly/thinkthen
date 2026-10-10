//! Named complete SQL doors preserve native evaluation and error ownership.
use crate::catalog::Catalog;
use crate::{ffi, guard, question, worker};
use rusqlite::functions::{Context, FunctionFlags};
use thinkthen::{Error, ErrorKind, Surface};
mod request;
mod selector;

fn prepare(
    verb: &str,
    source: &str,
    settings: &thinkthen::Settings,
) -> Result<crate::complete_native::Prepared, Error> {
    if !source.starts_with('@') {
        return crate::complete_native::prepare(verb, source, settings);
    }
    let reference = if let Some(name) = source.strip_prefix("@@") {
        thinkthen::QuestionFileReference::named(name)?
    } else {
        thinkthen::QuestionFileReference::reference(source)?
    };
    let text = question::resolved_file(source, &reference)
        .map_err(|failure| Error::new(failure.kind, failure.message))?;
    let parsed = crate::complete_native::settings::parse_file(verb, &text, &reference)?;
    crate::complete_native::settings::prepare_file(verb, &text, settings, parsed)
}

fn invoke(context: &Context<'_>, verb: &'static str) -> rusqlite::Result<Option<String>> {
    let result = guard("a complete SQL call", || {
        if (0..2).any(|slot| matches!(context.get_raw(slot), rusqlite::types::ValueRef::Null)) {
            return Ok(None);
        }
        let Some(question) = question::text(context.get_raw(0), "the question")? else {
            return Ok(None);
        };
        let inputs = match context.get_raw(1) {
            rusqlite::types::ValueRef::Blob(bytes) if verb == "decide" => {
                crate::complete_native::Inputs::from_record(crate::images::complete_record(
                    bytes, None,
                )?)
            }
            value => {
                let Some(source) = question::text(value, "complete inputs")? else {
                    return Ok(None);
                };
                crate::complete_native::Inputs::parse_request(&source)
                    .map_err(crate::Failure::from)?
            }
        };
        let settings = if context.len() > 2 {
            question::call_settings(context.get_raw(2))?
        } else {
            thinkthen::Settings::default()
        };
        let result = (|| {
            let selector = selector::admit(verb, &question)?;
            let prepared = prepare(verb, &question, &settings)?;
            let request = match selector {
                Some(selector) => {
                    selector.with_resolved_definition(request::definition(&prepared))?
                }
                None => request::admit(&prepared)?,
            };
            Ok::<_, thinkthen::Error>((prepared, inputs, request))
        })();
        let value = match result {
            Err(error) => crate::complete_native::admission(&error),
            Ok((prepared, inputs, request)) => {
                let shared = settings.context().map(str::to_owned);
                worker::run_settings(ffi::handle_of(context), settings, move |engine, options| {
                    let options = shared
                        .as_deref()
                        .map_or(options, |context| options.context(context));
                    Ok(request::run(
                        engine,
                        &prepared,
                        &request,
                        inputs,
                        options,
                        Surface::Sqlite,
                    ))
                })?
            }
        };
        Ok(Some(value.to_string()))
    });
    result.or_else(|failure| {
        if matches!(failure.kind, ErrorKind::Cancelled | ErrorKind::Deadline) {
            return Err(failure.into());
        }
        Ok(Some(
            crate::complete_native::admission(&Error::new(failure.kind, &failure.message))
                .to_string(),
        ))
    })
}
pub(crate) fn register(connection: &Catalog<'_>) -> rusqlite::Result<()> {
    for verb in [
        "decide",
        "choose",
        "tag",
        "score",
        "filter",
        "rank",
        "find",
        "annotate",
        "recognize",
        "relate",
    ] {
        let name = format!("thinkthen_{verb}_complete");
        for arity in [2, 3] {
            connection.create_scalar_function(
                name.as_str(),
                arity,
                FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DIRECTONLY,
                &format!(
                    "Run {verb} over native inputs and return complete result envelopes as JSON."
                ),
                move |context| invoke(context, verb),
            )?;
        }
    }
    Ok(())
}

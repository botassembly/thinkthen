//! Named complete SQL doors preserve native evaluation and error ownership.
use crate::{ffi, guard, question, worker};
use rusqlite::{
    Connection,
    functions::{Context, FunctionFlags},
};
use thinkthen::{Error, ErrorKind, Surface};
mod request;

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
        let Some(question) = question::text(context.get_raw(0), "the question")? else {
            return Ok(None);
        };
        let Some(inputs) = question::text(context.get_raw(1), "complete inputs")? else {
            return Ok(None);
        };
        let settings = if context.len() > 2 {
            question::call_settings(context.get_raw(2))?
        } else {
            thinkthen::Settings::default()
        };
        let result = (|| {
            let inputs = crate::complete_native::Inputs::parse_request(&inputs)?;
            let prepared = prepare(verb, &question, &settings)?;
            let request = request::admit(&prepared)?;
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
pub(crate) fn register(connection: &Connection) -> rusqlite::Result<()> {
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
                move |context| invoke(context, verb),
            )?;
        }
    }
    Ok(())
}

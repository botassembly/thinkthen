//! Named complete SQL doors preserve native evaluation and error ownership.
use crate::{ffi, guard, question, worker};
use rusqlite::{
    Connection,
    functions::{Context, FunctionFlags},
};
use thinkthen::Surface;

fn invoke(context: &Context<'_>, verb: &'static str) -> rusqlite::Result<Option<String>> {
    Ok(guard("a complete SQL call", || {
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
            let prepared = crate::complete_native::prepare(verb, &question, &settings)?;
            let inputs = crate::complete_native::Inputs::parse(&inputs, true)?;
            Ok::<_, thinkthen::Error>((prepared, inputs))
        })();
        let value = match result {
            Err(error) => crate::complete_native::failure(&error),
            Ok((prepared, inputs)) => {
                let shared = settings.context().map(str::to_owned);
                worker::run_settings(ffi::handle_of(context), settings, move |engine, options| {
                    let options = shared
                        .as_deref()
                        .map_or(options, |context| options.context(context));
                    Ok(crate::complete_native::run(
                        engine,
                        &prepared,
                        inputs,
                        options,
                        Surface::Sqlite,
                    ))
                })?
            }
        };
        Ok(Some(value.to_string()))
    })?)
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

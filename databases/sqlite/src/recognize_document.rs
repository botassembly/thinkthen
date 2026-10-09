//! The complete recognition result for one SQL text and question file.

use rusqlite::functions::Context;
use thinkthen::Recognize;

use crate::question::from_file;
use crate::question::text;
use crate::tables::ordered_file_json;
use crate::{ffi, guard, worker};

/// Return the public recognition JSON without dropping its relation edges.
pub(crate) fn recognize_document(context: &Context<'_>) -> rusqlite::Result<Option<String>> {
    Ok(guard("thinkthen_recognize_document", || {
        let Some(evidence) = text(context.get_raw(0), "the text")? else {
            return Ok(None);
        };
        let Some(argument) = text(context.get_raw(1), "the recognize spec")? else {
            return Ok(None);
        };
        let (whole, file) = ordered_file_json(&argument, "recognize")?;
        let ask = Recognize::from_json(&whole).map_err(|error| from_file(error, file))?;
        let result = worker::run(ffi::handle_of(context), None, move |engine, options| {
            let call = crate::request::run(
                engine,
                thinkthen::RequestFunction::Recognize,
                ask.into(),
                vec![crate::request::text(evidence)],
                options,
            )?;
            match call.into_value() {
                thinkthen::RequestValue::Recognized(rows) => rows
                    .into_iter()
                    .next()
                    .map(|row| row.result().value().clone())
                    .ok_or_else(|| crate::request::wrong_result().into()),
                _ => Err(crate::request::wrong_result().into()),
            }
        })?;
        Ok(Some(result.to_json()))
    })?)
}

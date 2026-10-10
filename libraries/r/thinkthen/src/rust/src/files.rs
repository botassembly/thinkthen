//! Explicit located calls using the shared native source dispatch.
use crate::calls::render;
use crate::calls::{Crossed, Pending, call_owned};
use extendr_api::prelude::*;

use thinkthen_host::source;

pub(crate) fn execute(
    question: String,
    selection: String,
    deadline: Option<f64>,
    pending: Pending<'_>,
) -> Crossed<List> {
    let completed = call_owned(
        deadline,
        pending,
        None,
        None,
        move |engine, options, account| {
            let started = account.start();
            let (value, facts) = source::dispatch(engine, &question, &selection, options)
                .map_err(|e| crate::carry(&e))?;
            account.include(started, &facts)?;
            serde_json::value::RawValue::from_string(value)
                .map_err(|_| crate::defect("located answer could not be written"))
        },
    )?;
    render::envelope(completed)
}

pub(crate) fn from_robj(
    question: Robj,
    selection: Robj,
    deadline: Robj,
    pending: Pending<'_>,
) -> Crossed<List> {
    execute(
        crate::ffi::text_of(&question, "question")?,
        crate::ffi::text_of(&selection, "source")?,
        crate::ffi::deadline_of(&deadline)?,
        pending,
    )
}

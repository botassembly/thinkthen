//! Explicit located calls using the shared native source dispatch.
use extendr_api::prelude::*;
use crate::calls::{Crossed, Pending, call_owned};
use crate::calls::render;

#[path = "../../../../../shared/source.rs"]
mod source;

pub(crate) fn execute(question: String, selection: String, deadline: Option<f64>, pending: Pending<'_>) -> Crossed<List> {
    let completed = call_owned(deadline, pending, None, None, move |engine, options, account| {
        let started = account.start();
        let (value, facts) = source::dispatch(engine, &question, &selection, options).map_err(|e| crate::carry(&e))?;
        account.include(started, &facts)?;
        serde_json::value::RawValue::from_string(value).map_err(|_| crate::defect("located answer could not be written"))
    })?;
    render::envelope(completed)
}

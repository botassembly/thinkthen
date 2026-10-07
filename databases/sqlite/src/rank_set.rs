//! Additive keyed turns rank, using the native set admission and one worker.
use rusqlite::ffi::sqlite3;
use rusqlite::types::{Value, ValueRef};

use crate::question::{call_controls, rank_set};
use crate::{Failure, many, worker};

pub(crate) fn ranked(
    db: *mut sqlite3,
    question: &str,
    packed: &str,
    settings: ValueRef<'_>,
) -> Result<Vec<Vec<Value>>, Failure> {
    let controls = call_controls(settings)?;
    let asked = rank_set(question)?;
    let records = many::keyed(packed)?;
    if records.is_empty() {
        return Ok(Vec::new());
    }
    let (keys, texts): (Vec<_>, Vec<_>) = records.into_iter().unzip();
    let context = controls.context().map(str::to_owned);
    worker::run_settings(db, controls, move |engine, options| {
        let options = match &context {
            Some(context) => options.context(context),
            None => options,
        };
        let call = engine.rank_set_with(&asked, texts, options)?;
        let facts = serde_json::to_string(call.facts())
            .map_err(|_| Failure::defect("rank facts could not be encoded"))?;
        call.value()
            .iter()
            .zip(1_i64..)
            .map(|(row, place)| {
                let key = keys
                    .get(row.index())
                    .ok_or_else(|| Failure::defect("a ranked row lost its record"))?;
                Ok(vec![
                    Value::Text(key.clone()),
                    Value::Integer(place),
                    Value::Real(row.probability()),
                    Value::Text(row.question_name().to_owned()),
                    Value::Text(facts.clone()),
                ])
            })
            .collect()
    })
}

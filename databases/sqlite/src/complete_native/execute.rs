//! One native invocation per SQL complete call; envelopes come from that owner.
use super::{Inputs, Prepared, defect, usage};
use serde_json::Value;
use thinkthen::{CallOptions, Engine, Error, LoadedQuestion};
macro_rules! rows {
    ($call:expr) => {{
        let call = $call?;
        let ordinals = call
            .value()
            .iter()
            .map(|row| row.ordinal())
            .collect::<Vec<_>>();
        let mut value =
            serde_json::to_value(call.complete().ok_or_else(defect)?).map_err(|_| defect())?;
        super::put(&mut value, "ordinals", serde_json::json!(ordinals))?;
        Ok(value)
    }};
}
macro_rules! envelope {
    ($call:expr) => {{
        let call = $call?;
        serde_json::to_value(call.complete().ok_or_else(defect)?).map_err(|_| defect())
    }};
}
pub(super) fn run(
    engine: &Engine,
    prepared: &Prepared,
    inputs: Inputs,
    options: CallOptions<'_>,
) -> Result<Value, Error> {
    let records = inputs.records(prepared.reading())?;
    if inputs.incremental {
        return streaming(engine, prepared, records, options);
    }
    let records = records.collect::<Result<Vec<_>, _>>()?;
    match prepared {
        Prepared::Atomic(LoadedQuestion::Banded(q)) => {
            rows!(engine.decide_records_complete_with(q, records, options))
        }
        Prepared::Atomic(LoadedQuestion::Question(q)) => match q.kind() {
            thinkthen::QuestionKind::Decide => {
                rows!(engine.decide_records_complete_with(q, records, options))
            }
            thinkthen::QuestionKind::Choose => {
                rows!(engine.choose_records_complete_with(q, records, options))
            }
            thinkthen::QuestionKind::Tag => {
                rows!(engine.tag_records_complete_with(q, records, options))
            }
            thinkthen::QuestionKind::Score => {
                rows!(engine.score_records_complete_with(q, records, options))
            }
            _ => Err(usage("wrong complete question kind")),
        },
        Prepared::Dynamic(q) => {
            rows!(engine.choose_dynamic_records_complete_with(q, records, options))
        }
        Prepared::Filter(q) => rows!(engine.filter_records_complete_with(q, records, options)),
        Prepared::Rank(q) => rows!(engine.rank_records_complete_with(q, records, options)),
        Prepared::RankSet(q) => rows!(engine.rank_set_records_complete_with(q, records, options)),
        Prepared::Find(q, _) => {
            let call = engine.find_records_complete_with(q, records, options)?;
            let index = match call.value().selection() {
                thinkthen::FindSelection::None => None,
                thinkthen::FindSelection::Unit(at) => Some(at),
            };
            let mut value =
                serde_json::to_value(call.complete().ok_or_else(defect)?).map_err(|_| defect())?;
            super::put(&mut value, "selection", serde_json::json!(index))?;
            Ok(value)
        }
        Prepared::Annotate(q) => rows!(engine.annotate_records_complete_with(q, records, options)),
        Prepared::Recognize(q, _) => {
            rows!(engine.recognize_records_complete_with(q, records, options))
        }
        Prepared::Relate(q) => envelope!(engine.relate_records_complete_with(q, records, options)),
    }
}
fn streaming(
    engine: &Engine,
    prepared: &Prepared,
    records: super::inputs::Records<'_>,
    options: CallOptions<'_>,
) -> Result<Value, Error> {
    macro_rules! stream {
        ($batch:expr) => {{
            let mut batch = $batch;
            let mut completed = Vec::new();
            while let Some(next) = batch.next() {
                match next {
                    Ok(row) => completed.push(row),
                    Err(error) => {
                        let mut value = super::failure(&error);
                        super::put(
                            &mut value,
                            "ordinals",
                            serde_json::json!(
                                completed
                                    .iter()
                                    .map(|row| row.ordinal())
                                    .collect::<Vec<_>>()
                            ),
                        )?;
                        super::put(
                            &mut value,
                            "completed",
                            serde_json::to_value(completed).map_err(|_| defect())?,
                        )?;
                        return Ok(value);
                    }
                }
            }
            let call = batch.into_call()?;
            let mut value =
                serde_json::to_value(call.complete().ok_or_else(defect)?).map_err(|_| defect())?;
            super::put(
                &mut value,
                "ordinals",
                serde_json::json!(
                    completed
                        .iter()
                        .map(|row| row.ordinal())
                        .collect::<Vec<_>>()
                ),
            )?;
            super::put(
                &mut value,
                "value",
                serde_json::to_value(completed).map_err(|_| defect())?,
            )?;
            Ok(value)
        }};
    }
    match prepared {
        Prepared::Atomic(LoadedQuestion::Banded(q)) => {
            stream!(engine.try_decide_records_complete_with(q, records, options))
        }
        Prepared::Atomic(LoadedQuestion::Question(q)) => match q.kind() {
            thinkthen::QuestionKind::Decide => {
                stream!(engine.try_decide_records_complete_with(q, records, options))
            }
            thinkthen::QuestionKind::Choose => {
                stream!(engine.try_choose_records_complete_with(q, records, options))
            }
            thinkthen::QuestionKind::Tag => {
                stream!(engine.try_tag_records_complete_with(q, records, options))
            }
            thinkthen::QuestionKind::Score => {
                stream!(engine.try_score_records_complete_with(q, records, options))
            }
            _ => Err(usage("wrong incremental question kind")),
        },
        Prepared::Filter(q) => {
            stream!(engine.try_filter_records_complete_with(q, records, options))
        }
        Prepared::Annotate(q) => {
            stream!(engine.try_annotate_records_complete_with(q, records, options))
        }
        _ => Err(usage(
            "incremental complete inputs require an atomic or annotate question",
        )),
    }
}

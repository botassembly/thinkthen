//! Ten concrete native complete methods over one resolved engine.
use super::{
    admission::Invocation,
    dispatch::PreparedQuestion,
    output::NativeObject,
    runtime::{Executor, NativeReply},
    tools::Tool,
};
use crate::{Call, CancelToken, Engine, Error, LoadedQuestion, QuestionInput};
use serde::Serialize;
use serde_json::Value;

pub(super) struct NativeExecutor {
    pub(super) engine: Engine,
    pub(super) schema: Value,
}
impl Executor for NativeExecutor {
    fn output_schema(&self) -> Value {
        self.schema.clone()
    }
    fn execute(&self, invocation: Invocation, token: &CancelToken) -> Result<NativeReply, Error> {
        let token = invocation.call_token(token);
        let controls = invocation.controls(&token)?.started()?;
        controls.admission()?;
        let mut prepared = invocation.question()?;
        apply(&mut prepared, &invocation)?;
        controls.admission()?;
        let mut engine = self.engine.clone();
        if let Some(model) = &invocation.arguments.options.model {
            engine.inner = self.engine.for_model(Some(
                &crate::core::ModelName::new(model).map_err(Error::refused)?,
            ))?;
        }
        let records = invocation.records(&prepared, controls)?;
        let single = invocation.arguments.records.is_none()
            && invocation.arguments.source.is_none()
            && invocation.arguments.inputs.is_none();
        match &prepared {
            PreparedQuestion::Atomic(q) => {
                atomic(&engine, q, &invocation, records, controls, single)
            }
            PreparedQuestion::Rank(q) => {
                let call = engine.try_rank_records_complete_with(q, records, controls)?;
                reply(top(call, invocation.arguments.options.top))
            }
            PreparedQuestion::RankSet(set) => {
                let call = engine.try_rank_set_records_complete_with(set, records, controls)?;
                reply(top(call, invocation.arguments.options.top))
            }
            PreparedQuestion::FindPrepared(q, _) => {
                reply(engine.try_find_records_complete_with(q, records, controls)?)
            }
            PreparedQuestion::RecognizePrepared(ask, _) => rows_reply(
                engine.try_recognize_records_complete_with(ask, records, controls)?,
                single,
                false,
            ),
            PreparedQuestion::Find(file) => {
                reply(engine.try_find_records_complete_with(file.question(), records, controls)?)
            }
            PreparedQuestion::Annotate(set) => {
                let call = if invocation.arguments.records.is_some()
                    || invocation.arguments.inputs.is_some()
                {
                    engine.annotate_records_complete_with(
                        set,
                        records.collect::<Result<Vec<_>, _>>()?,
                        controls,
                    )?
                } else {
                    match engine
                        .try_annotate_records_complete_with(set, records, controls)
                        .into_outcome()
                    {
                        crate::public::batch::Outcome::Complete(call) => call,
                        outcome => return batch_failure(&outcome),
                    }
                };
                let failed = call.value().iter().any(|row| {
                    row.result()
                        .members()
                        .any(|member| member.failure().is_some())
                });
                rows_reply(call, single, failed)
            }
            PreparedQuestion::Recognize(file) => rows_reply(
                engine.try_recognize_records_complete_with(file.question(), records, controls)?,
                single,
                false,
            ),
            PreparedQuestion::Relate(ask) => {
                let call = engine.try_relate_records_complete_with(ask, records, controls)?;
                let failed = call
                    .value()
                    .result()
                    .members()
                    .any(|member| member.failure().is_some());
                reply_failed(call, failed)
            }
            PreparedQuestion::Dynamic(q) => {
                // Native eager candidate admission owns this finite set and its scheduler.
                let records = records.collect::<Result<Vec<_>, _>>()?;
                rows_reply(
                    engine.choose_dynamic_records_complete_with(q, records, controls)?,
                    single,
                    false,
                )
            }
        }
    }
}

fn atomic<'a>(
    engine: &'a Engine,
    q: &'a LoadedQuestion,
    invocation: &'a Invocation,
    records: super::composition::Inputs<'a>,
    controls: crate::CallOptions<'a>,
    single: bool,
) -> Result<NativeReply, Error> {
    if invocation.tool == Tool::Find {
        let LoadedQuestion::Question(q) = q else {
            return Err(Error::usage("find takes no band"));
        };
        return reply(engine.try_find_records_complete_with(q, records, controls)?);
    }
    // Materialized originals require whole-call native admission before lookup.
    macro_rules! complete {
        ($eager:ident, $stream:ident, $question:expr) => {
            if invocation.arguments.records.is_some() || invocation.arguments.inputs.is_some() {
                engine.$eager($question, records.collect::<Result<Vec<_>, _>>()?, controls)?
            } else {
                match engine.$stream($question, records, controls).into_outcome() {
                    crate::public::batch::Outcome::Complete(call) => call,
                    outcome => return batch_failure(&outcome),
                }
            }
        };
    }
    match invocation.tool {
        Tool::Decide => rows_reply(
            complete!(
                decide_records_complete_with,
                try_decide_records_complete_with,
                q
            ),
            single,
            false,
        ),
        Tool::Choose => rows_reply(
            complete!(
                choose_records_complete_with,
                try_choose_records_complete_with,
                q
            ),
            single,
            false,
        ),
        Tool::Tag => rows_reply(
            complete!(tag_records_complete_with, try_tag_records_complete_with, q),
            single,
            false,
        ),
        Tool::Score => rows_reply(
            complete!(
                score_records_complete_with,
                try_score_records_complete_with,
                plain(q)?
            ),
            single,
            false,
        ),
        Tool::Filter => {
            let call = complete!(
                filter_records_complete_with,
                try_filter_records_complete_with,
                plain(q)?
            );
            reply(selected_files(
                call,
                invocation.arguments.options.files_only,
            ))
        }
        _ => Err(Error::defect("prepared question does not match tool")),
    }
}
fn plain(q: &LoadedQuestion) -> Result<&crate::Question, Error> {
    match q {
        LoadedQuestion::Question(q) => Ok(q),
        _ => Err(Error::usage("this function takes no band")),
    }
}
fn rows_reply<T: Serialize>(
    call: Call<Vec<T>>,
    single: bool,
    failed: bool,
) -> Result<NativeReply, Error> {
    if single {
        reply_failed(
            call.try_map(|rows| {
                let mut rows = rows.into_iter();
                let one = rows
                    .next()
                    .ok_or_else(|| Error::defect("one input produced no result"))?;
                if rows.next().is_some() {
                    return Err(Error::defect("one input produced extra results"));
                }
                Ok(one)
            })?,
            failed,
        )
    } else {
        reply_failed(call, failed)
    }
}
fn reply<T: Serialize>(call: Call<T>) -> Result<NativeReply, Error> {
    reply_failed(call, false)
}
fn reply_failed<T: Serialize>(call: Call<T>, failed: bool) -> Result<NativeReply, Error> {
    let complete = call
        .complete()
        .ok_or_else(|| Error::defect("native complete call has no identity"))?;
    let object = NativeObject::new(&complete)
        .map_err(|_| Error::defect("native complete call could not be written"))?;
    Ok(NativeReply { object, failed })
}
fn apply(prepared: &mut PreparedQuestion, invocation: &Invocation) -> Result<(), Error> {
    let options = &invocation.arguments.options;
    let q = match prepared {
        PreparedQuestion::Atomic(LoadedQuestion::Question(q)) | PreparedQuestion::Rank(q) => {
            Some(q)
        }
        PreparedQuestion::Atomic(LoadedQuestion::Banded(q)) => Some(&mut q.0),
        _ => None,
    };
    if let Some(q) = q {
        if let Some(model) = &options.model {
            q.model = Some(crate::core::ModelName::new(model).map_err(Error::refused)?);
        }
        if options.field.is_some() {
            q.metadata.reading.on.clear();
        }
        if let Some(reading) = &options.threshold {
            q.threshold = Some(reading.native()?);
        }
        if invocation.tool == Tool::Find && options.none {
            *q = q.clone().offering_none()?;
        }
    }
    if let PreparedQuestion::Find(file) = prepared {
        let (mut q, reading) = file.clone().into_parts();
        if options.none {
            q = q.offering_none()?;
        }
        if let Some(model) = &options.model {
            q.model = Some(crate::core::ModelName::new(model).map_err(Error::refused)?);
        }
        // Keep the native reading; the concrete file owns private preparation.
        // Reuse its question through dispatch rather than serialize and reparse.
        *prepared = PreparedQuestion::FindPrepared(q, reading);
    }
    if let PreparedQuestion::Recognize(file) = prepared {
        let (mut ask, reading) = file.clone().into_parts();
        if let Some(model) = &options.model {
            ask.0.model = Some(crate::core::ModelName::new(model).map_err(Error::refused)?);
        }
        *prepared = PreparedQuestion::RecognizePrepared(ask, reading);
    }
    if let PreparedQuestion::Relate(ask) = prepared
        && let Some(model) = &options.model
    {
        ask.0.model = Some(crate::core::ModelName::new(model).map_err(Error::refused)?);
    }
    if let PreparedQuestion::Dynamic(q) = prepared {
        if options.field.is_some() {
            *q = q.clone().without_authored_on();
        }
        if let Some(model) = &options.model {
            *q = q.clone().with_model_override(model)?;
        }
        if let Some(threshold) = &options.threshold {
            *q = q.clone().cut_at(threshold.native()?.bounds().0)?;
        }
    }
    Ok(())
}

fn top<T>(call: Call<Vec<T>>, top: Option<usize>) -> Call<Vec<T>> {
    call.map(|mut rows| {
        if let Some(top) = top {
            rows.truncate(top);
        }
        rows
    })
}
fn selected_files(
    call: Call<Vec<crate::CompleteRecord<QuestionInput, crate::CompleteFilter>>>,
    files_only: bool,
) -> Call<Vec<crate::CompleteRecord<QuestionInput, crate::CompleteFilter>>> {
    let mut files = std::collections::BTreeSet::new();
    call.map(|rows| {
        rows.into_iter()
            .filter(|row| {
                if !row.result().value() {
                    return false;
                }
                if !files_only {
                    return true;
                }
                let location = match row.original() {
                    QuestionInput::Record(record) => record.location(),
                    _ => None,
                };
                location.is_some_and(|source| files.insert(source.file().to_owned()))
            })
            .collect()
    })
}

fn batch_failure<T: Serialize>(
    outcome: &crate::public::batch::Outcome<T>,
) -> Result<NativeReply, Error> {
    let object = NativeObject::new(outcome)
        .map_err(|_| Error::defect("native batch outcome could not be written"))?;
    Ok(NativeReply {
        object,
        failed: true,
    })
}

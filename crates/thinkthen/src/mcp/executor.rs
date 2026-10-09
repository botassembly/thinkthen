//! Native Request execution with protocol-specific result presentation.
use super::{
    admission::Invocation,
    output::NativeObject,
    runtime::{Executor, NativeReply},
};
use crate::{Call, CancelToken, Engine, Error, RequestEnvironment, RequestOutcome, RequestValue};
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
        let (request, feed) = invocation.request()?;
        let single = invocation.arguments.records.is_none()
            && invocation.arguments.source.is_none()
            && invocation.arguments.inputs.is_none();
        match self
            .engine
            .execute_request(&request, RequestEnvironment { controls, feed })?
        {
            RequestOutcome::Complete(call) => complete(call, single),
            RequestOutcome::Failed { completed, error } => {
                #[derive(Serialize)]
                struct Prefix<'a> {
                    completed: &'a RequestValue,
                    #[serde(flatten)]
                    failure: crate::public::CompleteError<'a>,
                }
                let empty = match &completed {
                    RequestValue::Decisions(rows) => rows.is_empty(),
                    RequestValue::Choices(rows) => rows.is_empty(),
                    RequestValue::Tags(rows) => rows.is_empty(),
                    RequestValue::Scores(rows) => rows.is_empty(),
                    RequestValue::Filtered(rows) => rows.is_empty(),
                    RequestValue::Annotations(rows) => rows.is_empty(),
                    _ => false,
                };
                let object = if empty {
                    NativeObject::new(&error.complete())
                } else {
                    NativeObject::new(&Prefix {
                        completed: &completed,
                        failure: error.complete(),
                    })
                }
                .map_err(|_| Error::defect("native batch outcome could not be written"))?;
                Ok(NativeReply {
                    object,
                    failed: true,
                })
            }
        }
    }
}
fn complete(call: Call<RequestValue>, single: bool) -> Result<NativeReply, Error> {
    let failed = match call.value() {
        RequestValue::Annotations(rows) => rows.iter().any(|row| {
            row.result()
                .members()
                .any(|member| member.failure().is_some())
        }),
        RequestValue::Related(row) => row
            .result()
            .members()
            .any(|member| member.failure().is_some()),
        _ => false,
    };
    if !single {
        return reply(call, failed);
    }
    macro_rules! row {
        ($rows:expr) => {{
            let mut rows = $rows.into_iter();
            let one = rows
                .next()
                .ok_or_else(|| Error::defect("one input produced no result"))?;
            if rows.next().is_some() {
                return Err(Error::defect("one input produced extra results"));
            }
            Ok(one)
        }};
    }
    // Map the existing typed value without changing native facts or input carriers.
    match call.value() {
        RequestValue::Decisions(_) => reply(
            call.try_map(|v| match v {
                RequestValue::Decisions(rows) => row!(rows),
                _ => Err(Error::defect("result kind changed")),
            })?,
            failed,
        ),
        RequestValue::Choices(_) => reply(
            call.try_map(|v| match v {
                RequestValue::Choices(rows) => row!(rows),
                _ => Err(Error::defect("result kind changed")),
            })?,
            failed,
        ),
        RequestValue::Tags(_) => reply(
            call.try_map(|v| match v {
                RequestValue::Tags(rows) => row!(rows),
                _ => Err(Error::defect("result kind changed")),
            })?,
            failed,
        ),
        RequestValue::Scores(_) => reply(
            call.try_map(|v| match v {
                RequestValue::Scores(rows) => row!(rows),
                _ => Err(Error::defect("result kind changed")),
            })?,
            failed,
        ),
        RequestValue::Annotations(_) => reply(
            call.try_map(|v| match v {
                RequestValue::Annotations(rows) => row!(rows),
                _ => Err(Error::defect("result kind changed")),
            })?,
            failed,
        ),
        RequestValue::Recognized(_) => reply(
            call.try_map(|v| match v {
                RequestValue::Recognized(rows) => row!(rows),
                _ => Err(Error::defect("result kind changed")),
            })?,
            failed,
        ),
        _ => reply(call, failed),
    }
}
fn reply<T: Serialize>(call: Call<T>, failed: bool) -> Result<NativeReply, Error> {
    let complete = call
        .complete()
        .ok_or_else(|| Error::defect("native complete call has no identity"))?;
    Ok(NativeReply {
        object: NativeObject::new(&complete)
            .map_err(|_| Error::defect("native complete call could not be written"))?,
        failed,
    })
}

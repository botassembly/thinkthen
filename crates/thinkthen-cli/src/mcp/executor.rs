//! Native Request execution with protocol-specific result presentation.
use super::{
    admission::Invocation,
    output::NativeObject,
    runtime::{Executor, NativeReply},
};
use crate::{Call, CancelToken, Engine, Error, RequestEnvironment, RequestValue};
use serde::Serialize;

pub(super) struct NativeExecutor {
    pub(super) engine: Engine,
}
impl Executor for NativeExecutor {
    fn execute(&self, invocation: Invocation, token: &CancelToken) -> Result<NativeReply, Error> {
        let token = invocation.call_token(token);
        let controls = invocation
            .controls(&token)?
            .retained_attachment_limit(super::protocol::MAX_MESSAGE)?;
        let (arguments, feed) = invocation.request()?;
        let single = invocation.arguments.records.is_none()
            && invocation.arguments.source.is_none()
            && invocation.arguments.inputs.is_none();
        match self
            .engine
            .call(invocation.tool.name(), arguments, RequestEnvironment { controls, feed })
        {
            Ok(call) => complete(call, single),
            Err(error) => failure(error),
        }
    }
}
fn failure(error: Error) -> Result<NativeReply, Error> {
    let Some(completed) = error.completed() else {
        return Err(error);
    };
    #[derive(Serialize)]
    struct Prefix<'a> {
        completed: &'a RequestValue,
        #[serde(flatten)]
        failure: crate::CompleteError<'a>,
    }
    let empty = match completed {
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
            completed,
            failure: error.complete(),
        })
    }
    .map_err(|_| Error::new(crate::ErrorKind::Defect, "defect: native batch outcome could not be written"))?;
    Ok(NativeReply {
        object,
        failed: true,
    })
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
            match $rows.as_slice() {
                [one] => single_reply(&call, one, failed),
                rows => Err(Error::new(crate::ErrorKind::Defect, if rows.is_empty() {
                    "defect: one input produced no result"
                } else { "defect: one input produced extra results" }).with_facts(call.facts().clone())),
            }
        }};
    }
    // MCP selects one retained complete row for scalar input. The native answer
    // and complete facts remain borrowed from the original completed call.
    match call.value() {
        RequestValue::Decisions(rows) => row!(rows),
        RequestValue::Choices(rows) => row!(rows),
        RequestValue::Tags(rows) => row!(rows),
        RequestValue::Scores(rows) => row!(rows),
        RequestValue::Annotations(rows) => row!(rows),
        RequestValue::Recognized(rows) => row!(rows),
        _ => reply(call, failed),
    }
}
fn single_reply<T: Serialize>(call: &Call<RequestValue>, value: &T, failed: bool) -> Result<NativeReply, Error> {
    #[derive(Serialize)]
    struct Scalar<'a, T> {
        value: &'a T,
        #[serde(skip_serializing_if = "Option::is_none")]
        answer: Option<&'a crate::SettledAnswer>,
        facts: crate::CompleteFacts<'a>,
    }
    let facts = call.facts().complete().ok_or_else(||
        Error::new(crate::ErrorKind::Defect, "defect: a completed call has no identity")
            .with_facts(call.facts().clone()))?;
    Ok(NativeReply {
        object: NativeObject::new(&Scalar { value, answer: call.answer(), facts })
            .map_err(|_| Error::new(crate::ErrorKind::Defect, "defect: native complete call could not be written"))?,
        failed,
    })
}
fn reply<T: Serialize>(call: Call<T>, failed: bool) -> Result<NativeReply, Error> {
    Ok(NativeReply {
        object: NativeObject::new(&call)
            .map_err(|_| Error::new(crate::ErrorKind::Defect, "defect: native complete call could not be written"))?,
        failed,
    })
}

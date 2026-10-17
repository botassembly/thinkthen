//! Canonical admission and native ownership; R only supplies serialized host values.
use super::{Crossed, completion_of, deadline_of, interrupt_pending, text_of};
use crate::calls;
use extendr_api::prelude::*;
use thinkthen::{AdmittedRequest, RequestEnvironment, RequestSession};
mod inputs;

#[extendr]
fn tt_request_build(
    function_name: Robj,
    question: Robj,
    input: Robj,
    options: Robj,
) -> Crossed<Robj> {
    let (request, column) = inputs::request(
        &text_of(&function_name, "function")?,
        &question,
        &input,
        &options,
    )?;
    Ok(ExternalPtr::new(NativeRequest {
        admitted: request.admit().map_err(|e| crate::carry(&e))?,
        column,
    })
    .into())
}

#[derive(Debug)]
struct NativeRequest {
    admitted: AdmittedRequest,
    column: Option<thinkthen::RequestColumn>,
}

struct NativeSession {
    session: RequestSession,
    completion: Option<calls::Completion>,
    function: thinkthen::RequestFunction,
    column: Option<thinkthen::RequestColumn>,
}

#[extendr]
fn tt_request_native(request: Robj, deadline: Robj, completion: Robj) -> Crossed<Robj> {
    let request = ExternalPtr::<NativeRequest>::try_from(request)
        .map_err(|_| crate::usage("an admitted request is required"))?;
    let mut arguments = request.admitted.request().call.arguments().clone();
    if let Some(value) = deadline_of(&deadline)? {
        if value.fract() != 0.0 || !(-1.0..=4_294_967_295_000.0).contains(&value) {
            return Err(crate::usage(
                "deadline_ms is -1, 0, or at most 4294967295000 milliseconds",
            ));
        }
        #[expect(
            clippy::cast_possible_truncation,
            reason = "checked exact millisecond range"
        )]
        let milliseconds = value as i64;
        arguments.options.deadline_ms = Some(milliseconds);
    }
    let native =
        thinkthen::Request::for_function(request.admitted.request().call.function(), arguments);
    let function = native.call.function();
    if interrupt_pending() {
        return Err(crate::interrupted());
    }
    let handle = completion_of(&completion)?
        .unwrap_or_else(|| std::sync::Arc::new(calls::Completion::new()));
    let session = crate::engine()?
        .request_session_with_surface(native, thinkthen::Surface::R)
        .map_err(|e| crate::carry(&e))?;
    handle.start(
        session,
        request
            .column
            .as_ref()
            .map(|column| column.positions().to_vec()),
    );
    while !handle.drain()? {
        if interrupt_pending() {
            handle.cancel();
            return Err(crate::interrupted());
        }
        handle.wait();
    }
    let scalar = matches!(
        request.admitted.request().call.arguments().input,
        thinkthen::RequestInput::Text { .. } | thinkthen::RequestInput::Json { .. }
    );
    let value = handle.result(function, request.column.as_ref(), scalar)?;
    match &request.column {
        Some(column) => crate::request::attach(column, value),
        None => Ok(value),
    }
}

#[extendr]
fn tt_request_plan(request: Robj) -> Crossed<Robj> {
    let request = ExternalPtr::<NativeRequest>::try_from(request)
        .map_err(|_| crate::usage("an admitted request is required"))?;
    let plan = crate::engine()?
        .plan_request(
            &request.admitted,
            RequestEnvironment {
                controls: thinkthen::CallOptions::new().surface(thinkthen::Surface::R),
                feed: None,
            },
        )
        .map_err(|error| crate::carry(&error))?;
    let value =
        crate::native_results::tagged(crate::plan::render(plan)?, "Plan", "thinkthen_complete")?;
    match &request.column {
        Some(column) => crate::request::attach(column, value),
        None => Ok(value),
    }
}

#[extendr]
fn tt_request_batch_start(request: Robj, collect: bool) -> Crossed<Robj> {
    let built = ExternalPtr::<NativeRequest>::try_from(request)
        .map_err(|_| crate::usage("a native request is required"))?;
    let column = built.column.clone();
    let request = built.admitted.request().clone();
    let session = crate::engine()?
        .request_session_with_surface(request, thinkthen::Surface::R)
        .map_err(|error| crate::carry(&error))?;
    let mut pointer: Robj = ExternalPtr::new(NativeSession {
        session,
        column,
        completion: collect.then(calls::Completion::new),
        function: built.admitted.request().call.function(),
    })
    .into();
    let native = ExternalPtr::<NativeSession>::try_from(pointer.clone())
        .map_err(|_| crate::defect("native session pointer could not be retained"))?;
    if let Some(column) = &native.column {
        pointer
            .set_attrib(
                "positions",
                column
                    .positions()
                    .iter()
                    .map(|position| *position as f64)
                    .collect::<Vec<_>>(),
            )
            .and_then(|value| value.set_attrib("length", column.len() as f64))
            .map_err(|_| crate::defect("native column map could not be retained"))?;
    }
    Ok(pointer)
}

#[extendr]
fn tt_request_batch_poll(batch: Robj) -> Crossed<Robj> {
    let session = ExternalPtr::<NativeSession>::try_from(batch)
        .map_err(|_| crate::usage("a native request session is required"))?;
    if interrupt_pending() {
        session.session.cancel();
        return Err(crate::interrupted());
    }
    match session.session.try_read() {
        thinkthen::RequestSessionRead::Result(packet) => {
            let text = packet.to_json().map_err(|error| crate::carry(&error))?;
            if let Some(completion) = &session.completion {
                let document = serde_json::from_str(&text)
                    .map_err(|_| crate::defect("session packet could not be decoded"))?;
                completion.accept(packet, document)?;
            }
            crate::native_results::request_event(&text)
        }
        thinkthen::RequestSessionRead::Pending => Ok(().into()),
        thinkthen::RequestSessionRead::End => Ok(list!(kind = "end").into()),
    }
}

#[extendr]
fn tt_request_batch_result(batch: Robj) -> Crossed<Robj> {
    let session = ExternalPtr::<NativeSession>::try_from(batch)
        .map_err(|_| crate::usage("a native request session is required"))?;
    session
        .completion
        .as_ref()
        .ok_or_else(|| crate::usage("this batch does not collect a call result"))?
        .result(session.function, None, false)
}

#[extendr]
fn tt_request_batch_wait(batch: Robj, input: bool) -> Crossed<()> {
    let session = ExternalPtr::<NativeSession>::try_from(batch)
        .map_err(|_| crate::usage("a native request session is required"))?;
    if interrupt_pending() {
        session.session.cancel();
        return Err(crate::interrupted());
    }
    let interest = if input {
        thinkthen::RequestSessionWaitInterest::Input
    } else {
        thinkthen::RequestSessionWaitInterest::Settlement
    };
    let _ = session
        .session
        .wait(interest, std::time::Duration::from_millis(50));
    if interrupt_pending() {
        session.session.cancel();
        return Err(crate::interrupted());
    }
    Ok(())
}

#[extendr]
fn tt_request_batch_push(batch: Robj, descriptor: Robj) -> Crossed<Robj> {
    let session = ExternalPtr::<NativeSession>::try_from(batch)
        .map_err(|_| crate::usage("a native request session is required"))?;
    if interrupt_pending() {
        session.session.cancel();
        return Err(crate::interrupted());
    }
    let status = session
        .session
        .try_push_json(&text_of(&descriptor, "descriptor")?)
        .map_err(|error| crate::carry(&error))?;
    Ok(match status {
        thinkthen::RequestSessionPushStatus::Accepted => "accepted",
        thinkthen::RequestSessionPushStatus::Full => "full",
        thinkthen::RequestSessionPushStatus::Closed => "closed",
    }
    .into())
}

#[extendr]
fn tt_request_batch_finish(batch: Robj, failure: Robj) -> Crossed<()> {
    let session = ExternalPtr::<NativeSession>::try_from(batch)
        .map_err(|_| crate::usage("a native request session is required"))?;
    let failure = if failure.is_null() {
        None
    } else {
        Some(
            thinkthen::RequestReaderFailure::from_json(&text_of(&failure, "reader failure")?)
                .map_err(|error| crate::carry(&error))?,
        )
    };
    session
        .session
        .finish(failure)
        .map_err(|error| crate::carry(&error))
}

#[extendr]
fn tt_request_batch_cancel(batch: Robj) -> Crossed<()> {
    let session = ExternalPtr::<NativeSession>::try_from(batch)
        .map_err(|_| crate::usage("a native request session is required"))?;
    session.session.cancel();
    Ok(())
}

#[extendr]
fn tt_request_error_native(
    failure: Robj,
    completed: Robj,
    positions: Robj,
    length: Robj,
    parent: Robj,
) -> Crossed<Robj> {
    let failure_fields = List::try_from(failure.clone())
        .map_err(|_| crate::defect("native failure is not an R list"))?;
    let mut fields = failure_fields
        .iter()
        .filter(|(name, _)| ["message", "kind", "retryable", "facts"].contains(name))
        .collect::<Vec<_>>();
    let kind = fields
        .iter()
        .find(|(name, _)| *name == "kind")
        .ok_or_else(|| crate::defect("native failure has no kind"))?;
    let kind = text_of(&kind.1, "kind")?;
    fields.extend([
        ("complete", failure),
        ("completed", completed),
        ("positions", positions),
        ("length", length),
        ("parent", parent),
        ("call", ().into()),
    ]);
    let mut value = List::from_pairs(fields);
    value
        .set_class([
            format!("thinkthen_{kind}"),
            "thinkthen_error".into(),
            "error".into(),
            "condition".into(),
        ])
        .map_err(|_| crate::defect("native error class could not be assigned"))?;
    Ok(value.into())
}

extendr_module! {
    mod request;
    fn tt_request_error_native;
    fn tt_request_build;
    fn tt_request_native;
    fn tt_request_plan;
    fn tt_request_batch_start;
    fn tt_request_batch_poll;
    fn tt_request_batch_wait;
    fn tt_request_batch_result;
    fn tt_request_batch_cancel;
    fn tt_request_batch_push;
    fn tt_request_batch_finish;
}

//! Canonical admission and native ownership; R only supplies serialized host values.
use super::{Crossed, completion_of, deadline_of, interrupt_pending, text_of};
use crate::calls;
use extendr_api::prelude::*;
use thinkthen::{AdmittedRequest, Request, RequestEnvironment, RequestOutcome, RequestSession};

#[derive(Debug)]
struct NativeRequest(AdmittedRequest);

#[extendr]
fn tt_request_admit(request: Robj) -> Crossed<Robj> {
    let request = Request::from_json(&text_of(&request, "request")?)
        .and_then(Request::admit)
        .map_err(|error| crate::carry(&error))?;
    Ok(ExternalPtr::new(NativeRequest(request)).into())
}

#[extendr]
fn tt_request_native(request: Robj, deadline: Robj, completion: Robj) -> Crossed<Robj> {
    let request = ExternalPtr::<NativeRequest>::try_from(request)
        .map_err(|_| crate::usage("an admitted request is required"))?;
    let admitted = request.0.clone();
    let function = admitted.request().call.function();
    let done = calls::call_owned(
        deadline_of(&deadline)?,
        &interrupt_pending,
        completion_of(&completion)?,
        None,
        move |engine, controls, account| {
            let started = account.start();
            let outcome = engine
                .execute_request(
                    &admitted,
                    RequestEnvironment {
                        controls: controls.surface(thinkthen::Surface::R),
                        feed: None,
                    },
                )
                .map_err(|error| crate::carry(&error))?;
            match &outcome {
                RequestOutcome::Complete(call) => account.include(started, call.facts())?,
                RequestOutcome::Failed { error, .. } => {
                    if let Some(facts) = error.facts() {
                        account.include(started, facts)?;
                    }
                }
            }
            crate::request::written(&outcome)
        },
    )?;
    crate::native_results::request_packet(&done.result?, function)
}

#[extendr]
fn tt_request_plan(request: Robj) -> Crossed<Robj> {
    let request = ExternalPtr::<NativeRequest>::try_from(request)
        .map_err(|_| crate::usage("an admitted request is required"))?;
    let plan = crate::engine()?
        .plan_request(
            &request.0,
            RequestEnvironment {
                controls: thinkthen::CallOptions::new().surface(thinkthen::Surface::R),
                feed: None,
            },
        )
        .map_err(|error| crate::carry(&error))?;
    crate::native_results::tagged(
        crate::plan::render(plan)?.into(),
        "Plan",
        "thinkthen_complete",
    )
}

#[extendr]
fn tt_request_batch_start(request: Robj) -> Crossed<Robj> {
    let request =
        Request::from_json(&text_of(&request, "request")?).map_err(|error| crate::carry(&error))?;
    // Header admission precedes engine selection. The session retains its own request.
    request
        .clone()
        .admit()
        .map_err(|error| crate::carry(&error))?;
    let session = crate::engine()?
        .request_session(request)
        .map_err(|error| crate::carry(&error))?;
    Ok(ExternalPtr::new(session).into())
}

#[extendr]
fn tt_request_batch_poll(batch: Robj) -> Crossed<Robj> {
    let session = ExternalPtr::<RequestSession>::try_from(batch)
        .map_err(|_| crate::usage("a native request session is required"))?;
    if interrupt_pending() {
        session.cancel();
        return Err(crate::interrupted());
    }
    match session.try_read() {
        thinkthen::RequestSessionRead::Result(packet) => {
            let text = packet.to_json().map_err(|error| crate::carry(&error))?;
            crate::native_results::request_event(&text)
        }
        thinkthen::RequestSessionRead::Pending => Ok(().into()),
        thinkthen::RequestSessionRead::End => Ok(list!(kind = "end").into()),
    }
}

#[extendr]
fn tt_request_batch_cancel(batch: Robj) -> Crossed<()> {
    let session = ExternalPtr::<RequestSession>::try_from(batch)
        .map_err(|_| crate::usage("a native request session is required"))?;
    session.cancel();
    Ok(())
}

extendr_module! {
    mod request;
    fn tt_request_admit;
    fn tt_request_native;
    fn tt_request_plan;
    fn tt_request_batch_start;
    fn tt_request_batch_poll;
    fn tt_request_batch_cancel;
}

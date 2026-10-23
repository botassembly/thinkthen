//! R ownership around the public client and shared pulled call.
#![allow(
    missing_docs,
    reason = "extendr emits undocumented registration metadata; handwritten exports are private"
)]

use std::cell::RefCell;
use std::time::Duration;

use extendr_api::prelude::*;
use thinkthen::{Cancel, Client, OffsetUnit, PullCall, PullEvent, SdkRequest};

#[derive(Debug)]
struct NativeClient(Client);

#[derive(Debug)]
struct NativeCall(RefCell<Option<PullCall>>);

fn failure(error: thinkthen::Error) -> List {
    list!(
        error = list!(
            code = error.kind().code(),
            family = error.kind().family().as_str(),
            message = error.to_string()
        )
    )
}

/// Admit R client settings and return its owned handle or coded error.
#[extendr]
fn r_client(settings: &str) -> List {
    match Client::from_json(settings, |name| std::env::var(name).ok()) {
        Ok(client) => list!(value = ExternalPtr::new(NativeClient(client))),
        Err(error) => failure(error),
    }
}

/// Open the shared streamed call with code-point positions.
#[extendr]
fn r_open(client: ExternalPtr<NativeClient>, request: &str) -> List {
    let result = SdkRequest::from_json(request).and_then(|request| {
        client
            .0
            .open(&request, OffsetUnit::CodePoints, Cancel::default())
    });
    match result {
        Ok(call) => list!(value = ExternalPtr::new(NativeCall(RefCell::new(Some(call))))),
        Err(error) => failure(error),
    }
}

/// Wait briefly for an event so R can service interrupts between calls.
#[extendr]
fn r_poll(call: ExternalPtr<NativeCall>) -> List {
    let mut owned = call.0.borrow_mut();
    let Some(call) = owned.as_mut() else {
        return list!(event = "finished");
    };
    match call.poll(Duration::from_millis(20)) {
        None => list!(event = "pending"),
        Some(PullEvent::NeedsInput) => list!(event = "input"),
        Some(PullEvent::Row(row)) => list!(
            event = "row",
            json = row.json,
            detail = row.detail,
            position = row.position as f64,
            kept = row.kept
        ),
        Some(PullEvent::Finished) => match call.final_error() {
            Some(error) => failure(error.clone()),
            None => list!(event = "finished"),
        },
    }
}

fn send(call: &ExternalPtr<NativeCall>, value: &str, json: bool) -> List {
    let mut owned = call.0.borrow_mut();
    if let Some(call) = owned.as_mut() {
        let result = if json {
            call.send_json(value)
        } else {
            call.send_text(value)
        };
        if let Err(error) = result {
            return failure(error);
        }
    }
    list!()
}

/// Supply unwrapped UTF-8 text to the shared call.
#[extendr]
fn r_text(call: ExternalPtr<NativeCall>, text: &str) -> List {
    send(&call, text, false)
}

/// Supply one structured JSON record to the shared call.
#[extendr]
fn r_json(call: ExternalPtr<NativeCall>, json: &str) -> List {
    send(&call, json, true)
}

/// Finish the input collection.
#[extendr]
fn r_end(call: ExternalPtr<NativeCall>) {
    if let Some(call) = call.0.borrow_mut().as_mut() {
        call.end_input();
    }
}

/// Cancel and release the shared call without waiting for its worker.
#[extendr]
fn r_close(call: ExternalPtr<NativeCall>) {
    if let Some(call) = call.0.borrow_mut().take() {
        call.cancel();
    }
}

extendr_module! {
    mod thinkthen;
    fn r_client;
    fn r_open;
    fn r_poll;
    fn r_text;
    fn r_json;
    fn r_end;
    fn r_close;
}

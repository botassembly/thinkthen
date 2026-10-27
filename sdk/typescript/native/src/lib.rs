//! Node's thread-safe callback bridge. One thread owns one engine operation.

mod request;

use napi::threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode};
use std::sync::{
    Arc, OnceLock,
    mpsc::{Receiver, Sender},
};
use thinkthen::{Cancel, native::Events};

#[derive(Debug)]
struct Response {
    kind: &'static str,
    record: Option<thinkthen::records::Record>,
}

#[derive(Debug)]
struct Replies {
    sender: Sender<Response>,
    events: OnceLock<Events>,
}

impl Replies {
    fn send(&self, kind: &'static str, record: Option<thinkthen::records::Record>) {
        let _ = self.sender.send(Response { kind, record });
        if let Some(events) = self.events.get() {
            events.wake();
        }
    }
}

#[derive(Debug)]
struct Event {
    kind: &'static str,
    data: String,
    replies: Arc<Replies>,
}

struct Bridge {
    callback: ThreadsafeFunction<Event>,
    cancel: Cancel,
    replies: Arc<Replies>,
    receive: Receiver<Response>,
}

impl Bridge {
    fn publish(&self, kind: &'static str, data: String) -> bool {
        if self.callback.call(
            Ok(Event {
                kind,
                data,
                replies: self.replies.clone(),
            }),
            ThreadsafeFunctionCallMode::NonBlocking,
        ) != napi::Status::Ok
        {
            self.cancel.cancel();
            return false;
        }
        true
    }
}

// The binding macro emits class helpers outside each annotated impl.
#[expect(
    missing_docs,
    reason = "napi emits undocumented class helpers in this module"
)]
mod exports;
pub use exports::{
    Client, Operation, abi_version, parse_toolbox, reject_cancellation, reject_non_finite_json,
    reject_question_conversion, reject_settings_conversion, serialize_toolbox,
};

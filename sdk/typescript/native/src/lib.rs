//! Node's thread-safe callback bridge. One thread owns one engine operation.

mod request;

use napi::threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode};
use std::sync::mpsc::{Sender, channel};
use std::time::Duration;
use thinkthen::Cancel;

#[derive(Debug)]
struct Event {
    kind: &'static str,
    data: String,
    reply: Sender<Option<thinkthen::records::Record>>,
}

struct Bridge {
    callback: ThreadsafeFunction<Event>,
    cancel: Cancel,
}

impl Bridge {
    fn exchange(
        &self,
        kind: &'static str,
        data: String,
    ) -> Option<Option<thinkthen::records::Record>> {
        if self.cancel.is_cancelled() && kind != "done" {
            return None;
        }
        let (reply, receive) = channel();
        if self.callback.call(
            Ok(Event { kind, data, reply }),
            ThreadsafeFunctionCallMode::Blocking,
        ) != napi::Status::Ok
        {
            self.cancel.cancel();
            return None;
        }
        loop {
            match receive.recv_timeout(Duration::from_millis(20)) {
                Ok(value) => return Some(value),
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) if !self.cancel.is_cancelled() => {}
                _ => return None,
            }
        }
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

//! One bounded, pulled operation shared by native hosts.

mod row;
pub use row::{OffsetUnit, PullRow};

use std::sync::mpsc::{self, Receiver, SyncSender};
use std::time::Duration;

use crate::retry::cancelled;
use crate::{Cancel, Client, Error, ErrorKind, Record, SdkCall, SdkRequest};

/// Decode a host's UTF-16 text without replacing unpaired surrogates.
///
/// # Errors
/// Returns the library's record error for invalid Unicode text.
pub fn utf16_text(units: &[u16]) -> Result<String, Error> {
    String::from_utf16(units)
        .map_err(|_| Error::new(ErrorKind::Record, "the record is not valid Unicode text"))
}

/// The next action for a caller driving one operation.
#[derive(Debug)]
pub enum PullEvent {
    /// Supply one record or end input before pulling again.
    NeedsInput,
    /// A serialized row and its original input position.
    Row(PullRow),
    /// No more rows remain; inspect the call's final error.
    Finished,
}

#[derive(Debug)]
enum Input {
    Text(String),
    Json(String),
}

#[derive(Debug)]
enum Message {
    Need,
    Row(PullRow),
    Done(Result<(), Error>),
}

/// An admitted operation, with bounded input and output and owned final failure.
/// Dropping it cancels without waiting for a backend reply or host callback.
#[derive(Debug)]
pub struct PullCall {
    input: Option<SyncSender<Input>>,
    output: Option<Receiver<Message>>,
    cancel: Cancel,
    waiting: bool,
    error: Option<Error>,
}

impl Client {
    /// Open one operation from an admitted call; initial records precede sent records.
    ///
    /// # Errors
    /// Returns the library's admission error before starting any work.
    pub fn open(
        &self,
        request: &SdkRequest,
        offsets: OffsetUnit,
        cancel: Cancel,
    ) -> Result<PullCall, Error> {
        cancel.complete(Ok(()))?;
        let call = self.prepare(request, cancel.clone())?;
        let initial: Vec<_> = request.records().iter().map(|v| call.record(v)).collect();
        let (input, records) = mpsc::sync_channel(1);
        let (send, output) = mpsc::sync_channel(1);
        let stop = cancel.clone();
        std::thread::spawn(move || run(&call, initial, records, &send, offsets, &stop));
        Ok(PullCall {
            input: Some(input),
            output: Some(output),
            cancel,
            waiting: false,
            error: None,
        })
    }
}

impl PullCall {
    /// Send one text record as raw UTF-8, without JSON wrapping or escaping.
    ///
    /// # Errors
    /// Returns a usage error unless the preceding event requested input.
    pub fn send_text(&mut self, text: &str) -> Result<(), Error> {
        self.send(Input::Text(text.to_owned()))
    }

    /// Send one structured record as JSON; Rust owns its parsing and evidence checks.
    ///
    /// # Errors
    /// Returns a usage error unless the preceding event requested input.
    pub fn send_json(&mut self, json: &str) -> Result<(), Error> {
        // Evidence selection belongs to the admitted call on the worker.
        self.send(Input::Json(json.to_owned()))
    }

    fn send(&mut self, record: Input) -> Result<(), Error> {
        if !self.waiting {
            return Err(Error::new(
                ErrorKind::Record,
                "the call has not requested input",
            ));
        }
        self.waiting = false;
        if let Some(input) = &self.input {
            input.send(record).map_err(|_| cancelled())?;
        }
        Ok(())
    }

    /// Finish the input after its last record. Further pulls drain remaining rows.
    pub fn end_input(&mut self) {
        self.waiting = false;
        self.input.take();
    }

    /// Wait for the next row, input request or completion.
    pub fn next_event(&mut self) -> PullEvent {
        loop {
            if let Some(event) = self.poll(Duration::from_millis(20)) {
                return event;
            }
        }
    }

    /// Wait at most `timeout`, letting hosts service their own signals between polls.
    pub fn poll(&mut self, timeout: Duration) -> Option<PullEvent> {
        let Some(output) = &self.output else {
            return Some(PullEvent::Finished);
        };
        if let Err(error) = self.cancel.complete(Ok(())) {
            self.error = Some(error);
            self.output.take();
            self.input.take();
            return Some(PullEvent::Finished);
        }
        match output.recv_timeout(timeout) {
            Ok(Message::Need) if self.input.is_none() => None,
            Ok(Message::Need) => {
                self.waiting = true;
                Some(PullEvent::NeedsInput)
            }
            Ok(Message::Row(row)) => Some(PullEvent::Row(row)),
            Ok(Message::Done(result)) => {
                self.error = result.err();
                self.output.take();
                self.input.take();
                Some(PullEvent::Finished)
            }
            Err(mpsc::RecvTimeoutError::Timeout) => None,
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                self.error = Some(Error::new(
                    ErrorKind::Io,
                    "the call worker stopped before completion",
                ));
                self.output.take();
                Some(PullEvent::Finished)
            }
        }
    }

    /// Stop input and request cancellation. Freeing the call never joins its worker.
    pub fn cancel(&self) {
        self.cancel.cancel();
    }

    /// The final failure after completion, including a filter's first failed row.
    #[must_use]
    pub const fn final_error(&self) -> Option<&Error> {
        self.error.as_ref()
    }
}

impl Drop for PullCall {
    fn drop(&mut self) {
        if self.output.is_some() {
            self.cancel.cancel();
        }
    }
}

fn run(
    call: &SdkCall,
    initial: Vec<Record>,
    input: Receiver<Input>,
    output: &SyncSender<Message>,
    offsets: OffsetUnit,
    cancel: &Cancel,
) {
    let mut initial = initial.into_iter();
    let mut position = 0;
    let mut ended = false;
    let records = std::iter::from_fn(|| {
        let record = initial.next().or_else(|| {
            if ended || cancel.is_cancelled() || output.send(Message::Need).is_err() {
                return None;
            }
            loop {
                match input.recv_timeout(Duration::from_millis(20)) {
                    Ok(Input::Text(text)) => return Some(Record::text(text)),
                    Ok(Input::Json(json)) => return Some(call.json_record(&json)),
                    Err(mpsc::RecvTimeoutError::Timeout) if !cancel.is_cancelled() => {}
                    _ => {
                        ended = true;
                        return None;
                    }
                }
            }
        })?;
        position += 1;
        Some((record.clone(), (record, position)))
    });
    let mut failure = None;
    let result = call.run(records, |(record, position), row| {
        match PullRow::new(call, &record, row, position, offsets) {
            Ok(row) => {
                if output.send(Message::Row(row)).is_err() {
                    cancel.cancel();
                }
            }
            Err(error) => {
                failure.get_or_insert(error);
                cancel.cancel();
            }
        }
    });
    let result = result.and_then(|()| failure.map_or(Ok(()), Err));
    let _ = output.send(Message::Done(result));
}

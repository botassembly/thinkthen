//! Typed iterators and push input share one bounded execution driver.
#[cfg(any(feature = "native", feature = "command"))]
pub(crate) mod driver;
mod execute;
mod filtered;
#[cfg(feature = "native")]
pub mod native;
mod queue;
#[cfg(any(feature = "native", feature = "command"))]
mod signal;
#[cfg(feature = "stream")]
mod stream;
use crate::core::{
    records::Record,
    results::{self, AnyRow, Function, Row},
};
use crate::engine::RecordEvent;
use crate::{Cancel, Engine, Error, OffsetUnit, Question, Request};
pub use filtered::{FilterError, Filtered, Kept};
use queue::{Queue, Received};
use std::marker::PhantomData;
use std::sync::Arc;
#[cfg(feature = "stream")]
use std::task::{Context, Poll};
#[cfg(feature = "stream")]
pub use stream::{AnyRowStream, RowStream};

// Typed Rust calls report spans in UTF-8 bytes, so `&text[start..end]` slices the
// matched text.
const RUST_OFFSETS: OffsetUnit = OffsetUnit::Utf8;

// Empty queue polls describe host scheduling, not the end of a batch.
const INPUT_IDLE: std::time::Duration = std::time::Duration::from_millis(100);

#[derive(Clone, Copy)]
enum InputKind {
    Open,
    #[cfg(any(feature = "native", feature = "command"))]
    Finite,
}
impl InputKind {
    fn waits_for_end(self) -> bool {
        match self {
            Self::Open => false,
            #[cfg(any(feature = "native", feature = "command"))]
            Self::Finite => true,
        }
    }
}

/// Rows from a question selected at runtime.
#[non_exhaustive]
#[derive(Debug)]
pub struct AnyRows {
    output: Arc<Queue<Result<AnyRow, Error>>>,
    cancel: Cancel,
    done: bool,
}
impl AnyRows {
    /// Cancel this call and wake its blocked input and output operations.
    pub fn cancel(&self) {
        self.cancel.cancel();
    }
    fn received(
        &mut self,
        event: Received<Result<AnyRow, Error>>,
    ) -> Option<Result<AnyRow, Error>> {
        match event {
            Received::Item(row) => {
                if row.is_err() {
                    self.done = true;
                }
                Some(row)
            }
            Received::Closed => {
                self.done = true;
                None
            }
            Received::Cancelled => {
                self.done = true;
                Some(Err(crate::retry::cancelled()))
            }
        }
    }
    #[cfg(feature = "stream")]
    fn poll(&mut self, cx: &mut Context<'_>) -> Poll<Option<Result<AnyRow, Error>>> {
        if self.done {
            return Poll::Ready(None);
        }
        match self.output.poll(cx) {
            Poll::Ready(event) => Poll::Ready(self.received(event)),
            Poll::Pending => Poll::Pending,
        }
    }
}
impl Iterator for AnyRows {
    type Item = Result<AnyRow, Error>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let event = self.output.receive();
        self.received(event)
    }
}
impl Drop for AnyRows {
    fn drop(&mut self) {
        if !self.done {
            self.cancel.cancel();
        }
        self.output.close();
    }
}
/// A function's typed rows, followed by at most one call-level error.
#[non_exhaustive]
#[derive(Debug)]
pub struct Rows<F: Function> {
    inner: AnyRows,
    function: PhantomData<fn() -> F>,
}
impl<F: Function> Rows<F> {
    /// Cancel this call without affecting another call on the same engine.
    pub fn cancel(&self) {
        self.inner.cancel();
    }
    fn admit(&mut self, row: Result<AnyRow, Error>) -> Result<Row<F>, Error> {
        let row = row.and_then(AnyRow::typed::<F>);
        if row.is_err() {
            self.inner.done = true;
            self.inner.cancel.cancel();
        }
        row
    }
}
impl<F: Function> Iterator for Rows<F> {
    type Item = Result<Row<F>, Error>;
    fn next(&mut self) -> Option<Self::Item> {
        let row = self.inner.next()?;
        Some(self.admit(row))
    }
}
#[derive(Debug)]
struct SenderOwner {
    input: Arc<Queue<Record>>,
}
impl Drop for SenderOwner {
    fn drop(&mut self) {
        self.input.close();
    }
}
/// An owning sender whose last owner closes input.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct RecordSender(Arc<SenderOwner>);
impl RecordSender {
    /// Transfer one record to the call, waiting for bounded input capacity.
    /// Rows must be drained concurrently as described in [`Engine::open`].
    /// # Errors
    /// Returns cancellation when the call closes or is cancelled.
    pub fn send(&self, record: Record) -> Result<(), Error> {
        self.0.input.send(record)
    }
    /// Close input without waiting for output to be drained.
    pub fn close(&self) {
        self.0.input.close();
    }
}
/// A heterogeneous call with explicit input ownership.
#[non_exhaustive]
#[derive(Debug)]
pub struct AnyCall {
    sender: RecordSender,
    rows: AnyRows,
}
impl AnyCall {
    /// Retain an input owner independently of the receiving call.
    #[must_use]
    pub fn sender(&self) -> RecordSender {
        self.sender.clone()
    }
    /// Consume the call's output side, releasing its own input owner.
    #[must_use]
    pub fn rows(self) -> AnyRows {
        self.rows
    }
}
/// A typed call with bounded push input.
/// Follow [`Engine::open`] for concurrent input and output ownership.
#[non_exhaustive]
#[derive(Debug)]
pub struct Call<F: Function> {
    sender: RecordSender,
    rows: Rows<F>,
}
impl<F: Function> Call<F> {
    /// Retain an input owner independently of the receiving call.
    #[must_use]
    pub fn sender(&self) -> RecordSender {
        self.sender.clone()
    }
    /// Consume the output side and release this call's own input owner.
    #[must_use]
    pub fn rows(self) -> Rows<F> {
        self.rows
    }
}
impl Engine {
    /// Evaluate one record. Blank field is a `record.empty` error; every other
    /// record failure is returned as an ordinary row.
    ///
    /// The record is anything that converts into a [`Record`]: text as `&str` or
    /// `String`, a [`serde_json::Value`], or a built [`Record`]. A list is refused
    /// when the code is compiled; use [`Engine::rows`] for many records.
    /// ```compile_fail
    /// # let engine = thinkthen::Settings::default().engine(|_| None).unwrap();
    /// # let refund = thinkthen::Decide::new("Refund?").unwrap();
    /// let row = engine.run(&refund, vec!["Refund me.", "Thanks!"]);
    /// ```
    /// # Errors
    /// Returns a call-level error, or `record.empty` for blank field.
    pub fn run<F: Function>(
        &self,
        question: &F,
        record: impl Into<Record>,
    ) -> Result<Row<F>, Error> {
        let mut rows = self.rows(question, [record.into()])?;
        let row = rows.next().unwrap_or_else(|| Err(results::mismatch()))?;
        if let Some(result) = rows.next() {
            result?;
            return Err(results::mismatch());
        }
        if let Some(error) = row
            .error()
            .filter(|error| error.code() == crate::Code::RecordEmpty)
        {
            return Err(error.clone());
        }
        Ok(row)
    }
    /// Evaluate records through a lazy iterator on a bounded worker.
    ///
    /// Each record is anything that converts into a [`Record`]. A worker thread
    /// reads them, so they must be owned: collect borrowed items into a `Vec` first.
    /// # Errors
    /// Returns call setup or pre-existing cancellation errors.
    pub fn rows<F: Function, I>(&self, question: &F, records: I) -> Result<Rows<F>, Error>
    where
        I: IntoIterator + Send + 'static,
        I::Item: Into<Record>,
        I::IntoIter: Send,
    {
        Ok(Rows {
            inner: self.any_rows(
                question.clone().into(),
                self.name.clone(),
                records.into_iter().map(Into::into),
                RUST_OFFSETS,
            )?,
            function: PhantomData,
        })
    }
    /// Open a typed question for push input.
    ///
    /// Retain a sender with [`Call::sender`] and consume [`Call::rows`] on another
    /// thread or independently scheduled task while sending. `send` blocks the
    /// calling thread when input is full; the draining task must still run. Sending
    /// everything before draining can deadlock because both queues are bounded.
    /// Close or drop the last sender after the final record. For an existing
    /// iterator, prefer [`Engine::rows`], which owns input production.
    /// # Errors
    /// Returns call setup or pre-existing cancellation errors.
    pub fn open<F: Function>(&self, question: &F) -> Result<Call<F>, Error> {
        let call = self.open_any(
            question.clone().into(),
            self.name.clone(),
            Vec::new(),
            RUST_OFFSETS,
            InputKind::Open,
        )?;
        Ok(Call {
            sender: call.sender,
            rows: Rows {
                inner: call.rows,
                function: PhantomData,
            },
        })
    }
    /// Open a canonical dynamic request. Initial records precede pushed records.
    ///
    /// Binding plumbing: language bindings choose their own offset unit here.
    /// # Errors
    /// Returns call setup or pre-existing cancellation errors.
    #[doc(hidden)]
    pub fn open_request(&self, request: &Request, offsets: OffsetUnit) -> Result<AnyCall, Error> {
        self.open_any(
            request.question().clone(),
            request.name().map(str::to_owned),
            request.records().iter().map(Record::json).collect(),
            offsets,
            InputKind::Open,
        )
    }
    fn open_any(
        &self,
        question: Question,
        name: Option<String>,
        initial: Vec<Record>,
        offsets: OffsetUnit,
        input_kind: InputKind,
    ) -> Result<AnyCall, Error> {
        let engine = self.for_call();
        let input = Queue::new(2, &engine.cancel);
        let sender = RecordSender(Arc::new(SenderOwner {
            input: input.clone(),
        }));
        let mut flushed = initial.is_empty();
        let records = initial
            .into_iter()
            .map(RecordEvent::Record)
            .chain(std::iter::from_fn(move || {
                let event = if flushed || input_kind.waits_for_end() {
                    input.receive()
                } else if let Some(event) = input.receive_for(INPUT_IDLE) {
                    event
                } else {
                    flushed = true;
                    return Some(RecordEvent::Flush);
                };
                match event {
                    Received::Item(record) => {
                        flushed = false;
                        Some(RecordEvent::Record(record))
                    }
                    Received::Closed | Received::Cancelled => None,
                }
            }));
        let rows = engine.start(question, name, records, offsets)?;
        Ok(AnyCall { sender, rows })
    }
    fn any_rows<I>(
        &self,
        question: Question,
        name: Option<String>,
        records: I,
        offsets: OffsetUnit,
    ) -> Result<AnyRows, Error>
    where
        I: IntoIterator<Item = Record> + Send + 'static,
        I::IntoIter: Send,
    {
        self.for_call().start(
            question,
            name,
            records.into_iter().map(RecordEvent::Record),
            offsets,
        )
    }
    fn start<I>(
        &self,
        question: Question,
        name: Option<String>,
        records: I,
        offsets: OffsetUnit,
    ) -> Result<AnyRows, Error>
    where
        I: IntoIterator<Item = RecordEvent<Record>> + Send + 'static,
        I::IntoIter: Send,
    {
        self.cancel.complete(Ok(()))?;
        let output = Queue::new(2, &self.cancel);
        let send = output.clone();
        let engine = self.clone();
        std::thread::Builder::new()
            .spawn(move || {
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    execute::drive(
                        &engine,
                        &question,
                        name.as_deref(),
                        records.into_iter(),
                        offsets,
                        |row| {
                            if send.send(row).is_err() {
                                engine.cancel.cancel();
                            }
                        },
                    )
                }))
                .unwrap_or_else(|_| {
                    Err(crate::core::Error::from_contract(
                        crate::Code::InternalBug,
                        "the call worker failed",
                    ))
                });
                if let Err(error) = result {
                    let _ = send.send(Err(error));
                }
                send.close();
            })
            .map_err(|cause| {
                let mut error =
                    Error::from_contract(crate::Code::LocalIo, "the call worker could not start");
                error.source = Some(Arc::new(cause));
                error
            })?;
        Ok(AnyRows {
            output,
            cancel: self.cancel.clone(),
            done: false,
        })
    }
}

#[cfg(any(feature = "native", feature = "command"))]
fn abi_error() -> Error {
    Error::from_contract(crate::Code::UsageAbi, "the native ABI is incompatible")
}

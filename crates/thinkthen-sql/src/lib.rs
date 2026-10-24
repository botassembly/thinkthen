//! Drive a statement's row chunks through one shared engine call.
//!
//! A database retains its admitted [`Client`] per connection. It owns all cursor
//! access and passes owned records here only when [`SqlEvent::NeedsChunk`] asks.
//! The engine owns validation, batching, execution, row order and failures.

use std::time::Duration;
use std::vec::IntoIter;

use thinkthen::{Cancel, Client, Error, ErrorKind, OffsetUnit, PullCall, PullEvent, SdkRequest};

pub use thinkthen::PullRow;

mod request;
pub use request::{client, client_with_policy, request};

/// One owned database value, copied before advancing its source cursor.
#[derive(Debug)]
pub enum SqlRecord {
    /// Raw UTF-8 text, without JSON quoting.
    Text(String),
    /// A serialized structured JSON value, interpreted by the shared library.
    Json(String),
}

/// The next action for a database driving one statement invocation.
#[derive(Debug)]
pub enum SqlEvent {
    /// Fetch and submit a whole chunk, or end the source.
    NeedsChunk,
    /// One result, retaining its original one-based input position.
    Row(PullRow),
    /// No more rows remain; inspect [`SqlCall::final_error`].
    Finished,
}

/// One statement-owned call with only its current source chunk staged.
///
/// Rank keeps the engine's best-first order. Other functions retain input order.
/// Dropping this value cancels without waiting for a backend reply. No database
/// handle or callback enters the engine worker.
#[derive(Debug)]
pub struct SqlCall {
    call: PullCall,
    chunk: IntoIter<SqlRecord>,
    needs_chunk: bool,
    ended: bool,
}

impl SqlCall {
    /// Open a streamed function invocation using a connection's admitted client.
    /// Offsets count Unicode scalar values in both answers and detail.
    ///
    /// # Errors
    /// Returns the shared library's request admission error.
    pub fn open(client: &Client, request: &SdkRequest) -> Result<Self, Error> {
        Ok(Self {
            call: client.open(request, OffsetUnit::CodePoints, Cancel::default())?,
            chunk: Vec::new().into_iter(),
            needs_chunk: false,
            ended: false,
        })
    }

    /// Submit one owned chunk after [`SqlEvent::NeedsChunk`]. Empty chunks are
    /// allowed; only [`Self::end_input`] ends the input and flushes its last batch.
    ///
    /// # Errors
    /// Returns a record error if the call has not requested another chunk.
    pub fn send_chunk(&mut self, chunk: Vec<SqlRecord>) -> Result<(), Error> {
        if !self.needs_chunk {
            return Err(Error::new(
                ErrorKind::Record,
                "the SQL call has not requested a chunk",
            ));
        }
        self.chunk = chunk.into_iter();
        if let Some(record) = self.chunk.next() {
            self.send(record)?;
            self.needs_chunk = false;
        }
        Ok(())
    }

    /// End the source and let the shared call drain all remaining rows.
    pub fn end_input(&mut self) {
        self.ended = true;
        if self.needs_chunk {
            self.needs_chunk = false;
            self.call.end_input();
        }
    }

    /// Wait at most `timeout` for one engine event, then return to the database.
    /// The interrupt probe runs on the caller's thread before and after the wait.
    /// It must only read native interrupt state, without throwing or longjmp.
    /// A database raises its native cancellation error after this method returns.
    ///
    /// # Errors
    /// Returns a shared library record-send error.
    pub fn poll(
        &mut self,
        timeout: Duration,
        mut interrupted: impl FnMut() -> bool,
    ) -> Result<Option<SqlEvent>, Error> {
        if interrupted() {
            self.call.cancel();
        }
        let mut event = self.call.poll(if self.needs_chunk {
            Duration::ZERO
        } else {
            timeout
        });
        if interrupted() {
            self.call.cancel();
            event = self.call.poll(Duration::ZERO);
        }
        Ok(match event {
            Some(PullEvent::NeedsInput) => match self.chunk.next() {
                Some(record) => {
                    self.send(record)?;
                    None
                }
                None if self.ended => {
                    self.call.end_input();
                    None
                }
                None => {
                    self.needs_chunk = true;
                    Some(SqlEvent::NeedsChunk)
                }
            },
            Some(PullEvent::Row(row)) => Some(SqlEvent::Row(row)),
            Some(PullEvent::Finished) => {
                self.chunk = Vec::new().into_iter();
                self.needs_chunk = false;
                self.ended = true;
                Some(SqlEvent::Finished)
            }
            None if self.needs_chunk => Some(SqlEvent::NeedsChunk),
            None => None,
        })
    }

    fn send(&mut self, record: SqlRecord) -> Result<(), Error> {
        match record {
            SqlRecord::Text(text) => self.call.send_text(&text),
            SqlRecord::Json(json) => self.call.send_json(&json),
        }
    }

    /// The shared call's final failure after [`SqlEvent::Finished`].
    #[must_use]
    pub const fn final_error(&self) -> Option<&Error> {
        self.call.final_error()
    }
}

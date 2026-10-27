//! One private host driver reserves input and waits on the shared queue signal.
use super::signal::Signal;
use super::*;
use std::time::Duration;
/// Native driver event; record failures remain ordinary rows.
#[non_exhaustive]
#[derive(Debug)]
#[allow(
    clippy::large_enum_variant,
    reason = "the 208-byte row crosses a bounded queue without a per-row allocation"
)]
pub(crate) enum Event {
    /// One independently owned canonical row.
    Row(AnyRow),
    /// One input slot is reserved until send or end_input.
    NeedsInput,
    /// Successful completion after any record failures.
    Finished,
}
/// One native host's driver over a retained engine call.
#[non_exhaustive]
#[derive(Debug)]
pub(crate) struct Driver {
    sender: RecordSender,
    rows: AnyRows,
    signal: Arc<Signal>,
    waiting: bool,
}
impl Driver {
    /// Admit and open a dynamic call without creating another producer.
    /// # Errors
    /// Returns the shared setup or question error.
    pub(crate) fn open(
        engine: &Engine,
        request: &Request,
        offsets: OffsetUnit,
    ) -> Result<Self, Error> {
        Ok(Self::from_call(engine.open_request(request, offsets)?))
    }
    /// Open a finite source whose host sends records while draining results.
    /// Host work between sends does not end its pending batch.
    #[cfg(feature = "command")]
    pub(crate) fn open_finite(
        engine: &Engine,
        request: &Request,
        offsets: OffsetUnit,
    ) -> Result<Self, Error> {
        let call = engine.open_any(
            request.question().clone(),
            request.name().map(str::to_owned),
            request.records().iter().map(Record::json).collect(),
            offsets,
            InputKind::Finite,
        )?;
        Ok(Self::from_call(call))
    }
    pub(crate) fn open_records(
        engine: &Engine,
        request: &Request,
        records: Vec<Record>,
        offsets: OffsetUnit,
    ) -> Result<Self, Error> {
        if !request.records().is_empty() {
            return Err(abi_error());
        }
        let call = engine.open_any(
            request.question().clone(),
            request.name().map(str::to_owned),
            records,
            offsets,
            InputKind::Finite,
        )?;
        call.sender.close();
        Ok(Self::from_call(call))
    }
    fn from_call(call: AnyCall) -> Self {
        let AnyCall { sender, rows } = call;
        let signal = Arc::new(Signal::default());
        sender.0.input.observe(signal.clone());
        rows.output.observe(signal.clone());
        Self {
            sender,
            rows,
            signal,
            waiting: false,
        }
    }
    /// Transfer a record into the slot reserved by NeedsInput.
    /// # Errors
    /// Returns cancellation or usage.abi for a driver protocol error.
    pub(crate) fn send(&mut self, record: Record) -> Result<(), Error> {
        if !self.waiting {
            return Err(abi_error());
        }
        self.waiting = false;
        self.sender.0.input.send_reserved(record)
    }
    /// Close input without waiting for any output or worker.
    pub(crate) fn end_input(&mut self) {
        self.waiting = false;
        self.sender.close();
    }
    /// Request cancellation from the owning host.
    pub(crate) fn cancel(&self) {
        self.rows.cancel();
    }
    /// Retain the token for a host's concurrent cancellation entry point.
    #[must_use]
    #[cfg(feature = "native")]
    pub(crate) fn cancellation(&self) -> Cancel {
        self.rows.cancel.clone()
    }
    #[cfg(feature = "native")]
    pub(super) fn events(&self) -> Arc<Signal> {
        self.signal.clone()
    }
    fn ready(&mut self) -> Result<Option<Event>, Error> {
        if self.rows.done {
            return Ok(Some(Event::Finished));
        }
        if let Some(event) = self.rows.output.try_receive() {
            return self
                .rows
                .received(event)
                .map(|row| row.map(Event::Row))
                .transpose()
                .map(|event| Some(event.unwrap_or(Event::Finished)));
        }
        if self.waiting || self.sender.0.input.reserve() {
            self.waiting = true;
            return Ok(Some(Event::NeedsInput));
        }
        Ok(None)
    }
    /// Wait for a row, a safely reserved input slot or completion.
    /// # Errors
    /// Yields a terminal call error once, then Finished on later reads.
    pub(crate) fn next_event(&mut self) -> Result<Event, Error> {
        loop {
            let version = self.signal.version();
            if let Some(event) = self.ready()? {
                return Ok(event);
            }
            self.signal.wait(version, None);
        }
    }
    /// Wait at most timeout for a driver event on the calling host thread.
    /// # Errors
    /// Yields only call-level errors; record failures are Row events.
    pub(crate) fn poll(&mut self, timeout: Duration) -> Result<Option<Event>, Error> {
        let version = self.signal.version();
        if let Some(event) = self.ready()? {
            return Ok(Some(event));
        }
        self.signal.wait(version, Some(timeout));
        self.ready()
    }
}

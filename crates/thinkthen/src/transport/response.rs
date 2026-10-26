//! Observe HTTP input above TLS so handshake bytes cannot hide a stale socket.
use std::cell::Cell;
use ureq::unversioned::transport::{Buffers, ConnectionDetails, Connector, NextTimeout, Transport};

#[derive(Clone, Copy, Default)]
struct Progress {
    reused: bool,
    received: bool,
}

thread_local! {
    static PROGRESS: Cell<Progress> = Cell::default();
}

pub(super) fn begin() {
    PROGRESS.set(Progress::default());
}

pub(super) fn stale(error: &ureq::Error) -> bool {
    let progress = PROGRESS.get();
    progress.reused
        && !progress.received
        && matches!(error,
        ureq::Error::Io(error) if matches!(error.kind(),
            std::io::ErrorKind::UnexpectedEof | std::io::ErrorKind::BrokenPipe
            | std::io::ErrorKind::ConnectionReset | std::io::ErrorKind::ConnectionAborted))
}

#[derive(Debug)]
pub(super) struct ResponseConnector;

impl<In: Transport> Connector<In> for ResponseConnector {
    type Out = Response<In>;

    fn connect(
        &self,
        _: &ConnectionDetails,
        chained: Option<In>,
    ) -> Result<Option<Self::Out>, ureq::Error> {
        Ok(chained.map(|inner| Response {
            inner,
            replied: false,
        }))
    }
}

#[derive(Debug)]
pub(super) struct Response<In> {
    inner: In,
    replied: bool,
}

impl<In: Transport> Transport for Response<In> {
    fn buffers(&mut self) -> &mut dyn Buffers {
        self.inner.buffers()
    }

    fn transmit_output(&mut self, amount: usize, timeout: NextTimeout) -> Result<(), ureq::Error> {
        let mut progress = PROGRESS.get();
        progress.reused = self.replied;
        PROGRESS.set(progress);
        self.inner.transmit_output(amount, timeout)
    }

    fn await_input(&mut self, timeout: NextTimeout) -> Result<bool, ureq::Error> {
        let result = self.inner.await_input(timeout);
        if !self.inner.buffers().input().is_empty() {
            self.replied = true;
            let mut progress = PROGRESS.get();
            progress.received = true;
            PROGRESS.set(progress);
        }
        result
    }

    fn is_open(&mut self) -> bool {
        self.inner.is_open()
    }
    fn is_tls(&self) -> bool {
        self.inner.is_tls()
    }
}

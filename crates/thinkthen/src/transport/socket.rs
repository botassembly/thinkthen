//! TCP connections whose lifetime belongs to the operation's cancel signal.

use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::sync::Arc;
use std::time::{Duration, Instant};

use ureq::unversioned::transport::{
    Buffers, ConnectionDetails, Connector, Either, LazyBuffers, NextTimeout, Transport,
};

use crate::Cancel;

#[derive(Debug)]
pub(super) struct SocketConnector(pub(super) Cancel);

impl<In: Transport> Connector<In> for SocketConnector {
    type Out = Either<In, Socket>;

    fn connect(
        &self,
        details: &ConnectionDetails,
        chained: Option<In>,
    ) -> Result<Option<Self::Out>, ureq::Error> {
        if let Some(transport) = chained {
            return Ok(Some(Either::A(transport)));
        }
        let started = Instant::now();
        let budget = details.timeout.not_zero().map(|duration| *duration);
        let mut failure = io::Error::from(io::ErrorKind::ConnectionRefused);
        for (index, address) in details.addrs.iter().enumerate() {
            if self.0.is_cancelled() {
                return Err(io::Error::from(io::ErrorKind::Interrupted).into());
            }
            let connected = if let Some(budget) = budget {
                let remaining = budget.saturating_sub(started.elapsed());
                if remaining.is_zero() {
                    return Err(ureq::Error::Timeout(details.timeout.reason));
                }
                // Reserve time for later addresses when one address blackholes.
                let share = if index + 1 < details.addrs.len() {
                    (remaining / 2).max(Duration::from_nanos(1))
                } else {
                    remaining
                };
                TcpStream::connect_timeout(address, share)
            } else {
                TcpStream::connect(address)
            };
            match connected {
                Ok(stream) => {
                    stream.set_nodelay(details.config.no_delay())?;
                    let stream = Arc::new(stream);
                    self.0.register(&stream)?;
                    return Ok(Some(Either::B(Socket {
                        stream,
                        buffers: LazyBuffers::new(
                            details.config.input_buffer_size(),
                            details.config.output_buffer_size(),
                        ),
                    })));
                }
                Err(error) => failure = error,
            }
        }
        Err(failure.into())
    }
}

#[derive(Debug)]
pub(super) struct Socket {
    stream: Arc<TcpStream>,
    buffers: LazyBuffers,
}

fn timed<T>(result: io::Result<T>, timeout: NextTimeout) -> Result<T, ureq::Error> {
    result.map_err(|error| match error.kind() {
        io::ErrorKind::TimedOut | io::ErrorKind::WouldBlock => ureq::Error::Timeout(timeout.reason),
        _ => error.into(),
    })
}

impl Transport for Socket {
    fn buffers(&mut self) -> &mut dyn Buffers {
        &mut self.buffers
    }

    #[expect(
        clippy::indexing_slicing,
        reason = "ureq passes only the output length it filled in this buffer"
    )]
    fn transmit_output(&mut self, amount: usize, timeout: NextTimeout) -> Result<(), ureq::Error> {
        self.stream
            .set_write_timeout(timeout.not_zero().map(|duration| *duration))?;
        let output = &self.buffers.output()[..amount];
        timed((&*self.stream).write_all(output), timeout)
    }

    fn await_input(&mut self, timeout: NextTimeout) -> Result<bool, ureq::Error> {
        self.stream
            .set_read_timeout(timeout.not_zero().map(|duration| *duration))?;
        let count = timed(
            (&*self.stream).read(self.buffers.input_append_buf()),
            timeout,
        )?;
        self.buffers.input_appended(count);
        Ok(count > 0)
    }

    fn is_open(&mut self) -> bool {
        if self.stream.set_nonblocking(true).is_err() {
            return false;
        }
        let open = matches!(self.stream.peek(&mut [0]), Err(error) if error.kind() == io::ErrorKind::WouldBlock);
        self.stream.set_nonblocking(false).is_ok() && open
    }
}

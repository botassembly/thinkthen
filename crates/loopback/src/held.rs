//! Event-driven requests held open until their caller closes the connection.

use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, mpsc};
use std::time::Duration;

use crate::{Loopback, Reply, lock};

/// Which part of the HTTP reply remains incomplete.
#[derive(Debug, Clone, Copy)]
pub enum HeldReply {
    /// No response headers have arrived.
    Headers,
    /// Reset the connection after receiving the request.
    Reset,
    /// Successful response headers have arrived, but the body has not.
    Body,
    /// A refused response needs its body to determine whether to split.
    RefusalBody,
}

pub(super) struct Hold {
    reply: HeldReply,
    chosen: Arc<crate::Chosen>,
    received: mpsc::Sender<()>,
    closed: mpsc::Sender<std::io::Result<usize>>,
}

impl Hold {
    pub(super) fn chosen(&self, request: &crate::Request) -> bool {
        (self.chosen)(request)
    }
    pub(super) fn serve(&self, mut stream: TcpStream) {
        if matches!(self.reply, HeldReply::Reset) {
            let _ = socket2::SockRef::from(&stream).set_linger(Some(Duration::ZERO));
            return;
        }
        // A hang guard releases the fixture if a broken caller never closes.
        let _ = stream.set_read_timeout(Some(Duration::from_secs(30)));
        let status = match self.reply {
            HeldReply::Headers | HeldReply::Reset => None,
            HeldReply::Body => Some(200),
            HeldReply::RefusalBody => Some(400),
        };
        if let Some(status) = status {
            let _ = write!(
                stream,
                "HTTP/1.1 {status} Reply\r\ncontent-length: 100\r\n\r\n"
            );
            let _ = stream.flush();
        }
        let _ = self.received.send(());
        let _ = self.closed.send(stream.read(&mut [0]));
    }
}

impl Loopback {
    /// Hold each request until its connection closes, reporting arrival and EOF.
    ///
    /// # Errors
    /// When no loopback port can be bound.
    pub fn holding(
        reply: HeldReply,
    ) -> std::io::Result<(
        Self,
        mpsc::Receiver<()>,
        mpsc::Receiver<std::io::Result<usize>>,
    )> {
        let server = Self::start(|_| Reply::json(500, &serde_json::json!({})))?;
        let (arrivals, endings) = server.hold(reply);
        Ok((server, arrivals, endings))
    }

    /// Hold subsequent requests, including requests on an existing connection.
    pub fn hold(
        &self,
        reply: HeldReply,
    ) -> (mpsc::Receiver<()>, mpsc::Receiver<std::io::Result<usize>>) {
        self.hold_matching(reply, |_| true)
    }

    /// Hold only selected requests; all other requests use the normal reply rule.
    pub fn hold_matching(
        &self,
        reply: HeldReply,
        chosen: impl Fn(&crate::Request) -> bool + Send + Sync + 'static,
    ) -> (mpsc::Receiver<()>, mpsc::Receiver<std::io::Result<usize>>) {
        let (received, arrivals) = mpsc::channel();
        let (closed, endings) = mpsc::channel();
        lock(&self.state).held = Some(Arc::new(Hold {
            reply,
            chosen: Arc::new(chosen),
            received,
            closed,
        }));
        (arrivals, endings)
    }
}

//! An event-driven peer closes a pooled connection after its next request.
use std::io;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread::{self, JoinHandle};

use serde_json::{Value, json};

use crate::{Reply, Request};

/// The reply boundary at which the peer closes its connection.
#[derive(Clone, Copy, Debug)]
pub enum CloseAt {
    /// Close a new connection without any reply.
    Fresh,
    /// Close a reused connection without any reply.
    Reused,
    /// Close after part of the HTTP status line.
    Headers,
    /// Close after complete headers and part of the reply body.
    Body,
}

/// A peer that answers the first batch, then closes during the next one.
#[derive(Debug)]
pub struct ClosingPeer {
    base: String,
    address: SocketAddr,
    worker: JoinHandle<io::Result<usize>>,
}

impl ClosingPeer {
    /// Start a peer that closes at the requested boundary.
    /// # Errors
    /// Returns listener setup errors.
    pub fn start(at: CloseAt) -> std::io::Result<Self> {
        use std::io::Write;
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let address = listener.local_addr()?;
        let worker = thread::spawn(move || {
            let (mut socket, _) = listener.accept()?;
            let first = crate::read(&socket).ok_or(io::ErrorKind::UnexpectedEof)?;
            let mut received = 1;
            if !matches!(at, CloseAt::Fresh) {
                answer(&socket, &first)?;
                // Force the close past the client's pool probe without a sleep.
                crate::read(&socket).ok_or(io::ErrorKind::UnexpectedEof)?;
                received += 1;
                let partial = match at {
                    CloseAt::Headers => "HTTP/1.1 200",
                    CloseAt::Body => "HTTP/1.1 200 OK\r\nContent-Length: 100\r\n\r\n{",
                    _ => "",
                };
                socket.write_all(partial.as_bytes())?;
            }
            drop(socket);
            loop {
                let (socket, _) = listener.accept()?;
                let Some(body) = crate::read(&socket) else {
                    break;
                };
                received += 1;
                answer(&socket, &body)?;
            }
            Ok(received)
        });
        Ok(Self {
            base: format!("http://{address}/v1"),
            address,
            worker,
        })
    }

    /// The peer's backend address.
    #[must_use]
    pub fn base(&self) -> &str {
        &self.base
    }

    /// Stop after the synchronous client has finished and return requests received.
    /// # Errors
    /// Returns peer exchange or worker failures.
    pub fn finish(self) -> io::Result<usize> {
        drop(TcpStream::connect(self.address)?);
        self.worker
            .join()
            .map_err(|_| io::Error::other("closing peer panicked"))?
    }
}

fn answer(socket: &TcpStream, request: &Request) -> io::Result<()> {
    let body = request.json().map_err(io::Error::other)?;
    let answers: serde_json::Map<String, Value> = body
        .get("questions")
        .and_then(Value::as_object)
        .into_iter()
        .flatten()
        .map(|(name, _)| (name.clone(), json!({"type":"noul","noul":0.9})))
        .collect();
    crate::write(socket, &Reply::json(200, &json!({"answers":answers})), true)
}

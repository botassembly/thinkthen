//! A loopback listener that serves scripted responses and records what it was sent.
#![allow(
    dead_code,
    reason = "every test file compiles this module and each one uses part of it"
)]

use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;

/// One response the listener will serve, in the order the script gives.
pub(crate) struct Canned {
    status: u16,
    body: String,
    location: Option<String>,
    promised: Option<usize>,
}

impl Canned {
    /// Answer with this body and a status of 200.
    pub(crate) fn ok(body: &str) -> Self {
        Self::status(200, body)
    }

    /// Answer with a permanent redirect to somewhere else.
    pub(crate) fn redirect(url: &str) -> Self {
        Self {
            status: 302,
            body: String::new(),
            location: Some(url.to_owned()),
            promised: None,
        }
    }

    /// Promise a body of this length and close the connection without it.
    pub(crate) fn cut_short() -> Self {
        Self {
            status: 200,
            body: String::new(),
            location: None,
            promised: Some(4096),
        }
    }

    /// Answer with this status and this body.
    pub(crate) fn status(status: u16, body: &str) -> Self {
        Self {
            status,
            body: body.to_owned(),
            location: None,
            promised: None,
        }
    }
}

/// One request the listener read, kept for the assertions to compare.
#[derive(Debug)]
pub(crate) struct Recorded {
    pub(crate) line: String,
    pub(crate) headers: Vec<String>,
    pub(crate) body: Vec<u8>,
}

impl Recorded {
    /// Read one header value back, or `None` when the header is absent.
    pub(crate) fn header(&self, name: &str) -> Option<&str> {
        self.headers.iter().find_map(|header| {
            let (found, value) = header.split_once(": ")?;
            found.eq_ignore_ascii_case(name).then_some(value)
        })
    }
}

/// A listener serving one scripted response per connection, then closing.
pub(crate) struct Listener {
    base: String,
    url: String,
    recorded: Receiver<Recorded>,
}

impl Listener {
    /// Serve these responses in order, one per connection, on a free loopback port.
    pub(crate) fn serving(responses: Vec<Canned>) -> io::Result<Self> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let base = format!("http://{}/v1", listener.local_addr()?);
        let url = format!("{base}/systemone");
        let (sender, recorded) = channel();
        thread::spawn(move || serve_script(&listener, responses, &sender));
        Ok(Self {
            base,
            url,
            recorded,
        })
    }

    /// The base a command is given, which the tool posts under.
    pub(crate) fn base(&self) -> &str {
        &self.base
    }

    /// The URL a request reaches this listener at.
    pub(crate) fn url(&self) -> &str {
        &self.url
    }

    /// Every request the listener has read so far, in the order it read them.
    pub(crate) fn requests(&self) -> Vec<Recorded> {
        self.recorded.try_iter().collect()
    }
}

/// Serve one response per connection until the script runs out, then close.
fn serve_script(listener: &TcpListener, responses: Vec<Canned>, sender: &Sender<Recorded>) {
    for canned in responses {
        let Ok((stream, _)) = listener.accept() else {
            return;
        };
        let Some(request) = read_request(&stream) else {
            return;
        };
        if sender.send(request).is_err() {
            return;
        }
        serve(stream, &canned);
    }
}

/// Read one request line, its headers, and the body its content length names.
fn read_request(stream: &TcpStream) -> Option<Recorded> {
    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    reader.read_line(&mut line).ok()?;
    let mut headers = Vec::new();
    let mut length = 0;
    loop {
        let mut header = String::new();
        reader.read_line(&mut header).ok()?;
        let header = header.trim_end().to_owned();
        if header.is_empty() {
            break;
        }
        if let Some(value) = header.to_lowercase().strip_prefix("content-length: ") {
            length = value.trim().parse().ok()?;
        }
        headers.push(header);
    }
    let mut body = vec![0; length];
    reader.read_exact(&mut body).ok()?;
    Some(Recorded {
        line: line.trim_end().to_owned(),
        headers,
        body,
    })
}

/// Write one canned response and close the connection.
fn serve(mut stream: TcpStream, canned: &Canned) {
    let location = canned
        .location
        .as_ref()
        .map_or_else(String::new, |url| format!("location: {url}\r\n"));
    let head = format!(
        "HTTP/1.1 {} X\r\ncontent-type: application/json\r\n{location}content-length: {}\r\nconnection: close\r\n\r\n",
        canned.status,
        canned.promised.unwrap_or(canned.body.len())
    );
    let _ = stream.write_all(head.as_bytes());
    let _ = stream.write_all(canned.body.as_bytes());
    let _ = stream.flush();
}

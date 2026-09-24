//! A loopback listener that serves scripted responses and records what it was sent.
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;

use crate::support::ENDPOINT_PATH;

/// Whether this Linux process has the expected inode open.
#[cfg(target_os = "linux")]
pub(crate) fn process_has_file(process: u32, expected: &std::fs::Metadata) -> io::Result<bool> {
    use std::os::unix::fs::MetadataExt as _;

    let expected = (expected.dev(), expected.ino());
    for descriptor in std::fs::read_dir(format!("/proc/{process}/fd"))? {
        if std::fs::metadata(descriptor?.path())
            .is_ok_and(|metadata| (metadata.dev(), metadata.ino()) == expected)
        {
            return Ok(true);
        }
    }
    Ok(false)
}

/// One response the listener will serve, in the order the script gives.
pub(crate) struct Canned {
    status: u16,
    body: String,
    location: Option<String>,
    promised: Option<usize>,
    delay: Duration,
    release: Option<Arc<Barrier>>,
    answered: Option<Sender<()>>,
    asked: Vec<(String, String)>,
    close_without_reply: bool,
    reset: bool,
}

impl Canned {
    /// Answer with this body and a status of 200.
    pub(crate) fn ok(body: &str) -> Self {
        Self::status(200, body)
    }

    /// Answer with a permanent redirect to somewhere else.
    pub(crate) fn redirect(url: &str) -> Self {
        Self {
            location: Some(url.to_owned()),
            ..Self::status(302, "")
        }
    }

    /// Promise a body of this length and close the connection without it.
    pub(crate) fn cut_short() -> Self {
        Self {
            promised: Some(4096),
            ..Self::ok("")
        }
    }

    /// Read the complete request and close before writing response headers.
    pub(crate) fn close_without_reply() -> Self {
        Self {
            close_without_reply: true,
            ..Self::ok("")
        }
    }

    /// Leave the complete request unread and drop the connection.
    ///
    /// Linux and macOS answer a close over unread bytes with a reset.
    pub(crate) fn reset() -> Self {
        Self {
            reset: true,
            ..Self::ok("")
        }
    }

    /// Answer with this status and this body.
    pub(crate) fn status(status: u16, body: &str) -> Self {
        Self {
            status,
            body: body.to_owned(),
            location: None,
            promised: None,
            delay: Duration::ZERO,
            release: None,
            answered: None,
            asked: Vec::new(),
            close_without_reply: false,
            reset: false,
        }
    }

    /// Wait this long before answering, so a later request can answer first.
    pub(crate) fn after(mut self, millis: u64) -> Self {
        self.delay = Duration::from_millis(millis);
        self
    }

    /// Wait at this barrier before answering, so a test controls the release.
    pub(crate) fn after_release(mut self, release: Arc<Barrier>) -> Self {
        self.release = Some(release);
        self
    }

    /// Announce after this response has been written to the caller.
    pub(crate) fn notifying(mut self, answered: Sender<()>) -> Self {
        self.answered = Some(answered);
        self
    }

    /// Carry one more header, for the two forms a retry wait arrives in.
    pub(crate) fn asking(mut self, name: &str, value: &str) -> Self {
        self.asked.push((name.to_owned(), value.to_owned()));
        self
    }
}

/// What a listener answers one request body with.
pub(crate) type Reply = dyn Fn(&[u8]) -> Canned + Send + Sync;

/// One request or output event, ordered as the scheduling test observes it.
#[derive(Debug)]
pub(crate) enum Observed {
    /// A request reached the listener.
    Request,
    /// One line reached the process reading standard output.
    Output(String),
}

/// What the listener saw, for the assertions that count connections.
#[derive(Debug, Default)]
struct Counts {
    connections: AtomicUsize,
    in_flight: AtomicUsize,
    peak: AtomicUsize,
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
    counts: Arc<Counts>,
}

impl Listener {
    /// Serve these responses in order, one per connection, on a free loopback port.
    pub(crate) fn serving(responses: Vec<Canned>) -> io::Result<Self> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let base = format!("http://{}/v1", listener.local_addr()?);
        let url = format!("{base}/{ENDPOINT_PATH}");
        let (sender, recorded) = channel();
        let counts = Arc::new(Counts::default());
        thread::spawn(move || serve_script(&listener, responses, &sender));
        Ok(Self {
            base,
            url,
            recorded,
            counts,
        })
    }

    /// Answer every connection at once, from the body of each request.
    ///
    /// The connection is kept open until the caller closes it, so a run that
    /// reuses one pool opens one connection and a run with several workers
    /// opens no more than it has workers. Each answer waits the delay its
    /// `Canned` carries, which lets a later request answer first.
    pub(crate) fn answering(
        reply: impl Fn(&[u8]) -> Canned + Send + Sync + 'static,
    ) -> io::Result<Self> {
        Self::answering_observed(reply, None)
    }

    /// Answer every connection and announce each request on one event channel.
    pub(crate) fn answering_with_events(
        reply: impl Fn(&[u8]) -> Canned + Send + Sync + 'static,
        events: Sender<Observed>,
    ) -> io::Result<Self> {
        Self::answering_observed(reply, Some(events))
    }

    /// Build an answering listener with optional request observations.
    fn answering_observed(
        reply: impl Fn(&[u8]) -> Canned + Send + Sync + 'static,
        events: Option<Sender<Observed>>,
    ) -> io::Result<Self> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let base = format!("http://{}/v1", listener.local_addr()?);
        let url = format!("{base}/{ENDPOINT_PATH}");
        let (sender, recorded) = channel();
        let counts = Arc::new(Counts::default());
        let serving = Arc::clone(&counts);
        let reply: Arc<Reply> = Arc::new(reply);
        thread::spawn(move || accept_every(&listener, &reply, &sender, &serving, events.as_ref()));
        Ok(Self {
            base,
            url,
            recorded,
            counts,
        })
    }

    /// How many connections the listener has accepted.
    pub(crate) fn connections(&self) -> usize {
        self.counts.connections.load(Ordering::SeqCst)
    }

    /// The most requests the listener held at once.
    pub(crate) fn peak(&self) -> usize {
        self.counts.peak.load(Ordering::SeqCst)
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
        let Some((request, used)) = peek_request(&stream) else {
            return;
        };
        if sender.send(request).is_err() {
            return;
        }
        if canned.reset {
            continue;
        }
        if consume(&stream, used).is_none() {
            return;
        }
        serve(stream, &canned);
    }
}

/// Take every connection and answer each one in a thread of its own.
fn accept_every(
    listener: &TcpListener,
    reply: &Arc<Reply>,
    sender: &Sender<Recorded>,
    counts: &Arc<Counts>,
    events: Option<&Sender<Observed>>,
) {
    for accepted in listener.incoming() {
        let Ok(stream) = accepted else { return };
        counts.connections.fetch_add(1, Ordering::SeqCst);
        let reply = Arc::clone(reply);
        let counts = Arc::clone(counts);
        let sender = sender.clone();
        let events = events.cloned();
        thread::spawn(move || {
            serve_kept(&stream, reply.as_ref(), &sender, &counts, events.as_ref());
        });
    }
}

/// Answer every request on one connection until the caller closes it.
fn serve_kept(
    stream: &TcpStream,
    reply: &Reply,
    sender: &Sender<Recorded>,
    counts: &Counts,
    events: Option<&Sender<Observed>>,
) {
    loop {
        let Some((request, used)) = peek_request(stream) else {
            return;
        };
        let held = counts.in_flight.fetch_add(1, Ordering::SeqCst) + 1;
        counts.peak.fetch_max(held, Ordering::SeqCst);
        let canned = reply(&request.body);
        if sender.send(request).is_err() {
            return;
        }
        if let Some(events) = events {
            let _ = events.send(Observed::Request);
        }
        if canned.reset || consume(stream, used).is_none() {
            counts.in_flight.fetch_sub(1, Ordering::SeqCst);
            return;
        }
        thread::sleep(canned.delay);
        if let Some(release) = canned.release.as_ref() {
            release.wait();
        }
        write_answer(stream, &canned, false);
        if let Some(answered) = canned.answered.as_ref() {
            let _ = answered.send(());
        }
        counts.in_flight.fetch_sub(1, Ordering::SeqCst);
    }
}

/// Peek until one whole request sits in the socket buffer, then parse it.
///
/// The bytes stay unread, so a reset can drop them. The count says how many
/// bytes the request took, for [`consume`] to read past. A request too long to
/// peek whole is read at once instead, and a reset of it is a plain close.
fn peek_request(stream: &TcpStream) -> Option<(Recorded, usize)> {
    let mut buffer = vec![0; 32 * 1024];
    loop {
        let seen = stream.peek(&mut buffer).ok()?;
        if seen == 0 {
            return None;
        }
        let mut rest = buffer.get(..seen)?;
        if let Some(request) = read_request(&mut rest) {
            return Some((request, seen - rest.len()));
        }
        if seen == buffer.len() {
            return read_request(&mut BufReader::new(stream)).map(|request| (request, 0));
        }
        thread::sleep(Duration::from_millis(1));
    }
}

/// Read past the bytes one peeked request took.
fn consume(mut stream: &TcpStream, used: usize) -> Option<()> {
    stream.read_exact(&mut vec![0; used]).ok()
}

/// Read one request line, its headers, and the body its content length names.
fn read_request(reader: &mut impl BufRead) -> Option<Recorded> {
    let mut line = String::new();
    reader.read_line(&mut line).ok()?;
    let mut headers = Vec::new();
    let mut length = 0;
    loop {
        let mut header = String::new();
        reader.read_line(&mut header).ok()?;
        // A line cut off by the end of what arrived so far is no line yet.
        if !header.ends_with('\n') {
            return None;
        }
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
fn serve(stream: TcpStream, canned: &Canned) {
    if let Some(release) = canned.release.as_ref() {
        release.wait();
    }
    write_answer(&stream, canned, true);
    if let Some(answered) = canned.answered.as_ref() {
        let _ = answered.send(());
    }
}

/// Write one canned response, closing the connection or keeping it open.
fn write_answer(mut stream: &TcpStream, canned: &Canned, closing: bool) {
    if canned.close_without_reply {
        return;
    }
    let location = canned
        .location
        .as_ref()
        .map_or_else(String::new, |url| format!("location: {url}\r\n"));
    let ending = if closing {
        "connection: close\r\n"
    } else {
        "connection: keep-alive\r\n"
    };
    let asked: String = canned
        .asked
        .iter()
        .map(|(name, value)| format!("{name}: {value}\r\n"))
        .collect();
    let head = format!(
        "HTTP/1.1 {} X\r\ncontent-type: application/json\r\n{location}{asked}content-length: {}\r\n{ending}\r\n",
        canned.status,
        canned.promised.unwrap_or(canned.body.len())
    );
    let _ = stream.write_all(head.as_bytes());
    let _ = stream.write_all(canned.body.as_bytes());
    let _ = stream.flush();
}

/// Run the compiled binary with no environment but what the case names.
///
/// Every case on this binary drives the tool as a process, so the spawning,
/// the pipes, and the short retry wait live here once. The wait is set for
/// every run, because a case that never retries is not slowed by it.
pub(crate) fn spawn(
    arguments: &[&str],
    environment: &[(&str, &str)],
    evidence: &[u8],
) -> io::Result<Output> {
    static SPAWNS: AtomicUsize = AtomicUsize::new(0);
    let home = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "spawn-home-{}-{}",
        std::process::id(),
        SPAWNS.fetch_add(1, Ordering::Relaxed)
    ));
    let mut command = Command::new(env!("CARGO_BIN_EXE_thinkthen"));
    command
        .env_clear()
        .env("HOME", home)
        .env("THINKTHEN_TEST_RETRY_WAIT_MS", "1")
        .args(arguments)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for (name, value) in environment {
        command.env(name, value);
    }
    let mut child = command.spawn()?;
    let mut input = child
        .stdin
        .take()
        .ok_or_else(|| io::Error::other("no pipe to standard input"))?;
    let _ = input.write_all(evidence);
    drop(input);
    child.wait_with_output()
}

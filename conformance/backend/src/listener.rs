//! A loopback listener that serves scripted responses and records what it was sent.
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Barrier, Mutex};
use std::thread;
use std::time::Duration;

/// The path the engine appends to every base.
const ENDPOINT_PATH: &str = "systemone";

/// One response the listener will serve, in the order the script gives.
#[derive(Debug)]
pub struct Canned {
    status: u16,
    body: String,
    location: Option<String>,
    promised: Option<usize>,
    delay: Duration,
    release: Option<Arc<Barrier>>,
    held: Option<(Arc<Gate>, u64)>,
    answered: Option<Sender<()>>,
    asked: Vec<(String, String)>,
    close_without_reply: bool,
    reset: bool,
}

impl Canned {
    /// Answer with this body and a status of 200.
    pub fn ok(body: &str) -> Self {
        Self::status(200, body)
    }

    /// Answer with a permanent redirect to somewhere else.
    pub fn redirect(url: &str) -> Self {
        Self {
            location: Some(url.to_owned()),
            ..Self::status(302, "")
        }
    }

    /// Promise a body of this length and close the connection without it.
    pub fn cut_short() -> Self {
        Self {
            promised: Some(4096),
            ..Self::ok("")
        }
    }

    /// Read the complete request and close before writing response headers.
    pub fn close_without_reply() -> Self {
        Self {
            close_without_reply: true,
            ..Self::ok("")
        }
    }

    /// Leave the complete request unread and drop the connection.
    ///
    /// Linux and macOS answer a close over unread bytes with a reset.
    pub fn reset() -> Self {
        Self {
            reset: true,
            ..Self::ok("")
        }
    }

    /// Answer with this status and this body.
    pub fn status(status: u16, body: &str) -> Self {
        Self {
            status,
            body: body.to_owned(),
            location: None,
            promised: None,
            delay: Duration::ZERO,
            release: None,
            held: None,
            answered: None,
            asked: Vec::new(),
            close_without_reply: false,
            reset: false,
        }
    }

    /// Wait this long before answering, so a later request can answer first.
    pub fn after(mut self, millis: u64) -> Self {
        self.delay = Duration::from_millis(millis);
        self
    }

    /// Wait at this barrier before answering, so a test controls the release.
    pub fn after_release(mut self, release: Arc<Barrier>) -> Self {
        self.release = Some(release);
        self
    }

    /// Hold the answer until the gate opens or its round moves past this one.
    pub(crate) fn held_by(mut self, gate: Arc<Gate>) -> Self {
        let round = gate.round.load(Ordering::SeqCst);
        self.held = Some((gate, round));
        self
    }

    /// Announce after this response has been written to the caller.
    pub fn notifying(mut self, answered: Sender<()>) -> Self {
        self.answered = Some(answered);
        self
    }

    /// Carry one more header, for the two forms a retry wait arrives in.
    pub fn asking(mut self, name: &str, value: &str) -> Self {
        self.asked.push((name.to_owned(), value.to_owned()));
        self
    }
}

/// What a listener answers one request with.
pub(crate) type Reply = dyn Fn(&Recorded) -> Canned + Send + Sync;

/// A gate that holds answers until the next round, or for good once released.
#[derive(Debug, Default)]
pub(crate) struct Gate {
    open: AtomicBool,
    round: AtomicU64,
}

impl Gate {
    /// Let every held answer go, now and from here on.
    pub(crate) fn release(&self) {
        self.open.store(true, Ordering::SeqCst);
    }

    /// Let go every answer held now. A later answer holds again.
    pub(crate) fn next_round(&self) {
        self.round.fetch_add(1, Ordering::SeqCst);
    }

    /// Whether an answer taken in this round still waits.
    fn holds(&self, round: u64) -> bool {
        !self.open.load(Ordering::SeqCst) && self.round.load(Ordering::SeqCst) <= round
    }
}

/// One request or output event, ordered as the scheduling test observes it.
#[derive(Debug)]
pub enum Observed {
    /// A request reached the listener.
    Request,
    /// One line reached the process reading standard output.
    Output(String),
}

/// What the listener saw, for the assertions that count connections.
#[derive(Debug, Default)]
struct Counts {
    connections: AtomicUsize,
    requests: AtomicUsize,
    in_flight: AtomicUsize,
    peak: AtomicUsize,
}

/// One request the listener read, kept for the assertions to compare.
#[derive(Debug)]
pub struct Recorded {
    /// The request line, such as `POST /v1/systemone HTTP/1.1`.
    pub line: String,
    /// Each header line, without its line ending.
    pub headers: Vec<String>,
    /// The body its content length named.
    pub body: Vec<u8>,
}

impl Recorded {
    /// Read one header value back, or `None` when the header is absent.
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers.iter().find_map(|header| {
            let (found, value) = header.split_once(": ")?;
            found.eq_ignore_ascii_case(name).then_some(value)
        })
    }
}

/// A listener serving one scripted response per connection, then closing.
#[derive(Debug)]
pub struct Listener {
    origin: String,
    base: String,
    url: String,
    recorded: Mutex<Receiver<Recorded>>,
    counts: Arc<Counts>,
}

impl Listener {
    /// Serve these responses in order, one per connection, on a free loopback port.
    pub fn serving(responses: Vec<Canned>) -> io::Result<Self> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let origin = format!("http://{}", listener.local_addr()?);
        let base = format!("{origin}/v1");
        let url = format!("{base}/{ENDPOINT_PATH}");
        let (sender, recorded) = channel();
        let counts = Arc::new(Counts::default());
        thread::spawn(move || serve_script(&listener, responses, &sender));
        Ok(Self {
            origin,
            base,
            url,
            recorded: Mutex::new(recorded),
            counts,
        })
    }

    /// Answer every connection at once, from the body of each request.
    ///
    /// The connection is kept open until the caller closes it, so a run that
    /// reuses one pool opens one connection and a run with several workers
    /// opens no more than it has workers. Each answer waits the delay its
    /// `Canned` carries, which lets a later request answer first.
    pub fn answering(reply: impl Fn(&[u8]) -> Canned + Send + Sync + 'static) -> io::Result<Self> {
        Self::answering_observed(move |request: &Recorded| reply(&request.body), None, true)
    }

    /// Answer every connection from the whole request, and keep no copy of it.
    ///
    /// A long run through the conformance backend holds only its counts.
    pub(crate) fn routing(
        reply: impl Fn(&Recorded) -> Canned + Send + Sync + 'static,
    ) -> io::Result<Self> {
        Self::answering_observed(reply, None, false)
    }

    /// Answer every connection and announce each request on one event channel.
    pub fn answering_with_events(
        reply: impl Fn(&[u8]) -> Canned + Send + Sync + 'static,
        events: Sender<Observed>,
    ) -> io::Result<Self> {
        Self::answering_observed(
            move |request: &Recorded| reply(&request.body),
            Some(events),
            true,
        )
    }

    /// Build an answering listener with optional request observations.
    fn answering_observed(
        reply: impl Fn(&Recorded) -> Canned + Send + Sync + 'static,
        events: Option<Sender<Observed>>,
        keeping: bool,
    ) -> io::Result<Self> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let origin = format!("http://{}", listener.local_addr()?);
        let base = format!("{origin}/v1");
        let url = format!("{base}/{ENDPOINT_PATH}");
        let (sender, recorded) = channel();
        let sender = keeping.then_some(sender);
        let counts = Arc::new(Counts::default());
        let serving = Arc::clone(&counts);
        let reply: Arc<Reply> = Arc::new(reply);
        thread::spawn(move || {
            accept_every(
                &listener,
                &reply,
                sender.as_ref(),
                &serving,
                events.as_ref(),
            );
        });
        Ok(Self {
            origin,
            base,
            url,
            recorded: Mutex::new(recorded),
            counts,
        })
    }

    /// How many connections the listener has accepted.
    pub fn connections(&self) -> usize {
        self.counts.connections.load(Ordering::SeqCst)
    }

    /// How many requests the listener has read on kept connections.
    pub fn count(&self) -> usize {
        self.counts.requests.load(Ordering::SeqCst)
    }

    /// The most requests the listener held at once.
    pub fn peak(&self) -> usize {
        self.counts.peak.load(Ordering::SeqCst)
    }

    /// The scheme, address, and port, with no path.
    pub fn origin(&self) -> &str {
        &self.origin
    }

    /// The base a command is given, which the tool posts under.
    pub fn base(&self) -> &str {
        &self.base
    }

    /// The URL a request reaches this listener at.
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Every request the listener has read so far, in the order it read them.
    pub fn requests(&self) -> Vec<Recorded> {
        // The lock keeps a listener shareable across threads.
        self.recorded
            .lock()
            .map(|recorded| recorded.try_iter().collect())
            .unwrap_or_default()
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
    sender: Option<&Sender<Recorded>>,
    counts: &Arc<Counts>,
    events: Option<&Sender<Observed>>,
) {
    for accepted in listener.incoming() {
        let Ok(stream) = accepted else { return };
        counts.connections.fetch_add(1, Ordering::SeqCst);
        let reply = Arc::clone(reply);
        let counts = Arc::clone(counts);
        let sender = sender.cloned();
        let events = events.cloned();
        thread::spawn(move || {
            serve_kept(
                &stream,
                reply.as_ref(),
                sender.as_ref(),
                &counts,
                events.as_ref(),
            );
        });
    }
}

/// Answer every request on one connection until the caller closes it.
fn serve_kept(
    stream: &TcpStream,
    reply: &Reply,
    sender: Option<&Sender<Recorded>>,
    counts: &Counts,
    events: Option<&Sender<Observed>>,
) {
    loop {
        let Some((request, used)) = peek_request(stream) else {
            return;
        };
        // Choose before counting: a held answer takes its round number first,
        // so a `round` sent after the count reads this request lets it go.
        let canned = reply(&request);
        counts.requests.fetch_add(1, Ordering::SeqCst);
        let held = counts.in_flight.fetch_add(1, Ordering::SeqCst) + 1;
        counts.peak.fetch_max(held, Ordering::SeqCst);
        if sender.is_some_and(|sender| sender.send(request).is_err()) {
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
        while canned
            .held
            .as_ref()
            .is_some_and(|(gate, round)| gate.holds(*round))
        {
            thread::sleep(Duration::from_millis(5));
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

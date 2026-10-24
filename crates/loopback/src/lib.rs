//! A loopback backend for tests. It answers each request by a rule the test
//! passes in and records every request it receives.

use std::fmt;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex, PoisonError};
use std::thread;
use std::time::Duration;

use serde_json::{Map, Value, json};
use socket2::{Domain, Protocol, Socket, Type};

/// The API key tests send. Output must never show it.
pub const KEY: &str = "sk-test-0123456789";

/// An unavailable backend whose TCP port stays reserved until it is dropped.
#[derive(Debug)]
pub struct UnavailableBackend {
    _reservation: Socket,
    base: String,
}

impl UnavailableBackend {
    /// Reserve a loopback TCP port without listening for connections.
    ///
    /// # Errors
    ///
    /// When the socket cannot be created, bound or inspected.
    pub fn new() -> std::io::Result<Self> {
        let socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))?;
        let address = std::net::SocketAddrV4::new(std::net::Ipv4Addr::LOCALHOST, 0);
        socket.bind(&address.into())?;
        let address = socket.local_addr()?.as_socket_ipv4().ok_or_else(|| {
            std::io::Error::other("the reserved loopback socket has no IPv4 address")
        })?;
        Ok(Self {
            _reservation: socket,
            base: format!("http://{address}/v1"),
        })
    }

    /// The base URL that refuses connections while this fixture is held.
    #[must_use]
    pub fn base(&self) -> &str {
        &self.base
    }
}

/// One request the server received.
#[derive(Debug, Clone)]
pub struct Request {
    /// The request path, such as `/v1/systemone`.
    pub path: String,
    /// Each header's name in lowercase, with its value.
    pub headers: Vec<(String, String)>,
    /// The body bytes.
    pub body: Vec<u8>,
}

impl Request {
    /// The value of the header `name`, given in lowercase.
    #[must_use]
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(held, _)| held == name)
            .map(|(_, value)| value.as_str())
    }

    /// The body read as JSON.
    ///
    /// # Errors
    ///
    /// When the body is not JSON.
    pub fn json(&self) -> serde_json::Result<serde_json::Value> {
        serde_json::from_slice(&self.body)
    }
}

/// The reply the server sends.
#[derive(Debug, Clone)]
pub struct Reply {
    /// The HTTP status.
    pub status: u16,
    /// The body bytes.
    pub body: Vec<u8>,
    headers: Vec<(String, String)>,
}

impl Reply {
    /// A reply with `status` and `value` as its JSON body.
    #[must_use]
    pub fn json(status: u16, value: &serde_json::Value) -> Self {
        Self {
            status,
            body: value.to_string().into_bytes(),
            headers: Vec::new(),
        }
    }

    /// A redirect response to `location` with no body.
    #[must_use]
    pub fn redirect(status: u16, location: &str) -> Self {
        Self {
            status,
            body: Vec::new(),
            headers: vec![("location".to_owned(), location.to_owned())],
        }
    }
}

type Rule = dyn Fn(&Request) -> Reply + Send + Sync;

type Chosen = dyn Fn(&Request) -> bool + Send + Sync;

/// What the server has seen, and the requests it holds back.
#[derive(Default)]
struct State {
    seen: Vec<Request>,
    delay: Option<(Duration, Arc<Chosen>)>,
    held: Option<Arc<held::Hold>>,
    keep_alive: bool,
    connections: usize,
}

impl fmt::Debug for State {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("State")
            .field("seen", &self.seen.len())
            .finish_non_exhaustive()
    }
}

/// A running loopback server. It stops when the test process ends.
#[derive(Debug)]
pub struct Loopback {
    base: String,
    state: Arc<Mutex<State>>,
}

impl Loopback {
    /// Start a server that answers each request with `rule`.
    ///
    /// # Errors
    ///
    /// When no loopback port can be bound.
    pub fn start(
        rule: impl Fn(&Request) -> Reply + Send + Sync + 'static,
    ) -> std::io::Result<Self> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let base = format!("http://{}/v1", listener.local_addr()?);
        let state = Arc::new(Mutex::new(State::default()));
        let rule: Arc<Rule> = Arc::new(rule);
        let recorder = Arc::clone(&state);
        thread::spawn(move || {
            for stream in listener.incoming().flatten() {
                let rule = Arc::clone(&rule);
                let recorder = Arc::clone(&recorder);
                thread::spawn(move || serve(stream, rule.as_ref(), &recorder));
            }
        });
        Ok(Self { base, state })
    }

    /// Start a System One server that answers each wire question with `rule`.
    ///
    /// # Errors
    ///
    /// When no loopback port can be bound.
    pub fn answering(
        rule: impl Fn(&Value) -> Value + Send + Sync + 'static,
    ) -> std::io::Result<Self> {
        Self::start(move |request| {
            let body = request.json().unwrap_or_default();
            let answers: Map<String, Value> = body
                .get("questions")
                .and_then(Value::as_object)
                .into_iter()
                .flatten()
                .map(|(name, question)| (name.clone(), rule(question)))
                .collect();
            Reply::json(
                200,
                &json!({"model": body.get("model"), "answers": answers}),
            )
        })
    }

    /// Start a System One server that gives every yes or no question the probability `p`.
    ///
    /// # Errors
    ///
    /// When no loopback port can be bound.
    pub fn noul(p: f64) -> std::io::Result<Self> {
        Self::answering(move |_| json!({"type": "noul", "noul": p}))
    }

    /// The base address to send to, such as `http://127.0.0.1:4000/v1`.
    #[must_use]
    pub fn base(&self) -> &str {
        &self.base
    }

    /// Start a System One server that says yes to any text starting with
    /// "refund" and no to any other.
    ///
    /// # Errors
    ///
    /// When no loopback port can be bound.
    pub fn refunds() -> std::io::Result<Self> {
        Self::answering(|question| {
            let yes = question["instructions"]
                .as_str()
                .is_some_and(|text| text.contains("text is \"refund"));
            json!({"type": "noul", "noul": if yes { 0.9 } else { 0.1 }})
        })
    }

    /// Hold back by `by` the reply to each request that `chosen` picks.
    pub fn delay(&self, by: Duration, chosen: impl Fn(&Request) -> bool + Send + Sync + 'static) {
        lock(&self.state).delay = Some((by, Arc::new(chosen)));
    }

    /// Keep successful exchanges open so subsequent requests can reuse the socket.
    pub fn keep_alive(&self) {
        lock(&self.state).keep_alive = true;
    }

    /// How many TCP connections the server has accepted.
    #[must_use]
    pub fn connections(&self) -> usize {
        lock(&self.state).connections
    }

    /// Every request received so far, in arrival order.
    #[must_use]
    pub fn requests(&self) -> Vec<Request> {
        lock(&self.state).seen.clone()
    }

    /// How many requests arrived so far.
    #[must_use]
    pub fn count(&self) -> usize {
        lock(&self.state).seen.len()
    }
}

fn lock(state: &Mutex<State>) -> std::sync::MutexGuard<'_, State> {
    state.lock().unwrap_or_else(PoisonError::into_inner)
}

fn serve(stream: TcpStream, rule: &Rule, state: &Mutex<State>) {
    lock(state).connections += 1;
    while let Some(request) = read(&stream) {
        let (delay, held, keep_alive) = {
            let mut state = lock(state);
            state.seen.push(request.clone());
            (state.delay.clone(), state.held.clone(), state.keep_alive)
        };
        if let Some(held) = held {
            held.serve(stream);
            return;
        }
        if let Some((by, chosen)) = delay
            && chosen(&request)
        {
            thread::sleep(by);
        }
        let reply = rule(&request);
        // A client that hung up needs no reply.
        if write(&stream, &reply, keep_alive).is_err() || !keep_alive {
            return;
        }
    }
}

fn read(stream: &TcpStream) -> Option<Request> {
    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    reader.read_line(&mut line).ok()?;
    let path = line.split_whitespace().nth(1)?.to_owned();
    let mut headers = Vec::new();
    loop {
        line.clear();
        reader.read_line(&mut line).ok()?;
        let header = line.trim_end();
        if header.is_empty() {
            break;
        }
        let (name, value) = header.split_once(':')?;
        headers.push((name.trim().to_ascii_lowercase(), value.trim().to_owned()));
    }
    let length = headers
        .iter()
        .find(|(name, _)| name == "content-length")
        .and_then(|(_, value)| value.parse::<usize>().ok())
        .unwrap_or(0);
    let mut body = vec![0; length];
    reader.read_exact(&mut body).ok()?;
    Some(Request {
        path,
        headers,
        body,
    })
}

fn write(mut stream: &TcpStream, reply: &Reply, keep_alive: bool) -> std::io::Result<()> {
    write!(
        stream,
        "HTTP/1.1 {} Reply\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: {}\r\n",
        reply.status,
        reply.body.len(),
        if keep_alive { "keep-alive" } else { "close" }
    )?;
    for (name, value) in &reply.headers {
        write!(stream, "{name}: {value}\r\n")?;
    }
    write!(stream, "\r\n")?;
    stream.write_all(&reply.body)?;
    stream.flush()
}

mod case;

/// Remove route metadata when an existing case asserts only function behavior.
/// Shared contract and construction cases retain complete source metadata.
#[must_use]
pub fn without_source(mut value: serde_json::Value) -> serde_json::Value {
    if let Some(rows) = value.as_array_mut() {
        for row in rows {
            *row = without_source(row.take());
        }
    } else if value.get("status").is_some()
        && value
            .get("detail")
            .and_then(|detail| detail.get("source"))
            .is_some()
    {
        let probabilities = value
            .get_mut("detail")
            .and_then(|detail| detail.as_object_mut())
            .and_then(|detail| detail.remove("probabilities"));
        if let Some(probabilities) = probabilities
            && let Some(row) = value.as_object_mut()
        {
            row.insert("detail".to_owned(), probabilities);
        } else if let Some(row) = value.as_object_mut() {
            row.remove("detail");
        }
    }
    value
}

mod held;
pub use held::HeldReply;

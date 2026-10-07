use super::super::{
    admission::Invocation,
    runtime::{self, Executor, NativeReply},
};
use crate::CancelToken;
use serde_json::{Value, json};
use std::io::{self, BufReader, Cursor, Read, Write};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
    mpsc,
};
use std::thread::{self, JoinHandle};
use std::time::Duration;

const BOUND: Duration = Duration::from_secs(3);

struct Input {
    receiver: mpsc::Receiver<Vec<u8>>,
    cursor: Cursor<Vec<u8>>,
    stop: CancelToken,
}
impl Read for Input {
    fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
        loop {
            let size = self.cursor.read(bytes)?;
            if size != 0 {
                return Ok(size);
            }
            if self.stop.is_cancelled() {
                return Ok(0);
            }
            match self.receiver.recv_timeout(Duration::from_millis(10)) {
                Ok(bytes) => self.cursor = Cursor::new(bytes),
                Err(mpsc::RecvTimeoutError::Timeout) => continue,
                Err(mpsc::RecvTimeoutError::Disconnected) => return Ok(0),
            }
        }
    }
}

struct Output {
    sender: mpsc::Sender<Value>,
    bytes: Vec<u8>,
    broken: Arc<AtomicUsize>,
}
impl Write for Output {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if self.broken.load(Ordering::Acquire) != 0 {
            return Err(io::ErrorKind::BrokenPipe.into());
        }
        self.bytes.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        let value = serde_json::from_slice(&self.bytes).map_err(io::Error::other)?;
        self.bytes.clear();
        self.sender.send(value).map_err(io::Error::other)
    }
}

struct Fixture {
    started: mpsc::Sender<crate::mcp::tools::Tool>,
    retired: Arc<AtomicUsize>,
    hold: bool,
}
impl Executor for Fixture {
    fn output_schema(&self) -> Value {
        json!({"type":"object","required":["fixture"]})
    }
    fn execute(
        &self,
        invocation: Invocation,
        token: &CancelToken,
    ) -> Result<NativeReply, crate::Error> {
        self.started.send(invocation.tool).unwrap();
        while self.hold && !token.is_cancelled() {
            thread::sleep(Duration::from_millis(5));
        }
        self.retired.fetch_add(1, Ordering::AcqRel);
        Ok(NativeReply {
            object: super::super::output::NativeObject::new(&json!({"fixture":true,"value":false}))
                .unwrap(),
            failed: false,
        })
    }
}

struct Session {
    input: Option<mpsc::Sender<Vec<u8>>>,
    output: mpsc::Receiver<Value>,
    started: mpsc::Receiver<crate::mcp::tools::Tool>,
    retired: Arc<AtomicUsize>,
    broken: Arc<AtomicUsize>,
    done: mpsc::Receiver<io::Result<()>>,
    thread: Option<JoinHandle<()>>,
}
impl Session {
    fn new(hold: bool) -> Self {
        let (sender, receiver) = mpsc::channel();
        let (output, outputs) = mpsc::channel();
        let (started, starts) = mpsc::channel();
        let retired = Arc::new(AtomicUsize::new(0));
        let broken = Arc::new(AtomicUsize::new(0));
        let writer = Output {
            sender: output,
            bytes: Vec::new(),
            broken: Arc::clone(&broken),
        };
        let fixture = Fixture {
            started,
            retired: Arc::clone(&retired),
            hold,
        };
        let (done, completion) = mpsc::channel();
        let thread = thread::spawn(move || {
            let result = runtime::serve(
                move |stop| {
                    Ok(BufReader::new(Input {
                        receiver,
                        cursor: Cursor::new(Vec::new()),
                        stop,
                    }))
                },
                |_| Ok(writer),
                fixture,
            );
            done.send(result).unwrap();
        });
        Self {
            input: Some(sender),
            output: outputs,
            started: starts,
            retired,
            broken,
            done: completion,
            thread: Some(thread),
        }
    }
    fn send(&self, value: Value) {
        self.input
            .as_ref()
            .unwrap()
            .send(format!("{value}\n").into_bytes())
            .unwrap();
    }
    fn receive(&self) -> Value {
        self.output
            .recv_timeout(BOUND)
            .expect("bounded protocol response")
    }
    fn initialize(&self, version: &str) {
        self.send(json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{
            "protocolVersion":version,"capabilities":{},"clientInfo":{"name":"fixture","version":"1"}}}));
        let reply = self.receive();
        assert_eq!(reply["result"]["protocolVersion"], "2025-11-25");
        assert_eq!(
            reply["result"]["capabilities"],
            json!({"tools":{"listChanged":false}})
        );
        self.send(json!({"jsonrpc":"2.0","method":"notifications/initialized"}));
    }
    fn call(&self, id: Value, tool: &str) {
        self.send(
            json!({"jsonrpc":"2.0","id":id,"method":"tools/call","params":{
            "name":tool,"arguments":{"question":"literal","evidence":"private-evidence"}}}),
        );
    }
    fn finish(&mut self) -> io::Result<()> {
        self.input.take();
        let result = self.done.recv_timeout(BOUND).expect("joined owned workers");
        self.thread.take().unwrap().join().unwrap();
        result
    }
}
impl Drop for Session {
    fn drop(&mut self) {
        if self.thread.is_some() {
            self.input.take();
            let _result = self.done.recv_timeout(BOUND);
            if let Some(handle) = self.thread.take() {
                handle.join().unwrap();
            }
        }
    }
}

#[test]
fn lifecycle_negotiates_supported_version_and_gates_tools_until_initialized() {
    let mut session = Session::new(false);
    session.call(json!(2), "decide");
    assert_eq!(session.receive()["error"]["code"], -32000);
    session.send(json!({"jsonrpc":"2.0","id":3,"method":"tools/list"}));
    assert_eq!(session.receive()["error"]["code"], -32000);
    session.initialize("unsupported-client-version");
    session.send(json!({"jsonrpc":"2.0","id":4,"method":"tools/list"}));
    assert_eq!(
        session.receive()["result"]["tools"]
            .as_array()
            .unwrap()
            .len(),
        10
    );
    session.call(json!(5), "decide");
    let response = session.receive();
    assert_eq!(response["id"], 5);
    assert_eq!(response["result"]["isError"], false);
    assert_eq!(response["result"]["structuredContent"]["value"], false);
    session.finish().unwrap();
}

#[test]
fn unknown_methods_and_bad_arguments_keep_payloads_out_of_errors() {
    let mut session = Session::new(false);
    session.initialize("2025-11-25");
    for method in ["resources/list", "prompts/list", "private-key-in-method"] {
        session.send(json!({"jsonrpc":"2.0","id":2,"method":method}));
        let reply = session.receive();
        assert_eq!(reply["error"]["code"], -32601);
        assert!(!reply.to_string().contains("private-key"));
    }
    session.send(json!({"jsonrpc":"2.0","id":3,"method":"tools/call","params":{
        "name":"decide","arguments":{"question":"q","evidence":"private-evidence","api_key":"fake-secret"}}}));
    let reply = session.receive();
    assert_eq!(reply["error"]["code"], -32602);
    assert!(!reply.to_string().contains("fake-secret"));
    assert!(!reply.to_string().contains("private-evidence"));
    assert_eq!(session.retired.load(Ordering::Acquire), 0);
    session.finish().unwrap();
}

#[test]
fn cancellation_is_responsive_during_work_and_does_not_cancel_another_id() {
    let mut session = Session::new(true);
    session.initialize("2025-11-25");
    session.call(json!(17), "decide");
    session.started.recv_timeout(BOUND).unwrap();
    session.send(json!({"jsonrpc":"2.0","method":"notifications/cancelled","params":{"requestId":"17","reason":"fake-secret"}}));
    session.send(json!({"jsonrpc":"2.0","id":18,"method":"ping"}));
    assert_eq!(
        session.receive(),
        json!({"jsonrpc":"2.0","id":18,"result":{}})
    );
    assert_eq!(session.retired.load(Ordering::Acquire), 0);
    session.call(json!(19), "decide");
    assert_eq!(session.receive()["error"]["code"], -32001);
    session.send(json!({"jsonrpc":"2.0","method":"notifications/cancelled","params":{"requestId":17,"reason":"fake-secret"}}));
    let due = std::time::Instant::now() + BOUND;
    while session.retired.load(Ordering::Acquire) == 0 && std::time::Instant::now() < due {
        thread::sleep(Duration::from_millis(5));
    }
    assert_eq!(session.retired.load(Ordering::Acquire), 1);
    session.send(json!({"jsonrpc":"2.0","id":20,"method":"ping"}));
    assert_eq!(
        session.receive()["id"],
        20,
        "cancelled request has no response"
    );
    session.finish().unwrap();
}

#[test]
fn eof_cancels_and_joins_only_the_owned_dispatch_work() {
    let mut session = Session::new(true);
    session.initialize("2025-11-25");
    session.call(json!(2), "decide");
    session.started.recv_timeout(BOUND).unwrap();
    session.finish().unwrap();
    assert_eq!(session.retired.load(Ordering::Acquire), 1);
    assert!(session.output.try_recv().is_err());
}

#[test]
fn broken_output_unblocks_reader_and_joins_owned_dispatch_without_more_input() {
    let mut session = Session::new(true);
    session.initialize("2025-11-25");
    session.call(json!(2), "decide");
    session.started.recv_timeout(BOUND).unwrap();
    session.broken.store(1, Ordering::Release);
    session.send(json!({"jsonrpc":"2.0","id":3,"method":"ping"}));
    // Keep input open: output failure itself must stop and join the reader.
    let result = session
        .done
        .recv_timeout(BOUND)
        .expect("broken output stopped reader");
    assert_eq!(result.unwrap_err().kind(), io::ErrorKind::BrokenPipe);
    session.thread.take().unwrap().join().unwrap();
    assert_eq!(session.retired.load(Ordering::Acquire), 1);
}

#[cfg(unix)]
#[test]
fn cancellation_remains_readable_behind_full_output_and_eof_joins_every_owned_thread() {
    use super::super::input::{PollInput, PollOutput};
    use nix::poll::{PollFd, PollFlags, poll};
    use std::io::BufRead;
    use std::os::fd::AsFd;
    let (mut caller, input) = std::os::unix::net::UnixStream::pair().unwrap();
    let (read, write) = nix::unistd::pipe().unwrap();
    let mut replies = BufReader::new(std::fs::File::from(read));
    let output = std::fs::File::from(write);
    let mut filler = output.try_clone().unwrap();
    let (started, starts) = mpsc::channel();
    let retired = Arc::new(AtomicUsize::new(0));
    let fixture = Fixture {
        started,
        retired: Arc::clone(&retired),
        hold: true,
    };
    let (sent, done) = mpsc::channel();
    let server = thread::spawn(move || {
        sent.send(runtime::serve(
            move |stop| Ok(BufReader::new(PollInput::new(input, stop))),
            move |stop| Ok(PollOutput::new(output, stop)),
            fixture,
        ))
        .unwrap();
    });
    let initialize = json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{
        "protocolVersion":"2025-11-25","capabilities":{},"clientInfo":{"name":"fixture","version":"1"}}});
    writeln!(caller, "{initialize}").unwrap();
    let mut line = String::new();
    replies.read_line(&mut line).unwrap();
    assert_eq!(serde_json::from_str::<Value>(&line).unwrap()["id"], 1);
    writeln!(
        caller,
        "{}",
        json!({"jsonrpc":"2.0","method":"notifications/initialized"})
    )
    .unwrap();
    writeln!(
        caller,
        "{}",
        json!({"jsonrpc":"2.0","id":2,"method":"tools/call","params":{
        "name":"decide","arguments":{"question":"q","evidence":"x"}}})
    )
    .unwrap();
    starts.recv_timeout(BOUND).unwrap();
    let mut full = false;
    for _ in 0..256 {
        if poll(&mut [PollFd::new(filler.as_fd(), PollFlags::POLLOUT)], 0u16).unwrap() == 0 {
            full = true;
            break;
        }
        filler.write_all(&[b'x'; 512]).unwrap();
    }
    assert!(full, "output backpressure established");
    writeln!(
        caller,
        "{}",
        json!({"jsonrpc":"2.0","id":3,"method":"ping"})
    )
    .unwrap();
    // An invalid envelope queues an error; it cannot block the input reader
    // trying to print that error ahead of the following cancellation.
    writeln!(caller, "{{\"private\":\"withheld\"}}").unwrap();
    writeln!(
        caller,
        "{}",
        json!({"jsonrpc":"2.0","method":"notifications/cancelled","params":{"requestId":2}})
    )
    .unwrap();
    let due = std::time::Instant::now() + BOUND;
    while retired.load(Ordering::Acquire) == 0 && std::time::Instant::now() < due {
        thread::sleep(Duration::from_millis(5));
    }
    assert_eq!(
        retired.load(Ordering::Acquire),
        1,
        "cancel read while output peer remains unread"
    );
    assert!(
        done.try_recv().is_err(),
        "session still owns blocked protocol output"
    );
    drop(caller); // EOF itself interrupts output, with its peer still open.
    assert_eq!(
        done.recv_timeout(BOUND).unwrap().unwrap_err().kind(),
        io::ErrorKind::BrokenPipe
    );
    server.join().unwrap();
    drop(filler);
}

#[test]
fn reader_factory_failure_stops_session_before_dispatch_and_is_joined() {
    let (started, starts) = mpsc::channel();
    let fixture = Fixture {
        started,
        retired: Arc::new(AtomicUsize::new(0)),
        hold: false,
    };
    let result = runtime::serve(
        |_| Err::<BufReader<Cursor<Vec<u8>>>, _>(io::Error::other("input unavailable")),
        |_| Ok(Vec::<u8>::new()),
        fixture,
    );
    assert_eq!(result.unwrap_err().to_string(), "input unavailable");
    assert!(starts.try_recv().is_err());
}

use crate::child::ChildEnvironment as _;
use serde::Serialize;
use serde_json::{Value, json};
use std::{
    io::{BufRead, BufReader, Read, Write},
    path::PathBuf,
    process::{Child, ChildStdin, Command, Stdio},
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

const BOUND: Duration = Duration::from_secs(3);
static NEXT: AtomicUsize = AtomicUsize::new(0);

pub(super) struct Session {
    process: Child,
    input: Option<ChildStdin>,
    replies: mpsc::Receiver<Value>,
    reader: Option<JoinHandle<()>>,
    errors: Option<JoinHandle<Vec<u8>>>,
    pub(super) home: PathBuf,
    id: u64,
}

#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    reason = "subprocess fixture setup, bounded reads and fixed content positions must stop a failing behavior test"
)]
impl Session {
    pub(super) fn launch(base: &str) -> Self {
        let home = std::env::temp_dir().join(format!(
            "thinkthen-mcp-stdio-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&home).unwrap();
        let mut process = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
            .args(["mcp", "--url", base, "--no-cache", "--max-retries", "0"])
            .clear_environment()
            .isolated_home(&home)
            .env("LANG", "C.UTF-8")
            .env("THINKTHEN_API_KEY", "sk-mcp-loopback-only")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let input = process.stdin.take();
        let output = process.stdout.take().unwrap();
        let mut stderr = process.stderr.take().unwrap();
        let (sender, replies) = mpsc::channel();
        let reader = thread::spawn(move || {
            for line in BufReader::new(output).lines() {
                let reply = serde_json::from_str(&line.unwrap()).unwrap();
                sender.send(reply).unwrap();
            }
        });
        let errors = thread::spawn(move || {
            let mut bytes = Vec::new();
            stderr.read_to_end(&mut bytes).unwrap();
            bytes
        });
        Self {
            process,
            input,
            replies,
            reader: Some(reader),
            errors: Some(errors),
            home,
            id: 0,
        }
    }

    pub(super) fn send(&mut self, message: impl Serialize) {
        let input = self.input.as_mut().unwrap();
        // Raw saved questions retain authored order; remove formatting LF outside JSON strings.
        let encoded = serde_json::to_string(&message).unwrap();
        let compact: String = encoded.lines().map(str::trim).collect();
        input.write_all(compact.as_bytes()).unwrap();
        input.write_all(b"\n").unwrap();
        input.flush().unwrap();
    }

    pub(super) fn receive(&self) -> Value {
        self.replies
            .recv_timeout(BOUND)
            .expect("MCP response within routine bound")
    }

    pub(super) fn request(&mut self, method: &str, params: impl Serialize) -> Value {
        #[derive(Serialize)]
        struct Request<'a, T> {
            jsonrpc: &'static str,
            id: u64,
            method: &'a str,
            params: T,
        }
        self.id += 1;
        self.send(Request {
            jsonrpc: "2.0",
            id: self.id,
            method,
            params,
        });
        let reply = self.receive();
        assert_eq!(reply["id"], self.id);
        reply
    }

    pub(super) fn initialize(&mut self) {
        let reply = self.request(
            "initialize",
            json!({
                "protocolVersion":"2025-11-25","capabilities":{},
                "clientInfo":{"name":"outside-in","version":"1"}
            }),
        );
        assert_eq!(reply["result"]["protocolVersion"], "2025-11-25");
        self.send(json!({"jsonrpc":"2.0","method":"notifications/initialized"}));
    }

    pub(super) fn call(&mut self, tool: &str, arguments: impl Serialize) -> Value {
        #[derive(Serialize)]
        struct Params<'a, T> {
            name: &'a str,
            arguments: T,
        }
        let reply = self.request(
            "tools/call",
            Params {
                name: tool,
                arguments,
            },
        );
        assert!(reply.get("error").is_none(), "{reply}");
        let result = reply["result"].clone();
        let text: Value =
            serde_json::from_str(result["content"][0]["text"].as_str().unwrap()).unwrap();
        assert_eq!(text, result["structuredContent"]);
        result
    }

    pub(super) fn finish(mut self) {
        drop(self.input.take());
        let output =
            crate::child::wait::finish_after(&mut self.process, "thinkthen mcp", BOUND).unwrap();
        assert!(output.status.success());
        self.reader.take().unwrap().join().unwrap();
        assert!(self.errors.take().unwrap().join().unwrap().is_empty());
        std::fs::remove_dir_all(&self.home).unwrap();
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        drop(self.input.take());
        let _ = self.process.kill();
        let _ = self.process.wait();
        if let Some(reader) = self.reader.take() {
            let _ = reader.join();
        }
        if let Some(errors) = self.errors.take() {
            let _ = errors.join();
        }
        let _ = std::fs::remove_dir_all(&self.home);
    }
}

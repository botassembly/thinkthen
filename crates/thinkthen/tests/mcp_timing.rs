//! Explicit local adapter measurement; run only through test-stress --run.
#![cfg(feature = "cli")]
#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "invalid fixtures, replies or fixed sample counts stop this opt-in measurement"
)]
use conformance_backend::Backend;
use serde_json::{Value, json};
use std::{
    io::{BufRead, BufReader, Write},
    process::{Child, ChildStdin, ChildStdout, Command, Stdio},
    time::{Duration, Instant},
};
use thinkthen::{BatchSetting, CallOptions, Engine, Question, RecordInput, Surface};

struct Session {
    process: Child,
    input: ChildStdin,
    output: BufReader<ChildStdout>,
    id: u64,
}
impl Session {
    fn launch(base: &str, home: &std::path::Path) -> Self {
        let mut process = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
            .args(["mcp", "--url", base, "--no-cache", "--max-retries", "0"])
            .env_clear()
            .env("PATH", "/usr/bin:/bin")
            .env("LANG", "C.UTF-8")
            .env("HOME", home)
            .env("XDG_CONFIG_HOME", home)
            .env("XDG_CACHE_HOME", home)
            .env("XDG_STATE_HOME", home)
            .env("THINKTHEN_API_KEY", "sk-mcp-loopback-only")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let input = process.stdin.take().unwrap();
        let output = BufReader::new(process.stdout.take().unwrap());
        let mut session = Self {
            process,
            input,
            output,
            id: 0,
        };
        let initialized = session.call(
            "initialize",
            json!({
                "protocolVersion":"2025-11-25", "capabilities":{},
                "clientInfo":{"name":"local-timing","version":"1"},
            }),
        );
        assert_eq!(initialized["protocolVersion"], "2025-11-25");
        session.send(&json!({"jsonrpc":"2.0","method":"notifications/initialized"}));
        session
    }
    fn send(&mut self, message: &Value) {
        serde_json::to_writer(&mut self.input, message).unwrap();
        self.input.write_all(b"\n").unwrap();
        self.input.flush().unwrap();
    }
    fn call(&mut self, method: &str, params: Value) -> Value {
        self.id += 1;
        self.send(&json!({"jsonrpc":"2.0","id":self.id,"method":method,"params":params}));
        let mut line = String::new();
        self.output.read_line(&mut line).unwrap();
        let reply: Value = serde_json::from_str(&line).unwrap();
        assert_eq!(reply["id"], self.id);
        assert!(reply.get("error").is_none());
        reply["result"].clone()
    }
    fn finish(mut self) {
        drop(self.input);
        assert!(self.process.wait().unwrap().success());
    }
}

fn report(name: &str, samples: &[Duration]) {
    let mut samples = samples.to_vec();
    samples.sort();
    let millis = |sample: Duration| sample.as_secs_f64() * 1000.0;
    writeln!(
        std::io::stdout().lock(),
        "{name}: samples={} median_ms={:.3} exploratory_p95_ms={:.3} max_ms={:.3}",
        samples.len(),
        millis(samples[samples.len() / 2]),
        millis(samples[(samples.len() * 95).div_ceil(100) - 1]),
        millis(*samples.last().unwrap()),
    )
    .unwrap();
}

fn direct(
    engine: &Engine,
    name: &str,
    decide: &Question,
    find: &Question,
    options: CallOptions<'_>,
) {
    match name {
        "atomic" => assert_eq!(
            engine
                .decide_complete_with(decide, "x", options)
                .unwrap()
                .value()
                .value(),
            thinkthen::Answer::Yes
        ),
        "records" => assert_eq!(
            engine
                .decide_records_complete_with(
                    decide,
                    ["x", "y"].map(|original| RecordInput {
                        examples: None,
                        seed_spans: None,
                        original,
                        context: None,
                        options: None
                    }),
                    options
                )
                .unwrap()
                .value()
                .len(),
            2
        ),
        _ => assert_eq!(
            engine
                .find_complete_with(find, ["x", "y"], options)
                .unwrap()
                .value()
                .selected(),
            Some(&"x")
        ),
    }
}

#[test]
#[ignore = "bounded paired local timing belongs only to test-stress --run"]
fn paired_native_and_installed_mcp_calls_measure_atomic_records_and_whole_set_overhead() {
    let backend = Backend::start().unwrap();
    let base = format!("{}/generic/v1", backend.origin());
    let home = std::env::temp_dir().join(format!("thinkthen-mcp-timing-{}", std::process::id()));
    std::fs::create_dir(&home).unwrap();
    let engine = Engine::builder()
        .base_url(&base)
        .unwrap()
        .api_key("sk-mcp-loopback-only")
        .unwrap()
        .no_cache()
        .max_retries(0)
        .build()
        .unwrap();
    let started = Instant::now();
    let mut session = Session::launch(&base, &home);
    let startup = started.elapsed();
    let decide = Question::decide("Q?").unwrap().cut();
    let find = Question::find("Which?").unwrap();
    let options = CallOptions::new()
        .surface(Surface::Mcp)
        .batch(BatchSetting::Records(
            std::num::NonZeroUsize::new(1).unwrap(),
        ));
    for (name, tool, arguments, sends) in [
        (
            "atomic",
            "decide",
            json!({"question":"Q?","evidence":"x"}),
            1,
        ),
        (
            "records",
            "decide",
            json!({"question":"Q?","records":["x","y"],"options":{"batch":1}}),
            2,
        ),
        (
            "whole-set",
            "find",
            json!({"question":"Which?","records":["x","y"]}),
            1,
        ),
    ] {
        direct(&engine, name, &decide, &find, options);
        let warmup = session.call("tools/call", json!({"name":tool,"arguments":arguments}));
        assert_eq!(warmup["isError"], false);
        let before = backend.count();
        let mut native = Vec::new();
        let mut mcp = Vec::new();
        for round in 0..9 {
            let mut direct = || {
                let started = Instant::now();
                direct(&engine, name, &decide, &find, options);
                native.push(started.elapsed());
            };
            let mut adapted = || {
                let started = Instant::now();
                let result = session.call("tools/call", json!({"name":tool,"arguments":arguments}));
                mcp.push(started.elapsed());
                assert_eq!(result["isError"], false);
                assert_eq!(result["structuredContent"]["facts"]["requests_sent"], sends);
            };
            if round % 2 == 0 {
                direct();
                adapted();
            } else {
                adapted();
                direct();
            }
        }
        assert_eq!(backend.count() - before, 18 * sends);
        writeln!(std::io::stdout().lock(),
            "MCP local {name}: input_json_bytes={} paired_requests={} startup_ms={:.3}; cache disabled; proxy latency unmeasured",
            serde_json::to_vec(&arguments).unwrap().len(), 18 * sends, startup.as_secs_f64() * 1000.0,
        ).unwrap();
        report(&format!("native {name}"), &native);
        report(&format!("mcp {name}"), &mcp);
    }
    session.finish();
    drop(engine);
    std::fs::remove_dir_all(home).unwrap();
}

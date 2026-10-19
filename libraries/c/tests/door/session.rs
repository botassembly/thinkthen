//! Installed-header ownership and immediate control refusals.
use super::*;

/// Collected and streamed C text preserves content across platform line endings.
#[test]
fn consumer_text_preserves_content_across_platform_line_endings() {
    let binary = compile(&crate_dir().join("tests/c/text_output.c"));
    let output = run(&binary, "", b"");
    successful(&output);
    assert_eq!(
        text(&output.stdout),
        "cancelled\n{\"kind\":\"terminal\",\"facts\":{\"requests_sent\":1}}\nfreed\nembedded\rtail\n"
    );
    let mut child = start_with(&binary, "", &[]);
    drop(child.stdin.take());
    let mut output = BufReader::new(child.stdout.take().expect("output"));
    let mut line = String::new();
    output.read_line(&mut line).expect("consumer text");
    assert_eq!(text(line.as_bytes()), "cancelled\n");
    assert_eq!(
        cancelled_terminal(&mut output).expect("terminal before freed marker"),
        serde_json::json!({"kind":"terminal","facts":{"requests_sent":1}})
    );
    line.clear();
    output
        .read_line(&mut line)
        .expect("remaining consumer text");
    assert_eq!(text(line.as_bytes()), "embedded\rtail\n");
    line.clear();
    assert_eq!(output.read_line(&mut line).expect("end of text"), 0);
    successful(&finished(child));
}

#[test]
fn owned_session_controls_refuse_before_sending_and_preserve_outputs() {
    let backend = Backend::start().expect("loopback");
    let output = run_with(
        &compile(&crate_dir().join("tests/c/session.c")),
        &format!("{}/generic/v1", backend.origin()),
        b"",
        &[("SESSION_CONTROLS", Path::new("1"))],
    );
    super::successful(&output);
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        let output = run_with(
            &compile(&crate_dir().join("tests/c/session.c")),
            &format!("{}/generic/v1", backend.origin()),
            b"",
            &[
                ("SESSION_CONTROLS", Path::new("1")),
                ("SESSION_USAGE_DISABLED", Path::new("1")),
                ("HOME", Path::new("")),
                ("XDG_STATE_HOME", Path::new("")),
            ],
        );
        super::successful(&output);
    }
    assert_schema_admission_refusals(&backend);
    assert_definition_admission_refusals(&backend);
    assert_eq!(backend.count(), 0, "malformed controls send nothing");
}

fn assert_schema_admission_refusals(backend: &Backend) {
    for (schema, item) in [
        (
            "context_schema",
            serde_json::json!({"original":{"kind":"text","text":"private-evidence"},"context":"private-context"}),
        ),
        (
            "item_schema",
            serde_json::json!({"original":{"kind":"json","value":"private-evidence"}}),
        ),
    ] {
        let json = serde_json::json!({
            "schema":"thinkthen.request/1",
            "call":{"function":"decide","question":{"kind":"definition","value":{
                "decide":"Does it pass?",schema:{"type":"object","required":["body"],"properties":{"body":{"type":"string"}}}
            }},"input":{"kind":"records","items":[item]}}
        }).to_string();
        let error = thinkthen::Request::from_json(&json)
            .expect("valid authored request")
            .admit()
            .expect_err("native schema refusal");
        assert_eq!(error.kind(), thinkthen::ErrorKind::Usage);
        let output = run_with(
            &compile(&crate_dir().join("tests/c/session.c")),
            &format!("{}/generic/v1", backend.origin()),
            b"",
            &[("SESSION_ADMISSION", Path::new(&json))],
        );
        super::successful(&output);
        assert_eq!(text(&output.stdout), format!("{error}\n"));
        assert!(!text(&output.stdout).contains("private-"));
    }
}

fn assert_definition_admission_refusals(backend: &Backend) {
    let canonical = |question: &str, item: &str| {
        format!(
            "{{\"schema\":\"thinkthen.request/1\",\"call\":{{\"function\":\"decide\",\"question\":{{\"kind\":\"definition\",\"value\":{question}}},\"input\":{{\"kind\":\"records\",\"items\":[{item}]}}}}}}"
        )
    };
    for (question, item, message) in [
        (
            r#"{"decide":"private-wording","item_schema":"string"}"#,
            r#"{"original":{"kind":"text","text":"private-evidence"}}"#,
            "the question declaration uses an unsupported feature",
        ),
        (
            r#"{"decide":"private-wording","wording_version":2e0}"#,
            r#"{"original":{"kind":"text","text":"private-evidence"}}"#,
            "the question declaration uses an unsupported feature",
        ),
        (
            r#"{"decide":"private-wording","context_schema":{"type":"string"}}"#,
            r#"{"original":{"kind":"text","text":"private-evidence"},"context":null}"#,
            "the per-item context does not match context_schema",
        ),
        (
            r#"{"decide":"private-wording","private-canary":true}"#,
            r#"{"original":{"kind":"text","text":"private-evidence"}}"#,
            "invalid canonical request",
        ),
        (
            r#"{"version":1,"questions":{"ok":{"decide":"private-wording"}},"private-canary":true}"#,
            r#"{"original":{"kind":"text","text":"private-evidence"}}"#,
            "invalid canonical request",
        ),
        (
            r#"{"version":1,"questions":{"private-canary":{"decide":12}}}"#,
            r#"{"original":{"kind":"text","text":"private-evidence"}}"#,
            "invalid canonical request",
        ),
    ] {
        let json = canonical(question, item);
        let error =
            thinkthen::Request::from_json(&json).expect_err("invalid definition or context");
        assert_eq!(error.kind(), thinkthen::ErrorKind::Usage);
        assert_eq!(error.to_string(), message);
        let output = run_with(
            &compile(&crate_dir().join("tests/c/session.c")),
            &format!("{}/generic/v1", backend.origin()),
            b"",
            &[("SESSION_ADMISSION", Path::new(&json))],
        );
        super::successful(&output);
        let reported = if message == "invalid canonical request" {
            "invalid session arguments or input"
        } else {
            message
        };
        assert_eq!(text(&output.stdout), format!("{reported}\n"));
        assert!(!text(&output.stdout).contains("private-canary"));
        assert!(!text(&output.stdout).contains("private-wording"));
        assert!(!text(&output.stdout).contains("private-evidence"));
    }
}

#[test]
fn session_owns_inputs_engine_and_transferred_packets() {
    for (surface, legacy) in [("c", false), ("go", false), ("php", false), ("c", true)] {
        let backend = conformance_backend::Listener::answering(|_| conformance_backend::Canned::ok(
        r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}},"usage":{"input_tokens":0}}"#
    )).expect("owned loopback");
        let output = run_with(
            &compile(&crate_dir().join("tests/c/session.c")),
            backend.base(),
            b"",
            &{
                let mut values = vec![("SESSION_PARTIAL", Path::new("1"))];
                if surface != "c" {
                    values.push(("SESSION_SURFACE", Path::new(surface)));
                }
                if legacy {
                    values.push(("SESSION_TYPED", Path::new("1")));
                }
                values
            },
        );
        super::successful(&output);
        let packets: Vec<serde_json::Value> = text(&output.stdout)
            .lines()
            .map(|line| serde_json::from_str(line).expect("canonical packet JSON"))
            .collect();
        assert_eq!(
            packets.first().expect("reader terminal")["kind"],
            "terminal"
        );
        assert_eq!(
            packets.first().expect("reader terminal")["failure"]["error"]["kind"],
            "local"
        );
        assert_eq!(
            packets.last().expect("settled terminal")["kind"],
            "terminal"
        );
        let row = packets
            .iter()
            .find(|packet| packet["kind"] == "row")
            .expect("complete row");
        assert_eq!(row["function"], "decide");
        let observation = packets
            .iter()
            .find(|packet| packet["kind"] == "observation")
            .expect("native observation");
        assert_eq!(observation["function"], "decide");
        let documents = text(&output.stdout);
        for field in [
            "owned.txt",
            "Evidence.",
            "question_sha256",
            "answer_id",
            "observations",
            "question_sources",
        ] {
            assert!(documents.contains(field), "complete packet omitted {field}");
        }
        assert_eq!(backend.count(), 1, "only accepted owned input sends");
        assert_eq!(
            backend.requests()[0].header("user-agent"),
            Some(format!("thinkthen/{} ({surface})", env!("CARGO_PKG_VERSION")).as_str())
        );
    }
}

#[test]
fn session_wait_and_packet_read_cancel_promptly_and_drain_terminal_facts() {
    for blocking in [false, true] {
        let gate = std::sync::Arc::new(conformance_backend::Rendezvous::new(2));
        let worker_gate = gate.clone();
        let (arrived, arrival) = std::sync::mpsc::channel();
        let (answered, answer) = std::sync::mpsc::channel();
        let backend = conformance_backend::Listener::answering(move |_| {
            arrived.send(()).expect("arrival signal");
            conformance_backend::Canned::ok(
                r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}}}"#,
            )
            .after_release(worker_gate.clone())
            .notifying(answered.clone())
        })
        .expect("owned held listener");
        let base = backend.base().to_owned();
        let mut child = start_with(
            &compile(&crate_dir().join("tests/c/session.c")),
            &base,
            &if blocking {
                vec![
                    ("SESSION_HELD", Path::new("1")),
                    ("SESSION_NEXT", Path::new("1")),
                ]
            } else {
                vec![("SESSION_HELD", Path::new("1"))]
            },
        );
        let mut input = child.stdin.take().expect("input");
        let mut output = BufReader::new(child.stdout.take().expect("output"));
        arrival
            .recv_timeout(Duration::from_secs(60))
            .expect("provider holds the accepted request");
        input.write_all(b"c").expect("cancel signal");
        for expected in ["cancelled\n", if blocking { "woken\n" } else { "freed\n" }] {
            let mut line = String::new();
            output.read_line(&mut line).expect("operation return");
            assert_eq!(
                text(line.as_bytes()),
                expected,
                "operation must return while the provider stays held"
            );
        }
        gate.wait();
        answer
            .recv_timeout(Duration::from_secs(60))
            .expect("released listener delivered its response");
        if blocking {
            input.write_all(b"r").expect("settlement signal");
            let Some(terminal) = cancelled_terminal(&mut output) else {
                let result = finished(child);
                panic!(
                    "cancelled drain stopped: {:?} {}",
                    result.status,
                    text(&result.stderr)
                );
            };
            assert_eq!(terminal["failure"]["error"]["kind"], "cancelled");
            assert_eq!(terminal["facts"]["requests_sent"], 1);
        }
        input.write_all(b"r").expect("exit signal");
        drop(input);
        let result = finished(child);
        assert_eq!(
            (result.status.code(), text(&result.stderr)),
            (Some(0), String::new())
        );
        assert_eq!(backend.count(), 1);
        drop(backend); // Retire and join all fixture listener workers.
    }
}

fn cancelled_terminal(
    output: &mut BufReader<std::process::ChildStdout>,
) -> Option<serde_json::Value> {
    let mut terminal = None;
    loop {
        let mut line = String::new();
        if output
            .read_line(&mut line)
            .expect("drain after cancellation")
            == 0
        {
            return None;
        }
        if text(line.as_bytes()) == "freed\n" {
            return terminal;
        }
        let packet: serde_json::Value = serde_json::from_str(&line).expect("complete packet");
        if packet["kind"] == "terminal" {
            terminal = Some(packet);
        }
    }
}

#[test]
fn settlement_wait_interrupts_without_changing_legacy_drain_or_losing_final_facts() {
    let gate = std::sync::Arc::new(conformance_backend::Rendezvous::new(2));
    let held = gate.clone();
    let (arrived, arrival) = std::sync::mpsc::channel();
    let backend = conformance_backend::Listener::answering(move |_| {
        arrived.send(()).expect("arrival signal");
        conformance_backend::Canned::ok(
            r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}}}"#,
        )
        .after_release(held.clone())
    })
    .expect("owned held listener");
    let mut child = start_with(
        &compile(&crate_dir().join("tests/c/settlement_interrupt.c")),
        backend.base(),
        &[("SESSION_NEXT", Path::new("1"))],
    );
    let mut input = child.stdin.take().expect("input");
    let output = child.stdout.take().expect("output");
    let (lines, received) = std::sync::mpsc::channel();
    let reader = std::thread::spawn(move || {
        for line in BufReader::new(output).lines() {
            lines
                .send(line.expect("consumer output"))
                .expect("line receiver");
        }
    });
    arrival
        .recv_timeout(Duration::from_secs(60))
        .expect("provider holds request");
    input.write_all(b"s").expect("start drain");
    assert_eq!(
        received
            .recv_timeout(Duration::from_secs(5))
            .expect("armed drain"),
        "waiting"
    );
    input.write_all(b"c").expect("interrupt drain");
    assert_eq!(
        received
            .recv_timeout(Duration::from_secs(5))
            .expect("prompt interruption"),
        "interrupted"
    );
    assert!(
        matches!(
            received.recv_timeout(Duration::from_millis(30)),
            Err(std::sync::mpsc::RecvTimeoutError::Timeout)
        ),
        "legacy settlement must keep waiting for provider"
    );
    gate.wait();
    let mut terminal = None;
    while let Ok(line) = received.recv_timeout(Duration::from_secs(5)) {
        if line == "freed" {
            break;
        }
        let packet: serde_json::Value = serde_json::from_str(&line).expect("packet");
        if packet["kind"] == "terminal" {
            terminal = Some(packet);
        }
    }
    drop(input);
    let result = finished(child);
    reader.join().expect("retire owned output reader");
    super::successful(&result);
    let terminal = terminal.expect("final terminal survives interrupt");
    assert_eq!(terminal["failure"]["error"]["kind"], "cancelled");
    assert_eq!(terminal["facts"]["requests_sent"], 1);
    assert_eq!(backend.count(), 1, "interruption starts no request");
}

//! Installed-header ownership and immediate control refusals.
use super::*;

#[test]
fn owned_session_controls_refuse_before_sending_and_preserve_outputs() {
    let backend = Backend::start().expect("loopback");
    let output = run_with(
        &compile(&crate_dir().join("tests/c/session.c")),
        &format!("{}/generic/v1", backend.origin()),
        b"",
        &[("SESSION_CONTROLS", Path::new("1"))],
    );
    assert_eq!(
        (output.status.code(), text(&output.stderr)),
        (Some(0), String::new())
    );
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
        assert_eq!(
            (output.status.code(), text(&output.stderr)),
            (Some(0), String::new())
        );
    }
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
        assert_eq!(
            (output.status.code(), text(&output.stderr)),
            (Some(0), String::new())
        );
        assert_eq!(text(&output.stdout), format!("{error}\n"));
        assert!(!text(&output.stdout).contains("private-"));
    }
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
        assert_eq!(
            (output.status.code(), text(&output.stderr)),
            (Some(0), String::new())
        );
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
    assert_eq!(backend.count(), 0, "malformed controls send nothing");
}

#[test]
fn session_owns_inputs_engine_and_transferred_packets() {
    for surface in ["c", "go", "php"] {
        let backend = conformance_backend::Listener::answering(|_| conformance_backend::Canned::ok(
        r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}},"usage":{"input_tokens":0}}"#
    )).expect("owned loopback");
        let output = run_with(
            &compile(&crate_dir().join("tests/c/session.c")),
            backend.base(),
            b"",
            &if surface == "c" {
                vec![("SESSION_PARTIAL", Path::new("1"))]
            } else {
                vec![
                    ("SESSION_PARTIAL", Path::new("1")),
                    ("SESSION_SURFACE", Path::new(surface)),
                ]
            },
        );
        assert_eq!(
            (output.status.code(), text(&output.stderr)),
            (Some(0), String::new())
        );
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
fn session_cancel_and_free_return_before_the_held_provider_is_released() {
    let gate = std::sync::Arc::new(conformance_backend::Rendezvous::new(2));
    let worker_gate = gate.clone();
    let (arrived, arrival) = std::sync::mpsc::channel();
    let (answered, answer) = std::sync::mpsc::channel();
    let backend = conformance_backend::Listener::answering(move |body| {
        arrived.send(()).expect("arrival signal");
        super::sources::all_no_answers(body)
            .after_release(worker_gate.clone())
            .notifying(answered.clone())
    })
    .expect("owned held listener");
    let base = backend.base().to_owned();
    let mut child = start_with(
        &compile(&crate_dir().join("tests/c/session.c")),
        &base,
        &[("SESSION_HELD", Path::new("1"))],
    );
    let mut input = child.stdin.take().expect("input");
    let mut output = BufReader::new(child.stdout.take().expect("output"));
    arrival
        .recv_timeout(Duration::from_secs(60))
        .expect("provider holds the accepted request");
    input.write_all(b"c").expect("cancel signal");
    for expected in ["cancelled\n", "freed\n"] {
        let mut line = String::new();
        output.read_line(&mut line).expect("operation return");
        assert_eq!(
            line, expected,
            "operation must return while the provider stays held"
        );
    }
    gate.wait();
    answer
        .recv_timeout(Duration::from_secs(60))
        .expect("released listener delivered its response");
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

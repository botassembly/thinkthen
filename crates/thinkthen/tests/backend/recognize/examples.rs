//! Tagged example selection and exact boundary-only transport through the command.
use super::*;
use serde_json::json;

fn root(name: &str) -> PathBuf {
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("recognize-examples-{name}-{}", std::process::id()));
    fs::create_dir_all(&root).unwrap();
    root
}

fn plan(listener: &Listener, flags: &[&str], input: &[u8]) -> Value {
    let output = run(listener, &[flags, &["--plan"]].concat(), input);
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let output = stdout(&output);
    let rows: Vec<Value> = output
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    let bytes: u64 = rows[0]["requests"]
        .as_array()
        .unwrap()
        .iter()
        .map(|request| request["bytes"].as_u64().unwrap())
        .sum();
    assert_eq!(rows[1]["estimated_bytes"], bytes);
    assert!(rows[1]["estimated_input_tokens"]["upper"].as_u64().unwrap() > 0);
    rows[0].clone()
}

#[test]
fn tagged_examples_preserve_context_and_only_teach_boundaries() {
    let listener = Listener::answering(automatic).unwrap();
    let input = format!(
        "{}\n",
        json!({"body":"Ada met Acme.","context":"exact context","examples":["[Zoë | person] met [Orbit | organization]."]})
    );
    let output = run(
        &listener,
        &[
            "person",
            "organization",
            "--jsonl",
            "--field",
            "/body",
            "--context-field",
            "/context",
            "--examples-field",
            "/examples",
        ],
        input.as_bytes(),
    );
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 2);
    let boundary: Value = serde_json::from_slice(&requests[0].body).unwrap();
    assert_eq!(boundary["state"]["context"], "exact context");
    assert_eq!(boundary["state"]["evidence"]["evidence"], "Ada met Acme.");
    let rendered = boundary["state"]["evidence"]["examples"][0]
        .as_str()
        .unwrap();
    assert!(rendered.contains("Snippet: [[Zoë]] met Orbit."));
    assert!(rendered.contains("Answer: \"SINGLE\"; kind: \"person\""));
    assert_eq!(rendered.matches("Answer:").count(), 4);
    let kinds: Value = serde_json::from_slice(&requests[1].body).unwrap();
    assert_eq!(
        kinds["state"],
        json!({"context":"exact context","evidence":"Ada met Acme."})
    );
}

#[test]
fn both_file_forms_plan_the_exact_boundary_body_and_enforce_its_byte_limit() {
    let listener = Listener::answering(automatic).unwrap();
    let root = root("forms-limit");
    let brackets = root.join("brackets.txt");
    let spans = root.join("spans.jsonl");
    fs::write(&brackets, "[Zoë | ENTITY]\n").unwrap();
    fs::write(
        &spans,
        "{\"text\":\"Zoë\",\"entities\":[{\"start\":0,\"end\":3,\"kind\":\"ENTITY\"}]}\n",
    )
    .unwrap();
    let brackets = brackets.to_string_lossy();
    let spans = spans.to_string_lossy();
    let first = plan(&listener, &["--examples", &brackets], b"Ada");
    let second = plan(&listener, &["--examples", &spans], b"Ada");
    assert_eq!(first["requests"], second["requests"]);
    assert_eq!(listener.count(), 0);
    let bytes = first["requests"][0]["bytes"].as_u64().unwrap();
    let output = run(
        &listener,
        &[
            "--examples",
            &brackets,
            "--max-request-bytes",
            &bytes.to_string(),
        ],
        b"Ada",
    );
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(listener.count(), 1);
    let baseline = plan(&listener, &[], b"Ada");
    let refused = run(&listener, &["--max-request-bytes", "1", "--plan"], b"Ada");
    assert_eq!(refused.status.code(), Some(2));
    assert!(refused.stdout.is_empty());
    assert_eq!(listener.count(), 1);
    assert!(bytes > baseline["requests"][0]["bytes"].as_u64().unwrap());
    assert_eq!(
        listener.requests()[0].body,
        first["requests"][0]["body_utf8"]
            .as_str()
            .unwrap()
            .as_bytes()
    );
    let output = run(
        &listener,
        &[
            "--examples",
            &spans,
            "--max-request-bytes",
            &(bytes - 1).to_string(),
        ],
        b"Ada",
    );
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert_eq!(listener.count(), 1);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn record_examples_replace_clear_or_retain_shared_examples_at_jobs_eight() {
    let listener = Listener::answering(automatic).unwrap();
    let root = root("records");
    let shared = root.join("shared.txt");
    fs::write(&shared, "[shared | ENTITY]\n").unwrap();
    let shared = shared.to_string_lossy();
    let records = [
        json!({"body":"Ada","context":"first","examples":["[first | ENTITY]"]}),
        json!({"body":"Bob","context":"second","examples":["[second | ENTITY]"]}),
        json!({"body":"Cara","context":"","examples":[]}),
        json!({"body":"Dan","context":"fallback"}),
    ];
    let input: String = records.iter().map(|row| format!("{row}\n")).collect();
    let output = run(
        &listener,
        &[
            "--jsonl",
            "--field",
            "/body",
            "--context-field",
            "/context",
            "--examples",
            &shared,
            "--examples-field",
            "/examples",
            "--jobs",
            "8",
        ],
        input.as_bytes(),
    );
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(stdout(&output).lines().count(), 4);
    assert_eq!(listener.count(), 4);
    for request in listener.requests() {
        let body: Value = serde_json::from_slice(&request.body).unwrap();
        let state = &body["state"];
        if state == "Cara" {
            continue;
        }
        let context = state["context"].as_str().unwrap();
        let example = state["evidence"]["examples"][0].as_str().unwrap();
        let (evidence, marker) = match context {
            "first" => ("Ada", "first"),
            "second" => ("Bob", "second"),
            "fallback" => ("Dan", "shared"),
            other => panic!("unexpected context {other}"),
        };
        assert_eq!(state["evidence"]["evidence"], evidence);
        assert!(example.contains(&format!("Snippet: [[{marker}]]")));
    }
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn invalid_later_record_examples_refuse_the_collected_input_without_sends() {
    let listener = Listener::answering(automatic).unwrap();
    for bad in [
        json!(["[secret-example | unknown]"]),
        json!([{"text":"secret-example","entities":[{"start":1,"end":6,"kind":"ENTITY"}]}]),
        json!([{"text":"Ada","entities":[{"start":0,"end":3,"kind":"ENTITY"},{"start":0,"end":3,"kind":"ENTITY"}]}]),
        Value::Null,
        json!("[Ada | ENTITY]"),
    ] {
        let input = format!(
            "{}\n{}\n",
            json!({"body":"Ada","examples":["[Ada | ENTITY]"]}),
            json!({"body":"Bob","examples":bad})
        );
        let output = run(
            &listener,
            &[
                "--jsonl",
                "--field",
                "/body",
                "--examples-field",
                "/examples",
            ],
            input.as_bytes(),
        );
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
        assert!(String::from_utf8_lossy(&output.stderr).contains("record 2"));
        assert!(!String::from_utf8_lossy(&output.stderr).contains("secret-example"));
    }
    for flags in [
        ["--examples-field", "$.examples"],
        ["--examples-field", "/examples"],
    ] {
        assert_eq!(run(&listener, &flags, ADA).status.code(), Some(2));
    }
    assert_eq!(listener.count(), 0);
}

#[test]
fn example_changes_miss_only_boundary_cache_and_matching_recordings_replay() {
    let listener = Listener::answering(automatic).unwrap();
    let root = root("cache");
    let cache = root.join("cache");
    let cache = cache.to_string_lossy();
    let input =
        |example: &str| format!("{}\n", json!({"body":"Ada met Acme.","examples":[example]}));
    let flags = [
        "person",
        "organization",
        "--jsonl",
        "--field",
        "/body",
        "--examples-field",
        "/examples",
    ];
    let first = local(
        &listener,
        &[&flags[..], &["--cache", &cache]].concat(),
        Some("secret-key"),
        input("[Ada | person]").as_bytes(),
    );
    assert_eq!(first.status.code(), Some(0));
    assert_eq!(listener.count(), 2);
    let second = local(
        &listener,
        &[&flags[..], &["--cache", &cache]].concat(),
        Some("secret-key"),
        input("[Bob | person]").as_bytes(),
    );
    assert_eq!(second.status.code(), Some(0));
    assert_eq!(listener.count(), 3);
    let boundary: Value = serde_json::from_slice(&listener.requests()[2].body).unwrap();
    assert!(
        boundary["state"]["examples"][0]
            .as_str()
            .unwrap()
            .contains("[[Bob]]")
    );
    let replay = local(
        &listener,
        &[&flags[..], &["--no-cache", "--replay", &cache]].concat(),
        None,
        input("[Ada | person]").as_bytes(),
    );
    assert_eq!(replay.status.code(), Some(0));
    assert_eq!(stdout(&replay), stdout(&first));
    assert_eq!(listener.count(), 3);
    fs::remove_dir_all(root).unwrap();
}

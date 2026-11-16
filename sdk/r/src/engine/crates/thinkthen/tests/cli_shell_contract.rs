#![cfg(test)]
//! Shell records, identity, quiet branches and diagnostics.
#![cfg(feature = "command")]
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;
mod support;
use loopback::{KEY, Loopback};
use std::io::Write;
use std::process::{Output, Stdio};

fn run(server: &Loopback, args: &[&str], input: &[u8]) -> Output {
    let mut child = support::command()
        .args(args)
        .env("THINKTHEN_API_KEY", KEY)
        .env("THINKTHEN_BASE_URL", server.base())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let _ = child.stdin.take().unwrap().write_all(input);
    child.wait_with_output().unwrap()
}

#[test]
fn text_streams_name_ids_positions_and_text_files() {
    let server = Loopback::noul(0.9).unwrap();
    let output = run(
        &server,
        &["decide", "Ready?", "--from", "jsonl"],
        b"{\"id\":\"a\\n\\t\",\"body\":\"one\"}\n\"two\"\n",
    );
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(output.stdout, b"a  \ttrue\n2\ttrue\n");
    assert!(output.stderr.is_empty());
    let folder = tempfile::tempdir().unwrap();
    let a = folder.path().join("a.txt");
    let b = folder.path().join("b.txt");
    std::fs::write(&a, "one").unwrap();
    std::fs::write(&b, "two").unwrap();
    let output = run(
        &server,
        &["decide", "Ready?", a.to_str().unwrap(), b.to_str().unwrap()],
        b"",
    );
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        format!("{}\ttrue\n{}\ttrue\n", a.display(), b.display())
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn failed_text_prints_its_state_and_diagnostics_name_its_record() {
    let server = Loopback::noul(0.9).unwrap();
    let folder = tempfile::tempdir().unwrap();
    let path = folder.path().join("records.jsonl");
    std::fs::write(&path, "{\"id\":\"bad\",\"missing\":\"body\"}\n").unwrap();
    let output = run(
        &server,
        &[
            "decide",
            "Ready?",
            "--field",
            "/body",
            path.to_str().unwrap(),
        ],
        b"",
    );
    assert_eq!(output.status.code(), Some(6));
    assert_eq!(
        output.stdout,
        b"bad\tfailed: record.missing: the record holds nothing at /body\n"
    );
    let error = String::from_utf8(output.stderr).unwrap();
    assert!(error.contains("position 1"));
    assert!(error.contains("bad"));
    assert!(error.contains(path.to_str().unwrap()));
    assert!(error.contains("record.missing"));
    assert_eq!(server.count(), 0);
}

#[test]
fn quiet_suppresses_scalar_original_and_json_answers() {
    let server = Loopback::noul(0.9).unwrap();
    for extra in [vec![], vec!["--json"]] {
        for function in ["decide", "filter"] {
            let mut args = vec![function, "Ready?", "-q", "--check"];
            args.extend(extra.iter().copied());
            let output = run(&server, &args, b"one\ntwo");
            if function == "filter" && !extra.is_empty() {
                assert_eq!(output.status.code(), Some(2), "{output:?}");
                assert_eq!(support::reported(&output), "usage.option");
                assert!(
                    String::from_utf8_lossy(&output.stderr).contains("thinkthen decide --json")
                );
                assert!(output.stdout.is_empty());
            } else {
                assert_eq!(output.status.code(), Some(0), "{output:?}");
                assert!(output.stdout.is_empty());
                assert!(output.stderr.is_empty());
            }
        }
    }
}

#[cfg(unix)]
#[test]
fn shell_read_wc_capture_and_head_keep_record_boundaries() {
    let server = Loopback::noul(0.9).unwrap();
    let script = r#"
set -eu
set -o pipefail
bin=$1
count=$(printf 'one\ntwo\nthree' | "$bin" filter 'Ready?' | wc -l)
[ "$count" -eq 3 ]
count=0
while IFS= read -r row; do count=$((count+1)); done < <(printf 'one\ntwo\nthree' | "$bin" filter 'Ready?')
[ "$count" -eq 3 ]
answer=$(printf 'one' | "$bin" decide 'Ready?')
[ "$answer" = true ]
empty=$(printf '' | "$bin" decide 'Ready?' --from lines)
[ -z "$empty" ]
ranked=$(printf 'one\ntwo\nthree' | "$bin" rank 'Order?' --backend stub --level low --level high | wc -l)
[ "$ranked" -eq 3 ]
set +e
printf 'one\ntwo\nthree' | "$bin" decide 'Ready?' --from lines | head -1 > /dev/null
code=$?
[ "$code" -eq 0 ] || [ "$code" -eq 141 ]
"#;
    let isolated = support::command();
    let program = isolated.get_program();
    let mut command = std::process::Command::new("bash");
    command
        .env_clear()
        .envs(
            isolated
                .get_envs()
                .filter_map(|(key, value)| value.map(|value| (key, value))),
        )
        .env("THINKTHEN_API_KEY", KEY)
        .env("THINKTHEN_BASE_URL", server.base());
    let output = command
        .args(["-c", script, "shell-contract"])
        .arg(program)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    assert!(output.stdout.is_empty());
    assert!(output.stderr.is_empty(), "{output:?}");
}

#[test]
fn typed_option_values_are_actionable_without_disclosing_keys() {
    let server = Loopback::noul(0.9).unwrap();
    for (option, value) in [
        ("--threshold", "abc"),
        ("--backend", "nope"),
        ("--quiet", "perhaps"),
    ] {
        let args = if option == "--quiet" {
            vec!["decide", "Ready?", "--quiet=perhaps"]
        } else {
            vec!["decide", "Ready?", option, value]
        };
        let output = run(&server, &args, b"evidence");
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
        let error = String::from_utf8(output.stderr).unwrap();
        assert!(error.contains(value), "{error}");
        assert!(!error.contains(KEY));
    }
    assert_eq!(server.count(), 0);
}

#[test]
fn quiet_tables_and_saved_runs_complete_without_stdout() {
    let directory = tempfile::tempdir().unwrap();
    let table = directory.path().join("input.csv");
    std::fs::write(&table, "body\none\n").unwrap();
    let pipeline = directory.path().join("pipeline.json");
    std::fs::write(
        &pipeline,
        r#"{"run":[{"question":{"function":"decide","question":"Ready?"}},{"shell":["cat"]}]}"#,
    )
    .unwrap();
    let server = Loopback::noul(0.9).unwrap();
    for args in [
        vec![
            "decide",
            "Ready?",
            table.to_str().unwrap(),
            "--column",
            "body",
            "--into",
            "ready",
            "-q",
        ],
        vec![
            "decide", "Ready?", "--from", "csv", "--column", "body", "--into", "ready", "-q",
        ],
        vec![
            "run",
            pipeline.to_str().unwrap(),
            "--allow-shell",
            "-q",
            "--from",
            "lines",
        ],
    ] {
        let output = run(&server, &args, b"body\none\n");
        assert_eq!(output.status.code(), Some(0), "{output:?}");
        assert!(output.stdout.is_empty());
        assert!(output.stderr.is_empty());
    }
    assert!(directory.path().join("input.thinkthen.csv").exists());
}

#[test]
fn table_backend_failures_exit_six_with_or_without_answered_rows() {
    for (body, expected) in [("broken\n", 6), ("broken\nhealthy\n", 6)] {
        let server = Loopback::start(|request| {
            let body = request.json().unwrap();
            if body.to_string().contains("broken") {
                loopback::Reply::json(501, &serde_json::json!({"error":"refused"}))
            } else {
                let answers: serde_json::Map<String, serde_json::Value> = body["questions"]
                    .as_object()
                    .unwrap()
                    .keys()
                    .map(|key| (key.clone(), serde_json::json!({"type":"noul","noul":0.9})))
                    .collect();
                loopback::Reply::json(200, &serde_json::json!({"answers":answers}))
            }
        })
        .unwrap();
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("rows.csv");
        std::fs::write(&path, format!("body\n{body}")).unwrap();
        let output = run(
            &server,
            &[
                "decide",
                "Ready?",
                path.to_str().unwrap(),
                "--column",
                "body",
                "--into",
                "ready",
                "--batch",
                "1",
            ],
            b"",
        );
        assert_eq!(output.status.code(), Some(expected), "{output:?}");
        let destination = directory.path().join("rows.thinkthen.csv");
        assert_eq!(
            String::from_utf8(output.stdout).unwrap(),
            format!("{}\n", destination.display())
        );
        assert!(destination.exists());
        assert!(String::from_utf8_lossy(&output.stderr).contains("backend.status"));
    }
}

#[test]
fn blank_single_text_is_usage_while_blank_stream_rows_continue() {
    let server = Loopback::noul(0.9).unwrap();
    for input in [b"".as_slice(), b"   ", b"\n"] {
        let output = run(&server, &["decide", "Ready?", "--json"], input);
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
        assert!(
            String::from_utf8(output.stderr)
                .unwrap()
                .contains("record.empty")
        );
    }
    assert_eq!(server.count(), 0);
    let output = run(
        &server,
        &["decide", "Ready?", "--json", "--from", "jsonl"],
        b"{}\n\"evidence\"\n",
    );
    assert_eq!(output.status.code(), Some(6));
    let rows: Vec<serde_json::Value> = String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0]["error"]["code"], "record.empty");
    assert_eq!(rows[1]["answer"], true);
    assert_eq!(server.count(), 1);
}

#[test]
fn failed_text_keeps_a_message_with_record_controls_on_one_line() {
    let server = Loopback::noul(0.9).unwrap();
    let output = run(
        &server,
        &["decide", "Ready?", "--field", "/missing\nfield"],
        br#"{"id":"bad","body":"evidence"}"#,
    );
    assert_eq!(output.status.code(), Some(6));
    assert_eq!(
        output.stdout,
        b"failed: record.missing: the record holds nothing at /missing\\nfield\n"
    );
    assert_eq!(server.count(), 0);
}

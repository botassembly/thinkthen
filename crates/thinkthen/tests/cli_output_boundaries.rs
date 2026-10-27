#![cfg(test)]
//! Command output and call failure boundaries.
#![cfg(feature = "command")]
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;
mod support;
use loopback::{KEY, Loopback};
use std::io::Write;
use std::process::{Output, Stdio};
fn run(args: &[&str], input: &[u8]) -> Output {
    let mut child = support::command()
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(input).unwrap();
    child.wait_with_output().unwrap()
}
#[test]
fn empty_tag_answers_keep_one_marked_line_per_record() {
    let server = Loopback::noul(0.0).unwrap();
    let mut child = support::command()
        .args([
            "tag",
            "Which?",
            "--label",
            "a",
            "--from",
            "lines",
            "--url",
            server.base(),
        ])
        .env("THINKTHEN_API_KEY", KEY)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"one\ntwo\n")
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    assert_eq!(output.stdout, b"[1]\n[2]\n");
}
#[test]
fn unknown_option_uses_the_command_error_form() {
    let output = run(&["decide", "Ready?", "--unknown"], b"");
    assert_eq!(output.status.code(), Some(2));
    let message = String::from_utf8(output.stderr).unwrap();
    assert!(
        message.starts_with("thinkthen: usage.question: "),
        "{message}"
    );
    assert_eq!(message.lines().count(), 1);
}
#[test]
fn refused_connection_before_any_row_is_a_call_error() {
    use socket2::{Domain, Socket, Type};
    use std::net::{SocketAddr, TcpListener};
    let socket = Socket::new(Domain::IPV4, Type::STREAM, None).unwrap();
    socket
        .bind(&"127.0.0.1:0".parse::<SocketAddr>().unwrap().into())
        .unwrap();
    let address = socket.local_addr().unwrap().as_socket().unwrap();
    // Keep the bound, non-listening port owned throughout the call.
    let _reserved: TcpListener = socket.into();
    let directory = tempfile::tempdir().unwrap();
    let destination = directory.path().join("answers.csv");
    for table in [false, true] {
        let mut command = support::command();
        command.args(["decide", "Ready?", "--url", &format!("http://{address}")]);
        if table {
            command
                .args([
                    "--from", "csv", "--column", "body", "--into", "ready", "--output",
                ])
                .arg(&destination);
        } else {
            command.arg("--json");
        }
        let mut child = command
            .env("THINKTHEN_API_KEY", KEY)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(if table { b"body\none\n" } else { b"one" })
            .unwrap();
        let output = child.wait_with_output().unwrap();
        assert_eq!(output.status.code(), Some(4), "table={table}: {output:?}");
        assert!(output.stdout.is_empty());
        assert!(
            String::from_utf8_lossy(&output.stderr).starts_with("thinkthen: backend.transport: ")
        );
        assert!(!destination.exists());
    }
}

#[test]
fn filter_then_decide_judges_only_original_kept_items() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("pipeline.json");
    for second in ["decide", "filter"] {
        std::fs::write(&path, serde_json::json!({"run":[{"question":{"function":"filter","question":"Keep?"}},{"question":{"function":second,"question":"Ready?"}}]}).to_string()).unwrap();
        let server = Loopback::answering(|q| serde_json::json!({"type":"noul","noul":if q.to_string().contains("keep me") {0.9} else {0.1}})).unwrap();
        let mut child = support::command()
            .arg("run")
            .arg(&path)
            .args([
                "--json",
                "--from",
                "lines",
                "--batch",
                "1",
                "--url",
                server.base(),
            ])
            .env("THINKTHEN_API_KEY", KEY)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(b"drop me\nkeep me\n")
            .unwrap();
        let output = child.wait_with_output().unwrap();
        assert_eq!(output.status.code(), Some(0), "{output:?}");
        let rows: Vec<serde_json::Value> = String::from_utf8(output.stdout)
            .unwrap()
            .lines()
            .map(|s| serde_json::from_str(s).unwrap())
            .collect();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0]["answer"], true);
        assert_eq!(server.count(), 3);
        if second == "filter" {
            assert_eq!(rows[0]["item"], "keep me");
            assert_eq!(rows[0]["kept"], true);
        } else {
            assert!(rows[0].get("kept").is_none());
        }
    }
}

#[test]
fn text_drops_only_one_trailing_line_ending_before_judgment() {
    use std::sync::{Arc, Mutex};
    let requests = Arc::new(Mutex::new(Vec::new()));
    let received = requests.clone();
    let server = Loopback::answering(move |q| {
        received.lock().unwrap().push(q.clone());
        serde_json::json!({"type":"noul","noul":0.9})
    })
    .unwrap();
    for (args, input) in [
        (vec!["--from", "text"], b"msg\r\n".as_slice()),
        (
            vec!["--from", "csv", "--column", "body", "--into", "answer"],
            b"body\nmsg\n".as_slice(),
        ),
    ] {
        let mut child = support::command()
            .args(["decide", "Ready?", "--url", server.base()])
            .args(args)
            .env("THINKTHEN_API_KEY", KEY)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        child.stdin.take().unwrap().write_all(input).unwrap();
        let output = child.wait_with_output().unwrap();
        assert_eq!(output.status.code(), Some(0), "{output:?}");
    }
    let requests = requests.lock().unwrap();
    assert_eq!(requests.len(), 2);
    assert_eq!(requests[0], requests[1]);
}

#[test]
fn annotations_render_named_answers_in_text_mode() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("questions.json");
    std::fs::write(
        &path,
        r#"{"questions":{"ready":{"function":"decide","question":"Ready?"}}}"#,
    )
    .unwrap();
    let server = Loopback::noul(0.9).unwrap();
    let output = support::command()
        .arg("annotate")
        .arg(path)
        .args(["--from", "text", "--url", server.base()])
        .env("THINKTHEN_API_KEY", KEY)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(text.starts_with("[1]\tready: "), "{text}");
    assert!(text.contains("true"), "{text}");
    assert!(!text.contains(['{', '}', '"']), "{text}");
}

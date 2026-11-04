#![cfg(test)]
//! Command-specific admission, evidence and teaching behavior.
#![cfg(feature = "command")]
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;
mod support;
use loopback::{KEY, Loopback};
use serde_json::{Value, json};
use std::io::Write;
use std::process::{Output, Stdio};

fn run(server: &Loopback, args: &[&str], text: &[u8]) -> Output {
    let mut child = support::command()
        .args(args)
        .env("THINKTHEN_API_KEY", KEY)
        .env("THINKTHEN_URL", server.base())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let _ = child.stdin.take().unwrap().write_all(text);
    child.wait_with_output().unwrap()
}

#[test]
fn command_answer_meanings_and_label_descriptions_reach_the_backend() {
    for (function, flags, expected) in [
        (
            "decide",
            vec!["--true", "written by a Beatle", "--false", "a cover"],
            json!({"true":"written by a Beatle", "false":"a cover"}),
        ),
        (
            "filter",
            vec!["--true", "written by a Beatle", "--false", "a cover"],
            json!({"true":"written by a Beatle", "false":"a cover"}),
        ),
        (
            "choose",
            vec![
                "--choice",
                "a=b",
                "--choice",
                "cover",
                "--describe",
                "a=b=written by a Beatle",
            ],
            json!({"a=b":"written by a Beatle", "cover":null}),
        ),
        (
            "tag",
            vec![
                "--label",
                "original",
                "--label",
                "cover",
                "--describe",
                "original=written by a Beatle",
            ],
            json!({"true":"written by a Beatle"}),
        ),
    ] {
        let reply = if function == "choose" {
            json!({"distribution":{"a=b":0.9,"cover":0.1}})
        } else {
            json!({"probability":0.9})
        };
        let server = Loopback::case(&json!({"replies":[reply]})).unwrap();
        let mut args = vec![function, "Who wrote it?", "--json"];
        args.extend(flags);
        let output = run(&server, &args, b"Yesterday");
        assert_eq!(output.status.code(), Some(0), "{function}: {output:?}");
        let requests = server.requests();
        assert_eq!(requests.len(), 1);
        assert_eq!(
            requests[0].json().unwrap()["questions"]["q1"]["criteria"],
            expected
        );
    }
}

#[test]
fn one_json_record_supports_check_and_repeated_evidence_without_losing_types_or_id() {
    let server = Loopback::noul(0.9).unwrap();
    let output = run(&server, &["decide", "Written by a Beatle?", "--check", "--json", "--field", "/song/title", "--field", "/album"],
        br#"{"id":18446744073709551615,"song":{"title":"Yesterday"},"album":1965,"ignored":"do not send"}"#);
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    let row: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(row["id"], json!(u64::MAX));
    assert_eq!(row["answer"], true);
    let requests = server.requests();
    assert_eq!(requests.len(), 1);
    let body = requests[0].json().unwrap();
    let selected = serde_json::to_string(&json!({"title":"Yesterday","album":1965})).unwrap();
    let quoted = serde_json::to_string(&selected).unwrap();
    assert_eq!(
        body["questions"]["q1"]["instructions"],
        format!("The text is {quoted}. Written by a Beatle?")
    );
}

#[test]
fn help_teaches_examples_before_usage_and_groups_table_options() {
    let bare = support::command().output().unwrap();
    assert!(bare.status.success(), "{bare:?}");
    assert!(bare.stderr.is_empty());
    assert!(
        String::from_utf8(bare.stdout)
            .unwrap()
            .contains("Commands:")
    );
    for function in thinkthen::FunctionName::ALL {
        let name = serde_json::to_value(function).unwrap();
        let name = name.as_str().unwrap();
        let output = support::command().args([name, "--help"]).output().unwrap();
        assert!(output.status.success());
        let help = String::from_utf8(output.stdout).unwrap();
        assert!(
            help.find("Examples:").unwrap() < help.find("Usage:").unwrap(),
            "{help}"
        );
        assert!(
            help.find("Options:\n      --backend").unwrap() < help.find("Table options:").unwrap(),
            "{help}"
        );
        assert!(help.contains("--backend <NAME>"), "{help}");
        assert!(
            help.contains(&format!("https://thinkthen.dev/functions/{name}/")),
            "{help}"
        );
    }
    let missing = support::command().arg("decide").output().unwrap();
    assert_eq!(missing.status.code(), Some(2));
    assert!(missing.stdout.is_empty());
    let error = String::from_utf8(missing.stderr).unwrap();
    assert!(
        error.contains("Usage:") && error.contains("--help"),
        "{error}"
    );
    assert!(!error.contains("contents redacted"), "{error}");
}

#[test]
fn repeated_evidence_retains_filter_bytes_and_bad_records_do_not_stop_the_stream() {
    let server = Loopback::noul(0.9).unwrap();
    let first =
        "{\"id\":\"a\", \"title\":\"Yesterday\",\"album\":1965,\"private\":\"keep bytes\"}\n";
    let last = "{\"id\":\"c\",\"title\":\"Help!\",\"album\":1965}\n";
    let input = format!("{first}{{\"id\":\"b\",\"title\":\"missing album\"}}\n{last}");
    let output = run(
        &server,
        &[
            "filter",
            "Original?",
            "--from",
            "jsonl",
            "--field",
            "/title",
            "--field",
            "/album",
        ],
        input.as_bytes(),
    );
    assert_eq!(output.status.code(), Some(6), "{output:?}");
    assert_eq!(output.stdout, format!("{first}{last}").as_bytes());
    assert!(String::from_utf8_lossy(&output.stderr).contains("record.missing"));
    assert_eq!(server.count(), 1);
    let requests = server.requests();
    assert!(
        !requests[0]
            .json()
            .unwrap()
            .to_string()
            .contains("keep bytes")
    );
}

#[test]
fn malformed_description_or_colliding_evidence_names_fail_before_a_request() {
    let server = Loopback::noul(0.9).unwrap();
    for args in [
        vec![
            "choose",
            "Route?",
            "--choice",
            "a",
            "--choice",
            "b",
            "--describe",
            "c=unknown",
        ],
        vec![
            "choose",
            "Route?",
            "--choice",
            "a",
            "--choice",
            "b",
            "--describe",
            "a=one",
            "--describe",
            "a=two",
        ],
        vec![
            "tag",
            "Tags?",
            "--label",
            "a",
            "--describe",
            "missing equals",
        ],
        vec![
            "decide",
            "Ready?",
            "--field",
            "/one/title",
            "--field",
            "/two/title",
        ],
        vec!["decide", "Ready?", "--field", "", "--field", "/title"],
    ] {
        let output = run(&server, &args, b"{\"title\":\"Yesterday\"}");
        assert_eq!(output.status.code(), Some(2), "{args:?}: {output:?}");
        assert!(output.stdout.is_empty());
    }
    assert_eq!(server.count(), 0);
}

#[cfg(unix)]
#[test]
fn progress_clears_before_each_terminal_answer() {
    let script = r#"
import errno, os, pty, subprocess, sys
master, slave = pty.openpty()
child = subprocess.Popen([sys.argv[1], 'decide', 'Ready?', '--backend', 'stub', '--from', 'lines'], stdin=subprocess.PIPE, stdout=slave, stderr=slave)
os.close(slave)
child.stdin.write(b'first\nsecond\n'); child.stdin.close()
chunks = []
while True:
    try:
        chunk = os.read(master, 4096)
        if not chunk: break
        chunks.append(chunk)
    except OSError as error:
        if error.errno == errno.EIO: break
        raise
os.close(master)
assert child.wait() == 0
actual = b''.join(chunks)
assert b'\r\x1b[2K1\tfalse\r\n' in actual, actual
assert b'\r\x1b[2K2\tfalse\r\n' in actual, actual
assert actual.endswith(b'\r\x1b[2K'), actual
"#;
    let output = std::process::Command::new("python3")
        .args(["-c", script])
        .arg(env!("CARGO_BIN_EXE_thinkthen"))
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[cfg(unix)]
#[test]
fn beatles_setlist_branches_silently_on_one_selected_json_record() {
    let script = r#"
is_original() {
  "$1" decide 'Did a Beatle write this song?' \
    --true 'John, Paul, George, or Ringo wrote it.' \
    --false 'Someone else wrote it. It is a cover.' \
    --low 0.2 --high 0.8 --check --quiet --backend stub \
    --field /title --field /album
}
while IFS='|' read -r title song; do
  printf '%s\n' "$song" | is_original "$1"
  code=$?
  case $code in
    0) echo "play   $title" ;;
    1) echo "skip   $title" ;;
    3) echo "check  $title" ;;
    *) exit "$code" ;;
  esac
done
"#;
    let mut child = std::process::Command::new("bash")
        .env_clear()
        .env("PATH", std::env::var_os("PATH").unwrap())
        .args(["-c", script, "setlist", env!("CARGO_BIN_EXE_thinkthen")])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let titles = [
        "Something",
        "Act Naturally",
        "Words of Love",
        "Mr. Moonlight",
    ];
    let albums = [
        "Abbey Road",
        "Help!",
        "Beatles for Sale",
        "Beatles for Sale",
    ];
    for (title, album) in titles.iter().zip(albums) {
        writeln!(
            child.stdin.as_mut().unwrap(),
            "{title}|{}",
            json!({"title":title,"album":album})
        )
        .unwrap();
    }
    drop(child.stdin.take());
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{output:?}");
    assert!(output.stderr.is_empty(), "{output:?}");
    let text = String::from_utf8(output.stdout).unwrap();
    let lines = text.lines().collect::<Vec<_>>();
    assert_eq!(lines.len(), titles.len(), "{text}");
    for (line, title) in lines.iter().zip(titles) {
        assert!(
            ["play   ", "skip   ", "check  "]
                .iter()
                .any(|prefix| *line == format!("{prefix}{title}")),
            "{line}"
        );
    }
}

#[test]
fn repeated_table_evidence_uses_only_the_named_columns() {
    let server = Loopback::noul(0.9).unwrap();
    let args = [
        "decide",
        "Original?",
        "--from",
        "csv",
        "--into",
        "original",
        "--field",
        "/title",
        "--field",
        "/album",
    ];
    let output = run(
        &server,
        &args,
        b"title,album,ignored\nYesterday,1965,do not send\n",
    );
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    let mut reader = csv::Reader::from_reader(output.stdout.as_slice());
    assert!(
        reader
            .headers()
            .unwrap()
            .iter()
            .any(|name| name == "original")
    );
    assert_eq!(reader.records().count(), 1);
    let requests = server.requests();
    assert_eq!(requests.len(), 1);
    let wire = requests[0].json().unwrap().to_string();
    assert!(
        wire.contains("Yesterday") && wire.contains("1965"),
        "{wire}"
    );
    assert!(!wire.contains("do not send"), "{wire}");
    for pointer in ["/nested/title", "//title"] {
        let output = run(
            &server,
            &[
                "decide",
                "Original?",
                "--from",
                "csv",
                "--into",
                "original",
                "--field",
                pointer,
                "--field",
                "/album",
            ],
            b"title,album\nYesterday,1965\n",
        );
        assert_eq!(output.status.code(), Some(2), "{output:?}");
    }
    assert_eq!(server.count(), 1);
}

#[test]
fn single_selected_json_filter_keeps_its_structured_item() {
    let server = Loopback::noul(0.9).unwrap();
    let item = json!({"id":"original","title":"Yesterday","album":1965,"extra":"retained"});
    let output = run(
        &server,
        &[
            "filter",
            "Original?",
            "--from",
            "text",
            "--json",
            "--field",
            "/title",
            "--field",
            "/album",
        ],
        item.to_string().as_bytes(),
    );
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    let row: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(row["item"], item);
    assert_eq!(row["id"], "original");
}

#[test]
fn selected_empty_or_null_evidence_never_reaches_the_backend() {
    let server = Loopback::noul(0.9).unwrap();
    for value in [json!(""), json!("  "), json!({}), json!([]), Value::Null] {
        let item = json!({"id":"empty","text":value,"other":"real"});
        let output = run(
            &server,
            &["decide", "Ready?", "--field", "/text"],
            item.to_string().as_bytes(),
        );
        assert_eq!(
            output.status.code(),
            Some(if value.is_null() { 6 } else { 2 }),
            "{value}: {output:?}"
        );
        assert!(output.stdout.is_empty(), "{output:?}");
        let code = if value.is_null() {
            "record.missing"
        } else {
            "record.empty"
        };
        assert!(String::from_utf8_lossy(&output.stderr).contains(code));
    }
    let output = run(
        &server,
        &["decide", "Ready?", "--field", "/text", "--field", "/other"],
        br#"{"id":"null","text":null,"other":"real"}"#,
    );
    assert_eq!(output.status.code(), Some(6), "{output:?}");
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("record.missing"));
    assert_eq!(server.count(), 0);
}

#[test]
fn a_selected_blank_stream_row_preserves_later_answer_ids() {
    let server = Loopback::noul(0.9).unwrap();
    let output = run(
        &server,
        &[
            "decide",
            "Ready?",
            "--from",
            "jsonl",
            "--field",
            "/text",
        ],
        b"{\"id\":\"first\",\"text\":\"one\"}\n{\"id\":\"blank\",\"text\":\" \"}\n{\"id\":\"last\",\"text\":\"two\"}\n",
    );
    assert_eq!(output.status.code(), Some(6), "{output:?}");
    assert_eq!(output.stdout, b"first\ttrue\nlast\ttrue\n");
    assert!(String::from_utf8_lossy(&output.stderr).contains("record.empty"));
    assert_eq!(server.count(), 1);
    let wire = server.requests()[0].json().unwrap().to_string();
    assert!(wire.contains("one") && wire.contains("two"), "{wire}");
    assert!(!wire.contains("blank"), "{wire}");
}

//! `thinkthen audit` refuses bad input in one line that echoes nothing, sends
//! no request, reads no key, and names itself in help.
#![cfg(feature = "cli")]
#![allow(
    clippy::expect_used,
    clippy::panic,
    reason = "a failed fixture stops the proof"
)]

#[path = "support/measure.rs"]
mod measure_support;
#[path = "../src/test_deadline/wait.rs"]
mod wait;

use std::fs;
use std::io::{ErrorKind, Write as _};
use std::net::TcpListener;
use std::os::unix::fs::PermissionsExt as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use measure_support::{DIFF_GOLDENS, DIFF_TABLES, GOLDENS, TABLES, audit, fixtures, measure, run};

/// A decide line whose record id and text are planted secrets.
const YES: &str = "{\"input\":{\"id\":\"secret-id-5150\",\"text\":\"secret-text\"},\"value\":true,\"answer\":{\"probability\":0.9}}\n";

/// The key files a refusal names as `@name`, with a planted key value.
const FILES: [(&str, &str); 7] = [
    ("run.jsonl", YES),
    (
        "key.jsonl",
        "{\"id\":\"secret-id-5150\",\"value\":\"secret-value\"}\n",
    ),
    ("yes.jsonl", "{\"id\":\"secret-id-5150\",\"value\":true}\n"),
    ("dup.jsonl", "{\"id\":1,\"id\":2}\n"),
    (
        "part.jsonl",
        "{\"id\":\"secret-id-5150\",\"value\":true,\"part\":\"secret-value\"}\n",
    ),
    ("choose.jsonl", "{\"id\":\"secret-id-5150\",\"value\":5}\n"),
    (
        "mixed.jsonl",
        "{\"id\":\"secret-id-5150\",\"value\":true,\"part\":\"tune\"}\n{\"id\":\"b\",\"value\":true}\n",
    ),
];

/// One record twice under one answer, asked in two question texts.
const TWICE: &str = "{\"input\":{\"id\":\"secret-id-5150\"},\"value\":true,\"question\":{\"text\":\"secret-text\"}}\n{\"input\":{\"id\":\"secret-id-5150\"},\"value\":true,\"question\":{\"text\":\"other\"}}\n";

/// Each refusal: the command line, standard input (`YES` where empty is not asked), the exit code, and the sentence.
const REFUSALS: [(&str, &str, i32, &str); 33] = [
    (
        "audit @secret-path @key.jsonl",
        "",
        5,
        "cannot read the results file",
    ),
    ("audit - @secret-path", YES, 5, "cannot read the key file"),
    (
        "audit - @key.jsonl",
        "{\"input\":{\"id\":\"secret-id-5150\"}}\n\n[\"secret-text\"]\n",
        2,
        "results line 3 is not a JSON object",
    ),
    (
        "audit - @dup.jsonl",
        YES,
        2,
        "key line 1 is not a JSON object",
    ),
    (
        "audit - @key.jsonl",
        "{\"input\":{\"id\":1.5},\"value\":true}\n",
        2,
        "results line 1 has no string or integer id at the --id pointer",
    ),
    (
        "audit - @key.jsonl --id secret-text",
        YES,
        2,
        "--id takes a JSON pointer such as /id or ''",
    ),
    (
        "audit - @key.jsonl",
        "{\"input\":{\"id\":\"secret-id-5150\"},\"value\":[],\"question\":{\"verb\":\"tag\",\"text\":\"secret-text\"}}\n",
        2,
        "results line 1 holds an answer audit cannot grade; audit grades decide and choose",
    ),
    (
        "audit - @key.jsonl",
        "{\"input\":{\"id\":\"secret-id-5150\"},\"value\":true,\"question\":{\"verb\":\"choose\"}}\n",
        2,
        "results line 1 holds an answer audit cannot grade; audit grades decide and choose",
    ),
    (
        "audit - @key.jsonl",
        "{\"input\":{\"id\":\"secret-id-5150\"},\"value\":5,\"question\":{\"verb\":\"choose\"}}\n",
        2,
        "results line 1 holds an answer audit cannot grade; audit grades decide and choose",
    ),
    (
        "audit - @key.jsonl",
        "{\"input\":{\"id\":\"secret-id-5150\"},\"value\":true,\"answer\":{\"probability\":1.5}}\n",
        2,
        "results line 1 holds a probability outside 0 to 1 or an empty distribution",
    ),
    (
        "audit - @key.jsonl",
        "{\"input\":{\"id\":\"secret-id-5150\"},\"value\":\"secret-text\",\"answer\":{\"probabilities\":{}}}\n",
        2,
        "results line 1 holds a probability outside 0 to 1 or an empty distribution",
    ),
    (
        "audit - @key.jsonl",
        "{\"input\":{\"id\":\"secret-id-5150\"},\"value\":true}\n{\"input\":{\"id\":\"secret-id-5150\"},\"value\":false}\n",
        2,
        "results line 2 repeats a record for one question",
    ),
    (
        "audit - @part.jsonl",
        YES,
        2,
        "key line 1 needs a new id, a value, and a part of tune or held when present",
    ),
    (
        "audit - @choose.jsonl",
        "{\"input\":{\"id\":\"secret-id-5150\"},\"value\":\"secret-text\"}\n",
        2,
        "key line 1 gives a choose value that is not text",
    ),
    (
        "audit - @mixed.jsonl",
        "{\"input\":{\"id\":\"secret-id-5150\"},\"value\":true,\"answer\":{\"probability\":0.9}}\n{\"input\":{\"id\":\"b\"},\"value\":true,\"answer\":{\"probability\":0.9}}\n",
        2,
        "the key gives a part on some labeled records and not on others",
    ),
    (
        "audit - @key.jsonl",
        "{\"input\":{\"id\":\"secret-id-5150\"},\"value\":true,\"question\":{\"text\":\"secret-text\"}}\n{\"input\":{\"id\":\"b\"},\"value\":\"a\",\"question\":{\"text\":\"secret-text\"}}\n",
        2,
        "one question holds both decide and choose answers",
    ),
    (
        "audit - @yes.jsonl --threshold 0.5",
        "{\"input\":{\"id\":\"secret-id-5150\"},\"value\":true}\n",
        2,
        "--threshold needs probabilities; rerun the question with --details",
    ),
    (
        "audit small/choose.jsonl small/choose-key.jsonl --threshold 0.4:0.6",
        "",
        2,
        "choose takes a single cut; a band applies to decide",
    ),
    ("audit - -", YES, 2, "only one input may be standard input"),
    (
        "audit - @key.jsonl --threshold 0",
        YES,
        2,
        "--threshold: a single cut is above zero and at most one",
    ),
    (
        "diff @secret-path",
        "",
        2,
        "diff needs a second run or --compare-threshold",
    ),
    (
        "diff - @run.jsonl",
        TWICE,
        2,
        "first run line 2 repeats a record for one answer",
    ),
    (
        "diff @run.jsonl -",
        TWICE,
        2,
        "second run line 2 repeats a record for one answer",
    ),
    (
        "diff - @run.jsonl --key -",
        YES,
        2,
        "only one input may be standard input",
    ),
    (
        "diff @secret-path @run.jsonl",
        "",
        5,
        "cannot read the first run file",
    ),
    (
        "diff @run.jsonl @secret-path",
        "",
        5,
        "cannot read the second run file",
    ),
    (
        "diff @run.jsonl @run.jsonl --key @secret-path",
        "",
        5,
        "cannot read the key file",
    ),
    (
        "diff @run.jsonl --compare-threshold 2",
        "",
        2,
        "--compare-threshold: a single cut is above zero and at most one",
    ),
    (
        "diff @run.jsonl @run.jsonl --threshold secret-text",
        "",
        2,
        "--threshold: a threshold is a decimal fraction, or two of them as LOW:HIGH",
    ),
    (
        "diff - @run.jsonl",
        "{\"input\":{\"id\":\"secret-id-5150\"}}\n\n[\"secret-text\"]\n",
        2,
        "first run line 3 is not a JSON object",
    ),
    (
        "diff @run.jsonl -",
        "{\"input\":{\"id\":\"secret-id-5150\"},\"value\":[],\"question\":{\"verb\":\"tag\"}}\n",
        2,
        "second run line 1 holds an answer diff cannot grade; diff grades decide and choose",
    ),
    (
        "diff @run.jsonl @run.jsonl --key @dup.jsonl",
        "",
        2,
        "key line 1 is not a JSON object",
    ),
    (
        "diff - --compare-threshold 0.5",
        "{\"input\":{\"id\":\"secret-id-5150\"},\"value\":true}\n",
        2,
        "--threshold needs probabilities; rerun the question with --details",
    ),
];

#[test]
fn each_failure_prints_one_line_that_names_no_record_id_value_or_path() {
    let root = std::env::temp_dir().join(format!("thinkthen-0113-refused-{}", std::process::id()));
    let _absent = fs::remove_dir_all(&root);
    fs::create_dir(&root).expect("a folder");
    for (name, text) in FILES {
        fs::write(root.join(name), text).expect("a key file");
    }
    for (arguments, input, expected, sentence) in REFUSALS {
        let arguments = words(arguments, &root);
        let arguments: Vec<&str> = arguments.iter().map(String::as_str).collect();
        let (code, stdout, stderr) = measure(&arguments, input.as_bytes());
        assert_eq!(
            stderr,
            format!("thinkthen: {}: {sentence}\n", arguments[0]),
            "{arguments:?}"
        );
        assert_eq!((code, stdout.as_str()), (expected, ""), "{sentence}");
        for secret in [
            "secret-id",
            "5150",
            "secret-text",
            "secret-value",
            "secret-path",
            "thinkthen-0113",
        ] {
            assert!(!stderr.contains(secret), "{sentence}");
        }
    }
    for arguments in [
        &["--seed", "-1"][..],
        &["--target", "1.5"],
        &["--by", "record"],
    ] {
        let arguments = [&["small/decide.jsonl", "small/decide-key.jsonl"], arguments].concat();
        let (code, stdout, _) = audit(&arguments, b"");
        assert_eq!((code, stdout.as_str()), (2, ""), "{arguments:?}");
    }
    fs::remove_dir_all(&root).expect("cleanup");
}

#[test]
fn help_names_audit_after_transform_and_says_what_it_never_does() {
    const ROW: &str = "Grade saved decide and choose answers against an answer key.";
    let text =
        |arguments: &[&str]| String::from_utf8(run(arguments, b"").stdout).expect("UTF-8 help");
    let root = text(&["--help"]);
    assert!(root.contains(&format!("\n  transform  List or print the built-in jq transforms without running them\n  audit      {}\n  diff ", ROW.trim_end_matches('.'))), "{root}");
    assert!(text(&["audit", "-h"]).starts_with(&format!("{}\n\n", ROW.trim_end_matches('.'))));
    let long = text(&["audit", "--help"]);
    assert!(long.starts_with(&format!("{ROW}\n\n")), "{long}");
    for sentence in [
        "audit sends no request and reads no key.",
        "To grade a recording, replay it with --details and pass the output:",
    ] {
        assert!(long.contains(sentence), "{sentence}");
    }
}

/// Every path under a folder with its size, so a change anywhere shows.
fn tree(root: &Path) -> Vec<(PathBuf, u64)> {
    let mut found = Vec::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(path) = pending.pop() {
        let metadata = fs::symlink_metadata(&path).expect("metadata");
        found.push((path.clone(), metadata.len()));
        if metadata.is_dir() {
            pending.extend(
                fs::read_dir(&path)
                    .expect("a folder")
                    .map(|entry| entry.expect("an entry").path()),
            );
        }
    }
    found.sort();
    found
}

/// Split a refusal's arguments, naming `@file` inside the folder.
fn words(arguments: &str, folder: &Path) -> Vec<String> {
    arguments
        .split(' ')
        .map(|word| match word.strip_prefix('@') {
            Some(name) => folder.join(name).to_string_lossy().into_owned(),
            None => word.to_owned(),
        })
        .collect()
}

#[test]
fn audit_sends_no_request_reads_no_key_and_writes_nothing() {
    let root = std::env::temp_dir().join(format!("thinkthen-0113-guarded-{}", std::process::id()));
    let _absent = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("locked")).expect("a folder");
    for (name, text) in FILES {
        fs::write(root.join(name), text).expect("a key file");
    }
    let listener = TcpListener::bind("127.0.0.1:0").expect("a loopback listener");
    listener.set_nonblocking(true).expect("nonblocking");
    let url = format!("http://{}/v1", listener.local_addr().expect("an address"));
    let before = (tree(&root), tree(&fixtures()));
    fs::set_permissions(root.join("locked"), fs::Permissions::from_mode(0o000)).expect("locked");
    let owned = |list: &[&str]| list.iter().map(|word| (*word).to_owned()).collect();
    let mut lines: Vec<(Vec<String>, &str)> = GOLDENS
        .iter()
        .map(|(_, arguments)| (owned(&[&["audit"], *arguments].concat()), ""))
        .collect();
    lines.extend(
        TABLES
            .iter()
            .map(|(_, [results, key])| (owned(&["audit", results, key, "--table"]), "")),
    );
    lines.extend(
        DIFF_GOLDENS
            .iter()
            .chain(&DIFF_TABLES)
            .map(|(_, arguments)| (owned(&[&["diff"], *arguments, &["--table"]].concat()), "")),
    );
    lines.extend(
        DIFF_GOLDENS
            .iter()
            .map(|(_, arguments)| (owned(&[&["diff"], *arguments].concat()), "")),
    );
    lines.extend(
        REFUSALS
            .iter()
            .map(|(arguments, input, _, _)| (words(arguments, &root), *input)),
    );
    for (arguments, input) in lines {
        let mut command = Command::new(env!("CARGO_BIN_EXE_thinkthen"));
        command
            .args(&arguments)
            .env_clear()
            .current_dir(fixtures())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        for variable in [
            "HOME",
            "XDG_CONFIG_HOME",
            "XDG_CACHE_HOME",
            "THINKTHEN_CACHE",
        ] {
            command.env(variable, root.join("locked").join(variable));
        }
        let mut child = command
            .env("THINKTHEN_API_KEY", "canary-0113")
            .env("THINKTHEN_BASE_URL", &url)
            .spawn()
            .expect("the binary runs");
        let mut stdin = child.stdin.take().expect("standard input");
        let _ignored = stdin.write_all(input.as_bytes());
        drop(stdin);
        let output = wait::finish(child, "thinkthen").expect("the binary finishes");
        assert!(
            matches!(output.status.code(), Some(0 | 2 | 5)),
            "{arguments:?}"
        );
        for channel in [&output.stdout, &output.stderr] {
            assert!(
                !String::from_utf8_lossy(channel).contains("canary-0113"),
                "{arguments:?}"
            );
        }
    }
    fs::set_permissions(root.join("locked"), fs::Permissions::from_mode(0o755)).expect("unlocked");
    assert_eq!(
        listener.accept().map(|_| ()).map_err(|error| error.kind()),
        Err(ErrorKind::WouldBlock)
    );
    assert_eq!((tree(&root), tree(&fixtures())), before);
    fs::remove_dir_all(&root).expect("cleanup");
}

//! `thinkthen audit` matches the prototype's golden files, refuses bad input
//! in one line that echoes nothing, and sends no request.
#![allow(
    clippy::expect_used,
    clippy::panic,
    reason = "a failed fixture stops the proof"
)]

#[path = "support/measure.rs"]
mod measure_support;

use std::fs;
use std::io::ErrorKind;
use std::net::TcpListener;
use std::os::unix::fs::PermissionsExt as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use measure_support::{fixtures, run, same_lines};
use sha2::{Digest as _, Sha256};

/// Each golden file and the audit command line that prints it.
const GOLDENS: [(&str, &[&str]); 10] = [
    (
        "golden/audit-decide.jsonl",
        &["small/decide.jsonl", "small/decide-key.jsonl"],
    ),
    (
        "golden/audit-decide-0.4.jsonl",
        &[
            "small/decide.jsonl",
            "small/decide-key.jsonl",
            "--threshold",
            "0.4",
        ],
    ),
    (
        "golden/audit-decide-band.jsonl",
        &["small/decide-band.jsonl", "small/decide-key.jsonl"],
    ),
    (
        "golden/audit-decide-bare.jsonl",
        &["small/decide-bare.jsonl", "small/decide-key.jsonl"],
    ),
    (
        "golden/audit-choose.jsonl",
        &["small/choose.jsonl", "small/choose-key.jsonl"],
    ),
    (
        "golden/audit-annotate.jsonl",
        &["small/annotate.jsonl", "small/annotate-key.jsonl"],
    ),
    (
        "golden/audit-249.jsonl",
        &["249/control.jsonl", "249/key.jsonl", "--by", "verb"],
    ),
    (
        "golden/audit-249-seed.jsonl",
        &[
            "249/control.jsonl",
            "249/key-noparts.jsonl",
            "--by",
            "verb",
            "--seed",
            "249",
        ],
    ),
    (
        "golden/extra/audit-decide-reversed.jsonl",
        &[
            "small/decide-reversed.jsonl",
            "small/decide-key-noparts.jsonl",
        ],
    ),
    (
        "golden/extra/audit-decide-odd.jsonl",
        &["small/decide.jsonl", "small/decide-key-odd.jsonl"],
    ),
];

/// Each table capture and the inputs it grades.
const TABLES: [(&str, [&str; 2]); 3] = [
    (
        "golden/table/audit-decide.txt",
        ["small/decide.jsonl", "small/decide-key.jsonl"],
    ),
    (
        "golden/table/audit-choose.txt",
        ["small/choose.jsonl", "small/choose-key.jsonl"],
    ),
    (
        "golden/table/audit-annotate.txt",
        ["small/annotate.jsonl", "small/annotate-key.jsonl"],
    ),
];

fn audit(arguments: &[&str], input: &[u8]) -> (i32, String, String) {
    let output = run(&[&["audit"], arguments].concat(), input);
    let text = |bytes: Vec<u8>| String::from_utf8(bytes).expect("UTF-8 output");
    (
        output.status.code().expect("an exit code"),
        text(output.stdout),
        text(output.stderr),
    )
}

fn fixture(path: &str) -> String {
    fs::read_to_string(fixtures().join(path)).expect("a fixture")
}

/// One row member as its compact JSON text.
fn member(line: &str, pointer: &str) -> String {
    let row: serde_json::Value = serde_json::from_str(line).expect("a JSON row");
    row.pointer(pointer).expect("the member").to_string()
}

#[test]
fn goldens_match() {
    for (golden, arguments) in GOLDENS {
        let (code, stdout, stderr) = audit(arguments, b"");
        assert_eq!((code, stderr.as_str()), (0, ""), "{golden}");
        if let Err(difference) = same_lines(&stdout, &fixture(golden)) {
            panic!("{golden}: {difference}");
        }
    }
}

#[test]
fn tables_match_byte_for_byte() {
    for (capture, [results, key]) in TABLES {
        let (code, stdout, stderr) = audit(&[results, key, "--table"], b"");
        assert_eq!((code, stderr.as_str()), (0, ""), "{capture}");
        assert_eq!(stdout, fixture(capture), "{capture}");
    }
}

#[test]
fn every_fixture_keeps_its_checksum() {
    let listed: Vec<(String, String)> = fixture("README.md")
        .lines()
        .skip_while(|line| *line != "| File | SHA-256 |")
        .skip(2)
        .map(|line| {
            let cells: Vec<&str> = line.split('`').collect();
            (cells[1].to_owned(), cells[3].to_owned())
        })
        .collect();
    let mut found = Vec::new();
    let mut pending = vec![fixtures()];
    while let Some(folder) = pending.pop() {
        for entry in fs::read_dir(folder).expect("a folder") {
            let path = entry.expect("an entry").path();
            if path.is_dir() {
                pending.push(path);
            } else if path.file_name().is_some_and(|name| name != "README.md") {
                let bytes = fs::read(&path).expect("a fixture");
                let digest: String = Sha256::digest(&bytes)
                    .iter()
                    .map(|b| format!("{b:02x}"))
                    .collect();
                let name = path
                    .strip_prefix(fixtures())
                    .expect("inside")
                    .to_string_lossy()
                    .into_owned();
                found.push((name, digest));
            }
        }
    }
    found.sort();
    assert_eq!(found, listed);
    assert_eq!(listed.len(), 38);
}

#[test]
fn readers_ignore_members_they_do_not_use_and_a_band_prints_as_typed() {
    let extra = |text: String| {
        text.replace("{\"id\"", "{\"later\":[1],\"id\"")
            .replace("\"input\"", "\"later\":2,\"input\"")
    };
    let results = extra(fixture("small/decide.jsonl"));
    let key = fixtures().join("small/decide-key.jsonl");
    let wider = std::env::temp_dir().join(format!("thinkthen-0113-key-{}", std::process::id()));
    fs::write(&wider, extra(fixture("small/decide-key.jsonl"))).expect("a key");
    let (code, stdout, _) = audit(&["-", wider.to_str().expect("a path")], results.as_bytes());
    fs::remove_file(&wider).expect("cleanup");
    assert_eq!(code, 0);
    same_lines(&stdout, &fixture("golden/audit-decide.jsonl")).expect("the golden");

    let key = key.to_str().expect("a path");
    let (_, band, _) = audit(
        &["small/decide.jsonl", key, "--threshold", "0.40:0.60"],
        b"",
    );
    assert_eq!(member(&band, "/threshold"), "\"0.40:0.60\"");
    let (code, stdout, stderr) = audit(&["-", key], b"");
    assert_eq!((code, stdout.as_str(), stderr.as_str()), (0, "", ""));
}

/// Each failure row: the arguments, standard input, exit code, and the whole standard error.
#[test]
fn each_failure_prints_one_line_that_names_no_record_id_value_or_path() {
    let root = std::env::temp_dir().join(format!("thinkthen-0113-refused-{}", std::process::id()));
    let _absent = fs::remove_dir_all(&root);
    fs::create_dir(&root).expect("a folder");
    let file = |name: &str, text: &str| {
        let path = root.join(name);
        fs::write(&path, text).expect("a file");
        path.to_string_lossy().into_owned()
    };
    let line = |fields: &str| {
        format!("{{\"input\":{{\"id\":\"secret-id-5150\",\"text\":\"secret-text\"}},{fields}}}\n")
    };
    let yes = line("\"value\":true,\"answer\":{\"probability\":0.9}");
    let key = file(
        "key.jsonl",
        "{\"id\":\"secret-id-5150\",\"value\":\"secret-value\",\"part\":\"tune\"}\n",
    );
    let missing = root.join("secret-path").to_string_lossy().into_owned();
    let cases: Vec<(Vec<String>, String, i32, &str)> = vec![
        (
            vec![missing.clone(), key.clone()],
            String::new(),
            5,
            "cannot read the results file",
        ),
        (
            vec!["-".into(), missing],
            yes.clone(),
            5,
            "cannot read the key file",
        ),
        (
            vec!["-".into(), key.clone()],
            format!("{yes}\n[\"secret-text\"]\n"),
            2,
            "results line 3 is not a JSON object",
        ),
        (
            vec!["-".into(), file("dup.jsonl", "{\"id\":1,\"id\":2}\n")],
            yes.clone(),
            2,
            "key line 1 is not a JSON object",
        ),
        (
            vec!["-".into(), key.clone()],
            "{\"input\":{\"id\":1.5},\"value\":true}\n".into(),
            2,
            "results line 1 has no string or integer id at the --id pointer",
        ),
        (
            vec!["-".into(), key.clone(), "--id".into(), "secret-text".into()],
            yes.clone(),
            2,
            "--id takes a JSON pointer such as /id or ''",
        ),
        (
            vec!["-".into(), key.clone()],
            line("\"value\":[],\"question\":{\"verb\":\"tag\"}"),
            2,
            "results line 1 holds an answer audit cannot grade; audit grades decide and choose",
        ),
        (
            vec!["-".into(), key.clone()],
            line("\"value\":true,\"answer\":{\"probability\":1.5}"),
            2,
            "results line 1 holds a probability outside 0 to 1 or an empty distribution",
        ),
        (
            vec!["-".into(), key.clone()],
            line("\"value\":\"a\",\"answer\":{\"probabilities\":{}}"),
            2,
            "results line 1 holds a probability outside 0 to 1 or an empty distribution",
        ),
        (
            vec!["-".into(), key.clone()],
            format!("{yes}{yes}"),
            2,
            "results line 2 repeats a record for one question",
        ),
        (
            vec![
                "-".into(),
                file(
                    "part.jsonl",
                    "{\"id\":\"secret-id-5150\",\"value\":true,\"part\":\"secret-value\"}\n",
                ),
            ],
            yes.clone(),
            2,
            "key line 1 needs a new id, a value, and a part of tune or held when present",
        ),
        (
            vec![
                "-".into(),
                file("choose.jsonl", "{\"id\":\"secret-id-5150\",\"value\":5}\n"),
            ],
            line("\"value\":\"a\""),
            2,
            "key line 1 gives a choose value that is not text",
        ),
        (
            vec![
                "-".into(),
                file(
                    "mixed.jsonl",
                    "{\"id\":\"secret-id-5150\",\"value\":true,\"part\":\"tune\"}\n{\"id\":\"b\",\"value\":true}\n",
                ),
            ],
            format!("{yes}{}", yes.replace("secret-id-5150", "b")),
            2,
            "the key gives a part on some labeled records and not on others",
        ),
        (
            vec!["-".into(), key.clone()],
            [
                line("\"question\":{\"text\":\"secret-text\"},\"value\":true"),
                line("\"question\":{\"text\":\"secret-text\"},\"value\":\"a\"")
                    .replace("secret-id-5150", "b"),
            ]
            .concat(),
            2,
            "one question holds both decide and choose answers",
        ),
        (
            vec![
                "-".into(),
                file("yes.jsonl", "{\"id\":\"secret-id-5150\",\"value\":true}\n"),
                "--threshold".into(),
                "0.5".into(),
            ],
            line("\"value\":true"),
            2,
            "--threshold needs probabilities; rerun the question with --details",
        ),
        (
            vec![
                "small/choose.jsonl".into(),
                "small/choose-key.jsonl".into(),
                "--threshold".into(),
                "0.4:0.6".into(),
            ],
            String::new(),
            2,
            "choose takes a single cut; a band applies to decide",
        ),
        (
            vec!["-".into(), "-".into()],
            yes.clone(),
            2,
            "only one input may be standard input",
        ),
        (
            vec!["-".into(), key.clone(), "--threshold".into(), "0".into()],
            yes,
            2,
            "--threshold: a single cut is above zero and at most one",
        ),
    ];
    for (arguments, input, expected_code, sentence) in cases {
        let arguments: Vec<&str> = arguments.iter().map(String::as_str).collect();
        let (code, stdout, stderr) = audit(&arguments, input.as_bytes());
        assert_eq!(
            stderr,
            format!("thinkthen: audit: {sentence}\n"),
            "{arguments:?}"
        );
        assert_eq!((code, stdout.as_str()), (expected_code, ""), "{sentence}");
        for secret in [
            "secret-id",
            "5150",
            "secret-text",
            "secret-value",
            "secret-path",
        ] {
            assert!(!stderr.contains(secret), "{sentence}");
        }
    }
    for arguments in [
        &["--seed", "-1"][..],
        &["--target", "1.5"],
        &["--by", "record"],
    ] {
        let (code, stdout, _) = audit(
            &[&["small/decide.jsonl", "small/decide-key.jsonl"], arguments].concat(),
            b"",
        );
        assert_eq!((code, stdout.as_str()), (2, ""), "{arguments:?}");
    }
    fs::remove_dir_all(&root).expect("cleanup");
}

#[test]
fn a_replayed_recording_piped_to_audit_grades_as_the_prototype_does() {
    let rows = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../transforms/rows");
    let question = fs::read_to_string(rows.join("question.txt")).expect("the question");
    let decide = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .args([
            "decide",
            question.trim(),
            "--jsonl",
            "--field",
            "/body",
            "--details",
            "--replay",
        ])
        .arg(rows.join("recording"))
        .arg("--input")
        .arg(rows.join("cases.jsonl"))
        .env_clear()
        .stdout(Stdio::piped())
        .spawn()
        .expect("decide runs");
    let output = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .args(["audit", "-", "replay/key.jsonl"])
        .env_clear()
        .current_dir(fixtures())
        .stdin(decide.stdout.expect("the pipe"))
        .output()
        .expect("audit runs");
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        fixture("replay/audit.jsonl")
    );
}

#[test]
fn each_question_draws_its_own_bootstrap_and_the_held_out_half_reads_as_reported() {
    let control = fixture("249/control.jsonl");
    let (mut both, mut second) = (String::new(), String::new());
    for (place, line) in control.lines().enumerate() {
        let mut row: serde_json::Value = serde_json::from_str(line).expect("a line");
        row["question"]["text"] = if place % 2 == 1 { "B" } else { "A" }.into();
        let line = format!("{row}\n");
        both.push_str(&line);
        if place % 2 == 1 {
            second.push_str(&line);
        }
    }
    let interval = |input: &str| {
        let (code, stdout, _) = audit(&["-", "249/key.jsonl"], input.as_bytes());
        assert_eq!(code, 0);
        member(
            stdout.lines().last().expect("the second group"),
            "/calibration/interval",
        )
    };
    assert_eq!(interval(&both), "[0.049765,0.164636]");
    assert_eq!(interval(&second), "[0.049765,0.164636]");

    let held: String = fixture("249/key.jsonl")
        .lines()
        .filter(|l| l.contains("\"held\""))
        .map(|l| format!("{l}\n"))
        .collect();
    let path = std::env::temp_dir().join(format!("thinkthen-0113-held-{}", std::process::id()));
    fs::write(&path, held).expect("a key");
    let (_, stdout, _) = audit(
        &[
            "249/control.jsonl",
            path.to_str().expect("a path"),
            "--by",
            "verb",
        ],
        b"",
    );
    fs::remove_file(&path).expect("cleanup");
    let read = |pointer: &str| {
        member(&stdout, pointer)
            .parse::<f64>()
            .map(|v| format!("{v:.3}"))
            .expect("a number")
    };
    assert_eq!(member(&stdout, "/labeled"), "138");
    let numbers = [
        "/agreement",
        "/yes_recall",
        "/mean_probability",
        "/auc",
        "/calibration/error",
    ]
    .map(read);
    assert_eq!(numbers, ["0.630", "0.311", "0.397", "0.725", "0.092"]);
}

#[test]
fn help_names_audit_after_transform_and_says_what_it_never_does() {
    const ROW: &str = "Grade saved decide and choose answers against an answer key.";
    let text =
        |arguments: &[&str]| String::from_utf8(run(arguments, b"").stdout).expect("UTF-8 help");
    let root = text(&["--help"]);
    assert!(root.contains(&format!("\n  transform  List or print the built-in jq transforms without running them\n  audit      {}\n  help ", ROW.trim_end_matches('.'))), "{root}");
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

#[test]
fn audit_sends_no_request_reads_no_key_and_writes_nothing() {
    let root = std::env::temp_dir().join(format!("thinkthen-0113-guarded-{}", std::process::id()));
    let _absent = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("locked")).expect("a folder");
    let listener = TcpListener::bind("127.0.0.1:0").expect("a loopback listener");
    listener.set_nonblocking(true).expect("nonblocking");
    let url = format!("http://{}/v1", listener.local_addr().expect("an address"));
    let before = (tree(&root), tree(&fixtures()));
    fs::set_permissions(root.join("locked"), fs::Permissions::from_mode(0o000)).expect("locked");
    let mut lines: Vec<Vec<&str>> = GOLDENS
        .iter()
        .map(|(_, arguments)| arguments.to_vec())
        .collect();
    lines.extend(
        TABLES
            .iter()
            .map(|(_, [results, key])| vec![*results, *key, "--table"]),
    );
    lines.push(vec![
        "small/choose.jsonl",
        "small/choose-key.jsonl",
        "--threshold",
        "0.4:0.6",
    ]);
    for arguments in lines {
        let mut command = Command::new(env!("CARGO_BIN_EXE_thinkthen"));
        command
            .arg("audit")
            .args(&arguments)
            .env_clear()
            .current_dir(fixtures());
        for variable in [
            "HOME",
            "XDG_CONFIG_HOME",
            "XDG_CACHE_HOME",
            "THINKTHEN_CACHE",
        ] {
            command.env(variable, root.join("locked").join(variable));
        }
        let output = command
            .env("THINKTHEN_API_KEY", "canary-0113")
            .env("THINKTHEN_BASE_URL", &url)
            .output()
            .expect("the binary runs");
        assert!(matches!(output.status.code(), Some(0 | 2)), "{arguments:?}");
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

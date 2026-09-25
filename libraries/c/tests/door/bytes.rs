//! The door's own serializer against the command's bytes.
//!
//! The door writes the bare `decide`, `choose`, `score`, `tag`, `filter`,
//! `rank`, and `find` values itself; 0098's test holds the JSON methods to
//! the command. Each shared case of those verbs runs through the door and
//! through the compiled command on the same case arm. A single value and a
//! line feed equal the command's output. A list equals the command's
//! `--jsonl` lines joined into one array, and `find` runs on the generic arm.

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use conformance_backend::Backend;
use serde_json::json;

use crate::cases::{Members, Script, member, renamed_verb, replies, string, with};
use crate::{KEY, compile, crate_dir, run, scratch, text};

const CASES: &str = include_str!("../../../../conformance/cases.json");

#[test]
fn the_doors_bare_values_are_the_commands_bytes() {
    let written: Members = serde_json::from_str(CASES).expect("the shared cases");
    let cases: Vec<Members> = serde_json::from_str(written["cases"].get()).expect("a case list");
    let command = built_command();
    let backend = Backend::start().expect("the conformance backend");
    let folder = scratch("bytes");
    let mut script = Script::default();
    let mut printed = Vec::new();
    for case in &cases {
        let (id, verb) = (string(case, "id"), string(case, "verb"));
        let expect = member(case, "expect");
        let kind = expect["success"]["kind"].as_str().unwrap_or_default();
        if !matches!(kind, "single" | "filter" | "rank") {
            continue;
        }
        let base = format!("{}/case/{id}/v1", backend.origin());
        let (request, said) = asked(&command, &folder, case, &base, kind, &verb);
        // A cache folder answers one backend address, so each case gets its own.
        let cache = folder.join(format!("cache-{id}")).display().to_string();
        script.ask("env", &["THINKTHEN_CACHE", &cache]);
        script.ask("call", &[&base, &request]);
        printed.push((id, said));
    }
    let generic = format!("{}/generic/v1", backend.origin());
    let units = ["good morning", "I want a refund"];
    let said = print(
        &command,
        &[
            "find",
            "Which line asks for money back?",
            "--jsonl",
            "--url",
            &generic,
        ],
        &units
            .iter()
            .map(|one| json!(one).to_string() + "\n")
            .collect::<String>(),
    );
    let request = json!({"find": "Which line asks for money back?", "units": units}).to_string();
    let cache = folder.join("cache-find").display().to_string();
    script.ask("env", &["THINKTHEN_CACHE", &cache]);
    script.ask("call", &[&generic, &request]);
    printed.push(("find".to_owned(), said));

    let output = run(
        &compile(&crate_dir().join("tests/c/driver.c")),
        "",
        &script.0,
    );
    assert!(output.status.success(), "{}", text(&output.stderr));
    let answered = replies(&output.stdout).expect("framed replies");
    assert_eq!(answered.len(), printed.len());
    let differ: Vec<String> = printed
        .iter()
        .zip(&answered)
        .filter(|((_, said), (code, reply))| *code != 0 || format!("{reply}\n") != *said)
        .map(|((id, said), (code, reply))| {
            format!("{id}: the door wrote {code} {reply}, the command printed {said}")
        })
        .collect();
    assert!(differ.is_empty(), "{differ:#?}");
    assert_eq!(printed.len(), 21);
}

/// One case's request to the door and the command's printed bytes.
fn asked(
    command: &Path,
    folder: &Path,
    case: &Members,
    base: &str,
    kind: &str,
    verb: &str,
) -> (String, String) {
    let id = string(case, "id");
    let question = case.get("question").map_or("{}", |raw| raw.get());
    let path = folder.join(format!("{id}.json"));
    std::fs::write(&path, question).expect("a question file");
    let at = format!("@{}", path.display());
    let exchanges = member(case, "exchanges");
    let texts: Vec<&str> = exchanges
        .as_array()
        .into_iter()
        .flatten()
        .map(|exchange| exchange["evidence"].as_str().unwrap_or_default())
        .collect();
    let lines: String = texts
        .iter()
        .map(|one| json!(one).to_string() + "\n")
        .collect();
    let records = json!(texts).to_string();
    match kind {
        "single" => {
            let evidence = json!(texts.first().copied().unwrap_or_default()).to_string();
            let said = print(
                command,
                &[verb, &at, "--url", base],
                texts.first().copied().unwrap_or_default(),
            );
            (with(question, "evidence", &evidence), said)
        }
        "filter" => {
            let said = print(command, &["filter", &at, "--jsonl", "--url", base], &lines);
            (
                with(
                    &renamed_verb(question, "filter").expect("a decide question"),
                    "records",
                    &records,
                ),
                joined(&said),
            )
        }
        _ => {
            let asked = serde_json::from_str::<serde_json::Value>(question).expect("a question");
            let asked = asked["decide"].as_str().unwrap_or_default();
            let said = print(command, &["rank", asked, "--jsonl", "--url", base], &lines);
            (
                json!({"rank": asked, "records": texts}).to_string(),
                joined(&said),
            )
        }
    }
}

/// The command's `--jsonl` lines as one array and a line feed.
fn joined(lines: &str) -> String {
    format!("[{}]\n", lines.lines().collect::<Vec<_>>().join(","))
}

/// The root command, built once into this test's folder.
fn built_command() -> PathBuf {
    let target = Path::new(env!("CARGO_TARGET_TMPDIR")).join("command");
    let built = Command::new(env!("CARGO"))
        .args([
            "build",
            "--locked",
            "--offline",
            "--bin",
            "thinkthen",
            "--manifest-path",
        ])
        .arg(crate_dir().join("../../Cargo.toml"))
        .arg("--target-dir")
        .arg(&target)
        .output()
        .expect("cargo ran");
    assert!(built.status.success(), "{}", text(&built.stderr));
    target.join("debug/thinkthen")
}

/// The command's standard output for one input, without a cache.
fn print(command: &Path, arguments: &[&str], input: &str) -> String {
    let mut child = Command::new(command)
        .env_clear()
        .env("HOME", scratch("bytes-home"))
        .env("THINKTHEN_API_KEY", KEY)
        .args(arguments)
        .arg("--no-cache")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the command started");
    let mut stdin = child.stdin.take().expect("its input");
    std::io::Write::write_all(&mut stdin, input.as_bytes()).expect("its input was written");
    drop(stdin);
    let output = child.wait_with_output().expect("its output");
    assert!(
        matches!(output.status.code(), Some(0 | 1 | 3 | 6)),
        "{arguments:?}: {}",
        text(&output.stderr)
    );
    text(&output.stdout)
}

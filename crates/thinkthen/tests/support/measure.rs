//! Run a measuring command from the measure fixtures and read what it printed.
#![allow(
    dead_code,
    clippy::excessive_nesting,
    clippy::indexing_slicing,
    reason = "each test file uses part of the helper, and the walker reads JSON the command printed"
)]

use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

/// Each golden file and the audit command line that prints it.
pub(crate) const GOLDENS: [(&str, &[&str]); 12] = [
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
    (
        "golden/extra/audit-choose-target-1.jsonl",
        &[
            "small/choose.jsonl",
            "small/choose-key.jsonl",
            "--target",
            "1",
        ],
    ),
    (
        "golden/extra/audit-249-question-seed-7.jsonl",
        &[
            "249/control.jsonl",
            "249/key.jsonl",
            "--by",
            "question",
            "--seed",
            "7",
        ],
    ),
];

/// Each table capture and the inputs it grades.
pub(crate) const TABLES: [(&str, [&str; 2]); 3] = [
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

/// Each diff golden file and the diff command line that prints it.
pub(crate) const DIFF_GOLDENS: [(&str, &[&str]); 8] = [
    (
        "golden/diff-decide-cuts.jsonl",
        &[
            "small/decide.jsonl",
            "--key",
            "small/decide-key.jsonl",
            "--compare-threshold",
            "0.4",
        ],
    ),
    (
        "golden/diff-decide-wordings.jsonl",
        &[
            "small/decide.jsonl",
            "small/decide-b.jsonl",
            "--key",
            "small/decide-key.jsonl",
        ],
    ),
    (
        "golden/diff-decide-nokey.jsonl",
        &["small/decide.jsonl", "small/decide-b.jsonl"],
    ),
    (
        "golden/diff-choose.jsonl",
        &[
            "small/choose.jsonl",
            "small/choose-b.jsonl",
            "--key",
            "small/choose-key.jsonl",
        ],
    ),
    (
        "golden/diff-249-cuts.jsonl",
        &[
            "249/control.jsonl",
            "--key",
            "249/key.jsonl",
            "--compare-threshold",
            "0.42",
        ],
    ),
    (
        "golden/diff-249-soft.jsonl",
        &[
            "249/control.jsonl",
            "249/soft.jsonl",
            "--key",
            "249/key.jsonl",
        ],
    ),
    (
        "golden/extra/diff-annotate.jsonl",
        &[
            "small/annotate.jsonl",
            "--key",
            "small/annotate-key.jsonl",
            "--compare-threshold",
            "0.75",
        ],
    ),
    (
        "golden/extra/diff-249-cuts-nokey.jsonl",
        &["249/control.jsonl", "--compare-threshold", "0.42"],
    ),
];

/// Each diff table capture and the command line that prints it without `--table`.
pub(crate) const DIFF_TABLES: [(&str, &[&str]); 3] = [
    (
        "golden/table/diff-decide-cuts.txt",
        &[
            "small/decide.jsonl",
            "--key",
            "small/decide-key.jsonl",
            "--compare-threshold",
            "0.4",
        ],
    ),
    (
        "golden/table/diff-decide-nokey.txt",
        &["small/decide.jsonl", "small/decide-b.jsonl"],
    ),
    (
        "golden/table/diff-choose.txt",
        &[
            "small/choose.jsonl",
            "small/choose-b.jsonl",
            "--key",
            "small/choose-key.jsonl",
        ],
    ),
];

/// The fixture folder every command line runs from.
pub(crate) fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/measure")
}

/// Run the binary from the fixture folder with a clear environment and these bytes on standard input.
pub(crate) fn run(arguments: &[&str], input: &[u8]) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .args(arguments)
        .env_clear()
        .current_dir(fixtures())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the compiled binary runs");
    let mut stdin = child.stdin.take().expect("standard input");
    let _ignored = stdin.write_all(input);
    drop(stdin);
    crate::wait::finish(child, "thinkthen").expect("the binary finishes")
}

/// Run `audit` with these arguments and input: the exit code, standard output, and standard error.
pub(crate) fn audit(arguments: &[&str], input: &[u8]) -> (i32, String, String) {
    measure(&[&["audit"], arguments].concat(), input)
}

/// Run a command line with this input: the exit code, standard output, and standard error.
pub(crate) fn measure(arguments: &[&str], input: &[u8]) -> (i32, String, String) {
    let output = run(arguments, input);
    let text = |bytes: Vec<u8>| String::from_utf8(bytes).expect("UTF-8 output");
    (
        output.status.code().expect("an exit code"),
        text(output.stdout),
        text(output.stderr),
    )
}

/// A fixture file as text.
pub(crate) fn fixture(path: &str) -> String {
    fs::read_to_string(fixtures().join(path)).expect("a fixture")
}

/// One row member as its compact JSON text.
pub(crate) fn member(line: &str, pointer: &str) -> String {
    let row: serde_json::Value = serde_json::from_str(line).expect("a JSON row");
    row.pointer(pointer).expect("the member").to_string()
}

/// The members tickets 0125 and 0131 added beside the ones the goldens hold.
const ADDED_MEMBERS: [&str; 7] = [
    "precision",
    "f1",
    "r_precision",
    "mean_level_distance",
    "steady",
    "crossed",
    "curve",
];

/// The openings of the table lines tickets 0125 and 0131 added.
const ADDED_LINES: [&str; 7] = [
    "  precision ",
    "  r-precision",
    "  mean level distance",
    "  suggested level cuts",
    "  steady:",
    "  at the suggested cut on the held part",
    "  crossed:",
];

/// Compact JSON lines without the added members, every other byte copied.
pub(crate) fn without_added(lines: &str) -> String {
    let bytes = lines.as_bytes();
    let (mut at, mut out) = (0, Vec::new());
    while at < bytes.len() {
        if bytes[at] == b'{' {
            copy(bytes, &mut at, &mut out);
        } else {
            out.push(bytes[at]);
            at += 1;
        }
    }
    String::from_utf8(out).expect("UTF-8")
}

/// A table without the added lines.
pub(crate) fn without_added_lines(table: &str) -> String {
    table
        .split_inclusive('\n')
        .filter(|line| !ADDED_LINES.iter().any(|opening| line.starts_with(opening)))
        .collect()
}

fn copy(bytes: &[u8], at: &mut usize, out: &mut Vec<u8>) {
    match bytes[*at] {
        open @ (b'{' | b'[') => {
            let close = if open == b'{' { b'}' } else { b']' };
            out.push(open);
            *at += 1;
            let mut first = true;
            while bytes[*at] != close {
                if bytes[*at] == b',' {
                    *at += 1;
                }
                let mut item = Vec::new();
                let start = *at;
                copy(bytes, at, &mut item);
                if open == b'{' {
                    let name = &bytes[start + 1..*at - 1];
                    *at += 1;
                    item.push(b':');
                    copy(bytes, at, &mut item);
                    if ADDED_MEMBERS.iter().any(|added| added.as_bytes() == name) {
                        continue;
                    }
                }
                if !first {
                    out.push(b',');
                }
                first = false;
                out.extend(item);
            }
            out.push(close);
            *at += 1;
        }
        b'"' => {
            out.push(b'"');
            *at += 1;
            while bytes[*at] != b'"' {
                let step = if bytes[*at] == b'\\' { 2 } else { 1 };
                out.extend(&bytes[*at..*at + step]);
                *at += step;
            }
            out.push(b'"');
            *at += 1;
        }
        _ => {
            while !b",}]\n".contains(&bytes[*at]) {
                out.push(bytes[*at]);
                *at += 1;
            }
        }
    }
}

/// The repository folder that holds the demos and transforms.
pub(crate) fn repository() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Run the binary in a folder with a clear environment; standard output of a run that exits 0.
pub(crate) fn replay(folder: &Path, arguments: &[&str], input: &[u8]) -> String {
    let mut child = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .args(arguments)
        .env_clear()
        .env("THINKTHEN_BATCH", "1")
        .current_dir(folder)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the compiled binary runs");
    let mut stdin = child.stdin.take().expect("standard input");
    let _ignored = stdin.write_all(input);
    drop(stdin);
    let output = crate::wait::finish(child, "thinkthen").expect("the binary finishes");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(0), "{arguments:?} {stderr}");
    String::from_utf8(output.stdout).expect("UTF-8 output")
}

/// The payment question of `transforms/rows`, replayed through the question file FILE in FOLDER.
pub(crate) fn payment_rows(folder: &Path, file: &str) -> String {
    let rows = repository().join("transforms/rows");
    let [recording, cases] = [rows.join("recording"), rows.join("cases.jsonl")];
    let [recording, cases] = [&recording, &cases].map(|path| path.to_str().expect("a path"));
    let at = format!("@{file}");
    let arguments = ["decide", &at, "--jsonl", "--details", "--replay", recording];
    replay(folder, &[&arguments[..], &["--input", cases]].concat(), b"")
}

/// Each named member of a JSON line, as compact JSON with sorted keys.
pub(crate) fn members<const N: usize>(line: &str, pointers: [&str; N]) -> [String; N] {
    pointers.map(|pointer| member(line, pointer))
}

/// JSON text as compact JSON with sorted keys, to compare with a member.
pub(crate) fn compact(text: &str) -> String {
    serde_json::from_str::<serde_json::Value>(text)
        .expect("JSON")
        .to_string()
}

/// A single-input run prints no `input`, so a test gives each line its number as an id.
pub(crate) fn with_ids(lines: &str) -> String {
    lines
        .lines()
        .enumerate()
        .map(|(place, line)| {
            let mut row: serde_json::Value = serde_json::from_str(line).expect("a line");
            row["input"] = serde_json::json!({"id": place + 1});
            format!("{row}\n")
        })
        .collect()
}

/// Demo 06's hits as saved `rank` rows, asked with the question text or `@FILE`.
pub(crate) fn ranked(question: &str) -> String {
    let demo = repository().join("demos/06-top-search-hits");
    let hits = fs::read_to_string(demo.join("hits.jsonl")).expect("hits");
    let query = "Why is signing in slow or failing?";
    let records: String = hits
        .lines()
        .map(|line| {
            let hit: serde_json::Value = serde_json::from_str(line).expect("a hit");
            let record =
                serde_json::json!({"query": query, "passage": hit["body"], "path": hit["path"]});
            format!("{record}\n")
        })
        .collect();
    let options = "--jsonl --field /query --field /passage --replay recording --details";
    let arguments = [
        &["rank", question][..],
        &options.split(' ').collect::<Vec<_>>(),
    ]
    .concat();
    replay(&demo, &arguments, records.as_bytes())
}

/// Demo 15's saved `find` run.
pub(crate) fn found() -> String {
    let question = "When does a refund reach the customer?";
    let options = "--lines --model jev-1.13.0 --input policy.txt --replay recording --details";
    let arguments = [
        &["find", question][..],
        &options.split(' ').collect::<Vec<_>>(),
    ]
    .concat();
    replay(
        &repository().join("demos/15-find-the-line"),
        &arguments,
        b"",
    )
}

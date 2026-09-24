//! Run a measuring command from the measure fixtures, and compare its JSON lines with a golden file.
//!
//! Two outputs match when they hold the same JSON values in the same order,
//! objects with the same member names in the same order, and each golden
//! float is a float within `1e-6 + 1e-12`, one last place after rounding.
#![allow(dead_code, reason = "each test file uses part of the helper")]

use std::fmt;
use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use serde::de::{Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};

/// Each golden file and the audit command line that prints it.
pub(crate) const GOLDENS: [(&str, &[&str]); 10] = [
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
    child.wait_with_output().expect("the binary finishes")
}

/// One JSON value that keeps member order and tells a float from an integer.
#[derive(Debug)]
enum Ordered {
    Null,
    Bool(bool),
    Integer(i128),
    Float(f64),
    Text(String),
    List(Vec<Ordered>),
    Object(Vec<(String, Ordered)>),
}

struct Reader;

impl<'de> Visitor<'de> for Reader {
    type Value = Ordered;
    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a JSON value")
    }
    fn visit_unit<E>(self) -> Result<Ordered, E> {
        Ok(Ordered::Null)
    }
    fn visit_bool<E>(self, value: bool) -> Result<Ordered, E> {
        Ok(Ordered::Bool(value))
    }
    fn visit_i64<E>(self, value: i64) -> Result<Ordered, E> {
        Ok(Ordered::Integer(value.into()))
    }
    fn visit_u64<E>(self, value: u64) -> Result<Ordered, E> {
        Ok(Ordered::Integer(value.into()))
    }
    fn visit_f64<E>(self, value: f64) -> Result<Ordered, E> {
        Ok(Ordered::Float(value))
    }
    fn visit_str<E>(self, value: &str) -> Result<Ordered, E> {
        Ok(Ordered::Text(value.to_owned()))
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut access: A) -> Result<Ordered, A::Error> {
        let mut items = Vec::new();
        while let Some(item) = access.next_element()? {
            items.push(item);
        }
        Ok(Ordered::List(items))
    }
    fn visit_map<A: MapAccess<'de>>(self, mut access: A) -> Result<Ordered, A::Error> {
        let mut members = Vec::new();
        while let Some(member) = access.next_entry()? {
            members.push(member);
        }
        Ok(Ordered::Object(members))
    }
}

impl<'de> Deserialize<'de> for Ordered {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(Reader)
    }
}

fn same(got: &Ordered, want: &Ordered, at: &str) -> Result<(), String> {
    let differs = || Err(format!("{at}: got {got:?}, want {want:?}"));
    match (got, want) {
        (Ordered::Float(x), Ordered::Float(y)) if (x - y).abs() <= 1e-6 + 1e-12 => Ok(()),
        (Ordered::List(xs), Ordered::List(ys)) if xs.len() == ys.len() => xs
            .iter()
            .zip(ys)
            .enumerate()
            .try_for_each(|(place, (x, y))| same(x, y, &format!("{at}[{place}]"))),
        (Ordered::Object(xs), Ordered::Object(ys))
            if xs.iter().map(|m| &m.0).eq(ys.iter().map(|m| &m.0)) =>
        {
            xs.iter()
                .zip(ys)
                .try_for_each(|((name, x), (_, y))| same(x, y, &format!("{at}/{name}")))
        }
        (Ordered::Null, Ordered::Null) => Ok(()),
        (Ordered::Bool(x), Ordered::Bool(y)) if x == y => Ok(()),
        (Ordered::Integer(x), Ordered::Integer(y)) if x == y => Ok(()),
        (Ordered::Text(x), Ordered::Text(y)) if x == y => Ok(()),
        _ => differs(),
    }
}

/// Compare the JSON lines a command printed with a golden file's.
pub(crate) fn same_lines(got: &str, golden: &str) -> Result<(), String> {
    let parse = |text: &str| -> Vec<Ordered> {
        text.lines()
            .map(|line| serde_json::from_str(line).expect("a JSON line"))
            .collect()
    };
    same(
        &Ordered::List(parse(got)),
        &Ordered::List(parse(golden)),
        "",
    )
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

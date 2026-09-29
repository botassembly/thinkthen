//! A tuned audit candidate is a complete new file beside unchanged input.
#![cfg(feature = "cli")]
#![allow(clippy::expect_used, reason = "a failed fixture stops the proof")]

#[path = "support/measure.rs"]
mod measure_support;
#[path = "../src/test_deadline/wait.rs"]
mod wait;

use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};

use conformance_backend::{Canned, Listener};
use measure_support::{audit, fixture, fixtures, payment_rows, ranked, replay, repository};
use thinkthen::QuestionSet;

const SINGLE_OUTPUT: &str = "{\r\n  \"decide\": \"Does the message report a payment failure?\",\r\n  \"\\u0074hreshold\": 0.59,\r\n  \"on\": [\"/body\"],\r\n  \"model\": \"jev-1.13.0\"\r\n}\r\n";
const SET_OUTPUT: &str = concat!(
    "{\n  \"version\": 1,\n  \"threshold\": 0.95,\n  \"questions\": {\n",
    "    \"correct\": {\"decide\": \"Does the answer say the same thing as the reference answer to the question?\", \"on\": [\"/input\", \"/gold\", \"/output\"], \"threshold\": 0.54},\n",
    "    \"grounded\": {\"decide\": \"Is every claim in the answer stated in the source text?\", \"on\": [\"/context\", \"/output\"]},\n",
    "    \"complete\": {\"decide\": \"Does the answer include every fact needed to answer the question as fully as the reference answer?\", \"on\": [\"/input\", \"/gold\", \"/output\"]},\n",
    "    \"failure_kind\": {\"choose\": \"Which description best fits the answer's main failure?\", \"options\": [\"none\", \"wrong_fact\", \"unsupported\", \"incomplete\"], \"on\": [\"/input\", \"/gold\", \"/output\"]},\n",
    "    \"severity\": {\"score\": \"How much does the answer's main failure change what the reader should do?\", \"levels\": [\"No change.\", \"A small correction is needed.\", \"The answer could cause the wrong action.\"], \"on\": [\"/input\", \"/gold\", \"/output\"]}\n",
    "  }\n}\n",
);

struct Scratch(PathBuf);

static NEXT_SCRATCH: AtomicU64 = AtomicU64::new(0);

impl Scratch {
    fn new(name: &str) -> Self {
        loop {
            let number = NEXT_SCRATCH.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "thinkthen-0256-{name}-{}-{number}",
                std::process::id()
            ));
            match fs::create_dir(&path) {
                Ok(()) => return Self(path),
                Err(error) => assert!(
                    error.kind() == ErrorKind::AlreadyExists,
                    "cannot create scratch directory: {error}"
                ),
            }
        }
    }

    fn path(&self, name: &str) -> String {
        self.0.join(name).to_string_lossy().into_owned()
    }

    fn write(&self, name: &str, text: &str) -> String {
        fs::write(self.0.join(name), text).expect("scratch file");
        self.path(name)
    }

    fn names(&self) -> Vec<String> {
        let mut names: Vec<_> = fs::read_dir(&self.0)
            .expect("scratch entries")
            .map(|entry| {
                entry
                    .expect("entry")
                    .file_name()
                    .to_string_lossy()
                    .into_owned()
            })
            .collect();
        names.sort();
        names
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _removed = fs::remove_dir_all(&self.0);
    }
}

fn key() -> String {
    fixtures()
        .join("write/key.jsonl")
        .to_string_lossy()
        .into_owned()
}

#[test]
fn a_single_question_writes_literal_tuned_bytes_without_touching_the_source_or_sending() {
    let scratch = Scratch::new("single");
    let source_bytes = fixture("write/decide.json");
    let source = scratch.write("source.json", &source_bytes);
    let results = scratch.write("rows.jsonl", &payment_rows(&scratch.0, "source.json"));
    let output = scratch.path("candidate.json");
    let listener = Listener::answering(|_| Canned::ok("{}")).expect("listener");
    let child = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .args([
            "audit",
            &results,
            &key(),
            "--write",
            &source,
            "--write-to",
            &output,
        ])
        .env_clear()
        .env("THINKTHEN_BASE_URL", listener.base())
        .env("THINKTHEN_API_KEY", "canary-0256")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("audit child");
    let result = wait::finish(child, "audit output").expect("audit finishes");
    let stdout = String::from_utf8(result.stdout).expect("stdout");
    let stderr = String::from_utf8(result.stderr).expect("stderr");
    assert_eq!(result.status.code(), Some(0), "{stderr}");
    assert!(!stdout.is_empty());
    assert_eq!(
        stderr,
        "thinkthen: audit: wrote threshold 0.59 for the question; it was 0.9\n"
    );
    assert!(!stdout.contains("canary-0256") && !stderr.contains("canary-0256"));
    assert_eq!(listener.count(), 0);
    assert_eq!(fs::read(&source).expect("source"), source_bytes.as_bytes());
    assert_eq!(fs::read_to_string(&output).expect("output"), SINGLE_OUTPUT);
    let named = format!("@{output}");
    let (code, plan, diagnostic) = measure_support::measure(
        &["decide", &named, "--jsonl", "--dry-run"],
        b"{\"body\":\"payment failure\"}\n",
    );
    assert_eq!(code, 0, "{diagnostic}");
    assert!(plan.contains("payment failure?"));
    assert_eq!(
        scratch.names(),
        ["candidate.json", "rows.jsonl", "source.json"]
    );
}

#[test]
fn a_set_writes_only_its_named_member_and_keeps_its_source() {
    let scratch = Scratch::new("set");
    let source_bytes = fixture("write/set.json");
    let source = scratch.write("set.json", &source_bytes);
    let demo = repository().join("demos/14-grade-a-batch");
    let arguments = [
        "annotate",
        &source,
        "--jsonl",
        "--details",
        "--replay",
        "recording",
        "--input",
        "cases.jsonl",
    ];
    let results = scratch.write("set.jsonl", &replay(&demo, &arguments, b""));
    let key = fixtures()
        .join("write/set-key.jsonl")
        .to_string_lossy()
        .into_owned();
    let output = scratch.path("candidate.json");
    let (code, stdout, stderr) = audit(
        &[&results, &key, "--write", &source, "--write-to", &output],
        b"",
    );
    assert_eq!(code, 0, "{stderr}");
    assert!(!stdout.is_empty());
    assert_eq!(
        stderr.lines().next(),
        Some("thinkthen: audit: wrote threshold 0.54 for question correct; it was absent")
    );
    assert_eq!(fs::read(&source).expect("source"), source_bytes.as_bytes());
    assert_eq!(fs::read_to_string(&output).expect("output"), SET_OUTPUT);
    QuestionSet::from_json(SET_OUTPUT).expect("reusable set");
    assert_eq!(scratch.names(), ["candidate.json", "set.json", "set.jsonl"]);
}

#[test]
fn a_no_bar_run_still_creates_an_independent_copy() {
    let scratch = Scratch::new("no-bar");
    let source_bytes = fixture("write/rank.json");
    let source = scratch.write("rank.json", &source_bytes);
    let results = scratch.write("rank.jsonl", &ranked(&format!("@{source}")));
    let key = scratch.write("empty.jsonl", "");
    let output = scratch.path("candidate.json");
    let (code, stdout, stderr) = audit(
        &[
            &results,
            &key,
            "--id",
            "/path",
            "--write",
            &source,
            "--write-to",
            &output,
        ],
        b"",
    );
    assert_eq!(
        (code, stderr.as_str()),
        (
            0,
            "thinkthen: audit: kept the question unchanged; audit suggests no bar for rank or find\n"
        )
    );
    assert!(!stdout.is_empty());
    assert_eq!(fs::read(&output).expect("output"), source_bytes.as_bytes());
    fs::write(&output, "{}\n").expect("separate output");
    assert_eq!(fs::read(&source).expect("source"), source_bytes.as_bytes());
}

#[test]
fn output_refusals_keep_source_and_every_existing_destination() {
    let scratch = Scratch::new("refuse");
    let source_bytes = fixture("write/decide.json");
    let source = scratch.write("source.json", &source_bytes);
    let results = scratch.write("rows.jsonl", &payment_rows(&scratch.0, "source.json"));
    let existing = scratch.write("empty.json", "");
    let hard = scratch.path("hard.json");
    fs::hard_link(&source, &hard).expect("hard link");
    let directory = scratch.path("directory.json");
    fs::create_dir(&directory).expect("existing directory");
    let paths = [
        source.as_str(),
        existing.as_str(),
        hard.as_str(),
        directory.as_str(),
    ];
    let expected = "thinkthen: audit: the --write-to output already exists; choose a new path\n";
    for output in paths {
        let before = scratch.names();
        let (code, stdout, stderr) = audit(
            &[&results, &key(), "--write", &source, "--write-to", output],
            b"",
        );
        assert_eq!((code, stdout.as_str(), stderr.as_str()), (5, "", expected));
        assert_eq!(scratch.names(), before, "no temporary name remains");
        assert_eq!(fs::read(&source).expect("source"), source_bytes.as_bytes());
        assert_eq!(fs::read(&existing).expect("existing"), b"");
    }
    let missing = scratch.path("missing/candidate.json");
    let (code, stdout, stderr) = audit(
        &[&results, &key(), "--write", &source, "--write-to", &missing],
        b"",
    );
    assert_eq!(
        (code, stdout.as_str(), stderr.as_str()),
        (
            5,
            "",
            "thinkthen: audit: cannot write the --write-to output\n"
        )
    );
    assert_eq!(fs::read(&source).expect("source"), source_bytes.as_bytes());
    let (code, stdout, stderr) = audit(&[&results, &key(), "--write-to", &missing], b"");
    assert_eq!(
        (code, stdout.as_str(), stderr.as_str()),
        (
            2,
            "",
            "thinkthen: audit: --write-to needs --write QUESTIONS\n"
        )
    );
    let (code, stdout, stderr) = audit(
        &[&results, &key(), "--write", &source, "--write-to", "-"],
        b"",
    );
    assert_eq!(
        (code, stdout.as_str(), stderr.as_str()),
        (2, "", "thinkthen: audit: --write-to needs a file path\n")
    );
}

#[cfg(unix)]
#[test]
fn symlink_destinations_are_existing_even_when_dangling() {
    let scratch = Scratch::new("symlink");
    let source_bytes = fixture("write/decide.json");
    let source = scratch.write("source.json", &source_bytes);
    let results = scratch.write("rows.jsonl", &payment_rows(&scratch.0, "source.json"));
    let expected = "thinkthen: audit: the --write-to output already exists; choose a new path\n";
    for (name, target) in [
        ("alias.json", source.as_str()),
        ("dangling.json", "absent.json"),
    ] {
        let output = scratch.path(name);
        std::os::unix::fs::symlink(target, &output).expect("symlink");
        let before = scratch.names();
        let (code, stdout, stderr) = audit(
            &[&results, &key(), "--write", &source, "--write-to", &output],
            b"",
        );
        assert_eq!((code, stdout.as_str(), stderr.as_str()), (5, "", expected));
        assert_eq!(scratch.names(), before);
        assert_eq!(fs::read_link(&output).expect("link"), PathBuf::from(target));
        assert_eq!(fs::read(&source).expect("source"), source_bytes.as_bytes());
    }
}

#[test]
fn an_old_digest_cannot_publish_a_second_candidate() {
    let scratch = Scratch::new("digest");
    let source = scratch.write("source.json", &fixture("write/decide.json"));
    let results = scratch.write("rows.jsonl", &payment_rows(&scratch.0, "source.json"));
    let first = scratch.path("first.json");
    let second = scratch.path("second.json");
    let (code, _, stderr) = audit(
        &[&results, &key(), "--write", &source, "--write-to", &first],
        b"",
    );
    assert_eq!(code, 0, "{stderr}");
    let before = fs::read(&first).expect("first output");
    let names = scratch.names();
    let (code, stdout, stderr) = audit(
        &[&results, &key(), "--write", &first, "--write-to", &second],
        b"",
    );
    assert_eq!(
        (code, stdout.as_str(), stderr.as_str()),
        (
            2,
            "",
            "thinkthen: audit: results line 1 was not asked from the question file; --write needs --details lines from that file\n"
        )
    );
    assert_eq!(fs::read(&first).expect("first output"), before);
    assert_eq!(scratch.names(), names);
}

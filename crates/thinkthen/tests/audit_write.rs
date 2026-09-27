//! `thinkthen audit --write` changes one threshold in the question file the
//! results came from, and nothing else.
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
use std::os::unix::fs::PermissionsExt as _;
use std::path::PathBuf;

use measure_support::{
    audit, fixture, fixtures, found, payment_rows, ranked, replay, repository, with_ids,
};

/// The payment question of `transforms/rows`, pretty-printed with CRLF line
/// ends and an escaped `threshold` key, as `write/decide.json` holds it.
const HEAD: &str = "{\r\n  \"decide\": \"Does the message report a payment failure?\",\r\n";

/// A scratch folder that holds copies of the write fixtures.
struct Scratch(PathBuf);

impl Scratch {
    fn new(name: &str) -> Self {
        let folder =
            std::env::temp_dir().join(format!("thinkthen-0125-{name}-{}", std::process::id()));
        let _absent = fs::remove_dir_all(&folder);
        fs::create_dir(&folder).expect("a folder");
        Self(folder)
    }

    fn path(&self, name: &str) -> String {
        self.0.join(name).to_string_lossy().into_owned()
    }

    fn write(&self, name: &str, text: &str) -> String {
        fs::write(self.0.join(name), text).expect("a scratch file");
        self.path(name)
    }

    fn read(&self, name: &str) -> String {
        fs::read_to_string(self.0.join(name)).expect("a scratch file")
    }

    fn count(&self) -> usize {
        fs::read_dir(&self.0).expect("the folder").count()
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ignored = fs::remove_dir_all(&self.0);
    }
}

/// The write key: `replay/key.jsonl` with `C-12` keyed no, and `C-15` and
/// `C-29` held out. The tuning part puts the bar at 0.59; see the fixture README.
fn key() -> String {
    fixtures()
        .join("write/key.jsonl")
        .to_string_lossy()
        .into_owned()
}

#[test]
fn writes_one_value() {
    let scratch = Scratch::new("one");
    let file = scratch.write("decide.json", &fixture("write/decide.json"));
    let rows = scratch.write("rows.jsonl", &payment_rows(&scratch.0, "decide.json"));
    let count = scratch.count();
    let (code, stdout, stderr) = audit(&[&rows, &key(), "--write", &file], b"");
    assert_eq!(code, 0, "{stderr}");
    assert_eq!(
        stderr,
        "thinkthen: audit: wrote threshold 0.59 for the question; it was 0.9\n"
    );
    assert!(stdout.starts_with("{\"group\":"), "{stdout}");
    let expected = format!(
        "{HEAD}  \"\\u0074hreshold\": 0.59,\r\n  \"on\": [\"/body\"],\r\n  \"model\": \"jev-1.13.0\"\r\n}}\r\n"
    );
    assert_eq!(scratch.read("decide.json"), expected);
    assert_eq!(scratch.count(), count);
    let again = payment_rows(&scratch.0, "decide.json");
    let first = |text: &str| {
        let line = text.lines().next().expect("a line");
        let row: serde_json::Value = serde_json::from_str(line).expect("a row");
        (
            row["threshold"].to_string(),
            row["meta"]["question_sha256"].to_string(),
        )
    };
    let (before, after) = (first(&scratch.read("rows.jsonl")), first(&again));
    assert_eq!((before.0.as_str(), after.0.as_str()), ("0.9", "0.59"));
    assert_ne!(before.1, after.1);
}

#[test]
fn inserts_when_absent() {
    let scratch = Scratch::new("insert");
    let bare = format!("{HEAD}  \"on\": [\"/body\"]\r\n}}\r\n");
    let file = scratch.write("decide.json", &bare);
    let rows = scratch.write("rows.jsonl", &payment_rows(&scratch.0, "decide.json"));
    let (code, _, stderr) = audit(&[&rows, &key(), "--write", &file], b"");
    assert_eq!(code, 0, "{stderr}");
    assert_eq!(
        stderr,
        "thinkthen: audit: wrote threshold 0.59 for the question; it was absent\n"
    );
    let expected = format!(
        "{HEAD}  \"on\": [\"/body\"],\r\n  \"threshold\": 0.59,\r\n  \"model\": \"jev-1.13.0\"\r\n}}\r\n"
    );
    assert_eq!(scratch.read("decide.json"), expected);

    let demo = repository().join("demos/14-grade-a-batch");
    let set = scratch.write("set.json", &fixture("write/set.json"));
    let arguments = [
        "annotate",
        &set,
        "--jsonl",
        "--details",
        "--replay",
        "recording",
    ];
    let arguments = [&arguments[..], &["--input", "cases.jsonl"]].concat();
    let rows = scratch.write("set.jsonl", &replay(&demo, &arguments, b""));
    let set_key = fixtures().join("write/set-key.jsonl");
    let (code, _, stderr) = audit(
        &[&rows, set_key.to_str().expect("a path"), "--write", &set],
        b"",
    );
    assert_eq!(code, 0, "{stderr}");
    let kept = "kept the bar for question";
    let report = [
        "wrote threshold 0.54 for question correct; it was absent".to_owned(),
        format!("{kept} grounded; audit found no cut to suggest"),
        format!("{kept} complete; audit found no cut to suggest"),
        format!("{kept} failure_kind; audit found no cut to suggest"),
        "kept question severity unchanged; a score question takes no threshold".to_owned(),
    ];
    let report: String = report
        .iter()
        .map(|line| format!("thinkthen: audit: {line}\n"))
        .collect();
    assert_eq!(stderr, report);
    let member = "\"on\": [\"/input\", \"/gold\", \"/output\"]}";
    let expected = fixture("write/set.json").replacen(
        member,
        &member.replace('}', ", \"threshold\": 0.54}"),
        1,
    );
    assert_eq!(scratch.read("set.json"), expected);
}

#[test]
fn refusals() {
    let scratch = Scratch::new("refused");
    let text = fixture("write/decide.json");
    let file = scratch.write("decide.json", &text);
    let rows = payment_rows(&scratch.0, "decide.json");
    let other = scratch.write(
        "other.json",
        &text.replace("payment failure", "payment problem"),
    );
    let question_b = fs::read_to_string(repository().join("transforms/rows/question-b.txt"))
        .expect("question b");
    scratch.write(
        "b.json",
        &format!(
            "{{\"decide\": \"{}\", \"on\": [\"/body\"]}}\n",
            question_b.trim()
        ),
    );
    let mixed = rows.clone() + &payment_rows(&scratch.0, "b.json");
    let rows = scratch.write("rows.jsonl", &rows);
    let mixed = scratch.write("mixed.jsonl", &mixed);
    let digest = |line: usize| {
        format!(
            "thinkthen: audit: results line {line} was not asked from the question file; --write needs --details lines from that file\n"
        )
    };
    let cases = [
        (rows.clone(), other.clone(), 2, digest(1)),
        (mixed, file.clone(), 2, digest(41)),
    ];
    for (results, target, expected, sentence) in cases {
        let before = fs::read(&target).expect("the file");
        let (code, stdout, stderr) = audit(&[&results, &key(), "--write", &target], b"");
        assert_eq!((code, stdout.as_str(), stderr), (expected, "", sentence));
        assert_eq!(fs::read(&target).expect("the file"), before);
    }
    fs::set_permissions(&file, fs::Permissions::from_mode(0o444)).expect("read-only");
    let (code, stdout, stderr) = audit(&[&rows, &key(), "--write", &file], b"");
    assert_eq!(
        (code, stdout.as_str(), stderr.as_str()),
        (5, "", "thinkthen: audit: cannot write the question file\n")
    );
    assert_eq!(scratch.read("decide.json"), text);
}

#[test]
fn keeps() {
    let scratch = Scratch::new("keeps");
    let demos = repository().join("demos");
    let text = fixture("write/decide.json");
    let band = scratch.write("band.json", &text.replace("0.9", "\"0.2:0.8\""));
    let steady = scratch.write("steady.json", &text.replace("0.9", "0.5"));
    let score = scratch.write("score.json", &fixture("write/score.json"));
    let rank = scratch.write("rank.json", &fixture("write/rank.json"));
    let decide = scratch.write("decide.json", &text);
    let empty = scratch.write("empty.jsonl", "");
    let request =
        fs::read(demos.join("17-rate-and-sort/requests/password.txt")).expect("a request");
    let at_score = format!("@{score}");
    let arguments = ["score", &at_score, "--details", "--replay", "recording"];
    let scored = replay(&demos.join("17-rate-and-sort"), &arguments, &request);
    let ranked = ranked(&format!("@{rank}"));
    let found = found();
    let seeded = fixtures()
        .join("replay/key.jsonl")
        .to_string_lossy()
        .into_owned();
    let no_bar = "kept the question unchanged; audit suggests no bar for rank or find";
    let cases = [
        (
            payment_rows(&scratch.0, "band.json"),
            band,
            key(),
            "/id",
            "kept the band for the question; audit suggests a single cut",
        ),
        (
            payment_rows(&scratch.0, "steady.json"),
            steady,
            seeded,
            "/id",
            "kept the bar for the question; the steady bar beat it on 0 of 20 held parts",
        ),
        (
            with_ids(&scored),
            score,
            empty.clone(),
            "/id",
            "kept the question unchanged; a score question takes no threshold",
        ),
        (ranked, rank, empty.clone(), "/path", no_bar),
        (found, decide, empty, "/id", no_bar),
    ];
    for (results, target, key, pointer, line) in cases {
        let before = fs::read(&target).expect("the file");
        let (code, stdout, stderr) = audit(
            &["-", &key, "--id", pointer, "--write", &target],
            results.as_bytes(),
        );
        assert_eq!((code, stderr), (0, format!("thinkthen: audit: {line}\n")));
        assert!(!stdout.is_empty(), "{line}");
        assert_eq!(fs::read(&target).expect("the file"), before, "{line}");
    }
}

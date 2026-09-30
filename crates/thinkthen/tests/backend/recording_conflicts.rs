//! Repeated recording writes. The record functions store one answer per
//! question and `--record` replaces it, by ADR 0111 section 3. `find` keeps
//! the immutable request entries and their conflicts until slice 4.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::harness::{Canned, Listener, spawn_one as spawn};
use crate::support::stored;

const QUESTION: &str = "asks for a refund";
const EVIDENCE: &str = "Refund me please.";
const FALSE: &str = concat!(
    r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.1}},"#,
    r#""usage":{"input_tokens":10,"output_tokens":2}}"#,
);
const TRUE: &str = concat!(
    r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9}},"#,
    r#""usage":{"input_tokens":10,"output_tokens":2}}"#,
);
const PADDED_FIRST: &str = concat!(
    " \n",
    r#"{"model":"local-1","answers":{"q1":{"type":"choice","choice":"u001","#,
    r#""probabilities":{"u001":0.9,"u002":0.1}}}}"#,
    "\t ",
);
const PADDED_SECOND: &str = concat!(
    " \n",
    r#"{"model":"local-1","answers":{"q1":{"type":"choice","choice":"u002","#,
    r#""probabilities":{"u001":0.1,"u002":0.9}}}}"#,
    "\t ",
);
const UNITS: &str = "Refund me please.\nThanks for the fix.\n";

fn folder(name: &str) -> PathBuf {
    let path = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _absent = fs::remove_dir_all(&path);
    path
}

fn decide(
    base: &str,
    folder: &Path,
    arguments: &[&str],
    input: &str,
) -> io::Result<std::process::Output> {
    asked("decide", base, folder, arguments, input)
}

fn find(base: &str, folder: &Path) -> io::Result<std::process::Output> {
    asked("find", base, folder, &[], UNITS)
}

fn asked(
    verb: &str,
    base: &str,
    folder: &Path,
    arguments: &[&str],
    input: &str,
) -> io::Result<std::process::Output> {
    let common = [
        verb,
        QUESTION,
        "--url",
        base,
        "--model",
        "local-1",
        "--record",
        &folder.to_string_lossy(),
    ];
    spawn(
        &[&common[..], arguments].concat(),
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        input.as_bytes(),
    )
}

fn entry(folder: &Path) -> io::Result<PathBuf> {
    let entries = fs::read_dir(folder)?
        .filter_map(Result::ok)
        .map(|found| found.path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "json")
                && path
                    .file_name()
                    .is_some_and(|name| !name.to_string_lossy().starts_with('.'))
        })
        .collect::<Vec<_>>();
    let [path] = entries.as_slice() else {
        return Err(io::Error::other(format!("one entry, found {entries:?}")));
    };
    Ok(path.clone())
}

fn temporary_names(folder: &Path) -> io::Result<Vec<String>> {
    Ok(fs::read_dir(folder)?
        .filter_map(Result::ok)
        .filter(|found| found.file_type().is_ok_and(|kind| kind.is_file()))
        .map(|found| found.file_name().to_string_lossy().into_owned())
        .filter(|name| name.starts_with('.'))
        .filter(|name| name != ".thinkthen-backend.json")
        .collect())
}

#[test]
fn equal_records_in_one_record_run_ask_their_question_once() {
    let folder = folder("recording-divergent-duplicate");
    let listener =
        Listener::serving(vec![Canned::ok(FALSE), Canned::ok(TRUE)]).expect("a loopback listener");
    let output = decide(
        listener.base(),
        &folder,
        &["--lines", "--jobs", "1"],
        &format!("{EVIDENCE}\n{EVIDENCE}\n"),
    )
    .expect("the binary runs");

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        concat!(
            r#"{"input":"Refund me please.","value":false}"#,
            "\n",
            r#"{"input":"Refund me please.","value":false}"#,
            "\n",
        )
    );
    assert_eq!(listener.requests().len(), 1, "equal keys are asked once");
    let answers = stored(&folder).expect("the store");
    assert_eq!(answers.len(), 1);
    assert_eq!(answers[0]["answer"], r#"{"type":"noul","noul":0.1}"#);
}

#[test]
fn recording_the_same_whitespace_padded_response_again_is_idempotent() {
    let folder = folder("recording-idempotent");
    let listener = Listener::serving(vec![Canned::ok(PADDED_FIRST), Canned::ok(PADDED_FIRST)])
        .expect("a loopback listener");
    assert_eq!(
        find(listener.base(), &folder)
            .expect("the binary runs")
            .status
            .code(),
        Some(0)
    );
    let path = entry(&folder).expect("one entry");
    let written = fs::read_to_string(&path).expect("the entry is readable");
    let reformatted = written.lines().map(str::trim).collect::<String>();
    fs::write(&path, &reformatted).expect("the envelope is reformatted");
    let before = fs::read(&path).expect("the entry is readable");

    assert_eq!(
        find(listener.base(), &folder)
            .expect("the binary runs")
            .status
            .code(),
        Some(0)
    );
    assert_eq!(fs::read(path).expect("the entry is readable"), before);
    assert_eq!(before, reformatted.as_bytes());
    assert!(
        temporary_names(&folder)
            .expect("the folder is readable")
            .is_empty()
    );
}

#[test]
fn whitespace_padded_responses_with_different_values_conflict() {
    let folder = folder("recording-padded-conflict");
    let listener = Listener::serving(vec![Canned::ok(PADDED_FIRST), Canned::ok(PADDED_SECOND)])
        .expect("a loopback listener");
    assert_eq!(
        find(listener.base(), &folder)
            .expect("the binary runs")
            .status
            .code(),
        Some(0)
    );
    let path = entry(&folder).expect("one entry");
    let before = fs::read(&path).expect("the first entry is readable");

    let output = find(listener.base(), &folder).expect("the binary runs");
    assert_eq!(output.status.code(), Some(5));
    assert!(output.stdout.is_empty());
    let name = path.file_name().expect("a file name").to_string_lossy();
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        format!(
            "thinkthen: the backend answered the request in entry `{name}` differently from the saved response; \
             record into a fresh folder, or use --cache DIR to answer from the saved entries\n"
        )
    );
    assert_eq!(fs::read(path).expect("the entry is readable"), before);
    assert!(
        temporary_names(&folder)
            .expect("the folder is readable")
            .is_empty()
    );
}

#[test]
fn recording_again_replaces_the_stored_answer() {
    let folder = folder("recording-replaces");
    let listener =
        Listener::serving(vec![Canned::ok(TRUE), Canned::ok(FALSE)]).expect("a loopback listener");
    for (expected, answer) in [
        (0, r#"{"type":"noul","noul":0.9}"#),
        (1, r#"{"type":"noul","noul":0.1}"#),
    ] {
        let output = decide(listener.base(), &folder, &[], EVIDENCE).expect("the binary runs");
        assert_eq!(output.status.code(), Some(expected));
        assert!(output.stderr.is_empty());
        let answers = stored(&folder).expect("the store");
        assert_eq!(answers.len(), 1);
        assert_eq!(answers[0]["answer"], answer);
    }
    assert_eq!(listener.requests().len(), 2, "--record asks every question");
}

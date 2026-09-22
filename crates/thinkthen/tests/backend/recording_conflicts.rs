//! Immutable recording entries under repeated and concurrent writes.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::harness::{Canned, Listener, spawn};

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
const PADDED_TRUE: &str = concat!(
    " \n",
    r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9}},"#,
    r#""usage":{"input_tokens":10,"output_tokens":2}}"#,
    "\t ",
);
const PADDED_FALSE: &str = concat!(
    " \n",
    r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.1}},"#,
    r#""usage":{"input_tokens":10,"output_tokens":2}}"#,
    "\t ",
);
const CONFLICT: &str = "already records a different response";

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
    let common = [
        "decide",
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
        .collect())
}

#[test]
fn divergent_duplicate_records_stop_before_the_second_answer_prints() {
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

    assert_eq!(output.status.code(), Some(5));
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        concat!(r#"{"input":"Refund me please.","value":false}"#, "\n",)
    );
    let message = String::from_utf8_lossy(&output.stderr);
    assert!(message.contains(CONFLICT), "{message}");
    for hidden in [FALSE, TRUE, "0.1", "0.9"] {
        assert!(!message.contains(hidden), "{message}");
    }
    assert!(
        fs::read_to_string(entry(&folder).expect("one entry"))
            .expect("the entry is text")
            .contains(r#""noul":0.1"#)
    );
    assert!(
        temporary_names(&folder)
            .expect("the folder is readable")
            .is_empty()
    );
}

#[test]
fn recording_the_same_whitespace_padded_response_again_is_idempotent() {
    let folder = folder("recording-idempotent");
    let listener = Listener::serving(vec![Canned::ok(PADDED_TRUE), Canned::ok(PADDED_TRUE)])
        .expect("a loopback listener");
    assert_eq!(
        decide(listener.base(), &folder, &[], EVIDENCE)
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
        decide(listener.base(), &folder, &[], EVIDENCE)
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
    let listener = Listener::serving(vec![Canned::ok(PADDED_TRUE), Canned::ok(PADDED_FALSE)])
        .expect("a loopback listener");
    assert_eq!(
        decide(listener.base(), &folder, &[], EVIDENCE)
            .expect("the binary runs")
            .status
            .code(),
        Some(0)
    );

    let output = decide(listener.base(), &folder, &[], EVIDENCE).expect("the binary runs");
    assert_eq!(output.status.code(), Some(5));
    assert!(String::from_utf8_lossy(&output.stderr).contains(CONFLICT));
    assert!(
        fs::read_to_string(entry(&folder).expect("one entry"))
            .expect("the entry is text")
            .contains(r#""noul":0.9"#)
    );
    assert!(
        temporary_names(&folder)
            .expect("the folder is readable")
            .is_empty()
    );
}

#[test]
fn a_damaged_winner_is_refused_and_never_replaced() {
    let folder = folder("recording-damaged-winner");
    let listener =
        Listener::serving(vec![Canned::ok(TRUE), Canned::ok(FALSE)]).expect("a loopback listener");
    assert_eq!(
        decide(listener.base(), &folder, &[], EVIDENCE)
            .expect("the binary runs")
            .status
            .code(),
        Some(0)
    );
    let path = entry(&folder).expect("one entry");
    let damaged = b"not an entry at all";
    fs::write(&path, damaged).expect("the entry is writable");

    let output = decide(listener.base(), &folder, &[], EVIDENCE).expect("the binary runs");
    assert_eq!(output.status.code(), Some(5));
    assert!(String::from_utf8_lossy(&output.stderr).contains("not a recording entry"));
    assert_eq!(fs::read(path).expect("the entry is readable"), damaged);
    assert!(
        temporary_names(&folder)
            .expect("the folder is readable")
            .is_empty()
    );
}

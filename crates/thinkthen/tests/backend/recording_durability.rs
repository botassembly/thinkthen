//! Recording preflight, repair, concurrency, and file-size failures.

use std::fs;
use std::io::{self, Write as _};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::harness::{Canned, Listener, finish, spawn};

const QUESTION: &str = "asks for a refund";
const EVIDENCE: &str = "Refund me please.";
const TRUE: &str = concat!(
    r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9}},"#,
    r#""usage":{"input_tokens":10,"output_tokens":2}}"#,
);
const FALSE: &str = concat!(
    r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.1}},"#,
    r#""usage":{"input_tokens":10,"output_tokens":2}}"#,
);
const STORAGE: &str = "thinkthen: the recording folder could not be read or written; check its permissions and free space\n";

fn folder(name: &str) -> PathBuf {
    let path = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _absent = fs::remove_dir_all(&path);
    path
}

fn entries(folder: &Path) -> io::Result<Vec<PathBuf>> {
    Ok(fs::read_dir(folder)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension().is_some_and(|value| value == "json")
                && path
                    .file_name()
                    .is_some_and(|value| !value.to_string_lossy().starts_with('.'))
        })
        .collect())
}

#[test]
fn every_command_refuses_recording_storage_before_key_lookup_or_a_request() {
    let questions = folder("recording-preflight-questions.json");
    fs::write(
        &questions,
        r#"{"version":1,"questions":{"refund":{"decide":"asks for a refund"}}}"#,
    )
    .expect("question set written");
    let cases: [(&[&str], &[u8]); 8] = [
        (&["decide", QUESTION], EVIDENCE.as_bytes()),
        (&["choose", QUESTION, "yes", "no"], EVIDENCE.as_bytes()),
        (&["tag", QUESTION, "refund"], EVIDENCE.as_bytes()),
        (&["score", QUESTION, "low", "high"], EVIDENCE.as_bytes()),
        (&["filter", QUESTION, "--lines"], b"Refund me please.\n"),
        (&["rank", QUESTION, "--lines"], b"Refund me please.\n"),
        (
            &["annotate", &questions.to_string_lossy()],
            br#"{"body":"Refund me please."}"#,
        ),
        (
            &["find", QUESTION, "--lines"],
            b"Refund me please.\nAnother line.\n",
        ),
    ];

    for (command, input) in cases {
        let listener = Listener::answering(|_| Canned::ok(TRUE)).expect("a listener");
        let options = [
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--record",
            "/dev/null/recording",
        ];
        let output = spawn(&[command, &options].concat(), &[], input).expect("command runs");
        assert_eq!(output.status.code(), Some(5), "{command:?}");
        assert_eq!(String::from_utf8_lossy(&output.stderr), STORAGE);
        assert!(listener.requests().is_empty(), "{command:?}");
    }
}

#[test]
fn concurrent_record_only_processes_each_send_and_install_one_complete_entry() {
    let recording = folder("record-only-process-race");
    let named = recording.to_string_lossy().into_owned();
    let lock_states = Arc::new(Mutex::new(Vec::new()));
    let listener = Listener::answering({
        let recording = recording.clone();
        let lock_states = Arc::clone(&lock_states);
        move |_| {
            let locked = fs::read_dir(recording.join(".locks"))
                .is_ok_and(|mut entries| entries.next().is_some());
            if let Ok(mut states) = lock_states.lock() {
                states.push(locked);
            }
            Canned::ok(TRUE).after(150)
        }
    })
    .expect("a listener");
    let base = listener.base().to_owned();
    let run = || {
        spawn(
            &[
                "decide", QUESTION, "--url", &base, "--model", "local-1", "--record", &named,
            ],
            &[("THINKTHEN_API_KEY", "sk-test-value")],
            EVIDENCE.as_bytes(),
        )
    };
    let (first, second) = thread::scope(|scope| {
        let first = scope.spawn(run);
        let second = scope.spawn(run);
        (
            first.join().expect("first joins").expect("first runs"),
            second.join().expect("second joins").expect("second runs"),
        )
    });

    assert_eq!(first.status.code(), Some(0));
    assert_eq!(second.status.code(), Some(0));
    assert_eq!(listener.requests().len(), 2);
    assert_eq!(
        *lock_states.lock().expect("lock observations"),
        [true, false],
        "the missing entry is serialized and the valid entry is not"
    );
    assert_eq!(entries(&recording).expect("entries").len(), 1);
    assert!(
        fs::read_dir(recording.join(".locks"))
            .expect("lock directory")
            .next()
            .is_none()
    );
}

#[test]
fn a_damaged_cache_is_repaired_once_and_then_replays_without_a_request() {
    let cache = folder("damaged-cache-repair");
    let named = cache.to_string_lossy().into_owned();
    let listener =
        Listener::serving(vec![Canned::ok(TRUE), Canned::ok(FALSE)]).expect("a listener");
    let arguments = [
        "decide",
        QUESTION,
        "--url",
        listener.base(),
        "--model",
        "local-1",
        "--cache",
        &named,
    ];
    let first = spawn(
        &arguments,
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        EVIDENCE.as_bytes(),
    )
    .expect("first fill");
    assert_eq!(first.status.code(), Some(0));
    let found = entries(&cache).expect("one entry");
    let [entry] = found.as_slice() else {
        panic!("one cache entry expected")
    };
    fs::write(entry, b"damaged").expect("entry damaged");

    let repaired = spawn(
        &arguments,
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        EVIDENCE.as_bytes(),
    )
    .expect("repair runs");
    assert_eq!(repaired.status.code(), Some(1));
    assert_eq!(listener.requests().len(), 2);
    let replayed = spawn(&arguments, &[], EVIDENCE.as_bytes()).expect("replay runs");
    assert_eq!(replayed.status.code(), Some(1));
    assert!(listener.requests().is_empty());
}

#[cfg(unix)]
#[test]
fn a_file_size_limit_returns_the_fixed_failure_and_removes_the_temporary_entry() {
    let recording = folder("recording-file-size-limit");
    fs::create_dir(&recording).expect("recording folder");
    let listener = Listener::answering(|_| Canned::ok(TRUE)).expect("a listener");
    let script = concat!(
        "ulimit -f 1; exec \"$1\" decide 'asks for a refund' ",
        "--url \"$2\" --model local-1 --record \"$3\" --max-retries 0"
    );
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(script)
        .arg("sh")
        .arg(env!("CARGO_BIN_EXE_thinkthen"))
        .arg(listener.base())
        .arg(&recording)
        .env_clear()
        .env("HOME", env!("CARGO_TARGET_TMPDIR"))
        .env("THINKTHEN_API_KEY", "sk-test-value")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("limited child starts");
    child
        .stdin
        .take()
        .expect("input pipe")
        .write_all(&vec![b'x'; 8_192])
        .expect("evidence written");
    let output = finish(child, "the limited child").expect("limited child finishes");

    assert_eq!(output.status.code(), Some(5));
    assert_eq!(String::from_utf8_lossy(&output.stderr), STORAGE);
    assert_eq!(listener.requests().len(), 1, "no extra request");
    assert!(entries(&recording).expect("entries").is_empty());
    let temporary = fs::read_dir(&recording)
        .expect("recording folder")
        .filter_map(Result::ok)
        .any(|entry| {
            entry.file_type().is_ok_and(|kind| kind.is_file())
                && entry.file_name().to_string_lossy().starts_with('.')
                && entry.file_name().to_string_lossy() != ".thinkthen-backend.json"
        });
    assert!(!temporary, "no temporary recording entry remains");
}

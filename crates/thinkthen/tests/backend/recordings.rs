//! The compiled binary against a folder of recorded exchanges.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Output;

use crate::harness::{Canned, Listener, spawn};
use crate::result_assertions::normalized_details;
use crate::support::{
    DEFAULT_BASE, DEFAULT_MODEL, ENDPOINT_PATH, encoded_decide, plant_fixture, stored,
};

mod replay_context;

/// The response the listener gives to the one question the command asks.
const ANSWERED: &str = concat!(
    r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.92}},"#,
    r#""usage":{"input_tokens":312,"output_tokens":48}}"#,
);

/// A port nothing listens on, so a connection would be refused at once.
const CLOSED: &str = "http://127.0.0.1:1/v1";

/// A folder this test owns, removed and remade so each run starts empty.
pub(crate) fn folder(name: &str) -> PathBuf {
    let path = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _absent = fs::remove_dir_all(&path);
    path
}

/// The evidence every case on this page judges.
const EVIDENCE: &str = "Refund me please.";

/// Run the binary over one line of evidence, with the environment the case names.
fn run(arguments: &[&str], environment: &[(&str, &str)]) -> io::Result<Output> {
    spawn(arguments, environment, EVIDENCE.as_bytes())
}

/// Run `decide` against one named base, asking one question.
///
/// `key` is the value `THINKTHEN_API_KEY` holds, or `None` for a run with the
/// variable unset. A replay reads no key, and every other case names one.
fn judge(question: &str, base: &str, arguments: &[&str], key: Option<&str>) -> io::Result<Output> {
    let asked = ["decide", question, "--url", base, "--model", "local-1"];
    run(
        &[&asked[..], arguments].concat(),
        &key.map_or_else(Vec::new, |value| vec![("THINKTHEN_API_KEY", value)]),
    )
}

/// Run `decide` against one named base, asking the refund question.
fn decide(base: &str, arguments: &[&str], key: Option<&str>) -> io::Result<Output> {
    judge("asks for a refund", base, arguments, key)
}

/// The key a case sends when the case is not about the key itself.
const KEY: Option<&str> = Some("sk-test-value");

/// Record one answer into the folder, merge it into the folder's fixture
/// with `cache convert`, and give the fixture's text.
///
/// Every case below starts from a folder holding one answer. The listener is
/// given back at the URL the key was taken over, and it holds a second
/// answer for the case that asks it something more.
fn recorded(folder: &Path) -> io::Result<(Listener, String)> {
    let listener = Listener::serving(vec![Canned::ok(ANSWERED), Canned::ok(ANSWERED)])?;
    let output = decide(
        listener.base(),
        &["--record", &folder.to_string_lossy()],
        KEY,
    )?;
    if output.status.code() != Some(0) {
        return Err(io::Error::other("the recording run answered"));
    }
    let converted = spawn(&["cache", "convert", &folder.to_string_lossy()], &[], b"")?;
    if converted.status.code() != Some(0) {
        return Err(io::Error::other("the folder converted"));
    }
    let written = fs::read_to_string(folder.join("thinkthen.jsonl"))?;
    Ok((listener, written))
}

/// Write the fixture the default address answers this one question from,
/// and give its key.
fn plant(folder: &Path) -> io::Result<String> {
    let request = encoded_decide(EVIDENCE, DEFAULT_MODEL, "asks for a refund");
    let url = format!("{DEFAULT_BASE}/{ENDPOINT_PATH}");
    let keys = plant_fixture(
        folder,
        &url,
        &request,
        &[r#"{"type":"noul","noul":0.92}"#],
        Some((312, 48)),
    )?;
    keys.into_iter()
        .next()
        .ok_or_else(|| io::Error::other("one key"))
}

#[test]
fn a_recorded_exchange_replays_with_no_listener_and_no_key() {
    let folder = folder("replayed");
    let recorded = {
        let listener = Listener::serving(vec![Canned::ok(ANSWERED)]).expect("a loopback listener");
        let output = decide(
            listener.base(),
            &["--details", "--record", &folder.to_string_lossy()],
            Some("sk-secret-value"),
        )
        .expect("the compiled binary runs");
        assert_eq!(output.status.code(), Some(0));
        let requests = listener.requests();
        let request = requests.first().expect("one request reached the listener");
        assert_eq!(request.line, "POST /v1/systemone HTTP/1.1");
        assert_eq!(
            request.header("authorization"),
            Some("Bearer sk-secret-value")
        );
        assert!(
            String::from_utf8_lossy(&request.body).contains(EVIDENCE),
            "the request carried the evidence"
        );
        (listener.base().to_owned(), output)
    };

    assert_eq!(stored(&folder).expect("the store").len(), 1);
    let written = String::from_utf8_lossy(
        &fs::read(folder.join("thinkthen.sqlite")).expect("the store file"),
    )
    .into_owned();
    for header in [
        "authorization",
        "Authorization",
        "Bearer",
        "sk-secret-value",
    ] {
        assert!(!written.contains(header), "{written}");
    }

    // The listener is gone, so the recorded url answers nothing now, and no key
    // variable is set for the command to read.
    let output = decide(
        &recorded.0,
        &["--details", "--replay", &folder.to_string_lossy()],
        None,
    )
    .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(0));
    let attempts = |output: &Output| {
        let details: serde_json::Value =
            serde_json::from_slice(&output.stdout).expect("a result is JSON");
        details["meta"]["attempts"].as_array().map(Vec::len)
    };
    assert_eq!(
        attempts(&recorded.1),
        Some(1),
        "the live send is one attempt"
    );
    assert_eq!(attempts(&output), None, "a replay adds no attempt");
    let live = normalized_details(&recorded.1).expect("live details");
    let replayed = normalized_details(&output).expect("replayed details");
    assert_eq!((live.1, live.2), (false, 1));
    assert_eq!((replayed.1, replayed.2), (true, 0));
    assert_eq!(live.0, replayed.0);
}

#[test]
fn a_replay_at_the_default_address_reads_no_key_and_opens_no_connection() {
    let folder = folder("no-key");
    plant(&folder).expect("a fixture the default address answers");
    let asked: [&str; 2] = ["decide", "asks for a refund"];

    // The run reads THINKTHEN_API_KEY, which env_clear leaves unset. Without a
    // recording that is exit 4, and no connection opens.
    let output = run(&asked, &[]).expect("the compiled binary runs");
    assert_eq!(output.status.code(), Some(4));
    let message = String::from_utf8_lossy(&output.stderr);
    assert!(message.contains("THINKTHEN_API_KEY"), "{message}");

    let output = run(
        &[
            &asked[..],
            &["--details", "--replay", &folder.to_string_lossy()],
        ]
        .concat(),
        &[],
    )
    .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
    let printed = String::from_utf8_lossy(&output.stdout);
    assert!(printed.contains(r#""cached":true"#), "{printed}");
    assert!(
        printed.contains(r#""url":"https://api.typesafe.ai/v1/systemone""#),
        "{printed}"
    );
    assert!(printed.contains(r#""model":"jev-1.13.0""#), "{printed}");
}

/// `meta`, pinned field by field in the order it prints them.
#[test]
fn meta_holds_the_url_the_model_the_usage_and_the_cached_flag() {
    let folder = folder("meta");
    let key = plant(&folder).expect("a fixture the default address answers");
    let request = key.as_str();

    let output = run(
        &[
            "decide",
            "asks for a refund",
            "--details",
            "--replay",
            &folder.to_string_lossy(),
        ],
        &[],
    )
    .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(0));
    let printed = String::from_utf8_lossy(&output.stdout);
    let expected = [
        concat!(
            r#""meta":{"tool":"thinkthen 0.0.1","#,
            r#""question_sha256":"fa2ea2c0b995c700912479bb586ed00efa0227f47d06ede013bf6ac562166c79","#,
            r#""url":"https://api.typesafe.ai/v1/systemone","#,
            r#""model":"jev-1.13.0","usage":{"input_tokens":312,"output_tokens":48},"#,
            r#""requests_sent":0,"cached":true,"requests":[""#,
        ),
        request,
        r#""],"failed_questions":0}"#,
    ]
    .concat();
    assert!(printed.contains(&expected), "{printed}");
}

#[test]
fn replay_reads_only_the_fixture_and_safely_refuses_it_when_malformed() {
    const MALFORMED: &[u8] = b"{\n\"private\":\"DO_NOT_ECHO\"\n";
    let folder = folder("lazy-replay");
    let (listener, _) = recorded(&folder).expect("one recorded answer");
    assert_eq!(
        listener.requests().len(),
        1,
        "the recording run called once"
    );
    fs::write(folder.join("notes.txt"), MALFORMED).expect("an unrelated stray file");

    let output = decide(
        listener.base(),
        &["--replay", &folder.to_string_lossy()],
        None,
    )
    .expect("the compiled binary runs");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(output.stdout, b"true\n");
    assert!(output.stderr.is_empty());

    fs::write(folder.join("thinkthen.jsonl"), MALFORMED).expect("the fixture is writable");
    let output = decide(
        listener.base(),
        &["--replay", &folder.to_string_lossy()],
        None,
    )
    .expect("the compiled binary runs");
    assert_eq!(output.status.code(), Some(5));
    assert!(output.stdout.is_empty());
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "thinkthen: the entry `thinkthen.jsonl` was refused: line 1 is not a question entry\n"
    );
    assert!(!String::from_utf8_lossy(&output.stderr).contains("DO_NOT_ECHO"));
    assert_eq!(listener.requests().len(), 0, "replays open no connection");
}

#[test]
fn an_answer_that_records_another_question_is_refused_by_name() {
    let folder = folder("damaged");
    let (listener, written) = recorded(&folder).expect("one recorded answer");
    let edited = written.replace(
        r#"The text is \\\"Refund me please.\\\"."#,
        r#"The text is \\\"Something else.\\\"."#,
    );
    assert_ne!(
        edited, written,
        "the fixture holds the question it recorded"
    );
    fs::write(folder.join("thinkthen.jsonl"), edited).expect("the fixture is writable");

    let output = decide(
        listener.base(),
        &["--replay", &folder.to_string_lossy()],
        None,
    )
    .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(5));
    let message = String::from_utf8_lossy(&output.stderr);
    assert!(message.contains("thinkthen.jsonl"), "{message}");
    assert!(message.contains("damaged or hand-edited"), "{message}");

    fs::write(folder.join("thinkthen.jsonl"), "not an entry at all").expect("writable");
    let output = decide(
        listener.base(),
        &["--replay", &folder.to_string_lossy()],
        None,
    )
    .expect("the compiled binary runs");
    assert_eq!(output.status.code(), Some(5));
    let message = String::from_utf8_lossy(&output.stderr);
    assert!(message.contains("thinkthen.jsonl"), "{message}");
}

#[test]
fn the_two_options_over_one_folder_are_a_cache_that_calls_once() {
    let folder = folder("cached");
    let listener = Listener::serving(vec![Canned::ok(ANSWERED)]).expect("a loopback listener");
    let both: [&str; 5] = [
        "--details",
        "--record",
        &folder.to_string_lossy(),
        "--replay",
        &folder.to_string_lossy(),
    ];

    for run in 0..2 {
        let output = decide(listener.base(), &both, KEY).expect("the compiled binary runs");
        assert_eq!(output.status.code(), Some(0), "run {run}");
        let printed = String::from_utf8_lossy(&output.stdout);
        let replayed = format!(r#""cached":{}"#, run == 1);
        assert!(printed.contains(&replayed), "run {run}: {printed}");
        let requests_sent = format!(r#""requests_sent":{}"#, usize::from(run == 0));
        assert!(printed.contains(&requests_sent), "run {run}: {printed}");
    }

    assert_eq!(listener.requests().len(), 1);
}

#[test]
fn two_different_folders_and_a_dry_run_that_records_are_usage_errors() {
    let folder = folder("two");
    let cases: [&[&str]; 4] = [
        &["--record", "one", "--replay", "another"],
        &["--plan", "--record", "one"],
        &["--plan", "--replay", "one"],
        &["--plan", "--record", "one", "--replay", "another"],
    ];

    for arguments in cases {
        let output = decide(CLOSED, arguments, None).expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(2), "{arguments:?}");
        assert!(output.stdout.is_empty(), "{arguments:?}");
    }

    assert!(!folder.exists(), "a usage error writes no folder");
}

#[test]
fn a_recording_path_that_is_a_file_gets_a_fixed_action() {
    let folder = folder("recording-path-is-file");
    fs::write(&folder, b"private path marker").expect("a file blocks the folder");
    let listener = Listener::serving(vec![Canned::ok(ANSWERED)]).expect("a listener");
    let output = decide(
        listener.base(),
        &["--record", &folder.to_string_lossy()],
        KEY,
    )
    .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(5));
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "thinkthen: the recording directory is a file; choose another path or remove the file\n"
    );
}

/// The mode a path carries, on the one family of systems the tool targets.
#[cfg(unix)]
fn mode(path: &Path) -> io::Result<u32> {
    use std::os::unix::fs::PermissionsExt as _;
    Ok(fs::metadata(path)?.permissions().mode() & 0o777)
}

#[cfg(unix)]
#[test]
fn a_recording_is_written_for_its_owner_alone_and_leaves_no_side_file() {
    let folder = folder("private");
    let listener = Listener::serving(vec![Canned::ok(ANSWERED), Canned::ok(ANSWERED)])
        .expect("a loopback listener");
    let output = decide(
        listener.base(),
        &["--record", &folder.to_string_lossy()],
        KEY,
    )
    .expect("the compiled binary runs");
    assert_eq!(output.status.code(), Some(0));

    // A recording holds the evidence, so neither the folder the tool made nor
    // the store inside it is readable by anybody else, whatever the umask says.
    // The rollback journal leaves no side file once the write commits.
    assert_eq!(mode(&folder).expect("the folder is there"), 0o700);
    let store = folder.join("thinkthen.sqlite");
    assert_eq!(mode(&store).expect("the store is there"), 0o600);
    let names: Vec<_> = fs::read_dir(&folder)
        .expect("the folder is there")
        .filter_map(|entry| entry.ok().map(|found| found.file_name()))
        .collect();
    assert_eq!(names, ["thinkthen.sqlite"]);

    // A directory standing where the store would go is a storage failure,
    // and nothing else is left beside it.
    let blocked = folder.join("blocked");
    fs::create_dir_all(blocked.join("thinkthen.sqlite")).expect("a directory takes the name");
    let output = decide(
        listener.base(),
        &["--record", &blocked.to_string_lossy()],
        KEY,
    )
    .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(5));
    let left: Vec<_> = fs::read_dir(&blocked)
        .expect("the folder is there")
        .filter_map(|entry| entry.ok().map(|found| found.file_name()))
        .filter(|found| found != "thinkthen.sqlite")
        .collect();
    assert!(left.is_empty(), "{left:?}");
}

#[test]
fn a_fixture_that_cannot_be_read_is_not_reported_as_a_miss() {
    let folder = folder("unreadable");
    let (listener, _) = recorded(&folder).expect("one recorded answer");

    // A directory standing where the fixture stood is there and cannot be
    // read. Calling that a miss would tell the user to record what was
    // already recorded.
    fs::remove_file(folder.join("thinkthen.jsonl")).expect("the fixture is removable");
    fs::create_dir(folder.join("thinkthen.jsonl")).expect("a directory takes the name");
    let output = decide(
        listener.base(),
        &["--replay", &folder.to_string_lossy()],
        None,
    )
    .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(5));
    let message = String::from_utf8_lossy(&output.stderr);
    assert!(
        message.contains("could not be read or written"),
        "{message}"
    );
    assert!(!message.contains("holds no answer"), "{message}");
}

#[test]
fn a_failed_exchange_is_never_recorded() {
    let cases = [
        Canned::status(402, r#"{"error":"no credit"}"#),
        Canned::ok("not json at all"),
        Canned::redirect(CLOSED),
        Canned::cut_short(),
    ];

    for (place, canned) in cases.into_iter().enumerate() {
        let folder = folder(&format!("failed-{place}"));
        let listener = Listener::serving(vec![canned]).expect("a loopback listener");
        let output = decide(
            listener.base(),
            &["--record", &folder.to_string_lossy(), "--max-retries", "0"],
            KEY,
        )
        .expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(4));
        if folder.join("thinkthen.sqlite").exists() {
            assert!(
                stored(&folder).expect("the store").is_empty(),
                "a failed exchange stores no answer"
            );
        }
    }
}

#[test]
fn cache_is_the_two_options_on_one_folder_and_stands_beside_neither() {
    let folder = folder("cache-option");
    let listener = Listener::serving(vec![Canned::ok(ANSWERED)]).expect("a loopback listener");
    let named = folder.to_string_lossy().into_owned();

    // The first run pays and writes the entry, and the second reads it back
    // with the listener spent and no key in the environment.
    let output = decide(listener.base(), &["--cache", &named], KEY).expect("the binary runs");
    assert_eq!(output.status.code(), Some(0));
    let written = stored(&folder).expect("the store");
    assert_eq!(written.len(), 1, "one answer");
    assert_eq!(
        written[0]["url"],
        format!("{}/{ENDPOINT_PATH}", listener.base())
    );
    assert_eq!(listener.requests().len(), 1, "the first run paid once");

    let output =
        decide(listener.base(), &["--cache", &named, "--details"], None).expect("the binary runs");
    assert_eq!(output.status.code(), Some(0));
    let row = String::from_utf8_lossy(&output.stdout);
    assert!(row.contains(r#""cached":true"#), "{row}");
    assert!(listener.requests().is_empty(), "a cached run asks nothing");

    for beside in [["--record", &named], ["--replay", &named]] {
        let output = decide(
            listener.base(),
            &[&["--cache", &named][..], &beside].concat(),
            KEY,
        )
        .expect("the binary runs");
        assert_eq!(output.status.code(), Some(2), "{beside:?}");
        let message = String::from_utf8_lossy(&output.stderr);
        assert!(message.contains("--cache"), "{message}");
    }

    let output =
        decide(listener.base(), &["--cache", &named, "--plan"], KEY).expect("the binary runs");
    assert_eq!(output.status.code(), Some(2));
    let message = String::from_utf8_lossy(&output.stderr);
    assert!(message.contains("--plan"), "{message}");
}

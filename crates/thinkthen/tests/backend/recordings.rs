//! The compiled binary against a folder of recorded exchanges.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Output;

use crate::harness::{Canned, Listener, spawn};
use crate::result_assertions::normalized_details;
use crate::support::{DEFAULT_BASE, DEFAULT_MODEL, ENDPOINT_PATH, encoded_decide, plant_recording};

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

/// Record one answered exchange into the folder and give the entry it wrote.
///
/// Every case below starts from a folder holding one entry. The listener is
/// given back at the URL the entry's digest was taken over, and it holds a
/// second answer for the case that asks it something more.
fn recorded(folder: &Path) -> io::Result<(Listener, String, String)> {
    let listener = Listener::serving(vec![Canned::ok(ANSWERED), Canned::ok(ANSWERED)])?;
    let output = decide(
        listener.base(),
        &["--record", &folder.to_string_lossy()],
        KEY,
    )?;
    if output.status.code() != Some(0) {
        return Err(io::Error::other("the recording run answered"));
    }
    let (name, written) = only_entry(folder)?;
    Ok((listener, name, written))
}

/// Write the entry the default address would record for this response.
fn plant(folder: &Path, response: &str) -> Option<String> {
    let request = encoded_decide(EVIDENCE, DEFAULT_MODEL, "asks for a refund");
    let url = format!("{DEFAULT_BASE}/{ENDPOINT_PATH}");
    plant_recording(folder, &url, &request, response)
}

/// The one entry a folder holds, as the file name and the text inside it.
pub(crate) fn only_entry(folder: &Path) -> io::Result<(String, String)> {
    let mut entries = Vec::new();
    for entry in fs::read_dir(folder)? {
        let path = entry?.path();
        if path.extension().is_some_and(|value| value == "json")
            && path
                .file_name()
                .is_some_and(|name| !name.to_string_lossy().starts_with('.'))
        {
            entries.push(path);
        }
    }
    entries.sort();
    let [file] = entries.as_slice() else {
        return Err(io::Error::other(format!("one entry, found {entries:?}")));
    };
    let name = file
        .file_name()
        .ok_or_else(|| io::Error::other("an entry has a name"))?;
    Ok((
        name.to_string_lossy().into_owned(),
        fs::read_to_string(file)?,
    ))
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

    let (name, written) = only_entry(&folder).expect("one recorded entry");
    assert!(name.ends_with(".json"), "{name}");
    assert_eq!(name.len(), 69, "{name}");
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
    let name = plant(&folder, ANSWERED).expect("an entry the default address answers");
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
    assert!(name.ends_with(".json"), "{name}");
}

/// `meta`, pinned field by field in the order it prints them.
#[test]
fn meta_holds_the_url_the_model_the_usage_and_the_cached_flag() {
    let folder = folder("meta");
    let name = plant(&folder, ANSWERED).expect("an entry the default address answers");
    assert!(name.ends_with(".json"), "{name}");
    let request = name.strip_suffix(".json").expect("a recording name");

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
fn replay_reads_only_the_requested_entry_and_safely_refuses_it_when_malformed() {
    const MALFORMED: &[u8] = b"{\n\"private\":\"DO_NOT_ECHO\"\n";
    let folder = folder("lazy-replay");
    let (listener, name, _) = recorded(&folder).expect("one recorded entry");
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

    fs::write(folder.join(&name), MALFORMED).expect("the requested entry is writable");
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
        format!(
            "thinkthen: the entry `{name}` was refused: the file is not a recording entry: \
             the JSON at line 3 column 0 is not one\n"
        )
    );
    assert!(!String::from_utf8_lossy(&output.stderr).contains("DO_NOT_ECHO"));
    assert_eq!(listener.requests().len(), 0, "replays open no connection");
}

#[test]
fn an_entry_that_records_another_exchange_is_refused_by_name() {
    let folder = folder("damaged");
    let (listener, name, written) = recorded(&folder).expect("one recorded entry");
    let edited = written.replace(
        r#""state":"Refund me please.""#,
        r#""state":"Something else.""#,
    );
    assert_ne!(edited, written, "the entry holds the request it recorded");
    fs::write(folder.join(&name), edited).expect("the entry is writable");

    let output = decide(
        listener.base(),
        &["--replay", &folder.to_string_lossy()],
        None,
    )
    .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(5));
    let message = String::from_utf8_lossy(&output.stderr);
    assert!(message.contains(&name), "{message}");
    assert!(message.contains("damaged or hand-edited"), "{message}");

    fs::write(folder.join(&name), "not an entry at all").expect("the entry is writable");
    let output = decide(
        listener.base(),
        &["--replay", &folder.to_string_lossy()],
        None,
    )
    .expect("the compiled binary runs");
    assert_eq!(output.status.code(), Some(5));
    let message = String::from_utf8_lossy(&output.stderr);
    assert!(message.contains(&name), "{message}");
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
fn a_recording_is_written_for_its_owner_alone_and_leaves_no_partial_file() {
    let folder = folder("private");
    let (listener, name, _) = recorded(&folder).expect("one recorded entry");

    // A recording holds the evidence, so neither the folder the tool made nor
    // the entry inside it is readable by anybody else, whatever the umask says.
    assert_eq!(mode(&folder).expect("the folder is there"), 0o700);
    assert_eq!(
        mode(&folder.join(&name)).expect("the entry is there"),
        0o600
    );

    // The same exchange into a folder where a directory already holds the
    // entry's name. The hard link fails, and the temporary file the entry was
    // written under goes with it, so nothing private is left behind.
    let blocked = folder.join("blocked");
    fs::create_dir_all(blocked.join(&name)).expect("a directory stands where the entry would go");
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
        .filter(|found| found != name.as_str())
        .collect();
    assert!(left.is_empty(), "{left:?}");
}

#[test]
fn an_entry_that_cannot_be_read_is_not_reported_as_a_miss() {
    let folder = folder("unreadable");
    let (listener, name, _) = recorded(&folder).expect("one recorded entry");

    // A directory standing where the entry stood is there and cannot be read.
    // Calling that a miss would tell the user to record what was already recorded.
    fs::remove_file(folder.join(&name)).expect("the entry is removable");
    fs::create_dir(folder.join(&name)).expect("a directory takes the name");
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
    assert!(!message.contains("no entry named"), "{message}");
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
        let files = fs::read_dir(&folder)
            .expect("recording preflight made the private folder")
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| {
                path.extension().is_some_and(|value| value == "json")
                    && path
                        .file_name()
                        .is_some_and(|name| !name.to_string_lossy().starts_with('.'))
            })
            .collect::<Vec<_>>();
        assert!(files.is_empty(), "a failed exchange installs no entry");
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
    let (_, written) = only_entry(&folder).expect("one recorded entry");
    assert!(written.contains(r#""adapter": "systemone""#), "{written}");
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

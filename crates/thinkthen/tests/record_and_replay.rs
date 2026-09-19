//! The compiled binary against a folder of recorded exchanges.

mod harness;

use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use harness::{Canned, Listener};
use thinkthen_core::recording::{Entry, Exchange};
use thinkthen_core::{Adapter, Evidence, ModelName, Plan, Question, QuestionText, Url, systemone};

/// The response the listener gives to the one question the command asks.
const ANSWERED: &str = concat!(
    r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.92}},"#,
    r#""usage":{"input_tokens":312,"output_tokens":48}}"#,
);

/// A port nothing listens on, so a connection would be refused at once.
const CLOSED: &str = "http://127.0.0.1:1/v1";

/// A folder this test owns, removed and remade so each run starts empty.
fn folder(name: &str) -> PathBuf {
    let path = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _absent = fs::remove_dir_all(&path);
    path
}

/// The evidence every case on this page judges.
const EVIDENCE: &str = "Refund me please.";

/// Run the binary over one line of evidence, with the environment the case names.
fn run(arguments: &[&str], environment: &[(&str, &str)]) -> io::Result<Output> {
    let mut command = Command::new(env!("CARGO_BIN_EXE_thinkthen"));
    command
        .env_clear()
        .env("THINKTHEN_TEST_RETRY_WAIT_MS", "1")
        .args(arguments)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for (name, value) in environment {
        command.env(name, value);
    }
    let mut child = command.spawn()?;
    let mut input = child
        .stdin
        .take()
        .ok_or_else(|| io::Error::other("no pipe to standard input"))?;
    let _ = input.write_all(EVIDENCE.as_bytes());
    drop(input);
    child.wait_with_output()
}

/// Run `decide` against one ad-hoc URL, asking one question.
fn judge(question: &str, base: &str, arguments: &[&str], key: Option<&str>) -> io::Result<Output> {
    let ad_hoc = [
        "decide",
        question,
        "--url",
        base,
        "--adapter",
        "systemone",
        "--model",
        "local-1",
    ];
    let named: [&str; 2] = ["--key-env", "LOCAL_KEY"];
    let given = if key.is_some() { &named[..] } else { &[][..] };
    run(
        &[&ad_hoc[..], arguments, given].concat(),
        &key.map_or_else(Vec::new, |value| vec![("LOCAL_KEY", value)]),
    )
}

/// Run `decide` against one ad-hoc URL, asking the refund question.
fn decide(base: &str, arguments: &[&str], key: Option<&str>) -> io::Result<Output> {
    judge("asks for a refund", base, arguments, key)
}

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
        None,
    )?;
    if output.status.code() != Some(0) {
        return Err(io::Error::other("the recording run answered"));
    }
    let (name, written) = only_entry(folder)?;
    Ok((listener, name, written))
}

/// Write the entry the built-in profile would record for this response.
fn plant(folder: &Path, response: &str) -> Option<String> {
    let plan = Plan::new(
        Evidence::new(EVIDENCE).ok()?,
        ModelName::new("jev-latest").ok()?,
        vec![Question::new_decide(
            QuestionText::new("asks for a refund").ok()?,
        )],
    )
    .ok()?;
    let request = systemone::encode(&plan).ok()?;
    let url = Url::new("https://api.typesafe.ai/v1/systemone").ok()?;
    let exchange = Exchange::new(Adapter::SystemOne, &url, &request);
    let name = exchange.digest().file_name();
    let written = Entry::of(&exchange, response.as_bytes())
        .ok()?
        .written()
        .ok()?;
    fs::create_dir_all(folder).ok()?;
    fs::write(folder.join(&name), written).ok()?;
    Some(name)
}

/// The one entry a folder holds, as the file name and the text inside it.
fn only_entry(folder: &Path) -> io::Result<(String, String)> {
    let mut entries = Vec::new();
    for entry in fs::read_dir(folder)? {
        entries.push(entry?.path());
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
        (listener.base().to_owned(), output.stdout)
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
    let live = String::from_utf8(recorded.1).expect("a result is text");
    let replayed = String::from_utf8(output.stdout).expect("a result is text");
    assert_eq!(
        live.replace(r#""replayed":false"#, r#""replayed":true"#),
        replayed
    );
    assert!(live.contains(r#""replayed":false"#), "{live}");
}

#[test]
fn a_replay_under_the_built_in_profile_reads_no_key_and_opens_no_connection() {
    let folder = folder("no-key");
    let name = plant(&folder, ANSWERED).expect("an entry for the built-in profile");
    let asked: [&str; 2] = ["decide", "asks for a refund"];

    // The built-in profile names THINKTHEN_API_KEY, which env_clear leaves unset.
    // Without a recording that is exit 4, and no connection opens.
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
    assert!(printed.contains(r#""replayed":true"#), "{printed}");
    assert!(printed.contains(r#""profile":"jev""#), "{printed}");
    assert!(printed.contains(r#""model":"jev-1.13.0""#), "{printed}");
    assert!(name.ends_with(".json"), "{name}");
}

#[test]
fn a_replay_miss_is_a_local_failure_that_names_the_entry() {
    let folder = folder("missed");
    let (listener, name, _) = recorded(&folder).expect("one recorded entry");
    assert_eq!(
        listener.requests().len(),
        1,
        "the recording run called once"
    );

    // Another question makes other request bytes, so the digest names a file
    // this folder does not hold.
    let output = judge(
        "asks for something else",
        listener.base(),
        &["--replay", &folder.to_string_lossy()],
        None,
    )
    .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(5));
    assert!(output.stdout.is_empty());
    let message = String::from_utf8_lossy(&output.stderr);
    assert!(message.contains("no entry named"), "{message}");
    assert!(message.contains(".json"), "{message}");
    assert!(!message.contains(&name), "{message}");
    assert!(!message.contains(EVIDENCE), "{message}");
    assert_eq!(listener.requests().len(), 0, "a replay opens no connection");
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
        let output = decide(listener.base(), &both, None).expect("the compiled binary runs");
        assert_eq!(output.status.code(), Some(0), "run {run}");
        let printed = String::from_utf8_lossy(&output.stdout);
        let replayed = format!(r#""replayed":{}"#, run == 1);
        assert!(printed.contains(&replayed), "run {run}: {printed}");
    }

    assert_eq!(listener.requests().len(), 1);
}

#[test]
fn two_different_folders_and_a_dry_run_that_records_are_usage_errors() {
    let folder = folder("two");
    let cases: [&[&str]; 4] = [
        &["--record", "one", "--replay", "another"],
        &["--dry-run", "--record", "one"],
        &["--dry-run", "--replay", "one"],
        &["--dry-run", "--record", "one", "--replay", "another"],
    ];

    for arguments in cases {
        let output = decide(CLOSED, arguments, None).expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(2), "{arguments:?}");
        assert!(output.stdout.is_empty(), "{arguments:?}");
    }

    assert!(!folder.exists(), "a usage error writes no folder");
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
    // entry's name. The rename fails, and the temporary file the entry was
    // written under goes with it, so nothing private is left behind.
    let blocked = folder.join("blocked");
    fs::create_dir_all(blocked.join(&name)).expect("a directory stands where the entry would go");
    let output = decide(
        listener.base(),
        &["--record", &blocked.to_string_lossy()],
        None,
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
    let folder = folder("failed");
    let cases = [
        Canned::status(402, r#"{"error":"no credit"}"#),
        Canned::ok("not json at all"),
        Canned::redirect(CLOSED),
        Canned::cut_short(),
    ];

    for canned in cases {
        let listener = Listener::serving(vec![canned]).expect("a loopback listener");
        let output = decide(
            listener.base(),
            &["--record", &folder.to_string_lossy(), "--max-retries", "0"],
            None,
        )
        .expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(4));
        assert!(
            fs::read_dir(&folder).is_err_and(|error| error.kind() == io::ErrorKind::NotFound),
            "a failure writes no folder"
        );
    }
}

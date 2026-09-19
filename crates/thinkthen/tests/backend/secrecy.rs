//! The one sweep that proves no key leaves this process and no error quotes the
//! evidence.
//!
//! Every command runs down every path the backend can send it, on one document
//! and over records, in the bare view and under `--details`. One reader then
//! reads standard output, standard error, and every file the run wrote.
//! `refusals.rs` drives the same reader down every usage error, and
//! `crates/thinkthen/src/failure.rs` drives it over every diagnostic and every
//! `Debug` line a message is built from.
//!
//! A command enters the sweep by adding one row to [`VERBS`], and a path by
//! adding one row to `PATHS`.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Output;

use crate::harness::{Canned, Listener, spawn};

/// The key every run here carries.
///
/// It reaches the one authorization header and no other byte this tool writes.
pub(crate) const KEY: &str = "sk-marker-2f9d41c6";

/// The evidence every run here judges.
///
/// It reaches the request body, the recorded entry, and `--details` on standard
/// output. It reaches no diagnostic, because a diagnostic is read by a person
/// and the evidence is the untrusted string.
pub(crate) const EVIDENCE: &str = "marker-evidence-7b3ac5";

/// The question every run here asks.
pub(crate) const QUESTION: &str = "Does this report a payment failure?";

/// Each verb, the operands it needs, and the answer a good reply carries.
///
/// A command enters this sweep by adding one row here. Every path below then
/// runs over it, on one document and over records, in both views.
pub(crate) const VERBS: [(&str, &[&str], &str); 3] = [
    ("decide", &[], r#""type":"noul","noul":0.92"#),
    (
        "choose",
        &["late", "lost"],
        r#""type":"choice","choice":"late","probabilities":{"late":0.9,"lost":0.1}"#,
    ),
    (
        "score",
        &["none", "some", "much"],
        r#""type":"score","probabilities":{"0":0.1,"1":0.2,"2":0.7}"#,
    ),
];

/// A reply the adapter refuses, whatever question was asked.
const MALFORMED: &str = r#"{"model":"","answers":{}}"#;

/// A recording entry a reader takes, whose every field is hostile text.
///
/// It names another schema, so it is refused after it parses. No field of it
/// may reach a diagnostic: the text is unbounded, it holds a terminal escape,
/// and it quotes the evidence marker back.
const HOSTILE: &str = concat!(
    r#"{"schema":"\u001b[31mPWNED\u001b[0m marker-evidence-7b3ac5","#,
    r#""adapter":"\u001b[31mPWNED\u001b[0m marker-evidence-7b3ac5","#,
    r#""url":"\u001b[31mPWNED\u001b[0m marker-evidence-7b3ac5","#,
    r#""request":{},"response":{}}"#,
);

/// A recording entry that is not JSON, holding the evidence it recorded.
const DAMAGED: &str = concat!(
    r#"{"schema":"thinkthen.recording/1","adapter":"systemone","#,
    r#""request":{"evidence":"marker-evidence-7b3ac5"} "response":{}}"#,
);

/// The address of a port nothing listens on, which fails in the transport.
const CLOSED: &str = "http://127.0.0.1:1/v1";

/// A whole reply carrying this one answer.
fn good(answer: &str) -> String {
    format!(r#"{{"model":"jev-1.13.0","answers":{{"q1":{{{answer}}}}},"#,)
        + r#""usage":{"input_tokens":9,"output_tokens":3}}"#
}

/// An error body that quotes the evidence back, as a real backend may.
///
/// Nothing the tool prints may carry it, so every message names the status and
/// never the body.
fn quoting() -> String {
    format!(r#"{{"error":{{"message":"refused: {EVIDENCE}"}}}}"#)
}

/// A reply that is JSON no adapter reads, quoting the evidence back.
///
/// A JSON reader names the value it stopped on, so a reply shaped like this one
/// is what would carry the evidence into a diagnostic.
fn unreadable() -> String {
    format!(r#"{{"model":"jev-1.13.0","answers":"{EVIDENCE}"}}"#)
}

/// What the loopback backend answers one run with.
#[derive(Clone, Copy, Debug)]
enum Answers {
    /// A good reply for the one question the verb asked.
    Good,
    /// A status that fails at once, with a body that quotes the evidence.
    Status(u16),
    /// A rate limit that lifts, then a good reply.
    RateLimit,
    /// A rate limit that never lifts, past the one retry the run allows.
    RateLimited,
    /// A reply the adapter refuses.
    Malformed,
    /// A reply that is JSON no adapter reads, quoting the evidence back.
    Unreadable,
    /// Nothing at all, because the run must not reach the listener.
    Nothing,
}

impl Answers {
    /// The responses the listener serves, in order, for this verb.
    fn script(self, answer: &str) -> Vec<Canned> {
        match self {
            Self::Good => vec![Canned::ok(&good(answer))],
            Self::Status(status) => vec![Canned::status(status, &quoting())],
            Self::RateLimit => vec![Canned::status(429, &quoting()), Canned::ok(&good(answer))],
            Self::RateLimited => vec![
                Canned::status(429, &quoting()),
                Canned::status(429, &quoting()),
            ],
            Self::Malformed => vec![Canned::ok(MALFORMED)],
            Self::Unreadable => vec![Canned::ok(&unreadable())],
            Self::Nothing => Vec::new(),
        }
    }
}

/// One way a run can end, driven over every verb and both views.
///
/// `adds` may hold `{dir}`, which becomes this run's own folder, and `{closed}`,
/// which becomes the address of a port nothing listens on.
struct Route {
    named: &'static str,
    adds: &'static [&'static str],
    answers: Answers,
    /// How many requests the listener must see, which pins "sends nothing".
    requests: usize,
    /// The exit code the run earns, which pins that the path really ran.
    code: i32,
    /// Whether a recording is written into the folder before the run.
    primed: bool,
    /// What every entry the priming run wrote is overwritten with, if anything.
    damage: Option<&'static str>,
    /// Whether the run carries the key at all.
    keyed: bool,
}

/// Every path the backend and the recording folder can send a run down.
const PATHS: [Route; 17] = [
    route("a success", &[], Answers::Good, 1, 0),
    route("a plan", &["--dry-run"], Answers::Nothing, 0, 0),
    route("a record run", &["--record", "{dir}"], Answers::Good, 1, 0),
    route("a cache", &["--cache", "{dir}"], Answers::Good, 1, 0),
    // The listener answers the priming run alone. The replay that follows it
    // finds no answer waiting, so the one request the listener counted is the
    // priming one and the replay opened no connection at all.
    Route {
        named: "a replay",
        adds: &["--replay", "{dir}"],
        answers: Answers::Good,
        requests: 1,
        code: 0,
        primed: true,
        damage: None,
        keyed: true,
    },
    // The entry is damaged after it is written, so the reply the run reads is
    // untrusted bytes holding the evidence that was recorded.
    Route {
        named: "a damaged entry",
        adds: &["--replay", "{dir}"],
        answers: Answers::Good,
        requests: 1,
        code: 5,
        primed: true,
        damage: Some(DAMAGED),
        keyed: true,
    },
    // The entry parses and every field of it is hostile text, so the refusal
    // comes after the reading rather than during it.
    Route {
        named: "a hostile entry",
        adds: &["--replay", "{dir}"],
        answers: Answers::Good,
        requests: 1,
        code: 5,
        primed: true,
        damage: Some(HOSTILE),
        keyed: true,
    },
    route(
        "a replay miss",
        &["--replay", "{dir}"],
        Answers::Nothing,
        0,
        5,
    ),
    route(
        "a refused address",
        &["--url", "http://example.com/v1"],
        Answers::Nothing,
        0,
        2,
    ),
    route(
        "a failed request",
        &["--url", "{closed}"],
        Answers::Nothing,
        0,
        4,
    ),
    route("a refused key", &[], Answers::Status(401), 1, 4),
    route("a rate limit that lifts", &[], Answers::RateLimit, 2, 0),
    route(
        "a rate limit that stays",
        &["--max-retries", "1"],
        Answers::RateLimited,
        2,
        4,
    ),
    route("a malformed answer", &[], Answers::Malformed, 1, 4),
    route("an unreadable answer", &[], Answers::Unreadable, 1, 4),
    // The exchange succeeds and the entry cannot be written, which is the one
    // failure that happens after a key has already crossed the wire.
    route(
        "a recording folder that cannot be made",
        &["--record", "/dev/null/x"],
        Answers::Good,
        1,
        5,
    ),
    Route {
        named: "a run with no key",
        adds: &[],
        answers: Answers::Nothing,
        requests: 0,
        code: 4,
        primed: false,
        damage: None,
        keyed: false,
    },
];

/// One route with the two uncommon fields at their usual values.
const fn route(
    named: &'static str,
    adds: &'static [&'static str],
    answers: Answers,
    requests: usize,
    code: i32,
) -> Route {
    Route {
        named,
        adds,
        answers,
        requests,
        code,
        primed: false,
        damage: None,
        keyed: true,
    }
}

/// A folder this run owns, remade so each run starts empty.
fn folder(named: &str) -> io::Result<PathBuf> {
    let path = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join("secrecy")
        .join(named);
    let _absent = fs::remove_dir_all(&path);
    fs::create_dir_all(&path)?;
    Ok(path)
}

/// Every file under this folder, however deep.
fn written(folder: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let Ok(entries) = fs::read_dir(folder) else {
        return found;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            found.extend(written(&path));
        } else {
            found.push(path);
        }
    }
    found
}

/// Refuse both markers everywhere they may not be, for one finished run.
///
/// This is the whole claim of secrecy, in one reader every case calls. The key
/// may reach no byte of standard output, of standard error, or of any file the
/// run wrote, and no header name that carries it may reach a file either. The
/// evidence may reach no byte of standard error.
pub(crate) fn nothing_leaked(named: &str, output: &Output, folder: &Path) {
    let out = String::from_utf8_lossy(&output.stdout);
    let err = String::from_utf8_lossy(&output.stderr);
    assert!(!out.contains(KEY), "{named}: the key is on standard output");
    assert!(!err.contains(KEY), "{named}: the key is on standard error");
    assert!(
        !err.contains(EVIDENCE),
        "{named}: the evidence is in a diagnostic\n{err}"
    );
    for path in written(folder) {
        let read = fs::read(&path).unwrap_or_default();
        let text = String::from_utf8_lossy(&read).to_lowercase();
        let place = path.display();
        assert!(
            !text.contains(&KEY.to_lowercase()),
            "{named}: key in {place}"
        );
        for header in ["authorization", "bearer"] {
            assert!(!text.contains(header), "{named}: {header} in {place}");
        }
    }
}

/// Run one verb down one route, in one view and one framing.
fn sweep(
    route: &Route,
    verb: (&str, &[&str], &str),
    view: &[&str],
    records: bool,
) -> io::Result<()> {
    let (name, operands, answer) = verb;
    let framing = format!("{}-{}-{}", route.named.replace(' ', "-"), name, records);
    let into = folder(&framing)?;
    let dir = into.join("recording");
    let listener = Listener::serving(route.answers.script(answer))?;
    let evidence = if records {
        format!("{{\"body\":\"{EVIDENCE}\"}}\n")
    } else {
        EVIDENCE.to_owned()
    };
    let named = |argument: &&str| match *argument {
        "{dir}" => dir.to_string_lossy().into_owned(),
        "{closed}" => CLOSED.to_owned(),
        other => other.to_owned(),
    };
    let adds: Vec<String> = route.adds.iter().map(named).collect();
    let mut asked = vec![name.to_owned(), QUESTION.to_owned()];
    asked.extend(operands.iter().map(|operand| (*operand).to_owned()));
    // A route that names its own address keeps it, and every other route posts
    // to the listener this case opened.
    if !route.adds.contains(&"--url") {
        asked.extend(["--url".to_owned(), listener.base().to_owned()]);
    }
    asked.extend(["--model".to_owned(), "local-1".to_owned()]);
    if records {
        asked.extend([
            "--jsonl".to_owned(),
            "--field".to_owned(),
            "/body".to_owned(),
        ]);
    }
    asked.extend(view.iter().map(|option| (*option).to_owned()));
    asked.extend(adds);

    if route.primed {
        let priming: Vec<&str> = asked
            .iter()
            .map(String::as_str)
            .map(|argument| {
                if argument == "--replay" {
                    "--record"
                } else {
                    argument
                }
            })
            .collect();
        let first = spawn(&priming, &environment(true), evidence.as_bytes())?;
        assert_eq!(first.status.code(), Some(0), "{framing}: the priming run");
    }
    if let Some(damage) = route.damage {
        let entries = written(&dir);
        assert!(!entries.is_empty(), "{framing}: an entry to damage");
        for entry in entries {
            fs::write(&entry, damage)?;
        }
    }

    let arguments: Vec<&str> = asked.iter().map(String::as_str).collect();
    let output = spawn(&arguments, &environment(route.keyed), evidence.as_bytes())?;

    assert_eq!(
        output.status.code(),
        Some(route.code),
        "{framing} {view:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        listener.requests().len(),
        route.requests,
        "{framing} {view:?}: the requests the listener saw"
    );
    nothing_leaked(&format!("{framing} {view:?}"), &output, &into);
    Ok(())
}

/// The four ways one route is driven: both views, one document and records.
const WAYS: [(&[&str], bool); 4] = [
    (&[], false),
    (&[], true),
    (&["--details"], false),
    (&["--details"], true),
];

/// The environment one run is given, with or without the key.
fn environment(keyed: bool) -> Vec<(&'static str, &'static str)> {
    if keyed {
        vec![("THINKTHEN_API_KEY", KEY)]
    } else {
        Vec::new()
    }
}

#[test]
fn no_command_on_any_backend_path_writes_the_key_or_quotes_the_evidence() {
    for route in &PATHS {
        for verb in VERBS {
            for (view, records) in WAYS {
                sweep(route, verb, view, records).expect("the compiled binary runs");
            }
        }
    }
}

/// The key reaches the one authorization header, and the listener saw it there.
///
/// A sweep that proved only absence would pass on a run that sent no key at
/// all, so one case pins where the key does go.
#[test]
fn the_key_reaches_the_authorization_header_and_nothing_else() {
    for (name, operands, answer) in VERBS {
        let into = folder(&format!("header-{name}")).expect("a folder for this run");
        let dir = into.join("recording");
        let listener =
            Listener::serving(vec![Canned::ok(&good(answer))]).expect("a loopback listener");
        let base = listener.base().to_owned();
        let kept = dir.to_string_lossy().into_owned();
        let asked = [name, QUESTION];
        let named = [
            "--url",
            &base,
            "--model",
            "local-1",
            "--details",
            "--record",
            &kept,
        ];

        let output = spawn(
            &[&asked[..], operands, &named[..]].concat(),
            &environment(true),
            EVIDENCE.as_bytes(),
        )
        .expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(0), "{name}");
        let requests = listener.requests();
        let request = requests.first().expect("one request reached the listener");
        assert_eq!(
            request.header("authorization"),
            Some(format!("Bearer {KEY}").as_str()),
            "{name}"
        );
        assert_eq!(written(&dir).len(), 1, "{name}: one entry was recorded");
        nothing_leaked(name, &output, &into);
    }
}

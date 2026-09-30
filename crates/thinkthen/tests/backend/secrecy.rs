//! One sweep checks output and files across commands, paths, framings, and views.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Output;

use crate::harness::{Canned, Listener, spawn};

mod routes;
use routes::good;
pub(crate) use routes::{PATHS, Route};

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

pub(crate) const SECOND: &str = "marker-second-e41d09";

pub(crate) const KIND: &str = "marker-kind-93c2f0";

pub(crate) const QUESTION: &str = "Does this report a payment failure?";

pub(crate) const VERBS: [(&str, &[&str], &str); 6] = [
    ("decide", &[], r#""type":"noul","noul":0.92"#),
    (
        "choose",
        &["late", "lost"],
        r#""type":"choice","choice":"late","probabilities":{"late":0.9,"lost":0.1}"#,
    ),
    ("tag", &["billing"], r#""type":"noul","noul":0.92"#),
    (
        "score",
        &["none", "some", "much"],
        r#""type":"score","probabilities":{"0":0.1,"1":0.2,"2":0.7}"#,
    ),
    (
        "recognize",
        &["person"],
        r#""type":"choice","choice":"OUT","probabilities":{"BEGIN":0,"INSIDE":0,"END":0,"SINGLE":0,"OUT":1}"#,
    ),
    // Two entities of one kind ask one unordered yes/no question.
    (
        "relate",
        &["linked", "--either"],
        r#""type":"noul","noul":0.92"#,
    ),
];

/// Whether the verb takes the shared question as its first operand.
pub(crate) fn asks_question(verb: &str) -> bool {
    !["recognize", "relate"].contains(&verb)
}

/// The standard input one verb judges under one framing, holding the evidence.
///
/// `relate` reads a complete entity set, so the evidence is one entity name.
pub(crate) fn evidence(verb: &str, framing: Option<&str>) -> Vec<u8> {
    let text = match (verb, framing) {
        ("relate", None) => {
            format!(
                r#"[{{"name":"{EVIDENCE}","kind":"{KIND}"}},{{"name":"{SECOND}","kind":"{KIND}"}}]"#
            )
        }
        ("relate", Some("--jsonl")) => format!(
            "{{\"body\":\"{EVIDENCE}\",\"kind\":\"{KIND}\"}}\n{{\"body\":\"{SECOND}\",\"kind\":\"{KIND}\"}}\n"
        ),
        ("relate", Some("--lines")) => format!("{EVIDENCE}\n{SECOND}\n"),
        ("relate", Some("--csv")) => format!("name,kind\n{EVIDENCE},{KIND}\n{SECOND},{KIND}\n"),
        ("relate", Some("--tsv")) => format!("name\tkind\n{EVIDENCE}\t{KIND}\n{SECOND}\t{KIND}\n"),
        (_, Some("--jsonl")) => format!("{{\"body\":\"{EVIDENCE}\"}}\n"),
        (_, Some("--lines")) => format!("{EVIDENCE}\n"),
        _ => EVIDENCE.to_owned(),
    };
    text.into_bytes()
}

const RECORD_VERBS: [(&str, &[&str], &str); 2] = [
    ("filter", &[], r#""type":"noul","noul":0.92"#),
    ("rank", &[], r#""type":"noul","noul":0.92"#),
];

/// A recording entry a reader takes, whose every field is hostile text.
///
/// It names another schema, so it is refused after it parses. No field of it
/// may reach a diagnostic: the text is unbounded, it holds a terminal escape,
/// and it quotes the evidence marker back.
pub(crate) const HOSTILE: &str = concat!(
    r#"{"schema":"\u001b[31mPWNED\u001b[0m marker-evidence-7b3ac5","#,
    r#""adapter":"\u001b[31mPWNED\u001b[0m marker-evidence-7b3ac5","#,
    r#""url":"\u001b[31mPWNED\u001b[0m marker-evidence-7b3ac5","#,
    r#""request":{},"response":{}}"#,
);

pub(crate) const HOSTILE_SCHEMA_MARKER: &str = "PWNED";

/// A recording entry that is not JSON, holding the evidence it recorded.
pub(crate) const DAMAGED: &str = concat!(
    r#"{"schema":"thinkthen.recording/1","adapter":"systemone","#,
    r#""request":{"evidence":"marker-evidence-7b3ac5"} "response":{}}"#,
);

/// The address of a port nothing listens on, which fails in the transport.
pub(crate) const CLOSED: &str = "http://127.0.0.1:1/v1";

/// A folder this run owns, remade so each run starts empty.
pub(crate) fn folder(named: &str) -> io::Result<PathBuf> {
    let path = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join("secrecy")
        .join(named);
    let _absent = fs::remove_dir_all(&path);
    fs::create_dir_all(&path)?;
    Ok(path)
}

/// Every file under this folder, however deep.
pub(crate) fn written(folder: &Path) -> Vec<PathBuf> {
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

/// Refuse the key and each evidence marker wherever it may not be, in one run.
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
    for marker in [EVIDENCE, SECOND, KIND] {
        assert!(
            !err.contains(marker),
            "{named}: {marker} is in a diagnostic\n{err}"
        );
    }
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

fn sweep(
    route: &Route,
    verb: (&str, &[&str], &str),
    view: &[&str],
    framing: Option<&str>,
) -> io::Result<()> {
    let (name, operands, answer) = verb;
    let case = format!(
        "{}-{name}-{}",
        route.named.replace(' ', "-"),
        framing.unwrap_or("document")
    );
    let into = folder(&case)?;
    let dir = into.join("recording");
    let listener = Listener::serving(route.answers.script(answer))?;
    let evidence = evidence(name, framing);
    let named = |argument: &&str| match *argument {
        "{dir}" => dir.to_string_lossy().into_owned(),
        "{closed}" => CLOSED.to_owned(),
        other => other.to_owned(),
    };
    let adds: Vec<String> = route.adds.iter().map(named).collect();
    let mut asked = vec![name.to_owned()];
    if asks_question(name) {
        asked.push(QUESTION.to_owned());
    }
    asked.extend(operands.iter().map(|operand| (*operand).to_owned()));
    // A route that names its own address keeps it, and every other route posts
    // to the listener this case opened.
    if !route.adds.contains(&"--url") {
        asked.extend(["--url".to_owned(), listener.base().to_owned()]);
    }
    asked.extend(["--model".to_owned(), "local-1".to_owned()]);
    if let Some(framing) = framing {
        asked.push(framing.to_owned());
        if framing == "--jsonl" {
            asked.extend(["--field".to_owned(), "/body".to_owned()]);
        }
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
        let first = spawn(&priming, &environment(true), &evidence)?;
        assert_eq!(first.status.code(), Some(0), "{case}: the priming run");
    }
    if let Some(damage) = route.damage {
        damage_entries(&case, &dir, damage)?;
    }

    let arguments: Vec<&str> = asked.iter().map(String::as_str).collect();
    // The platform default cache lands inside this case's folder, so the reader
    // below reads every file the default cache wrote.
    let cache = into.join("cache").to_string_lossy().into_owned();
    let mut environment = environment(route.keyed);
    environment.push(("XDG_CACHE_HOME", &cache));
    let output = spawn(&arguments, &environment, &evidence)?;

    assert_eq!(
        output.status.code(),
        Some(route.code),
        "{case} {view:?}: {}{}",
        String::from_utf8_lossy(&output.stderr),
        seen(&listener)
    );
    assert_eq!(
        listener.requests().len(),
        route.requests,
        "{case} {view:?}: the requests the listener saw"
    );
    if let Some(says) = route.says {
        let said = String::from_utf8_lossy(&output.stderr);
        assert!(said.contains(says), "{case} {view:?}: {said}");
    }
    if route.named == "a success" {
        assert!(
            !written(&into.join("cache")).is_empty(),
            "{case} {view:?}: the default cache kept the answer"
        );
    }
    nothing_leaked(&format!("{case} {view:?}"), &output, &into);
    if route.damage == Some(HOSTILE) {
        for bytes in [&output.stdout, &output.stderr] {
            let text = String::from_utf8_lossy(bytes);
            for forbidden in [HOSTILE_SCHEMA_MARKER, EVIDENCE] {
                assert!(!text.contains(forbidden), "{case} {view:?}: {forbidden}");
            }
        }
    }
    Ok(())
}

/// The connections and requests a listener saw, without the bodies.
fn seen(listener: &Listener) -> String {
    let mut saw = format!("the listener saw {} connections:", listener.connections());
    for request in listener.requests() {
        saw += &format!(" {} ({} body bytes)", request.line, request.body.len());
    }
    saw
}

/// Overwrite every entry a priming run recorded with the damaged bytes.
fn damage_entries(case: &str, dir: &Path, damage: &str) -> io::Result<()> {
    let entries = written(dir);
    assert!(!entries.is_empty(), "{case}: an entry to damage");
    for entry in entries.into_iter().filter(|entry| {
        entry
            .file_name()
            .is_some_and(|name| !name.to_string_lossy().starts_with('.'))
    }) {
        fs::write(&entry, damage)?;
    }
    Ok(())
}

/// The four ways one route is driven: both views, one document and records.
const WAYS: [(&[&str], Option<&str>); 4] = [
    (&[], None),
    (&[], Some("--jsonl")),
    (&["--details"], None),
    (&["--details"], Some("--jsonl")),
];

/// Both record framings in the bare and detailed views.
const RECORD_WAYS: [(&[&str], Option<&str>); 4] = [
    (&[], Some("--lines")),
    (&[], Some("--jsonl")),
    (&["--details"], Some("--lines")),
    (&["--details"], Some("--jsonl")),
];

/// The complete-set framings only `relate` reads, in the bare and detailed views.
const TABLE_WAYS: [(&[&str], Option<&str>); 6] = [
    (&[], Some("--lines")),
    (&[], Some("--csv")),
    (&[], Some("--tsv")),
    (&["--details"], Some("--lines")),
    (&["--details"], Some("--csv")),
    (&["--details"], Some("--tsv")),
];

/// The environment one run is given, with or without the key.
pub(crate) fn environment(keyed: bool) -> Vec<(&'static str, &'static str)> {
    if keyed {
        vec![("THINKTHEN_API_KEY", KEY)]
    } else {
        Vec::new()
    }
}

/// One spawn of the sweep: a verb, a view, and a framing.
type Case = (
    (&'static str, &'static [&'static str], &'static str),
    &'static [&'static str],
    Option<&'static str>,
);

/// Every case one route drives: each verb in each way, `relate` over its table
/// framings, and on the hostile replay route the record verbs too.
fn cases(route: &Route) -> Vec<Case> {
    let mut cases = Vec::new();
    for verb in VERBS {
        for (view, framing) in WAYS {
            cases.push((verb, view, framing));
        }
    }
    for verb in VERBS.into_iter().filter(|(name, _, _)| *name == "relate") {
        for (view, framing) in TABLE_WAYS {
            cases.push((verb, view, framing));
        }
    }
    if route.damage == Some(HOSTILE) {
        for verb in RECORD_VERBS {
            for (view, framing) in RECORD_WAYS {
                cases.push((verb, view, framing));
            }
        }
    }
    cases
}

/// Drive every case of one route.
///
/// Each route is its own test, so the runner spreads the 518 spawns across its
/// threads.
fn sweep_route(named: &str) -> io::Result<()> {
    let route = PATHS
        .iter()
        .find(|route| route.named == named)
        .ok_or_else(|| io::Error::other("the route left the matrix"))?;
    for (verb, view, framing) in cases(route) {
        sweep(route, verb, view, framing)?;
    }
    Ok(())
}

macro_rules! sweep_tests {
    ($($test:ident => $named:literal,)*) => {
        $(
            #[test]
            fn $test() {
                sweep_route($named).expect("the compiled binary runs");
            }
        )*

        /// A route added to the matrix without its own test fails here.
        #[test]
        fn every_route_has_its_own_sweep() {
            let tested = [$($named),*];
            let routes: Vec<&str> = PATHS.iter().map(|route| route.named).collect();
            assert_eq!(routes, tested);
            let hostile = PATHS.iter().filter(|route| route.damage == Some(HOSTILE)).count();
            assert_eq!(hostile, 1, "exactly one hostile replay route");
            let total: usize = PATHS.iter().map(|route| cases(route).len()).sum();
            assert_eq!(total, 518, "the sweep drives 518 cases");
        }
    };
}

// No command on any backend path writes the key or quotes the evidence.
sweep_tests! {
    no_leak_on_a_success => "a success",
    no_leak_on_a_plan => "a plan",
    no_leak_on_a_record_run => "a record run",
    no_leak_on_a_cache => "a cache",
    no_leak_on_a_replay => "a replay",
    no_leak_on_a_damaged_entry => "a damaged entry",
    no_leak_on_a_hostile_entry => "a hostile entry",
    no_leak_on_a_replay_miss => "a replay miss",
    no_leak_on_a_refused_address => "a refused address",
    no_leak_on_a_failed_request => "a failed request",
    no_leak_on_a_refused_request => "a refused request",
    no_leak_on_a_rate_limit_that_lifts => "a rate limit that lifts",
    no_leak_on_a_rate_limit_that_stays => "a rate limit that stays",
    no_leak_on_an_exhausted_backend_failure => "an exhausted backend failure",
    no_leak_on_an_unreadable_answer => "an unreadable answer",
    no_leak_on_a_recording_folder_that_cannot_be_made => "a recording folder that cannot be made",
    no_leak_on_a_run_with_no_key => "a run with no key",
}

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
        let asked = if asks_question(name) {
            vec![name, QUESTION]
        } else {
            vec![name]
        };
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
            &[&asked, operands, &named[..]].concat(),
            &environment(true),
            &evidence(name, None),
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
        let files = written(&dir);
        let marker = dir.join(".thinkthen-backend.json");
        assert!(files.contains(&marker), "{name}: backend marker");
        let entries: Vec<_> = files
            .iter()
            .filter(|path| {
                path.parent() == Some(dir.as_path()) && path.as_path() != marker.as_path()
            })
            .collect();
        assert_eq!(entries.len(), 1, "{name}: one recorded entry");
        assert_eq!(entries[0].extension(), Some(std::ffi::OsStr::new("json")));
        let digest = entries[0].file_stem().expect("entry digest");
        let lock = dir.join(".locks").join(digest);
        assert!(files.contains(&lock), "{name}: retained digest lock");
        assert_eq!(fs::metadata(lock).expect("digest lock").len(), 0);
        assert_eq!(files.len(), 3, "{name}: marker, entry, and digest lock");
        nothing_leaked(name, &output, &into);
    }
}

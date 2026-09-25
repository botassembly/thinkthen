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

/// The second entity `relate` reads, held to the same rule as the evidence.
pub(crate) const SECOND: &str = "marker-second-e41d09";

/// The kind both relate entities carry, held to the same rule as the evidence.
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
        r#""type":"choice","choice":"IN","probabilities":{"IN":0.9,"OUT":0.1}"#,
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
/// `relate` reads a complete entity set: the evidence and the second entity,
/// both of the marked kind wherever the framing carries a kind.
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

/// Refuse every marker everywhere it may not be, for one finished run.
///
/// This is the whole claim of secrecy, in one reader every case calls. The key
/// may reach no byte of standard output, of standard error, or of any file the
/// run wrote, and no header name that carries it may reach a file either. The
/// evidence, the second relate entity, and their kind may reach no byte of
/// standard error.
pub(crate) fn nothing_leaked(named: &str, output: &Output, folder: &Path) {
    let out = String::from_utf8_lossy(&output.stdout);
    let err = String::from_utf8_lossy(&output.stderr);
    assert!(!out.contains(KEY), "{named}: the key is on standard output");
    assert!(!err.contains(KEY), "{named}: the key is on standard error");
    for (marker, what) in [
        (EVIDENCE, "evidence"),
        (SECOND, "second entity"),
        (KIND, "kind"),
    ] {
        assert!(
            !err.contains(marker),
            "{named}: the {what} is in a diagnostic\n{err}"
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
    let requests: Vec<String> = listener
        .requests()
        .iter()
        .map(|request| format!("{} ({} body bytes)", request.line, request.body.len()))
        .collect();
    format!(
        "the listener saw {} connections and requests {requests:?}",
        listener.connections()
    )
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

#[test]
fn no_command_on_any_backend_path_writes_the_key_or_quotes_the_evidence() {
    for route in &PATHS {
        for verb in VERBS {
            for (view, framing) in WAYS {
                sweep(route, verb, view, framing).expect("the compiled binary runs");
            }
        }
    }
    let hostile = PATHS
        .iter()
        .find(|route| route.damage == Some(HOSTILE))
        .expect("the hostile replay route stays in the matrix");
    for verb in RECORD_VERBS {
        for (view, framing) in RECORD_WAYS {
            sweep(hostile, verb, view, framing).expect("the compiled binary runs");
        }
    }
    let relate = VERBS
        .into_iter()
        .find(|(name, _, _)| *name == "relate")
        .expect("relate stays in the matrix");
    for route in &PATHS {
        for (view, framing) in TABLE_WAYS {
            sweep(route, relate, view, framing).expect("the compiled binary runs");
        }
    }
}

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
        assert_eq!(
            written(&dir).len(),
            2,
            "{name}: marker and one entry were recorded"
        );
        nothing_leaked(name, &output, &into);
    }
}

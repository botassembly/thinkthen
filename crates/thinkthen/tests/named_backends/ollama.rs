//! The `ollama` built-in (ticket 0339, ADR 0115) against loopback mimics of
//! each backend's refusals. No test reaches a network or a real Ollama.
//!
//! The Ollama mimic answers 400 to an object description, as Ollama 0.35 does.
//! The Liquid mimic answers 422 to a null `noul` side, as Liquid d1 did. The
//! TypeSafe mimic accepts every form.

use std::process::Output;

use conformance_backend::{Canned, Listener};
use serde_json::Value;

use crate::support::{Home, KEYLESS, LIQUID_BASE, Proxy, TYPESAFE_BASE, by_marker, only, said};

/// The sentence `check` and `--plan` print for the Ollama workaround.
const DROPPED: &str = "backend `ollama` sends each description object as its `what` text, a temporary workaround for an Ollama bug, so its other fields are left out";

/// A question whose one option description is an object with more than `what`.
const OBJECT: &str = r#"{"choose":"Which team?","options":{"billing":{"what":"Money matters","not_for":"shipping","examples":["refund"]},"other":"Anything else."}}"#;

/// The same question with string descriptions only.
const STRINGS: &str =
    r#"{"choose":"Which team?","options":{"billing":"Money and bills","other":"Anything else."}}"#;

/// Whether any description in this wire question is an object.
fn object_description(question: &Value) -> bool {
    match &question["criteria"] {
        Value::Object(criteria) => criteria.values().any(Value::is_object),
        Value::Array(levels) => levels.iter().any(Value::is_object),
        _ => false,
    }
}

/// Whether this wire question is a `noul` with a null side.
fn null_noul_side(question: &Value) -> bool {
    question["type"] == "noul"
        && question["criteria"]
            .as_object()
            .is_some_and(|criteria| criteria.values().any(Value::is_null))
}

/// A loopback System One that answers `status` to any request holding a
/// question `refuses` names, and otherwise answers every question with a
/// confidence and usage.
fn mimic(refuses: fn(&Value) -> bool, status: u16) -> Listener {
    Listener::answering(move |body| {
        let request: Value = serde_json::from_slice(body).unwrap_or_default();
        let empty = serde_json::Map::new();
        let questions = request["questions"].as_object().unwrap_or(&empty);
        if questions.values().any(refuses) {
            return Canned::status(status, r#"{"detail":"refused by the mimic"}"#);
        }
        let answers: Vec<String> = questions
            .iter()
            .map(|(name, question)| format!(r#""{name}":{}"#, answer(question)))
            .collect();
        Canned::ok(&format!(
            r#"{{"model":{},"answers":{{{}}},"usage":{{"input_tokens":9,"output_tokens":3}}}}"#,
            request["model"],
            answers.join(",")
        ))
    })
    .expect("a loopback mimic")
}

/// One answer: 0.92 for a yes/no question, and 0.9 on the first option or
/// level of a choice or score, the rest shared.
fn answer(question: &Value) -> String {
    let kind = question["type"].as_str().unwrap_or_default();
    if kind == "noul" {
        return r#"{"type":"noul","noul":0.92}"#.to_owned();
    }
    let keys: Vec<String> = match &question["criteria"] {
        Value::Object(criteria) => criteria.keys().cloned().collect(),
        Value::Array(levels) => (0..levels.len()).map(|level| level.to_string()).collect(),
        _ => Vec::new(),
    };
    let rest = keys.len().saturating_sub(1).max(1);
    let shares: Vec<String> = keys
        .iter()
        .enumerate()
        .map(|(place, key)| {
            let share = if place == 0 { 0.9 } else { 0.1 / rest as f64 };
            format!(r#""{key}":{share}"#)
        })
        .collect();
    format!(
        r#"{{"type":"{kind}","probabilities":{{{}}},"confidence":0.9}}"#,
        shares.join(",")
    )
}

fn ollama_mimic() -> Listener {
    mimic(object_description, 400)
}

fn liquid_mimic() -> Listener {
    mimic(null_noul_side, 422)
}

fn typesafe_mimic() -> Listener {
    mimic(|_| false, 200)
}

/// The report lines `check` prints after the replies.
fn report(stdout: &str) -> String {
    let start = stdout.find("ok connection").unwrap_or(stdout.len());
    stdout[start..].to_owned()
}

#[test]
fn check_warns_for_the_ollama_workaround_and_each_other_backend_sends_as_authored() {
    let home = Home::new("ollama-check");
    let ollama = ollama_mimic();
    let run = home.run(
        &["check", "--backend", "ollama", "--url", ollama.base()],
        &[("OLLAMA_API_KEY", "")],
    );
    let (stdout, stderr) = said(&run);
    assert_eq!((run.status.code(), stderr.as_str()), (Some(0), ""));
    assert_eq!(
        report(&stdout),
        format!(
            "ok connection\nok key\nok endpoint\nok noul\nwarning choice: {DROPPED}\nwarning score: {DROPPED}\nwarning mixed: {DROPPED}\nok usage\ncritical 0, warning 3\n"
        )
    );
    assert!(stdout.contains("\nmodel sent nimble\n"), "{stdout}");
    assert_eq!(
        by_marker(&ollama),
        only(KEYLESS, 4),
        "loopback Ollama takes no key"
    );

    // The unnamed path sends the authored bytes, which Ollama refuses.
    let unnamed = home.run(&["check", "--url", ollama.base()], &[]);
    let (stdout, _) = said(&unnamed);
    assert_eq!(unnamed.status.code(), Some(4));
    assert!(stdout.ends_with("\ncritical 3, warning 0\n"), "{stdout}");
    assert_eq!(ollama.count(), 8);

    // Liquid's own mimic accepts the authored bytes: no null `noul` side travels.
    let liquid = liquid_mimic();
    let run = home.run(
        &["check", "--backend", "liquid", "--url", liquid.base()],
        &[],
    );
    let (stdout, stderr) = said(&run);
    assert_eq!((run.status.code(), stderr.as_str()), (Some(0), ""));
    assert!(
        stdout.ends_with("\nok usage\ncritical 0, warning 0\n"),
        "{stdout}"
    );
    assert_eq!(by_marker(&liquid), only(1, 4));

    let typesafe = typesafe_mimic();
    let run = home.run(
        &["check", "--backend", "typesafe", "--url", typesafe.base()],
        &[],
    );
    let (stdout, _) = said(&run);
    assert_eq!(run.status.code(), Some(0));
    assert!(stdout.ends_with("\ncritical 0, warning 0\n"), "{stdout}");
    assert_eq!(by_marker(&typesafe), only(0, 4));
    home.assert_no_marker_in_files();
}

/// The request bodies a check plan prints, in probe order.
fn planned_bodies(stdout: &str) -> String {
    stdout
        .lines()
        .filter_map(|line| line.strip_prefix("request "))
        .filter_map(|line| line.split_once(' ').map(|(_, body)| format!("{body}\n")))
        .collect()
}

#[test]
fn a_plan_says_once_when_detail_is_dropped_and_pins_each_form() {
    let home = Home::new("ollama-plan");
    let listener = typesafe_mimic();
    let fixture = |name: &str| {
        std::fs::read_to_string(format!(
            "{}/../../specification/fixtures/check/{name}",
            env!("CARGO_MANIFEST_DIR")
        ))
        .expect("a check fixture")
    };
    let text = fixture("requests-text.jsonl");
    let authored = fixture("requests.jsonl");
    let run = home.run(
        &[
            "check",
            "--backend",
            "ollama",
            "--url",
            listener.base(),
            "--plan",
        ],
        &[],
    );
    let (stdout, stderr) = said(&run);
    assert_eq!(run.status.code(), Some(0));
    assert_eq!(stderr, format!("thinkthen: {DROPPED}\n"));
    assert_eq!(planned_bodies(&stdout), text);
    // The default Ollama base plans the same bodies.
    let run = home.run(&["check", "--backend", "ollama", "--plan"], &[]);
    let (stdout, stderr) = said(&run);
    assert_eq!(run.status.code(), Some(0));
    assert_eq!(stderr, format!("thinkthen: {DROPPED}\n"));
    assert_eq!(planned_bodies(&stdout), text);
    assert!(
        stdout.starts_with("url http://localhost:11434/v1/systemone\n"),
        "{stdout}"
    );
    // TypeSafe and Liquid keep the authored fixture's bodies, model aside.
    for (name, model) in [("typesafe", "jev-1.13.0"), ("liquid", "d1:free")] {
        let run = home.run(
            &[
                "check",
                "--backend",
                name,
                "--url",
                listener.base(),
                "--plan",
            ],
            &[],
        );
        let (stdout, stderr) = said(&run);
        assert_eq!(
            (run.status.code(), stderr.as_str()),
            (Some(0), ""),
            "{name}"
        );
        assert_eq!(
            planned_bodies(&stdout),
            authored.replace(r#""model":"jev-1.13.0""#, &format!(r#""model":"{model}""#)),
            "{name}"
        );
    }

    // An asking plan says it for an object description, and not for strings.
    for (question, flags, sentence) in [
        (
            OBJECT,
            &["--backend", "ollama"][..],
            format!("thinkthen: {DROPPED}\n"),
        ),
        (STRINGS, &["--backend", "ollama"][..], String::new()),
        (OBJECT, &["--backend", "liquid"][..], String::new()),
    ] {
        let run = ask(&home, question, &[flags, &["--plan"]].concat(), &[]);
        let (stdout, stderr) = said(&run);
        assert_eq!(run.status.code(), Some(0), "{stderr}");
        assert_eq!(stderr, sentence, "{flags:?}");
        let body = if flags[1] == "ollama" && question == OBJECT {
            r#""criteria":{"billing":"Money matters","other":"Anything else."}"#
        } else if question == OBJECT {
            r#""criteria":{"billing":{"what":"Money matters","not_for":"shipping","examples":["refund"]},"other":"Anything else."}"#
        } else {
            r#""criteria":{"billing":"Money and bills","other":"Anything else."}"#
        };
        assert!(stdout.contains(body), "{stdout}");
    }
    assert_eq!(listener.count(), 0, "a plan sends nothing");
    home.assert_no_marker_in_files();
}

/// Choose over two lines with this question file, one request per line.
fn ask(home: &Home, question: &str, flags: &[&str], changes: &[(&str, &str)]) -> Output {
    let file = home.root.join("question.json");
    std::fs::write(&file, question).expect("a question file");
    let file = format!("@{}", file.display());
    let input = home.evidence(&["alpha", "beta"]);
    let mut arguments = vec!["choose", &file, "--lines", "--input", &input];
    arguments.extend_from_slice(flags);
    home.run(&arguments, &[&[("THINKTHEN_BATCH", "1")], changes].concat())
}

#[test]
fn ollama_needs_no_key_at_loopback_and_its_key_stays_off_other_hosts() {
    let proxy = Proxy::start();
    let home = Home::new("ollama-keys");
    let target = ollama_mimic();
    let keyless = ask(
        &home,
        OBJECT,
        &["--backend", "ollama", "--url", target.base(), "--no-cache"],
        &[("OLLAMA_API_KEY", "")],
    );
    let (stdout, stderr) = said(&keyless);
    assert_eq!(keyless.status.code(), Some(0), "{stderr}");
    assert_eq!(stdout.lines().count(), 2);
    assert_eq!(
        by_marker(&target),
        only(KEYLESS, 2),
        "no Authorization header"
    );

    let refusal = |name: &str, variable: &str, owner: &str, other: &str| {
        format!(
            "thinkthen: backend `{name}` reads `{variable}`, the key of backend `{owner}`, which never goes to the address of backend `{other}`\n"
        )
    };
    let stolen = format!(
        r#"{{"schema":"thinkthen.config/1","backends":{{"stolen":{{"url":"{LIQUID_BASE}","key_env":"OLLAMA_API_KEY","model":"m"}}}}}}"#
    );
    let described = r#"{"schema":"thinkthen.config/1","backends":{"local":{"url":"http://127.0.0.1:9/v1","key_env":"K","model":"m","descriptions":"text"}}}"#;
    // (flags, changes, configuration, standard error, exit code)
    type Case<'a> = (Vec<&'a str>, Vec<(&'a str, &'a str)>, &'a str, String, i32);
    let cases: Vec<Case<'_>> = vec![
        (vec!["--backend", "ollama", "--url", "https://ollama.example/v1"], vec![("OLLAMA_API_KEY", "")], "", "thinkthen: the environment variable `OLLAMA_API_KEY` is unset or blank, so no key is sent\nthinkthen: stopped at record 1; 0 records finished\n".to_owned(), 4),
        (vec!["--backend", "ollama", "--url", "https://ollama.example/v1"], vec![("OLLAMA_API_KEY", " ")], "", "thinkthen: the environment variable `OLLAMA_API_KEY` is unset or blank, so no key is sent\nthinkthen: stopped at record 1; 0 records finished\n".to_owned(), 4),
        (vec!["--backend", "ollama", "--url", TYPESAFE_BASE], vec![], "", refusal("ollama", "OLLAMA_API_KEY", "ollama", "typesafe"), 2),
        (vec!["--backend", "ollama", "--url", LIQUID_BASE], vec![], "", refusal("ollama", "OLLAMA_API_KEY", "ollama", "liquid"), 2),
        (vec!["--backend", "stolen"], vec![], &stolen, refusal("stolen", "OLLAMA_API_KEY", "ollama", "liquid"), 2),
        (vec![], vec![], described, "thinkthen: configuration backend entries hold only `url`, `key_env`, `model`, and `requests_per_minute`\n".to_owned(), 5),
    ];
    for (index, (flags, changes, config, sentence, code)) in cases.into_iter().enumerate() {
        let home = Home::new("ollama-refused");
        home.config(config);
        let changes = [&[("HTTPS_PROXY", proxy.url.as_str())], changes.as_slice()].concat();
        let output = ask(&home, STRINGS, &flags, &changes);
        let (stdout, stderr) = said(&output);
        assert_eq!(
            (stdout.as_str(), stderr.as_str()),
            ("", sentence.as_str()),
            "{index}"
        );
        assert_eq!(output.status.code(), Some(code), "{index}");
        home.assert_no_marker_in_files();
    }
    assert_eq!(proxy.count(), 0, "no refusal opened a connection");

    // A loopback built-in base is no other built-in's host (ADR 0115 section 2).
    for (name, place) in [("liquid", 1), ("typesafe", 0)] {
        let target = typesafe_mimic();
        let local = target.base().replace("127.0.0.1", "localhost");
        let output = ask(
            &home,
            STRINGS,
            &["--backend", name, "--url", &local, "--no-cache"],
            &[],
        );
        let (_, stderr) = said(&output);
        assert_eq!(output.status.code(), Some(0), "{name}: {stderr}");
        assert_eq!(by_marker(&target), only(place, 2), "{name}");
    }
    let status = home.run(
        &["status", "--backend", "ollama"],
        &[("OLLAMA_API_KEY", "")],
    );
    let (stdout, _) = said(&status);
    assert!(
        stdout.contains("backend ollama\nurl http://localhost:11434/v1/systemone\nurl_source backend\nmodel nimble\nmodel_source backend\nkey_variable OLLAMA_API_KEY\napi_key_set false\n"),
        "{stdout}"
    );
    home.assert_no_marker_in_files();
}

/// The question keys a recording folder's store holds, which hash the posting
/// URL, the model, the state and each question as sent (ADR 0111).
fn keys(folder: &str) -> Vec<Vec<u8>> {
    let store = rusqlite::Connection::open(std::path::Path::new(folder).join("thinkthen.sqlite"))
        .expect("the recording store");
    let mut query = store
        .prepare("SELECT key FROM answers ORDER BY key")
        .expect("a query");
    let rows = query
        .query_map([], |row| row.get::<_, Vec<u8>>(0))
        .expect("the keys");
    rows.map(|key| key.expect("a key")).collect()
}

#[test]
fn a_question_key_holds_the_bytes_as_sent_so_forms_share_only_equal_questions() {
    let target = typesafe_mimic();
    let home = Home::new("ollama-cache");
    home.config(&format!(
        r#"{{"schema":"thinkthen.config/1","backends":{{"local":{{"url":"{}","key_env":"LOCAL_D1_KEY","model":"nimble"}}}}}}"#,
        target.base()
    ));
    let ollama = ["--backend", "ollama", "--url", target.base()];
    let local = ["--backend", "local"];
    let run = |question: &str, flags: &[&str]| {
        let output = ask(&home, question, flags, &[]);
        let (stdout, stderr) = said(&output);
        assert_eq!(output.status.code(), Some(0), "{stderr}");
        stdout
    };
    // One object description: each form writes other bytes, so each asks.
    run(OBJECT, &ollama);
    assert_eq!(target.count(), 2);
    run(OBJECT, &local);
    assert_eq!(target.count(), 4, "the authored form reuses no text answer");
    // String descriptions only: both forms write the same bytes and share them.
    let first = run(STRINGS, &ollama);
    assert_eq!(target.count(), 6);
    let again = run(STRINGS, &local);
    assert_eq!(
        target.count(),
        6,
        "the same question as sent is answered once"
    );
    assert_eq!(again, first);

    // A recording keys each question the same way, so it follows the same rule.
    let folder = |name: &str| home.path(name);
    for (question, name) in [(OBJECT, "object"), (STRINGS, "strings")] {
        for (flags, side) in [(&ollama[..], "text"), (&local[..], "authored")] {
            let record = folder(&format!("{name}-{side}"));
            run(question, &[flags, &["--record", &record]].concat());
        }
    }
    let object = keys(&folder("object-text"));
    assert_eq!(object.len(), 2);
    assert!(
        keys(&folder("object-authored"))
            .iter()
            .all(|key| !object.contains(key))
    );
    let strings = keys(&folder("strings-text"));
    assert_eq!(strings.len(), 2);
    assert_eq!(keys(&folder("strings-authored")), strings);
    home.assert_no_marker_in_files();
}

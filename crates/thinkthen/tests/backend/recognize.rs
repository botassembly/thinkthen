//! The compiled recognize command against a counted loopback backend.

use crate::harness::{Canned, Listener, spawn};
use serde_json::Value;
use std::{fs, path::PathBuf, process::Output, sync::Arc};

mod rules;
mod stores;

const ADA: &[u8] = b"Ada met Acme.";

const ADA_AND_ACME: &str = r#"{"entities":[{"text":"Ada","start":0,"end":3,"length":3,"kind":"person","strength":0.9},{"text":"Acme","start":8,"end":12,"length":4,"kind":"organization","strength":0.9}]}"#;

fn capital(word: &str) -> bool {
    word.chars().next().is_some_and(char::is_uppercase)
}

/// The text before, inside and after the `[[ ]]` stretch a question shows.
fn marked(words: &str) -> (&str, &str, &str) {
    let (_, shown) = words
        .split_once("\n\nSnippet: ")
        .or_else(|| words.split_once("\n\nText: "))
        .expect("a shown text");
    let (before, rest) = shown.split_once("[[").expect("an opening mark");
    let (inside, after) = rest.split_once("]]").expect("a closing mark");
    (before, inside, after)
}

/// A step-1 tag from capitals: a capitalized piece is part of a name, and
/// capitalized neighbours across a space join it.
fn tag(words: &str) -> &'static str {
    let (before, piece, after) = marked(words);
    let before = before
        .ends_with(' ')
        .then(|| before.split_whitespace().last())
        .flatten();
    let after = after
        .starts_with(' ')
        .then(|| after.split_whitespace().next())
        .flatten();
    let joined = |word: Option<&str>| word.is_some_and(capital);
    match (capital(piece), joined(before), joined(after)) {
        (false, ..) => "OUT",
        (true, false, false) => "SINGLE",
        (true, false, true) => "BEGIN",
        (true, true, true) => "INSIDE",
        (true, true, false) => "END",
    }
}

/// A kind by spelling: `Nobody` is declined, `Town…` is a place, `Acme`,
/// `Corp` and `O…` are organizations, and any other name is a person.
fn kind(words: &str) -> &'static str {
    match marked(words).1 {
        "Nobody" => "none of these",
        name if name.starts_with("Town") => "place",
        "Acme" | "Corp" => "organization",
        name if name.starts_with('O') => "organization",
        _ => "person",
    }
}

/// Answer every question: step-1 tags from capitals at 1.0, a kind by
/// spelling at 0.9, the edge as found, and each pair at 0.9.
pub(super) fn automatic(body: &[u8]) -> Canned {
    let request: Value = serde_json::from_slice(body).expect("request");
    let mut answers = serde_json::Map::new();
    for (name, question) in request["questions"].as_object().expect("questions") {
        if question["type"] == "noul" {
            answers.insert(name.clone(), serde_json::json!({"type":"noul","noul":0.9}));
            continue;
        }
        let labels = question["criteria"].as_object().expect("criteria");
        let words = question["instructions"].as_str().expect("instructions");
        let (pick, (share, other)) = if labels.contains_key("BEGIN") {
            (tag(words), (1.0, 0.0))
        } else if labels.contains_key("none of these") {
            let held = kind(words);
            (
                if labels.contains_key(held) {
                    held
                } else {
                    "none of these"
                },
                (0.9, 0.1),
            )
        } else {
            (marked(words).1, (1.0, 0.0))
        };
        let rest = labels
            .keys()
            .find(|label| *label != pick)
            .expect("a second option");
        let probabilities = labels
            .keys()
            .map(|label| {
                let value = if label == pick {
                    share
                } else if label == rest {
                    other
                } else {
                    0.0
                };
                (label.clone(), Value::from(value))
            })
            .collect::<serde_json::Map<_, _>>();
        answers.insert(
            name.clone(),
            serde_json::json!({"type":"choice","choice":pick,"probabilities":probabilities}),
        );
    }
    Canned::ok(&serde_json::json!({"model":"local-1","answers":answers,"usage":{"input_tokens":10,"output_tokens":2}}).to_string())
}

pub(super) fn questions(body: &[u8]) -> Vec<Value> {
    let request: Value = serde_json::from_slice(body).expect("request");
    request["questions"]
        .as_object()
        .expect("questions")
        .values()
        .cloned()
        .collect()
}

fn options(question: &Value) -> Vec<String> {
    question["criteria"]
        .as_object()
        .expect("criteria")
        .keys()
        .cloned()
        .collect()
}

pub(super) fn stdout(output: &Output) -> String {
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout.clone()).expect("output text")
}

pub(super) fn json(output: &Output) -> Value {
    serde_json::from_str(&stdout(output)).expect("one JSON value")
}

/// Run `recognize` at the listener, under `key` when one is given.
pub(super) fn local(
    listener: &Listener,
    options: &[&str],
    key: Option<&str>,
    input: &[u8],
) -> Output {
    let mut arguments = vec!["recognize", "--url", listener.base(), "--model", "local-1"];
    arguments.extend_from_slice(options);
    let environment: Vec<(&str, &str)> = key
        .map(|key| ("THINKTHEN_API_KEY", key))
        .into_iter()
        .collect();
    spawn(&arguments, &environment, input).expect("command")
}

pub(super) fn run(listener: &Listener, options: &[&str], input: &[u8]) -> Output {
    local(
        listener,
        &[options, &["--no-cache"]].concat(),
        Some("secret-value"),
        input,
    )
}

/// A listener that sends `reply` to each request holding `marker` and answers the rest.
pub(super) fn failing_on(marker: &'static str, reply: String) -> Listener {
    Listener::answering(move |body| {
        if String::from_utf8_lossy(body).contains(marker) {
            Canned::ok(&reply)
        } else {
            automatic(body)
        }
    })
    .expect("listener")
}

pub(super) const REFUSED: &str = "thinkthen: the reply was refused: the answer to question `q1` is not the shape the question asked for\n";

fn profile(name: &str, body: &str) -> PathBuf {
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("recognize-{name}.json"));
    fs::write(
        &path,
        format!(r#"{{"schema":"thinkthen.backend-profile/1","name":"{name}",{body}}}"#),
    )
    .expect("profile");
    path
}

const KINDS: [&str; 2] = ["person", "organization"];

const WORKS: [&str; 4] = [
    "person",
    "organization",
    "--relation",
    "works_for=person:organization",
];

/// Ticket 0147, test 6: one step-1 request, then one step-2 request holding
/// both kind questions and `Acme`'s edge question.
#[test]
fn ada_met_acme_sends_one_step_one_request_then_one_step_two_request() {
    let listener = Listener::answering(automatic).expect("listener");
    assert_eq!(
        stdout(&run(&listener, &KINDS, ADA)),
        format!("{ADA_AND_ACME}\n")
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 2);
    let first = questions(&requests[0].body);
    assert_eq!(first.len(), 4);
    assert!(
        first
            .iter()
            .all(|question| options(question) == ["BEGIN", "END", "INSIDE", "OUT", "SINGLE"])
    );
    let kinds = vec!["none of these", "organization", "person"];
    let second: Vec<Vec<String>> = questions(&requests[1].body).iter().map(options).collect();
    assert_eq!(second, [kinds.clone(), kinds, vec!["Acme", "Acme."]]);
}

/// Ticket 0147, test 6: no kinds asks only edge questions, a step-2 request
/// with no questions is not sent, and a text with no names sends one request.
#[test]
fn no_kinds_asks_only_edges_and_sends_no_empty_step_two_request() {
    let entity = |text: &str, start: usize| {
        format!(
            r#"{{"text":"{text}","start":{start},"end":{},"length":{},"kind":"ENTITY","strength":1.0}}"#,
            start + text.len(),
            text.len()
        )
    };
    let listener = Listener::answering(automatic).expect("listener");
    let names = [entity("Ada", 0), entity("Acme", 8)].join(",");
    assert_eq!(
        stdout(&run(&listener, &[], ADA)),
        format!("{{\"entities\":[{names}]}}\n")
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 2);
    let second: Vec<Vec<String>> = questions(&requests[1].body).iter().map(options).collect();
    assert_eq!(second, [vec!["Acme", "Acme."]]);

    let names = [entity("Ada", 0), entity("Bob", 8)].join(",");
    assert_eq!(
        stdout(&run(&listener, &[], b"Ada met Bob")),
        format!("{{\"entities\":[{names}]}}\n")
    );
    assert_eq!(listener.requests().len(), 1);

    assert_eq!(
        stdout(&run(&listener, &["person"], b"the cat sat")),
        "{\"entities\":[]}\n"
    );
    assert_eq!(listener.requests().len(), 1);
}

/// Ticket 0147, test 6: a kind's description reaches its step-2 option and
/// never the step-1 request.
#[test]
fn descriptions_reach_the_step_two_option_and_never_step_one() {
    let listener = Listener::answering(automatic).expect("listener");
    let output = run(
        &listener,
        &[
            "--kind",
            "person=DESC-PERSON",
            "--kind",
            "organization=DESC-ORG",
        ],
        ADA,
    );
    assert_eq!(stdout(&output), format!("{ADA_AND_ACME}\n"));
    let requests = listener.requests();
    assert!(!String::from_utf8_lossy(&requests[0].body).contains("DESC-"));
    let second = questions(&requests[1].body);
    assert_eq!(second[0]["criteria"]["person"], "DESC-PERSON");
    assert_eq!(second[0]["criteria"]["organization"], "DESC-ORG");
}

/// Ticket 0147, test 6: a failed step-2 request fails the text at exit 4
/// with no partial names.
#[test]
fn a_failed_step_two_request_fails_the_text() {
    let wrong = r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.5}}}"#;
    let listener = failing_on("none of these", wrong.to_owned());
    let output = run(&listener, &KINDS, ADA);
    assert_eq!(output.status.code(), Some(4));
    assert!(output.stdout.is_empty());
    assert_eq!(String::from_utf8_lossy(&output.stderr), REFUSED);
    assert_eq!(listener.requests().len(), 2);
}

/// Ticket 0147, test 6: the guard plans 600,000 bytes, refuses 600,001 before
/// any send, and moves with `--max-text-bytes`.
#[test]
fn the_guard_plans_600000_bytes_and_refuses_600001_before_any_send() {
    let listener = Listener::answering(automatic).expect("listener");
    let at_limit = format!("{} ", "w".repeat(9_999)).repeat(60);
    assert_eq!(at_limit.len(), 600_000);
    let plan = json(&run(
        &listener,
        &["person", "--dry-run"],
        at_limit.as_bytes(),
    ));
    assert_eq!(
        (&plan["pieces"], &plan["request_count"]),
        (&Value::from(60), &Value::from(2))
    );
    let over = format!("{at_limit}w");
    let refused = run(&listener, &["person"], over.as_bytes());
    assert_eq!(refused.status.code(), Some(2));
    assert_eq!(
        String::from_utf8_lossy(&refused.stderr),
        "thinkthen: recognize: the text is 600001 bytes, over the limit of 600000; raise it with --max-text-bytes\n"
    );
    let raised = json(&run(
        &listener,
        &["person", "--dry-run", "--max-text-bytes", "700000"],
        over.as_bytes(),
    ));
    assert_eq!(raised["pieces"], 61);
    assert_eq!(
        run(&listener, &["person", "--max-text-bytes", "0"], ADA)
            .status
            .code(),
        Some(2)
    );
    assert_eq!(listener.connections(), 0);
}

/// A dry run prints the digests and bodies of the step-1 requests a live run sends first.
#[test]
fn the_dry_run_prints_the_step_one_requests_a_live_run_sends() {
    let listener = Listener::answering(automatic).expect("listener");
    let plan = json(&run(&listener, &[&KINDS[..], &["--dry-run"]].concat(), ADA));
    let head = [
        "schema",
        "url",
        "model",
        "key_env",
        "pieces",
        "request_count",
        "name_requests_upper_bound",
    ]
    .map(|key| plan[key].clone());
    let url = format!("{}/systemone", listener.base());
    let expected = serde_json::json!([
        "thinkthen.recognize-plan/2",
        url,
        "local-1",
        "THINKTHEN_API_KEY",
        4,
        1,
        1
    ]);
    assert_eq!(Value::from(head.to_vec()), expected);
    assert_eq!(listener.connections(), 0);
    let result = json(&run(&listener, &[&KINDS[..], &["--details"]].concat(), ADA));
    let planned = &plan["requests"][0];
    assert_eq!(planned["digest"], result["meta"]["requests"][0]);
    let sent = listener.requests();
    assert_eq!(
        planned["body_utf8"],
        Value::from(String::from_utf8_lossy(&sent[0].body))
    );
    assert_eq!(planned["bytes"], sent[0].body.len());
}

#[test]
fn question_file_dry_run_attributes_source_and_sums_every_rule_bound() {
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("recognize-question.json");
    fs::write(&path, r#"{"version":1,"recognize":{"kinds":{"person":"A person.","organization":"An organization."},"relations":[{"name":"directed","source":"person","target":"person"},{"name":"either","source":"person","target":"person","either":true},{"name":"cross","source":"person","target":"organization"}]}}"#).expect("question file");
    let listener = Listener::serving(Vec::new()).expect("listener");
    let report = json(&local(
        &listener,
        &[&format!("@{}", path.display()), "--dry-run"],
        None,
        ADA,
    ));
    assert_eq!(report["from"], serde_json::json!({"question":"file"}));
    assert_eq!(report["relation_pairs_upper_bound"], 30);
    assert_eq!(report["relation_requests_upper_bound"], 30);
    assert_eq!(listener.connections(), 0);
}

#[test]
fn a_saved_recognize_name_reaches_details_identity_and_warning() {
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("recognize-calibrated.json");
    fs::write(
        &path,
        r#"{"version":1,"recognize":{"kinds":{"person":"A person."}},"profile":"old"}"#,
    )
    .expect("question file");
    let running = profile("new", r#""max_evidence_bytes":1000"#);
    let listener = Listener::answering(automatic).expect("listener");
    let output = run(
        &listener,
        &[
            &format!("@{}", path.display()),
            "--details",
            "--profile",
            &running.to_string_lossy(),
        ],
        ADA,
    );
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let row: Value = serde_json::from_slice(&output.stdout).expect("details");
    assert_eq!(
        row["meta"]["question_sha256"],
        "a5514b1a486b91807b0bc90a2f7da62ac618f6a9a1c3c31a1f850d8022e1825d"
    );
    assert_eq!(
        row["meta"]["profile_warning"],
        serde_json::json!({"tuned_for":"old","running":"new"})
    );
    assert!(!listener.requests().is_empty());
}

#[test]
fn local_validation_matrix_never_sends() {
    let lacking_sign: [&[&str]; 2] = [&["--kind", "PER"], &["--kind", "PER", "--kind", "ORG"]];
    for options in lacking_sign {
        let listener = Listener::answering(automatic).expect("listener");
        let output = run(&listener, options, ADA);
        assert_eq!(
            String::from_utf8_lossy(&output.stderr),
            "thinkthen: --kind is KIND=DESCRIPTION, and this one holds no `=`; give a bare kind without --kind\n",
            "{options:?}"
        );
        assert_eq!(
            (output.status.code(), listener.connections()),
            (Some(2), 0),
            "{options:?}"
        );
    }
    let refused: [&[&str]; 5] = [
        &["person", "person"],
        &["person", "organization", "--relation", "x=person:place"],
        &["person", "--threshold", "0"],
        &["person", "--kind", "place=A place."],
        &["person", "--relation", "works_for=person"],
    ];
    for options in refused {
        let listener = Listener::answering(automatic).expect("listener");
        let output = run(&listener, options, ADA);
        assert_eq!(
            (output.status.code(), listener.connections()),
            (Some(2), 0),
            "{options:?}"
        );
    }
}

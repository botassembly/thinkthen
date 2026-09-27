//! Ticket 0147, test 7: rules with no kind limits.

use super::{automatic, questions, run, stdout};
use crate::harness::{Listener, spawn};
use serde_json::Value;

const SPELLINGS: [&str; 3] = ["knows", "knows=*:*", "knows=ANY:ANY"];

/// The canonical question and its digest, as `--details` prints them.
fn question(listener: &Listener, options: &[&str], text: &[u8]) -> (Value, Value) {
    let mut options = options.to_vec();
    options.push("--details");
    let result: Value =
        serde_json::from_str(&stdout(&run(listener, &options, text))).expect("details");
    (
        result["question"].clone(),
        result["meta"]["question_sha256"].clone(),
    )
}

#[test]
fn bare_star_and_any_rules_give_one_plan_and_one_digest() {
    let listener = Listener::answering(automatic).expect("listener");
    let plans: Vec<String> = SPELLINGS
        .iter()
        .map(|rule| {
            stdout(&run(
                &listener,
                &["person", "organization", "--relation", rule, "--dry-run"],
                b"Ada met Acme.",
            ))
        })
        .collect();
    assert_eq!(plans[0], plans[1]);
    assert_eq!(plans[0], plans[2]);
    let asked: Vec<(Value, Value)> = SPELLINGS
        .iter()
        .map(|rule| {
            question(
                &listener,
                &["person", "organization", "--relation", rule],
                b"Ada met Acme.",
            )
        })
        .collect();
    assert_eq!(
        asked[0].0["relations"],
        serde_json::json!([{"name":"knows","source":"*","target":"*","reads":"knows","either":false}])
    );
    assert_eq!(asked[0], asked[1]);
    assert_eq!(asked[0], asked[2]);
}

/// The digest is sha256 of the canonical line
/// `{"verb":"recognize","kinds":{},"threshold":0.5,"relation_threshold":0.5}`.
#[test]
fn no_kinds_writes_the_canonical_empty_kinds_and_its_pinned_digest() {
    let listener = Listener::answering(automatic).expect("listener");
    let (canonical, digest) = question(&listener, &[], b"Ada met Acme.");
    assert_eq!(
        canonical,
        serde_json::json!({"verb":"recognize","kinds":{},"threshold":0.5,"relation_threshold":0.5})
    );
    assert_eq!(
        digest,
        "ef040b9c844faf5a4a2cece2fabd38b70b4b181a04baaeb6ef5a41b75b698484"
    );
}

/// A declined name never enters a pair: two kept names give two ordered pairs.
#[test]
fn a_declined_name_enters_no_pair_under_a_bare_rule() {
    let listener = Listener::answering(automatic).expect("listener");
    let output = run(
        &listener,
        &["person", "organization", "--relation", "knows"],
        b"Ada met Nobody and Acme.",
    );
    let value: Value = serde_json::from_str(&stdout(&output)).expect("result");
    let names: Vec<&str> = value["entities"]
        .as_array()
        .unwrap()
        .iter()
        .map(|name| name["text"].as_str().unwrap())
        .collect();
    assert_eq!(names, ["Ada", "Acme"]);
    let pairs = listener.requests().pop().expect("pair request");
    let asked: Vec<Value> = questions(&pairs.body)
        .iter()
        .map(|question| question["instructions"].clone())
        .collect();
    assert_eq!(
        asked,
        [
            "Does the text itself state that i1 knows i2?",
            "Does the text itself state that i2 knows i1?"
        ]
    );
    let state = &serde_json::from_slice::<Value>(&pairs.body).unwrap()["state"];
    assert_eq!(
        state["entities"],
        serde_json::json!([{"id":"i1","name":"Ada","kind":"person"},{"id":"i2","name":"Acme","kind":"organization"}])
    );
}

#[test]
fn relate_gives_one_plan_for_the_three_spellings() {
    let listener = Listener::answering(automatic).expect("listener");
    let names = br#"[{"name":"Ada","kind":"person"},{"name":"Acme","kind":"organization"}]"#;
    let plans: Vec<String> = SPELLINGS
        .iter()
        .map(|rule| {
            let arguments = [
                "relate",
                rule,
                "--dry-run",
                "--url",
                listener.base(),
                "--model",
                "local-1",
                "--no-cache",
            ];
            stdout(&spawn(&arguments, &[], names).expect("relate"))
        })
        .collect();
    assert_eq!(plans[0], plans[1]);
    assert_eq!(plans[0], plans[2]);
    assert_eq!(listener.connections(), 0);
}

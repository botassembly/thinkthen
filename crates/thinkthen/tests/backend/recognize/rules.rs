//! Ticket 0147, test 7: rules with no kind limits.

use super::{automatic, json, questions, run, stdout};
use crate::harness::{Listener, spawn};
use serde_json::Value;

const SPELLINGS: [&str; 3] = ["knows", "knows=*:*", "knows=ANY:ANY"];

/// The canonical question and its digest, as `--details` prints them.
fn question(listener: &Listener, options: &[&str]) -> (Value, Value) {
    let result = json(&run(
        listener,
        &[options, &["--details"]].concat(),
        b"Ada met Acme.",
    ));
    (
        result["question"].clone(),
        result["meta"]["question_sha256"].clone(),
    )
}

/// Every spelling gives what the first one gives.
fn one_answer<T: PartialEq + std::fmt::Debug>(each: impl Fn(&str) -> T) -> T {
    let [first, second, third] = SPELLINGS.map(each);
    assert_eq!((&second, &third), (&first, &first));
    first
}

#[test]
fn bare_star_and_any_rules_give_one_plan_and_one_digest() {
    let listener = Listener::answering(automatic).expect("listener");
    one_answer(|rule| {
        stdout(&run(
            &listener,
            &["person", "organization", "--relation", rule, "--dry-run"],
            b"Ada met Acme.",
        ))
    });
    let (canonical, _) =
        one_answer(|rule| question(&listener, &["person", "organization", "--relation", rule]));
    assert_eq!(
        canonical["relations"],
        serde_json::json!([{"name":"knows","source":"*","target":"*","reads":"knows","either":false}])
    );
}

/// The digest is sha256 of the canonical line
/// `{"verb":"recognize","kinds":{},"threshold":0.5,"relation_threshold":0.5}`.
#[test]
fn no_kinds_writes_the_canonical_empty_kinds_and_its_pinned_digest() {
    let listener = Listener::answering(automatic).expect("listener");
    let (canonical, digest) = question(&listener, &[]);
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
/// A rule that allows no pair sends no pair request and prints no edge.
#[test]
fn a_declined_name_enters_no_pair_under_a_bare_rule() {
    let listener = Listener::answering(automatic).expect("listener");
    let value = json(&run(
        &listener,
        &["person", "organization", "--relation", "knows"],
        b"Ada met Nobody and Acme.",
    ));
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
        *state,
        serde_json::json!({"evidence":"Ada met Nobody and Acme.","entities":[{"id":"i1","name":"Ada","kind":"person"},{"id":"i2","name":"Acme","kind":"organization"}]})
    );
    let none = json(&run(
        &listener,
        &[
            "person",
            "organization",
            "--relation",
            "visits=organization:organization",
        ],
        b"Ada met Acme.",
    ));
    assert_eq!(none["relations"], serde_json::json!([]));
    assert_eq!(listener.requests().len(), 2);
}

#[test]
fn relate_gives_one_plan_for_the_three_spellings() {
    let listener = Listener::answering(automatic).expect("listener");
    let names = br#"[{"name":"Ada","kind":"person"},{"name":"Acme","kind":"organization"}]"#;
    one_answer(|rule| {
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
    });
    assert_eq!(listener.connections(), 0);
}

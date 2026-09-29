//! Ticket 0147, test 7: rules with no kind limits.

use super::{automatic, json, profile, questions, run, stdout};
use crate::harness::{Canned, Gathering, Listener, spawn};
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
            &["person", "organization", "--relation", rule, "--plan"],
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
            "--plan",
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

/// Repeated mentions remain output spans, while each distinct name/kind is
/// asked once. Both rules share the one relevant state, and details retain a
/// pair that the relation cut removes from the bare value.
#[test]
fn repeated_names_and_two_rules_keep_one_pair_each_and_show_a_dropped_edge() {
    const TEXT: &[u8] = b"Ada met Acme, then Ada met Acme in Town.";
    let listener = Listener::answering(|body| {
        if String::from_utf8_lossy(body).contains("Does the text itself state that") {
            Canned::ok(r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.2}},"usage":{"input_tokens":10,"output_tokens":2}}"#)
        } else {
            automatic(body)
        }
    })
    .expect("listener");
    let result = json(&run(
        &listener,
        &[
            "person",
            "organization",
            "place",
            "--relation",
            "works_for=person:organization",
            "--relation",
            "knows=person:organization",
            "--details",
        ],
        TEXT,
    ));
    let requests = listener.requests();
    assert_eq!(requests.len(), 3, "one request per recognition stage");
    let body: Value = serde_json::from_slice(&requests[2].body).expect("relation request");
    assert_eq!(
        body,
        serde_json::json!({
            "state": {"evidence":"Ada met Acme, then Ada met Acme in Town.","entities":[
                {"id":"i1","name":"Ada","kind":"person"},
                {"id":"i2","name":"Acme","kind":"organization"}
            ]},
            "model":"local-1",
            "questions":{
                "q1":{"type":"noul","instructions":"Does the text itself state that i1 works for i2?"},
                "q2":{"type":"noul","instructions":"Does the text itself state that i1 knows i2?"}
            }
        })
    );
    assert_eq!(
        result["value"]["entities"],
        serde_json::json!([
            {"text":"Ada","start":0,"end":3,"length":3,"kind":"person","strength":0.9},
            {"text":"Acme","start":8,"end":12,"length":4,"kind":"organization","strength":0.9},
            {"text":"Ada","start":19,"end":22,"length":3,"kind":"person","strength":0.9},
            {"text":"Acme","start":27,"end":31,"length":4,"kind":"organization","strength":0.9},
            {"text":"Town","start":35,"end":39,"length":4,"kind":"place","strength":0.9}
        ])
    );
    assert_eq!(
        result["value"]["relations"],
        serde_json::json!([{"relation":"works_for",
            "source":{"text":"Ada","start":0,"end":3,"length":3,"kind":"person","strength":0.9},
            "target":{"text":"Acme","start":8,"end":12,"length":4,"kind":"organization","strength":0.9},
            "probability":0.9}])
    );
    assert_eq!(
        result["answer"]["pairs"],
        serde_json::json!([
            {"relation":"works_for","source":{"start":0,"end":3},"target":{"start":8,"end":12},"probability":0.9},
            {"relation":"knows","source":{"start":0,"end":3},"target":{"start":8,"end":12},"probability":0.2}
        ])
    );
    assert_eq!(result["meta"]["requests_sent"], 3);
}

/// A one-question profile makes the two rules separate chunks. Holding both
/// replies proves that a later rule enters within the same width, rather than
/// waiting for the earlier rule's response.
#[test]
fn split_two_rule_requests_overlap_within_width() {
    let one = profile("two-rule-overlap", r#""max_questions":1"#);
    let gathered = Gathering::new(2);
    let listener = Listener::answering(move |body| {
        if String::from_utf8_lossy(body).contains("Does the text itself state that") {
            gathered.hold();
        }
        automatic(body)
    })
    .expect("listener");
    let result = json(&run(
        &listener,
        &[
            "person",
            "organization",
            "--relation",
            "works_for=person:organization",
            "--relation",
            "knows=person:organization",
            "--profile",
            one.to_str().expect("profile path"),
            "--lines",
            "--jobs",
            "2",
            "--details",
        ],
        b"Ada met Acme.\n",
    ));
    assert_eq!(listener.peak(), 2);
    let relation = listener
        .requests()
        .into_iter()
        .filter(|request| {
            String::from_utf8_lossy(&request.body).contains("Does the text itself state that")
        })
        .collect::<Vec<_>>();
    assert_eq!(relation.len(), 2);
    let mut asked = relation
        .iter()
        .map(|request| {
            let questions = questions(&request.body);
            assert_eq!(questions.len(), 1);
            questions[0]["instructions"]
                .as_str()
                .expect("question")
                .to_owned()
        })
        .collect::<Vec<_>>();
    asked.sort();
    assert_eq!(
        asked,
        [
            "Does the text itself state that i1 knows i2?",
            "Does the text itself state that i1 works for i2?",
        ]
    );
    assert_eq!(
        result["value"]["relations"].as_array().map(Vec::len),
        Some(2)
    );
    assert_eq!(result["meta"]["requests_sent"], 9);
}

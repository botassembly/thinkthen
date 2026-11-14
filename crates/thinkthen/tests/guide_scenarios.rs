#![cfg(test)]
//! Designated guide scenarios through the public engine, on both wire formats.
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;
use loopback::{KEY, Loopback};
use serde_json::{Value, json};
use thinkthen::{BackendName, Engine, Key, OffsetUnit, Request, Settings};

fn run(backend: BackendName, scenario: &str, question: Value) -> Value {
    let server = Loopback::guide_scenario(scenario).unwrap();
    let engine = Settings::default()
        .with_backend(backend)
        .url(server.base())
        .api_key(Key::new(KEY).unwrap())
        .engine(|_| None)
        .unwrap();
    let row = request_row(&engine, question, "first\nsecond");
    assert!(server.count() > 0);
    row
}

fn request_row(engine: &Engine, question: Value, text: &str) -> Value {
    let request =
        Request::from_json(&json!({"question":question,"records":[text]}).to_string()).unwrap();
    let call = engine
        .open_request(&request, OffsetUnit::CodePoints)
        .unwrap();
    call.sender().close();
    let rows: Vec<_> = call
        .rows()
        .map(|row| serde_json::to_value(row.unwrap()).unwrap())
        .collect();
    assert_eq!(rows.len(), 1);
    rows.into_iter().next().unwrap()
}

#[test]
fn designated_states_answers_probabilities_and_errors_are_not_hash_answers() {
    for backend in [BackendName::SystemOne, BackendName::OpenAi] {
        for (scenario, status, probability, answer) in [
            ("yes", "answered", Some(0.95), Some(true)),
            ("no", "answered", Some(0.05), Some(false)),
            ("unsure", "unsure", Some(0.5), None),
            ("failed", "failed", None, None),
        ] {
            let row = run(
                backend,
                scenario,
                json!({"function":"decide","question":"Unchanged?","threshold":{"low":0.2,"high":0.8}}),
            );
            assert_eq!(row["status"], status);
            assert_eq!(row.get("answer"), answer.map(|v| json!(v)).as_ref());
            assert_eq!(
                row.get("probability"),
                probability.map(|v| json!(v)).as_ref()
            );
            if scenario == "failed" {
                assert_eq!(row["error"]["code"], "backend.status");
                assert_eq!(row["error"]["http_status"], 503);
            }
        }
        for (scenario, status) in [
            ("yes", "answered"),
            ("no", "answered"),
            ("unsure", "unsure"),
            ("failed", "failed"),
        ] {
            for function in ["choose", "find"] {
                let question = if function == "choose" {
                    json!({"function":"choose","question":"Unchanged?","choices":["billing","shipping","other"],"threshold":0.8})
                } else {
                    json!({"function":"find","question":"Unchanged?","threshold":0.8})
                };
                let row = run(backend, scenario, question);
                assert_eq!(
                    row["status"], status,
                    "{backend:?}/{scenario}/{function}: {row}"
                );
                if status == "answered" {
                    let expected = match (function, scenario) {
                        ("choose", "yes") => json!("billing"),
                        ("choose", _) => json!("shipping"),
                        (_, "yes") => json!({"text":"second","start":6,"end":12}),
                        _ => Value::Null,
                    };
                    assert_eq!(row["answer"], expected);
                    assert_eq!(row["probability"], 1.0);
                } else {
                    assert!(row.get("answer").is_none());
                    if status == "failed" {
                        assert_eq!(row["error"]["code"], "backend.status");
                        assert_eq!(row["error"]["http_status"], 503);
                    } else {
                        assert_eq!(row["probability"], 1.0 / 3.0);
                    }
                }
            }
        }
    }
}

#[test]
fn each_canonical_form_child_can_fail_or_be_unsure_without_affecting_siblings() {
    for backend in [BackendName::SystemOne, BackendName::OpenAi] {
        for (scenario, target, state) in [
            ("refund-failed", "refund", "failed"),
            ("topic-failed", "topic", "failed"),
            ("urgency-failed", "urgency", "failed"),
            ("refund-unsure", "refund", "unsure"),
            ("topic-unsure", "topic", "unsure"),
        ] {
            let row = run(
                backend,
                scenario,
                json!({"function":"annotate","questions":{
                    "refund":{"function":"decide","question":"Unchanged?","threshold":{"low":0.2,"high":0.8}},
                    "topic":{"function":"choose","question":"Unchanged?","choices":["billing","shipping","other"],"threshold":0.8},
                    "urgency":{"function":"score","question":"Unchanged?","levels":["routine","urgent"]}
                }}),
            );
            assert_eq!(row["status"], "answered", "{scenario}: {row}");
            for name in ["refund", "topic", "urgency"] {
                let child = &row["answer"][name];
                assert_eq!(
                    child["status"],
                    if name == target { state } else { "answered" },
                    "{backend:?}/{scenario}: {row}"
                );
                if name == target {
                    assert!(child.get("answer").is_none());
                    if state == "failed" {
                        assert_eq!(child["error"]["code"], "backend.reply");
                        assert!(child.get("probability").is_none());
                    } else {
                        let probability = if name == "refund" { 0.5 } else { 1.0 / 3.0 };
                        assert_eq!(child["probability"], probability);
                    }
                } else {
                    let expected = match name {
                        "refund" => json!(true),
                        "topic" => json!("billing"),
                        _ => json!(1.0),
                    };
                    assert_eq!(child["answer"], expected);
                    match name {
                        "refund" => assert_eq!(child["probability"], 0.95),
                        "topic" => assert_eq!(child["probability"], 1.0),
                        _ => assert!(child.get("probability").is_none()),
                    }
                }
            }
        }
    }
    assert!(Loopback::guide_scenario("typo").is_err());
}

#[test]
fn structural_relations_are_positive_on_both_formats_without_arrival_or_wording_rules() {
    for backend in [BackendName::SystemOne, BackendName::OpenAi] {
        let server = Loopback::guide_scenario("relations").unwrap();
        let engine = Settings::default()
            .with_backend(backend)
            .url(server.base())
            .api_key(Key::new(KEY).unwrap())
            .engine(|_| None)
            .unwrap();
        // Reuse one server: calls and internal recognition rounds arrive in
        // different orders. Renamed evidence and rule wording keep the structure.
        for (person, organization, reads, order) in [
            (
                "Morgan",
                "Example Group",
                "works for",
                ["recognize", "relate"],
            ),
            (
                "Taylor",
                "Another Group",
                "is employed by",
                ["relate", "recognize"],
            ),
            (
                "Morgan",
                "Example Group",
                "has an employer",
                ["relate", "recognize"],
            ),
        ] {
            let text = format!("{person} works for {organization}.\nPlease refund my order.");
            let names = json!([
                {"kind":"person","text":person,"start":0,"end":6,"probability":1.0},
                {"kind":"organization","text":organization,"start":17,"end":30,"probability":1.0}
            ]);
            for function in order {
                let question = if function == "recognize" {
                    json!({"function":"recognize","kinds":["person","organization"]})
                } else {
                    json!({"function":"relate","rules":[{
                        "source":"person","target":"organization","relation":"works_for","reads":reads
                    }]})
                };
                let row = request_row(&engine, question, &text);
                assert_eq!(row["status"], "answered", "{backend:?}: {row}");
                if function == "recognize" {
                    assert_eq!(row["answer"], names);
                } else {
                    let mut source = names[0].clone();
                    let mut target = names[1].clone();
                    source.as_object_mut().unwrap().remove("probability");
                    target.as_object_mut().unwrap().remove("probability");
                    assert_eq!(
                        row["answer"],
                        json!([{
                            "source":source,"target":target,"relation":"works_for","probability":0.95
                        }])
                    );
                    assert_eq!(row["detail"]["names"], names);
                }
            }
        }
        assert!(server.count() > 6);
    }
}

#[test]
fn low_score_is_targeted_without_changing_existing_yes_and_no_scales() {
    for backend in [BackendName::SystemOne, BackendName::OpenAi] {
        for (scenario, score) in [("yes", 1.0), ("no", 1.0), ("low-score", 0.0)] {
            let row = run(
                backend,
                scenario,
                json!({
                    "function":"annotate","questions":{
                        "refund":{"function":"decide","question":"Unchanged?"},
                        "topic":{"function":"choose","question":"Unchanged?","choices":["billing","shipping","other"]},
                        "urgency":{"function":"score","question":"Unchanged?","levels":["routine","urgent"]}
                    }
                }),
            );
            assert_eq!(row["status"], "answered");
            assert_eq!(row["answer"]["urgency"]["status"], "answered");
            assert_eq!(row["answer"]["urgency"]["answer"], score);
            if scenario == "low-score" {
                assert_eq!(row["answer"]["refund"]["answer"], true);
                assert_eq!(row["answer"]["topic"]["answer"], "billing");
            }
        }
    }
}

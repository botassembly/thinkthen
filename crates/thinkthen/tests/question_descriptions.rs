#![cfg(test)]
//! Answer meanings survive the shared question contract and backend request.
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;
use loopback::{KEY, Loopback};
use serde_json::{Value, json};
use thinkthen::{BackendName, Key, OffsetUnit, Question, Request, Settings};

#[test]
fn answer_meanings_roundtrip_and_reach_each_backend_without_renaming_answers() {
    for definition in [
        json!({"function":"decide","question":"Ready?","yes_means":"safe to proceed","no_means":"needs work","threshold":0.5}),
        json!({"function":"filter","question":"Ready?","yes_means":"safe to proceed","no_means":"needs work","threshold":0.5}),
        json!({"function":"choose","question":"Route?","choices":[{"name":"now","description":"safe to proceed"},"later"],"threshold":0.5}),
        json!({"function":"tag","question":"Flags?","labels":[{"name":"now","description":"safe to proceed"},"later"],"threshold":0.5}),
        json!({"function":"score","question":"Readiness?","levels":["low",{"name":"high","description":"safe to proceed"}]}),
    ] {
        use thinkthen::questions::{AnswerOption, Choose, Decide, Filter, Score, Tag};
        let described = || {
            AnswerOption::new("now")
                .unwrap()
                .description("safe to proceed")
        };
        let later = || AnswerOption::new("later").unwrap();
        let typed: Question = match definition["function"].as_str().unwrap() {
            "decide" => Decide::new("Ready?")
                .unwrap()
                .yes_means("safe to proceed")
                .no_means("needs work")
                .into(),
            "filter" => Filter::new("Ready?")
                .unwrap()
                .yes_means("safe to proceed")
                .no_means("needs work")
                .into(),
            "choose" => Choose::with_options("Route?", [described(), later()])
                .unwrap()
                .into(),
            "tag" => Tag::with_options("Flags?", [described(), later()])
                .unwrap()
                .into(),
            _ => Score::with_options(
                "Readiness?",
                [
                    AnswerOption::new("low").unwrap(),
                    AnswerOption::new("high")
                        .unwrap()
                        .description("safe to proceed"),
                ],
            )
            .unwrap()
            .into(),
        };
        assert_eq!(serde_json::to_value(&typed).unwrap(), definition);
        let question = Question::from_json(&definition.to_string()).unwrap();
        assert_eq!(serde_json::to_value(&question).unwrap(), definition);
        let request_json = json!({"question":definition,"records":["evidence"]});
        let schema: Value = serde_json::to_value(schemars::schema_for!(Request)).unwrap();
        assert!(jsonschema::is_valid(&schema, &request_json));
        let request = Request::from_json(&request_json.to_string()).unwrap();
        let changed = definition
            .to_string()
            .replace("safe to proceed", "must wait");
        let engine = Settings::default()
            .with_backend(BackendName::Stub)
            .engine(|_| None)
            .unwrap();
        let mut identities = Vec::new();
        for question in [definition.clone(), serde_json::from_str(&changed).unwrap()] {
            let request = Request::from_json(
                &json!({"question":question,"records":["evidence"]}).to_string(),
            )
            .unwrap();
            let rows = engine
                .open_request(&request, OffsetUnit::CodePoints)
                .unwrap()
                .rows()
                .collect::<Result<Vec<_>, _>>()
                .unwrap();
            identities.push(rows[0].source().question_identity().to_owned());
        }
        assert_ne!(identities[0], identities[1]);

        for backend in [BackendName::SystemOne, BackendName::OpenAi] {
            let rules = match definition["function"].as_str().unwrap() {
                "decide" | "filter" | "tag" if backend == BackendName::OpenAi => json!([
                    {"criterion":"true","distribution":{"true":0.9,"false":0.1}},
                    {"probability":0.9}
                ]),
                "decide" | "filter" | "tag" => json!([{"probability":0.9}]),
                "choose" => json!([{"distribution":{"now":0.9,"later":0.1}}]),
                _ => json!([{"distribution":{"low":0.1,"high":0.9}}]),
            };
            let server = Loopback::case(&json!({"replies":rules})).unwrap();
            let engine = Settings::default()
                .with_backend(backend)
                .url(server.base())
                .api_key(Key::new(KEY).unwrap())
                .engine(|_| None)
                .unwrap();
            let rows = engine
                .open_request(&request, OffsetUnit::CodePoints)
                .unwrap()
                .rows()
                .collect::<Result<Vec<_>, _>>()
                .unwrap();
            assert_eq!(rows.len(), 1);
            let row = serde_json::to_value(&rows[0]).unwrap();
            let expected = match definition["function"].as_str().unwrap() {
                "decide" | "filter" => json!(true),
                "choose" => json!("now"),
                "tag" => json!(["now", "later"]),
                _ => json!(0.9),
            };
            assert_eq!(row["answer"], expected, "{row}");
            let requests = server.requests();
            assert_eq!(requests.len(), 1);
            let body = requests[0].json().unwrap();
            let first = if backend == BackendName::SystemOne {
                &body["questions"]["q1"]
            } else {
                &body["questions"][0]
            };
            match definition["function"].as_str().unwrap() {
                "decide" | "filter" | "tag" => {
                    if backend == BackendName::SystemOne {
                        assert_eq!(first["criteria"]["true"], "safe to proceed");
                    } else {
                        assert_eq!(first["choices"][0]["description"], "safe to proceed");
                    }
                }
                "choose" => {
                    if backend == BackendName::SystemOne {
                        assert_eq!(first["criteria"]["now"], "safe to proceed");
                        assert!(first["criteria"]["later"].is_null());
                    } else {
                        assert_eq!(first["choices"][0]["value"], "now");
                        assert_eq!(first["choices"][0]["description"], "safe to proceed");
                        assert!(first["choices"][1].get("description").is_none());
                    }
                }
                _ => {
                    assert!(
                        first["instructions"]
                            .as_str()
                            .unwrap()
                            .contains("Level \"high\" means: \"safe to proceed\"")
                    );
                    if backend == BackendName::SystemOne {
                        assert_eq!(first["criteria"], json!(["low", "high"]));
                    } else {
                        assert_eq!(first["levels"], json!([{"label":"low"},{"label":"high"}]));
                    }
                }
            }
            let wire = serde_json::to_string(&body).unwrap();
            assert!(wire.contains("safe to proceed"), "{wire}");
            if matches!(definition["function"].as_str(), Some("decide" | "filter")) {
                assert!(wire.contains("needs work"));
            }
        }
    }
}

#[test]
fn plain_domains_keep_their_serialized_shape_and_rich_names_remain_strict() {
    for (function, domain) in [
        ("choose", "choices"),
        ("tag", "labels"),
        ("score", "levels"),
        ("rank", "levels"),
    ] {
        let mut plain = json!({"function":function,"question":"Ready?"});
        plain[domain] = json!(["low", "high"]);
        if matches!(function, "choose" | "tag") {
            plain["threshold"] = json!(0.5);
        }
        let question = Question::from_json(&plain.to_string()).unwrap();
        assert_eq!(serde_json::to_value(question).unwrap(), plain);
        plain[domain] = json!([{"name":"bad\u{0000}name","description":"meaning"}, "other"]);
        assert_eq!(
            Question::from_json(&plain.to_string())
                .unwrap_err()
                .code()
                .as_str(),
            "usage.name"
        );
        for invalid in [
            json!({"name":"low"}),
            json!({"description":"meaning"}),
            json!({"name":7,"description":"meaning"}),
            json!({"name":"low","description":7}),
            json!({"name":"low","description":"meaning","extra":true}),
            json!({"name":"low","description":"duplicate name"}),
        ] {
            plain[domain] = json!(["low", invalid]);
            assert!(Question::from_json(&plain.to_string()).is_err(), "{plain}");
        }
    }
}

#[test]
fn rank_level_descriptions_roundtrip_and_reach_both_backends() {
    let definition = json!({"function":"rank","question":"Rank priority?","levels":["low",{"name":"high","description":"Urgent"}]});
    let question = Question::from_json(&definition.to_string()).unwrap();
    assert_eq!(serde_json::to_value(question).unwrap(), definition);
    let typed = thinkthen::Rank::with_options(
        "Rank priority?",
        [
            thinkthen::questions::AnswerOption::new("low").unwrap(),
            thinkthen::questions::AnswerOption::new("high")
                .unwrap()
                .description("Urgent"),
        ],
    )
    .unwrap();
    assert_eq!(
        serde_json::to_value(Question::from(typed)).unwrap(),
        definition
    );
    let request_json = json!({"question":definition,"records":["evidence"]});
    let schema: Value = serde_json::to_value(schemars::schema_for!(Request)).unwrap();
    assert!(jsonschema::is_valid(&schema, &request_json));
    let request = Request::from_json(&request_json.to_string()).unwrap();
    for backend in [BackendName::SystemOne, BackendName::OpenAi] {
        let server =
            Loopback::case(&json!({"replies":[{"distribution":{"low":0.1,"high":0.9}}]})).unwrap();
        let engine = Settings::default()
            .with_backend(backend)
            .url(server.base())
            .api_key(Key::new(KEY).unwrap())
            .engine(|_| None)
            .unwrap();
        let rows = engine
            .open_request(&request, OffsetUnit::CodePoints)
            .unwrap()
            .rows()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(serde_json::to_value(&rows[0]).unwrap()["answer"], 1);
        let requests = server.requests();
        assert_eq!(requests.len(), 1);
        let body = requests[0].json().unwrap();
        let question = if backend == BackendName::SystemOne {
            &body["questions"]["q1"]
        } else {
            &body["questions"][0]
        };
        assert!(
            question["instructions"]
                .as_str()
                .unwrap()
                .contains("Level \"high\" means: \"Urgent\"")
        );
    }
}

#[cfg(feature = "native")]
#[test]
fn native_question_snapshots_keep_filter_meanings_and_rank_descriptions() {
    for definition in [
        json!({"function":"filter","question":"Ready?","yes_means":"Keep refund requests","no_means":"Omit other messages","threshold":0.5}),
        json!({"function":"rank","question":"Priority?","levels":["low",{"name":"high","description":"Urgent"}],"top":2}),
    ] {
        let question = Question::from_json(&definition.to_string()).unwrap();
        let builder = thinkthen::native::QuestionBuilder::snapshot(&question);
        let rebuilt = builder.build().unwrap();
        assert_eq!(serde_json::to_value(&rebuilt).unwrap(), definition);
        let snapshot = thinkthen::native::QuestionBuilder::snapshot(&rebuilt);
        assert_eq!(
            serde_json::to_value(snapshot.build().unwrap()).unwrap(),
            definition
        );
    }
}

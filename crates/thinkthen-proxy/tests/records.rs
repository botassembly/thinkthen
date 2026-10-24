#![cfg(test)]
//! Public service records, typed admission and competing correction proofs.

use serde_json::json;
use thinkthen::{Answer, Backend, Cancel, Engine, SdkRequest};
use thinkthen_proxy::{ActorKind, Attribution, Evidence, EvidenceId, GoldId, QuestionId, Service};

fn request(function: &str, question: serde_json::Value, options: serde_json::Value) -> String {
    json!({"function":function,"question":{"type":"inline","value":question},"options":options})
        .to_string()
}
fn author(kind: ActorKind) -> Attribution {
    Attribution {
        actor: "curator".into(),
        kind,
        reason: "Review the original evidence".into(),
    }
}

#[test]
fn essential_asks_and_explicit_wordings_keep_gold_history() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("records.sqlite");
    let mut service = Service::open(&path).unwrap();
    let refund_request = request("decide", json!("Is a refund requested?"), json!({}));
    let refund = service
        .admit_question("Customer requests money back", &refund_request)
        .unwrap();
    let shipping = service
        .admit_question(
            "Customer asks about parcel delivery",
            &request("decide", json!("Is this about shipping?"), json!({})),
        )
        .unwrap();
    assert_ne!(refund.question, shipping.question);
    let equivalent = request(
        "decide",
        json!("Does the customer want their money returned?"),
        json!({"threshold":0.9}),
    );
    let wording = service
        .attach_wording(&refund.question, &equivalent)
        .unwrap();
    assert_ne!(refund.wording, wording);
    assert_eq!(
        service.wording(&wording).unwrap().unwrap()[0],
        json!(refund.question)
    );
    let changed_threshold = request(
        "decide",
        json!("Is a refund requested?"),
        json!({"threshold":0.7}),
    );
    assert_eq!(
        service
            .attach_wording(&refund.question, &changed_threshold)
            .unwrap(),
        refund.wording
    );
    assert_ne!(
        service.admit_inline(&equivalent).unwrap().question,
        service.admit_inline(&refund_request).unwrap().question
    );
    let evidence = service
        .admit_evidence(&Evidence::Text("Please return my money".into()))
        .unwrap();
    let original = service
        .set_gold(
            &refund.question,
            &evidence,
            None,
            &Answer::Bool(false),
            author(ActorKind::Agent),
        )
        .unwrap();
    let corrected = service
        .set_gold(
            &refund.question,
            &evidence,
            Some(original.id),
            &Answer::Bool(true),
            author(ActorKind::Person),
        )
        .unwrap();
    assert_eq!(corrected.supersedes, Some(original.id));
    assert_eq!(
        service
            .set_gold(
                &refund.question,
                &evidence,
                Some(original.id),
                &Answer::Bool(false),
                author(ActorKind::Agent)
            )
            .unwrap_err()
            .code(),
        "proxy.conflict"
    );
    drop(service);
    let service = Service::open(path).unwrap();
    assert_eq!(
        service
            .current_gold(&refund.question, &evidence)
            .unwrap()
            .unwrap()
            .id,
        corrected.id
    );
    assert_eq!(service.gold(original.id).unwrap().unwrap().answer, false);
    let history = service
        .gold_history(&refund.question, &evidence, None)
        .unwrap();
    assert_eq!(history.len(), 2);
    assert_eq!(history[0].attribution.kind, ActorKind::Agent);
    assert_eq!(history[1].attribution.kind, ActorKind::Person);
    assert!(
        service
            .current_gold(&shipping.question, &evidence)
            .unwrap()
            .is_none()
    );
    let edited = service
        .admit_evidence(&Evidence::Text("Please return my money!".into()))
        .unwrap();
    assert!(
        service
            .current_gold(&refund.question, &edited)
            .unwrap()
            .is_none()
    );
    assert_eq!(
        service.question(&refund.question).unwrap().unwrap()[0],
        "Customer requests money back"
    );
}

#[test]
fn canonical_domains_and_evidence_preserve_meaning() {
    let directory = tempfile::tempdir().unwrap();
    let mut service = Service::open(directory.path().join("records.sqlite")).unwrap();
    let first = service
        .admit_question(
            "Message topic",
            &request(
                "choose",
                json!("Topic?"),
                json!({"options":["billing", "shipping"]}),
            ),
        )
        .unwrap();
    let reordered = service
        .admit_question(
            "Message topic",
            &request(
                "choose",
                json!("Pick topic"),
                json!({"options":["shipping", "billing"]}),
            ),
        )
        .unwrap();
    assert_eq!(first.question, reordered.question);
    assert_ne!(first.wording, reordered.wording);
    assert_eq!(
        service
            .attach_wording(
                &first.question,
                &request(
                    "choose",
                    json!("Topic?"),
                    json!({"options":["billing", "other"]})
                )
            )
            .unwrap_err()
            .code(),
        "proxy.question"
    );
    let a = service
        .admit_evidence(&Evidence::Json(
            serde_json::from_str(r#"{"b":[1,2],"a":"x"}"#).unwrap(),
        ))
        .unwrap();
    let b = service
        .admit_evidence(&Evidence::Json(
            serde_json::from_str(r#"{"a":"x","b":[1,2]}"#).unwrap(),
        ))
        .unwrap();
    assert_eq!(a, b);
    assert_ne!(
        a,
        service
            .admit_evidence(&Evidence::Json(json!({"a":"x","b":[2,1]})))
            .unwrap()
    );
    assert_ne!(
        service.admit_evidence(&Evidence::Text("x".into())).unwrap(),
        service.admit_evidence(&Evidence::Json(json!("x"))).unwrap()
    );
    assert_eq!(
        service.evidence(&a).unwrap().unwrap()["value"]["b"],
        json!([1, 2])
    );
    let forward = service
        .admit_question(
            "Urgency",
            &request("score", json!("Urgent?"), json!({"levels":["low","high"]})),
        )
        .unwrap();
    let reversed = service
        .admit_question(
            "Urgency",
            &request("score", json!("Urgent?"), json!({"levels":["high","low"]})),
        )
        .unwrap();
    assert_ne!(forward.question, reversed.question);
}

#[test]
fn gold_uses_the_shared_typed_function_contract() {
    use thinkthen::{Found, Name, Outcome, Probability, Relation, RelationName};
    let directory = tempfile::tempdir().unwrap();
    let mut service = Service::open(directory.path().join("records.sqlite")).unwrap();
    let evidence = service
        .admit_evidence(&Evidence::Text("Ana met Bob".into()))
        .unwrap();
    let p = Probability::new(1.0).unwrap();
    let name = Name {
        text: "Ana".into(),
        start: 0,
        end: 3,
        kind: "PERSON".into(),
        p,
    };
    let endpoint = |text: &str, start, end| RelationName {
        text: text.into(),
        start,
        end,
        kind: "PERSON".into(),
    };
    let relation = Relation {
        source: endpoint("Ana", 0, 3),
        target: endpoint("Bob", 8, 11),
        relation: "met".into(),
        p,
        either: false,
    };
    let cases = [
        (
            "decide",
            json!("Met?"),
            json!({}),
            Answer::Bool(true),
            Answer::Label("yes".into()),
        ),
        (
            "filter",
            json!("Met?"),
            json!({}),
            Answer::Bool(true),
            Answer::Position(1),
        ),
        (
            "choose",
            json!("Who?"),
            json!({"options":["Ana","Bob"]}),
            Answer::Label("Ana".into()),
            Answer::Label("Cal".into()),
        ),
        (
            "tag",
            json!("Who?"),
            json!({"labels":["Ana","Bob"]}),
            Answer::Labels(vec!["Ana".into()]),
            Answer::Labels(vec!["Cal".into()]),
        ),
        (
            "score",
            json!("Related?"),
            json!({"levels":["no","yes"]}),
            Answer::Number(0.5),
            Answer::Number(f64::NAN),
        ),
        (
            "rank",
            json!("Related?"),
            json!({}),
            Answer::Position(1),
            Answer::Position(0),
        ),
        (
            "find",
            json!("Who met?"),
            json!({}),
            Answer::Found(Some(Found {
                text: "Ana met Bob".into(),
                start: 0,
                end: 11,
            })),
            Answer::Found(Some(Found {
                text: "wrong".into(),
                start: 0,
                end: 3,
            })),
        ),
        (
            "recognize",
            json!(["PERSON"]),
            json!({}),
            Answer::Names(vec![name.clone()]),
            Answer::Names(vec![Name {
                kind: "PLACE".into(),
                ..name
            }]),
        ),
        (
            "relate",
            json!([{"name":"met","source":"PERSON","target":"PERSON"}]),
            json!({}),
            Answer::Relations(vec![relation.clone()]),
            Answer::Relations(vec![Relation {
                relation: "owns".into(),
                ..relation
            }]),
        ),
        (
            "annotate",
            json!({"questions":{"member":{"decide":"Met?"}}}),
            json!({}),
            Answer::Annotations(std::collections::BTreeMap::from([(
                "member".into(),
                Outcome::Answered {
                    answer: Answer::Bool(true),
                    p: Some(p),
                    detail: None,
                },
            )])),
            Answer::Annotations(Default::default()),
        ),
    ];
    for (function, question, options, valid, invalid) in cases {
        let request = request(function, question, options);
        let shared = SdkRequest::from_json(&request)
            .unwrap()
            .prepare(&Engine::new(Backend::stub()), Cancel::default())
            .unwrap()
            .definition()
            .unwrap();
        if function == "annotate" {
            assert!(shared.validate_answer(&valid, "Ana met Bob").is_ok());
            assert!(shared.validate_answer(&invalid, "Ana met Bob").is_err());
            assert_eq!(
                service
                    .admit_question("Composite annotation", &request)
                    .unwrap_err()
                    .code(),
                "proxy.question"
            );
            continue;
        }
        let admitted = service
            .admit_question("Judge this evidence", &request)
            .unwrap();
        let additional_invalid = match &valid {
            Answer::Number(_) => vec![
                Answer::Number(-0.1),
                Answer::Number(1.1),
                Answer::Number(f64::INFINITY),
            ],
            Answer::Labels(labels) => vec![Answer::Labels(
                labels.iter().chain(labels).cloned().collect(),
            )],
            Answer::Found(_) => vec![Answer::Found(Some(Found {
                text: "Ana".into(),
                start: 0,
                end: 3,
            }))],
            Answer::Names(names) => {
                let ana = names[0].clone();
                let bob = Name {
                    text: "Bob".into(),
                    start: 8,
                    end: 11,
                    ..ana.clone()
                };
                vec![
                    Answer::Names(vec![ana.clone(), ana.clone()]),
                    Answer::Names(vec![bob, ana]),
                ]
            }
            Answer::Relations(relations) => {
                let relation = relations[0].clone();
                vec![
                    Answer::Relations(vec![relation.clone(), relation.clone()]),
                    Answer::Relations(vec![Relation {
                        target: relation.source.clone(),
                        ..relation
                    }]),
                ]
            }
            _ => Vec::new(),
        };
        for invalid in additional_invalid {
            assert_eq!(
                service
                    .set_gold(
                        &admitted.question,
                        &evidence,
                        None,
                        &invalid,
                        author(ActorKind::Person)
                    )
                    .unwrap_err()
                    .code(),
                "proxy.question",
                "{function}"
            );
        }
        assert!(
            shared.validate_answer(&valid, "Ana met Bob").is_ok(),
            "{function}"
        );
        assert!(
            shared.validate_answer(&invalid, "Ana met Bob").is_err(),
            "{function}"
        );
        assert_eq!(
            service
                .set_gold(
                    &admitted.question,
                    &evidence,
                    None,
                    &invalid,
                    author(ActorKind::Person)
                )
                .unwrap_err()
                .code(),
            "proxy.question",
            "{function}"
        );
        assert!(
            service
                .current_gold(&admitted.question, &evidence)
                .unwrap()
                .is_none()
        );
        service
            .set_gold(
                &admitted.question,
                &evidence,
                None,
                &valid,
                author(ActorKind::Person),
            )
            .unwrap();
    }
    // Unordered relation rules permit reversed endpoint kinds in text order.
    let unordered = service
        .admit_question(
            "Meeting",
            &request(
                "relate",
                json!([{"name":"met","source":"PERSON","target":"PLACE","either":true}]),
                json!({}),
            ),
        )
        .unwrap();
    let place = service
        .admit_evidence(&Evidence::Text("Paris met Ana".into()))
        .unwrap();
    let reversed = Answer::Relations(vec![Relation {
        source: RelationName {
            text: "Paris".into(),
            start: 0,
            end: 5,
            kind: "PLACE".into(),
        },
        target: RelationName {
            text: "Ana".into(),
            start: 10,
            end: 13,
            kind: "PERSON".into(),
        },
        relation: "met".into(),
        p,
        either: true,
    }]);
    service
        .set_gold(
            &unordered.question,
            &place,
            None,
            &reversed,
            author(ActorKind::Person),
        )
        .unwrap();
    if let Answer::Relations(relations) = &reversed {
        let first = relations[0].clone();
        let mirrored = Relation {
            source: first.target.clone(),
            target: first.source.clone(),
            ..first.clone()
        };
        assert_eq!(
            service
                .set_gold(
                    &unordered.question,
                    &place,
                    None,
                    &Answer::Relations(vec![first, mirrored]),
                    author(ActorKind::Person)
                )
                .unwrap_err()
                .code(),
            "proxy.question"
        );
    }
    let find = service
        .admit_question(
            "Find a name",
            &request("find", json!("Name?"), json!({"on":"/body"})),
        )
        .unwrap();
    let json_string = service
        .admit_evidence(&Evidence::Json(json!("Ana")))
        .unwrap();
    service
        .set_gold(
            &find.question,
            &json_string,
            None,
            &Answer::Found(Some(Found {
                text: "Ana".into(),
                start: 0,
                end: 3,
            })),
            author(ActorKind::Person),
        )
        .unwrap();
    let selected_object = json!({"id":"Ana","body":"Bob"});
    let selected_text = selected_object.to_string();
    let selected = service
        .admit_evidence(&Evidence::Json(selected_object))
        .unwrap();
    service
        .set_gold(
            &find.question,
            &selected,
            None,
            &Answer::Found(Some(Found {
                text: selected_text.clone(),
                start: 0,
                end: selected_text.chars().count(),
            })),
            author(ActorKind::Person),
        )
        .unwrap();
}

#[test]
fn competing_process_corrections_append_only_one_successor() {
    use std::io::{Read, Write};
    use std::process::{Command, Stdio};
    if let Ok(fixture) = std::env::var("THINKTHEN_CORRECTION_FIXTURE") {
        let (path, question, evidence, expected, result): (
            String,
            QuestionId,
            EvidenceId,
            GoldId,
            String,
        ) = serde_json::from_str(&fixture).unwrap();
        let mut service = Service::open(path).unwrap();
        std::io::stdin().read_exact(&mut [0u8]).unwrap();
        let outcome = service.set_gold(
            &question,
            &evidence,
            Some(expected),
            &Answer::Bool(true),
            author(ActorKind::Person),
        );
        std::fs::write(
            result,
            match outcome {
                Ok(_) => "accepted",
                Err(error) => error.code(),
            },
        )
        .unwrap();
        return;
    }
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("records.sqlite");
    let mut service = Service::open(&path).unwrap();
    let admission = service
        .admit_question(
            "Refund request",
            &request("decide", json!("Refund?"), json!({})),
        )
        .unwrap();
    let evidence = service
        .admit_evidence(&Evidence::Text("Refund please".into()))
        .unwrap();
    let gold = service
        .set_gold(
            &admission.question,
            &evidence,
            None,
            &Answer::Bool(false),
            author(ActorKind::Person),
        )
        .unwrap();
    let mut workers = Vec::new();
    for index in 0..2 {
        let result = directory.path().join(format!("result-{index}"));
        let fixture = json!([path, admission.question, evidence, gold.id, result]).to_string();
        let child = Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "competing_process_corrections_append_only_one_successor",
            ])
            .env("THINKTHEN_CORRECTION_FIXTURE", fixture)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        workers.push((child, result));
    }
    for (child, _) in &mut workers {
        child.stdin.take().unwrap().write_all(b"x").unwrap();
    }
    let mut outcomes = Vec::new();
    for (child, result) in workers {
        let output = child.wait_with_output().unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        outcomes.push(std::fs::read_to_string(result).unwrap());
    }
    outcomes.sort();
    assert_eq!(outcomes, ["accepted", "proxy.conflict"]);
    assert_eq!(
        service
            .gold_history(&admission.question, &evidence, None)
            .unwrap()
            .len(),
        2
    );
}

#[test]
fn record_bounds_and_history_cursor_bound_storage_work() {
    use thinkthen_proxy::{HISTORY_PAGE, RECORD_BYTES};
    let directory = tempfile::tempdir().unwrap();
    let mut service = Service::open(directory.path().join("records.sqlite")).unwrap();
    let request = request("decide", json!("Refund?"), json!({}));
    assert_eq!(
        service.admit_question("", &request).unwrap_err().code(),
        "proxy.question"
    );
    assert_eq!(
        service
            .admit_question(&"x".repeat(RECORD_BYTES + 1), &request)
            .unwrap_err()
            .code(),
        "proxy.storage"
    );
    assert_eq!(
        service
            .admit_question("Refund", &"x".repeat(RECORD_BYTES + 1))
            .unwrap_err()
            .code(),
        "proxy.storage"
    );
    assert_eq!(
        service
            .admit_evidence(&Evidence::Text("x".repeat(RECORD_BYTES + 1)))
            .unwrap_err()
            .code(),
        "proxy.storage"
    );
    let admission = service.admit_question("Refund", &request).unwrap();
    let evidence = service
        .admit_evidence(&Evidence::Text("Refund please".into()))
        .unwrap();
    let mut current = None;
    for index in 0..=HISTORY_PAGE {
        current = Some(
            service
                .set_gold(
                    &admission.question,
                    &evidence,
                    current,
                    &Answer::Bool(index % 2 == 0),
                    author(ActorKind::Person),
                )
                .unwrap()
                .id,
        );
    }
    let first = service
        .gold_history(&admission.question, &evidence, None)
        .unwrap();
    assert_eq!(first.len(), HISTORY_PAGE);
    let second = service
        .gold_history(
            &admission.question,
            &evidence,
            Some(first.last().unwrap().id),
        )
        .unwrap();
    assert_eq!(second.len(), 1);
    assert_eq!(Some(second[0].id), current);
}

#![cfg(test)]
//! Public service records, typed admission and competing correction proofs.

use serde_json::json;
use thinkthen_proxy::{ActorKind, Attribution, Evidence, EvidenceId, GoldId, QuestionId, Service};

fn request(function: &str, question: serde_json::Value, options: serde_json::Value) -> String {
    let mut fields = options.as_object().unwrap().clone();
    if let Some(value) = fields.remove("options") {
        fields.insert("choices".into(), value);
    }
    if let Some(value) = fields.remove("on") {
        fields.insert("evidence".into(), value);
    }
    fields.insert("function".into(), json!(function));
    match function {
        "recognize" => {
            fields.insert("kinds".into(), question);
        }
        "relate" => {
            fields.insert("rules".into(), question);
        }
        _ => {
            fields.insert("question".into(), question);
        }
    }
    json!({"question":fields}).to_string()
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
            &json!(false),
            author(ActorKind::Agent),
        )
        .unwrap();
    let corrected = service
        .set_gold(
            &refund.question,
            &evidence,
            Some(original.id),
            &json!(true),
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
                &json!(false),
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
    let directory = tempfile::tempdir().unwrap();
    let mut service = Service::open(directory.path().join("records.sqlite")).unwrap();
    let evidence = service
        .admit_evidence(&Evidence::Text("Ana met Bob".into()))
        .unwrap();
    let ana = json!({"text":"Ana","start":0,"end":3,"kind":"PERSON","probability":1.0});
    let bob = json!({"text":"Bob","start":8,"end":11,"kind":"PERSON","probability":1.0});
    let source = json!({"text":"Ana","start":0,"end":3,"kind":"PERSON"});
    let target = json!({"text":"Bob","start":8,"end":11,"kind":"PERSON"});
    let relation = json!({"source":source,"target":target,"relation":"met","probability":1.0});
    let mut wrong_name = ana.clone();
    wrong_name["kind"] = json!("PLACE");
    let mut wrong_relation = relation.clone();
    wrong_relation["relation"] = json!("owns");
    let mut self_relation = relation.clone();
    self_relation["target"] = source.clone();
    let cases = [
        (
            "decide",
            json!("Met?"),
            json!({}),
            json!(true),
            vec![json!("yes")],
        ),
        (
            "filter",
            json!("Met?"),
            json!({}),
            json!(true),
            vec![json!(1)],
        ),
        (
            "choose",
            json!("Who?"),
            json!({"choices":["Ana","Bob"]}),
            json!("Ana"),
            vec![json!("Cal")],
        ),
        (
            "tag",
            json!("Who?"),
            json!({"labels":["Ana","Bob"]}),
            json!(["Ana"]),
            vec![json!(["Cal"]), json!(["Ana", "Ana"])],
        ),
        (
            "score",
            json!("Related?"),
            json!({"levels":["no","yes"]}),
            json!(0.5),
            vec![json!(null), json!(-0.1), json!(1.1)],
        ),
        (
            "rank",
            json!("Related?"),
            json!({"levels":["no","yes"]}),
            json!(1),
            vec![json!(0)],
        ),
        (
            "find",
            json!("Who met?"),
            json!({}),
            json!({"text":"Ana met Bob","start":0,"end":11}),
            vec![
                json!({"text":"wrong","start":0,"end":3}),
                json!({"text":"Ana","start":0,"end":3}),
            ],
        ),
        (
            "recognize",
            json!(["PERSON"]),
            json!({}),
            json!([ana]),
            vec![json!([wrong_name]), json!([ana, ana]), json!([bob, ana])],
        ),
        (
            "relate",
            json!([{"relation":"met","source":"PERSON","target":"PERSON"}]),
            json!({}),
            json!([relation]),
            vec![
                json!([wrong_relation]),
                json!([relation, relation]),
                json!([self_relation]),
            ],
        ),
    ];
    for (function, question, options, valid, invalids) in cases {
        let request = request(function, question, options);
        let shared = thinkthen::Request::from_json(&request).unwrap();
        let admitted = service
            .admit_question("Judge this evidence", &request)
            .unwrap();
        for invalid in invalids {
            assert!(
                thinkthen::native::validate_recording_answer(
                    shared.question(),
                    &invalid.to_string(),
                    "Ana met Bob"
                )
                .is_err(),
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
        }
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
    let score = thinkthen::Request::from_json(&request(
        "score",
        json!("Related?"),
        json!({"levels":["no","yes"]}),
    ))
    .unwrap();
    for invalid in ["NaN", "Infinity", "-Infinity"] {
        assert!(
            thinkthen::native::validate_recording_answer(score.question(), invalid, "Ana met Bob")
                .is_err()
        );
    }
    assert!(service.admit_question("Composite annotation", &json!({"question":{"function":"annotate","questions":{"member":{"function":"decide","question":"Met?"}}}}).to_string()).is_err());
    let unordered = service
        .admit_question(
            "Meeting",
            &request(
                "relate",
                json!([{"relation":"met","source":"PERSON","target":"PLACE","either":true}]),
                json!({}),
            ),
        )
        .unwrap();
    let place = service
        .admit_evidence(&Evidence::Text("Paris met Ana".into()))
        .unwrap();
    let first = json!({"source":{"text":"Paris","start":0,"end":5,"kind":"PLACE"},"target":{"text":"Ana","start":10,"end":13,"kind":"PERSON"},"relation":"met","probability":1.0,"either":true});
    service
        .set_gold(
            &unordered.question,
            &place,
            None,
            &json!([first]),
            author(ActorKind::Person),
        )
        .unwrap();
    let mut mirrored = first.clone();
    mirrored["source"] = first["target"].clone();
    mirrored["target"] = first["source"].clone();
    assert_eq!(
        service
            .set_gold(
                &unordered.question,
                &place,
                None,
                &json!([first, mirrored]),
                author(ActorKind::Person)
            )
            .unwrap_err()
            .code(),
        "proxy.question"
    );
    let find = service
        .admit_question(
            "Find a name",
            &request("find", json!("Name?"), json!({"evidence":"/body"})),
        )
        .unwrap();
    for selected in [json!("Ana"), json!({"id":"Ana","body":"Bob"})] {
        let text = selected
            .as_str()
            .map_or_else(|| selected.to_string(), str::to_owned);
        let evidence = service.admit_evidence(&Evidence::Json(selected)).unwrap();
        service
            .set_gold(
                &find.question,
                &evidence,
                None,
                &json!({"text":text,"start":0,"end":text.chars().count()}),
                author(ActorKind::Person),
            )
            .unwrap();
    }
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
            &json!(true),
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
            &json!(false),
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
                    &json!(index % 2 == 0),
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

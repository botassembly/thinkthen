//! Shared context controls through the released aggregate and scalar calls.

use super::*;

#[test]
fn ineligible_calls_refuse_shared_context_before_a_send() {
    let _serial = serial();
    let listener = Listener::answering(|_| Canned::ok(DECIDED)).expect("listener");
    let engine = engine(listener.base());
    let one = question();
    let options = CallOptions::new().context("shared evidence");
    let denied = [
        (
            "decide",
            engine.decide_with(&one, "Ada", options).map(|_| ()),
        ),
        (
            "details",
            engine.details_with(&one, "Ada", options).map(|_| ()),
        ),
    ];
    for (name, result) in denied {
        let error = result.expect_err(name);
        assert_eq!(error.kind(), ErrorKind::Usage, "{name}");
        assert_eq!(
            error.to_string(),
            "a single-document call does not take a shared context",
            "{name}"
        );
        assert!(error.facts().is_none(), "{name} never started");
    }
    assert_eq!(listener.count(), 0);
}

#[test]
fn find_recognize_and_relate_keep_shared_context_separate_from_evidence() {
    let _serial = serial();
    let replies = AtomicUsize::new(0);
    let listener = Listener::answering(move |_| match replies.fetch_add(1, Ordering::Relaxed) {
        0 => Canned::ok(r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"choice","probabilities":{"u001":0.9,"u002":0.1}}}}"#),
        1 => Canned::ok(r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"choice","probabilities":{"BEGIN":0,"INSIDE":0,"END":0,"SINGLE":0,"OUT":1}}}}"#),
        2 => Canned::ok(r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.9}}}"#),
        _ => panic!("unexpected aggregate send"),
    })
    .expect("listener");
    let engine = engine(listener.base());
    let options = CallOptions::new().context("shared evidence");
    let find = Question::find("Which asks for a refund?").expect("find");
    let found = engine
        .find_with(&find, ["Ada", "Acme"], options)
        .expect("find");
    assert_eq!(found.value().selected(), Some(&"Ada"));
    assert_eq!(found.facts().requests_sent(), 1);
    let request: serde_json::Value =
        serde_json::from_slice(&listener.requests()[0].body).expect("request");
    assert_eq!(
        request,
        serde_json::json!({
            "state":{"context":"shared evidence","evidence":r#"[{"id":"u001","evidence":"Ada"},{"id":"u002","evidence":"Acme"}]"#},
            "model":"jev-1.13.0",
            "questions":{"q1":{"type":"choice","instructions":"Which asks for a refund?","criteria":{"u001":null,"u002":null}}}
        })
    );
    let recognize = thinkthen::Recognize::builder()
        .kind(thinkthen::Kind::new("person", None).expect("kind"))
        .and_then(thinkthen::RecognizeBuilder::build)
        .expect("recognize");
    let recognized = engine
        .recognize_with(&recognize, "Ada", options)
        .expect("recognize");
    assert!(recognized.value().entities().is_empty());
    assert_eq!(recognized.facts().requests_sent(), 1);
    let relate = Relate::builder()
        .relation(
            thinkthen::RelationRule::one_way("works_with", "person", "organization").expect("rule"),
        )
        .and_then(thinkthen::RelateBuilder::build)
        .expect("relate");
    let related = engine
        .relate_with(
            &relate,
            [
                Entity::new("Ada", "person").expect("entity"),
                Entity::new("Acme", "organization").expect("entity"),
            ],
            options,
        )
        .expect("relate");
    assert_eq!(related.value().len(), 1);
    assert_eq!(related.facts().requests_sent(), 1);
    assert_eq!(listener.count(), 3);
    let evidence = [
        serde_json::json!("Ada"),
        serde_json::json!({"entities":[
            {"id":"i1","name":"Ada","kind":"person"},
            {"id":"i2","name":"Acme","kind":"organization"}
        ]}),
    ];
    let requests = listener.requests();
    assert_eq!(requests.len(), 2);
    for (request, evidence) in requests.iter().zip(evidence) {
        let request: serde_json::Value = serde_json::from_slice(&request.body).expect("request");
        assert_eq!(
            request["state"],
            serde_json::json!({"context":"shared evidence","evidence":evidence})
        );
    }
}

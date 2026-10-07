use super::*;
use thinkthen::{LoadedQuestion, QuestionKind, ResolvedThreshold};

#[test]
fn complete_getters_keep_authored_content_order_null_and_raw_choice_before_cut() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"choice","probabilities":{"second":0.7,"first":0.3},"confidence":0.8}}}"#)).unwrap();
    let engine = engine(&listener);
    let LoadedQuestion::Question(question) = Question::from_json(
        r#"{"choose":{"z":"Authored.","a":["original",null]},"options":{"second":{"z":null,"a":"kept"},"first":null},"threshold":0.9}"#).unwrap() else { panic!("choose") };
    let call = engine
        .choose_complete_with(&question, "Original evidence.", CallOptions::new())
        .unwrap();
    let result = call.value();
    assert_eq!(result.value(), None);
    assert_eq!(result.raw_pick(), Some("second"));
    assert_eq!(result.threshold(), Some(ResolvedThreshold::Cut(0.9)));
    assert_eq!(result.question().kind(), QuestionKind::Choose);
    assert_eq!(result.question().text().text(), None);
    assert_eq!(
        result.question().text().to_json().unwrap(),
        r#"{"z":"Authored.","a":["original",null]}"#
    );
    let reading = result.question();
    let options = reading.options().collect::<Vec<_>>();
    assert_eq!(
        options.iter().map(|o| o.name()).collect::<Vec<_>>(),
        ["second", "first"]
    );
    assert_eq!(
        options[0].description().unwrap().to_json().unwrap(),
        r#"{"z":null,"a":"kept"}"#
    );
    // In the ordinary choose grammar null descriptions mean absent.
    assert!(options[1].description().is_none());
    assert!(!format!("{:?}", result.question()).contains("Authored"));
    assert_eq!(listener.count(), 1);
    let body: Value = serde_json::from_slice(&listener.requests()[0].body).unwrap();
    assert_eq!(
        body,
        json!({"model":"fixed","state":"Original evidence.","questions":{"q1":{"type":"choice","instructions":{"z":"Authored.","a":["original",null]},"criteria":{"second":{"z":null,"a":"kept"},"first":null}}}})
    );
}

#[test]
fn named_member_getters_retain_admitted_readings_failure_ids_and_actual_sources() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.5},"q2":{"type":"choice","probabilities":{"wrong":1}}},"usage":{"input_tokens":887}}"#)).unwrap();
    let set = thinkthen::QuestionSet::from_json(r#"{"version":1,"questions":{"uncertain":{"decide":"Sure?","true":null,"false":{"z":"No.","a":null},"threshold":"0.2:0.8"},"failed":{"decide":"Failed?","threshold":0.9}}}"#).unwrap();
    let engine = engine(&listener);
    let call = engine
        .annotate_complete_with(&set, ["Original."], CallOptions::new())
        .unwrap();
    let members = call.value()[0].result().members().collect::<Vec<_>>();
    assert_eq!(
        members[0].threshold(),
        Some(ResolvedThreshold::Band {
            low: 0.2,
            high: 0.8
        })
    );
    assert!(members[0].question().yes().unwrap().is_null());
    assert_eq!(
        members[0].question().no().unwrap().to_json().unwrap(),
        r#"{"z":"No.","a":null}"#
    );
    assert_eq!(members[1].question().text().text(), Some("Failed?"));
    assert_eq!(members[1].threshold(), Some(ResolvedThreshold::Cut(0.9)));
    assert!(members[1].answer_id().is_none());
    let [Observation::Failed { failure_id }] = members[1].observations() else {
        panic!("actual failure")
    };
    assert_eq!(Some(failure_id), members[1].failure_id());
    assert_eq!(members[0].question_sources()[0].origin(), Origin::Live);
    assert_eq!(members[0].question_sources()[0].batch_size(), Some(2));
    assert_eq!(
        members[0].reported_usage().unwrap().input_tokens(),
        Some(444)
    );
    assert_eq!(
        members[1].reported_usage().unwrap().input_tokens(),
        Some(443)
    );
    assert_eq!(members[0].reported_usage().unwrap().output_tokens(), None);
    assert_eq!(listener.count(), 1);
}

#[test]
fn reported_model_remains_truthful_but_debug_cannot_echo_a_private_value() {
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"complete-private","answers":{"q1":{"type":"noul","noul":0.9}}}"#)
    })
    .unwrap();
    let engine = engine(&listener);
    let question = Question::decide("Question?").unwrap().cut();
    let result = engine
        .decide_complete_with(&question, "Evidence.", CallOptions::new())
        .unwrap();
    assert_eq!(
        result.value().identity().answered_by(),
        Some("complete-private")
    );
    let shown = format!(
        "{:?} {:?} {:?}",
        result.value().identity(),
        result.value().meta(),
        result.value().identity().question_sources()
    );
    assert!(!shown.contains("complete-private"));
    assert_eq!(listener.count(), 1);
}

#[test]
fn aggregate_request_debug_withholds_authored_names_models_and_private_rules() {
    let secret = "private-credential-shaped-input";
    let kind = thinkthen::Kind::new(secret, None).unwrap();
    let rule = thinkthen::RelationRule::one_way(secret, "*", "*").unwrap();
    let recognize = thinkthen::Recognize::builder()
        .kind(kind.clone())
        .unwrap()
        .relation(rule.clone())
        .unwrap()
        .model(secret)
        .unwrap();
    let relate = thinkthen::Relate::builder()
        .relation(rule.clone())
        .unwrap()
        .model(secret)
        .unwrap();
    let reading = thinkthen::RecordReading::new(&[&format!("/{secret}")], None, None).unwrap();
    let saved = thinkthen::RecognizeQuestionFile::from_json(&format!(
        r#"{{"version":1,"recognize":{{}},"on":"/{secret}"}}"#
    ))
    .unwrap();
    assert!(!format!("{reading:?}").contains(secret));
    assert!(!format!("{saved:?}").contains(secret));
    for rendered in [
        format!("{kind:?}"),
        format!("{rule:?}"),
        format!("{recognize:?}"),
        format!("{relate:?}"),
        format!("{:?}", recognize.build().unwrap()),
        format!("{:?}", relate.build().unwrap()),
    ] {
        assert!(!rendered.contains(secret));
    }
}

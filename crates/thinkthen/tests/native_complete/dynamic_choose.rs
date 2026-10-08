use super::*;
use thinkthen::{BatchSetting, RecordChooseQuestion, RecordInput, RecordOptions};

#[test]
fn record_only_choose_resolves_saved_reading_and_sends_each_whole_actual_shortlist() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"saved-model","answers":{"q1":{"type":"choice","probabilities":{"b":0.7,"a":0.3}},"q2":{"type":"choice","probabilities":{"d":0.8,"c":0.2}}}}"#)).unwrap();
    let question = RecordChooseQuestion::from_json(
        r#"{"choose":"Which?","threshold":0.75,"model":"saved-model","batch":2}"#,
    )
    .unwrap();
    let engine = engine(&listener);
    let call = engine
        .choose_dynamic_records_complete_with(
            &question,
            [
                RecordInput {
                    examples: None,
                    seed_spans: None,
                    original: "One.",
                    context: None,
                    options: Some(
                        RecordOptions::project(
                            r#"{"opts":{"b":{"z":"B","a":null},"a":null}}"#,
                            "/opts",
                        )
                        .unwrap(),
                    ),
                },
                RecordInput {
                    examples: None,
                    seed_spans: None,
                    original: "Two.",
                    context: None,
                    options: Some(
                        RecordOptions::project(r#"{"opts":["d","c"]}"#, "/opts").unwrap(),
                    ),
                },
            ],
            CallOptions::new(),
        )
        .unwrap();
    assert_eq!(call.value()[0].result().value(), None);
    assert_eq!(call.value()[0].result().raw_pick(), Some("b"));
    assert_eq!(call.value()[1].result().value(), Some("d"));
    assert_eq!(call.value()[0].original(), &"One.");
    assert_eq!(call.value()[1].original(), &"Two.");
    assert_eq!(listener.count(), 1);
    assert_eq!(listener.questions(), 2);
    assert_eq!(
        std::str::from_utf8(&listener.requests()[0].body).unwrap(),
        r#"{"state":"Each question quotes the text it asks about.","model":"saved-model","questions":{"q1":{"type":"choice","instructions":"The text is \"One.\". Which?","criteria":{"b":{"z":"B","a":null},"a":null}},"q2":{"type":"choice","instructions":"The text is \"Two.\". Which?","criteria":{"d":null,"c":null}}}}"#
    );
    assert_eq!(question.text().text(), Some("Which?"));
    assert!(!format!("{question:?}").contains("Which?"));
}

#[test]
fn missing_later_record_candidates_refuse_before_sending_and_empty_choose_has_no_fake_question() {
    let listener = Listener::answering(|_| Canned::ok("unused")).unwrap();
    let engine = engine(&listener);
    let question = Question::choose_records("Which?")
        .unwrap()
        .cut_at(0.8)
        .unwrap();
    let error = engine
        .choose_dynamic_records_complete_with(
            &question,
            [
                RecordInput {
                    examples: None,
                    seed_spans: None,
                    original: "One.",
                    context: None,
                    options: Some(
                        RecordOptions::project(r#"{"opts":["b","a"]}"#, "/opts").unwrap(),
                    ),
                },
                RecordInput {
                    examples: None,
                    seed_spans: None,
                    original: "Two.",
                    context: None,
                    options: None,
                },
            ],
            CallOptions::new(),
        )
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(error.stopped().at(), Some(2));
    assert!(error.facts().is_none());
    let empty = engine
        .choose_dynamic_records_complete_with(
            &question,
            std::iter::empty::<RecordInput<String>>(),
            CallOptions::new().attempts(true),
        )
        .unwrap();
    assert!(empty.value().is_empty());
    assert_eq!(empty.facts().records(), 0);
    assert_eq!(empty.facts().requests_sent(), 0);
    assert!(empty.facts().attempts().unwrap().is_empty());
    assert!(empty.complete().is_some());
    let invalid_batch =
        RecordChooseQuestion::from_json(r#"{"choose":"Which?","batch":0}"#).unwrap();
    let refused = engine
        .choose_dynamic_records_complete_with(
            &invalid_batch,
            std::iter::empty::<RecordInput<String>>(),
            CallOptions::new(),
        )
        .unwrap_err();
    assert_eq!(refused.kind(), ErrorKind::Usage);
    let overridden = engine
        .choose_dynamic_records_complete_with(
            &invalid_batch,
            std::iter::empty::<RecordInput<String>>(),
            CallOptions::new().batch(BatchSetting::Max),
        )
        .unwrap();
    assert!(overridden.value().is_empty());
    assert_eq!(listener.count(), 0);
}

#[test]
fn explicit_record_choose_model_replaces_saved_model_and_retains_the_whole_shortlist() {
    let listener = Listener::answering(|_| Canned::ok(
        r#"{"model":"caller-model","answers":{"q1":{"type":"choice","probabilities":{"b":0.7,"a":0.3}}}}"#
    )).unwrap();
    let saved = RecordChooseQuestion::from_json(
        r#"{"choose":"Which?","threshold":0.75,"model":"saved-model","batch":2}"#,
    )
    .unwrap();
    assert!(saved.clone().with_model_override("").is_err());
    let selected = saved.with_model_override("caller-model").unwrap();
    let call = engine(&listener)
        .choose_dynamic_records_complete_with(
            &selected,
            [RecordInput {
                examples: None,
                seed_spans: None,
                original: "One.",
                context: None,
                options: Some(
                    RecordOptions::project(
                        r#"{"opts":{"b":{"z":"B","a":null},"a":null}}"#,
                        "/opts",
                    )
                    .unwrap(),
                ),
            }],
            CallOptions::new(),
        )
        .unwrap();
    assert_eq!(listener.count(), 1);
    assert_eq!(call.facts().model(), Some("caller-model"));
    assert_eq!(call.value()[0].original(), &"One.");
    assert_eq!(call.value()[0].result().value(), None);
    assert_eq!(call.value()[0].result().raw_pick(), Some("b"));
    assert_eq!(
        std::str::from_utf8(&listener.requests()[0].body).unwrap(),
        r#"{"state":"Each question quotes the text it asks about.","model":"caller-model","questions":{"q1":{"type":"choice","instructions":"The text is \"One.\". Which?","criteria":{"b":{"z":"B","a":null},"a":null}}}}"#
    );
}

#[test]
fn saved_record_choose_pointer_selects_parsed_evidence_and_refuses_literal_json_text() {
    let listener = Listener::answering(|_| Canned::ok(
        r#"{"model":"fixed","answers":{"q1":{"type":"choice","probabilities":{"red":0.9,"blue":0.1}}}}"#
    )).unwrap();
    let engine = engine(&listener);
    let question = RecordChooseQuestion::from_json(
        r#"{"choose":"Which?","on":"/body","item_schema":{"type":"string"}}"#,
    )
    .unwrap();
    let text = r#"{"body":"x","options":["red","blue"],"private":false}"#;
    let candidates = || RecordOptions::project(text, "/options").unwrap();
    let reading = thinkthen::RecordReading::new(&[], None, None).unwrap();
    for input in [
        thinkthen::QuestionInput::Text(text.into()),
        thinkthen::QuestionInput::Record(
            reading
                .compose(thinkthen::RawRecord::text(text).unwrap())
                .unwrap()
                .original,
        ),
    ] {
        let error = engine
            .choose_dynamic_records_complete_with(
                &question,
                [RecordInput {
                    examples: None,
                    seed_spans: None,
                    original: input,
                    context: None,
                    options: Some(candidates()),
                }],
                CallOptions::new(),
            )
            .unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Usage);
        assert_eq!(listener.count(), 0);
    }
    let input = reading
        .compose(thinkthen::RawRecord::json(text).unwrap())
        .unwrap()
        .original;
    let call = engine
        .choose_dynamic_records_complete_with(
            &question,
            [RecordInput {
                examples: None,
                seed_spans: None,
                original: input,
                context: None,
                options: Some(candidates()),
            }],
            CallOptions::new(),
        )
        .unwrap();
    assert_eq!(listener.count(), 1);
    assert_eq!(call.value()[0].result().value(), Some("red"));
    let row = serde_json::to_value(&call.value()[0]).unwrap();
    assert_eq!(row["input"], serde_json::from_str::<Value>(text).unwrap());
    assert!(row.get("source").is_none());
    assert_eq!(
        std::str::from_utf8(&listener.requests()[0].body).unwrap(),
        r#"{"state":"Each question quotes the text it asks about.","model":"fixed","questions":{"q1":{"type":"choice","instructions":"The text is \"x\". Which?","criteria":{"red":null,"blue":null}}}}"#
    );
    super::schema::call(&call, "completeChoose");
}

//! Native physical recognition retains local Unicode offsets and actual source spans.
use super::*;
use thinkthen::{
    Kind, RawRecord, Recognize, RecordEvidence, RecordInput, RecordReading, RelationRule,
    SourceLocation,
};
const TEXT: &str = "🙂\r\nAda met\r\nAcme.";
#[cfg(test)]
fn item(text: &str, file: &str, first: Option<usize>) -> RecordInput<RecordEvidence> {
    let mut input = RecordReading::new(&[], None, None)
        .unwrap()
        .compose(RawRecord::text(text).unwrap())
        .unwrap();
    input.original = input.original.with_location(
        SourceLocation::new(file.into(), first, first.map(|line| line + 2)).unwrap(),
    );
    input
}
#[cfg(test)]
fn ask() -> Recognize {
    Recognize::builder()
        .kind(Kind::new("person", None).unwrap())
        .unwrap()
        .kind(Kind::new("organization", None).unwrap())
        .unwrap()
        .relation(RelationRule::one_way("works_for", "person", "organization").unwrap())
        .unwrap()
        .build()
        .unwrap()
}
#[test]
fn located_names_and_relation_endpoints_map_unicode_crlf_and_survive_renamed_zero_send_replay() {
    let listener = Listener::answering(super::aggregates::recognized_response).unwrap();
    let root = folder();
    let build = || {
        Engine::builder()
            .base_url(listener.base())
            .unwrap()
            .model("fixed")
            .unwrap()
            .api_key("complete-private")
            .unwrap()
            .max_retries(0)
    };
    let engine = build().cache_at(&root).unwrap().build().unwrap();
    let call = engine
        .recognize_records_complete_with(
            &ask(),
            [item(TEXT, "first.txt", Some(40))],
            CallOptions::new().context("Separate.").attempts(true),
        )
        .unwrap();
    let replay = build().replay(&root).unwrap().build().unwrap();
    let held = replay
        .recognize_records_complete_with(
            &ask(),
            [item(TEXT, "renamed.txt", Some(80))],
            CallOptions::new().context("Separate.").attempts(true),
        )
        .unwrap();
    assert_eq!(listener.count(), 3);
    assert_eq!(held.facts().requests_sent(), 0);
    assert_eq!(
        call.value()[0].result().answer_id(),
        held.value()[0].result().answer_id()
    );
    assert_eq!(
        call.value()[0].result().identity().observations(),
        held.value()[0].result().identity().observations()
    );
    let requests = listener.requests();
    for (at, request) in requests.into_iter().enumerate() {
        assert_stage(&request.body, at);
    }
    drop(engine);
    drop(replay);
    let result = held.value()[0].result();
    assert_spans(result);
    assert!(result.meta().attempts().unwrap().is_empty());
    assert_eq!(result.reported_usage().unwrap().input_tokens(), Some(2661));
    assert_eq!(result.reported_usage().unwrap().output_tokens(), None);
    let row = serde_json::to_value(&held.value()[0]).unwrap();
    assert_eq!(row["input"], TEXT);
    assert_eq!(row["value"]["entities"][0]["file"], "renamed.txt");
    assert_eq!(row["value"]["entities"][0]["first_line"], 81);
    assert_eq!(row["value"]["relations"][0]["target"]["last_line"], 82);
    schema::call(&call, "completeRecognition");
    schema::call(&held, "completeRecognition");
}
#[cfg(test)]
fn assert_spans(result: &thinkthen::CompleteRecognized) {
    let source = result.source_value().unwrap();
    assert_eq!(source.location().first_line(), Some(80));
    assert_eq!(source.entities().len(), 2);
    let names = source.entities();
    assert_eq!(
        (
            names[0].entity().text(),
            names[0].entity().start(),
            names[0].entity().end()
        ),
        ("Ada", 3, 6)
    );
    assert_eq!(
        (
            names[1].entity().text(),
            names[1].entity().start(),
            names[1].entity().end()
        ),
        ("Acme", 12, 16)
    );
    assert_eq!(names[0].location().first_line(), Some(81));
    assert_eq!(names[1].location().last_line(), Some(82));
    let edge = &source.relations().unwrap()[0];
    assert_eq!(edge.relation(), "works_for");
    assert_eq!(edge.source(), &names[0]);
    assert_eq!(edge.target(), &names[1]);
    assert_eq!(edge.probability(), 0.9);
    assert!(!edge.either());
    assert!(!format!("{source:?}").contains("renamed.txt"));
}
#[test]
fn named_documents_omit_unobserved_lines_and_empty_record_sets_have_zero_facts() {
    let listener = Listener::answering(super::aggregates::recognized_response).unwrap();
    let engine = engine(&listener);
    let call = engine
        .recognize_records_complete_with(
            &ask(),
            [item("Ada met Acme.", "document.txt", None)],
            CallOptions::new().attempts(true),
        )
        .unwrap();
    let source = call.value()[0].result().source_value().unwrap();
    assert_eq!(source.entities()[0].location().file(), "document.txt");
    assert_eq!(source.entities()[0].location().first_line(), None);
    assert_eq!(source.entities()[0].location().last_line(), None);
    let row = serde_json::to_value(&call.value()[0]).unwrap();
    assert!(row["value"]["entities"][0].get("first_line").is_none());
    assert!(
        row["value"]["relations"][0]["source"]
            .get("last_line")
            .is_none()
    );
    let empty = engine
        .recognize_records_complete_with(
            &ask(),
            Vec::<RecordInput<RecordEvidence>>::new(),
            CallOptions::new().attempts(true),
        )
        .unwrap();
    assert!(empty.value().is_empty());
    assert_eq!(empty.facts().records(), 0);
    assert_eq!(empty.facts().requests_sent(), 0);
    assert_eq!(empty.facts().model(), None);
    assert_eq!(empty.facts().attempts(), Some(&[][..]));
    assert_eq!(listener.count(), 3);
    schema::call(&call, "completeRecognition");
}
#[test]
fn decoded_json_field_locations_are_refused_before_any_send_or_replay_lookup() {
    let listener = Listener::answering(super::aggregates::recognized_response).unwrap();
    let root = folder();
    let bad = || {
        let mut input = RecordReading::new(&["/body"], None, None)
            .unwrap()
            .compose(RawRecord::json(r#"{"body":"Ada met Acme.","private":false}"#).unwrap())
            .unwrap();
        input.original = input
            .original
            .with_location(SourceLocation::new("private.jsonl".into(), Some(1), Some(1)).unwrap());
        input
    };
    let engines = [
        engine(&listener),
        Engine::builder()
            .base_url(listener.base())
            .unwrap()
            .model("fixed")
            .unwrap()
            .replay(&root)
            .unwrap()
            .build()
            .unwrap(),
    ];
    for engine in engines {
        let error = engine
            .recognize_records_complete_with(
                &ask(),
                [item("Ada met Acme.", "good.txt", Some(1)), bad()],
                CallOptions::new(),
            )
            .unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Usage);
        assert_eq!(
            error.detail().message(),
            "located recognize takes literal text units, not decoded JSON fields"
        );
        assert_eq!(error.stopped().at(), Some(2));
        assert!(error.facts().is_none());
        assert!(!format!("{error:?}").contains("private.jsonl"));
    }
    assert_eq!(listener.count(), 0);
}

#[cfg(test)]
fn assert_stage(bytes: &[u8], at: usize) {
    let body: Value = serde_json::from_slice(bytes).unwrap();
    let evidence = if at < 2 {
        json!(TEXT)
    } else {
        json!({"evidence":TEXT,"entities":[{"id":"i1","name":"Ada","kind":"person"},{"id":"i2","name":"Acme","kind":"organization"}]})
    };
    assert_eq!(
        body["state"],
        json!({"context":"Separate.","evidence":evidence})
    );
    assert!(!String::from_utf8_lossy(bytes).contains("first.txt"));
    assert!(!String::from_utf8_lossy(bytes).contains("renamed.txt"));
}

#[test]
fn invalid_physical_range_refuses_the_whole_native_source_set_before_sends() {
    let listener = Listener::answering(super::aggregates::recognized_response).unwrap();
    let engine = engine(&listener);
    for first in [1, usize::MAX] {
        let mut bad = item(TEXT, "private.txt", None);
        bad.original = bad.original.with_location(
            SourceLocation::new("private.txt".into(), Some(first), Some(first)).unwrap(),
        );
        let error = engine
            .recognize_records_complete_with(
                &ask(),
                [item("Ada met Acme.", "valid.txt", Some(1)), bad],
                CallOptions::new(),
            )
            .unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Usage);
        assert_eq!(
            error.detail().message(),
            "source span is outside its record"
        );
        assert_eq!(error.stopped().at(), Some(2));
        assert!(error.facts().is_none());
    }
    assert_eq!(listener.count(), 0);
}

//! Native source relations deduplicate wire evidence and preserve physical occurrences.
use super::*;
use thinkthen::{
    InputEvidence, RawRecord, RecordEvidence, RecordInput, RecordReading, Relate, SourceLocation,
};
#[cfg(test)]
fn ask() -> Relate {
    Relate::from_records_json(r#"{"version":1,"relate":{"relations":[{"name":"follows","source":"person","target":"person","reads":"follows"}]}}"#).unwrap()
}
#[cfg(test)]
fn item(name: &str, ordinal: usize, file: &str, extra: &str) -> RecordInput<RecordEvidence> {
    let raw = RawRecord::json(&format!(
        r#"{{"name":"{name}","kind":"person","private":{extra}}}"#
    ))
    .unwrap();
    let mut input = RecordReading::new(&[], None, None)
        .unwrap()
        .compose(raw)
        .unwrap();
    input.original = input
        .original
        .with_location(SourceLocation::new(file.into(), Some(ordinal), Some(ordinal)).unwrap());
    input
}
#[test]
fn equal_source_entities_share_wire_questions_and_expand_original_occurrences_after_zero_send_replay()
 {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.1}},"usage":{"input_tokens":887}}"#)).unwrap();
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
    let engine = build().record(&root).unwrap().build().unwrap();
    let records = |file| {
        [
            item("Ada", 1, file, "false"),
            item("Grace", 3, file, "null"),
            item("Ada", 5, file, "[1,2]"),
        ]
    };
    let live = engine
        .relate_records_complete_with(
            &ask(),
            records("first.jsonl"),
            CallOptions::new().attempts(true),
        )
        .unwrap();
    let replay = build().replay(&root).unwrap().build().unwrap();
    let held = replay
        .relate_records_complete_with(
            &ask(),
            records("renamed.jsonl"),
            CallOptions::new().attempts(true),
        )
        .unwrap();
    assert_eq!(listener.count(), 1);
    assert_eq!(listener.questions(), 2);
    assert_eq!(held.facts().requests_sent(), 0);
    assert_eq!(live.value().original().len(), 3);
    let actual = serde_json::from_slice::<Value>(&listener.requests()[0].body).unwrap();
    assert_eq!(
        actual,
        json!({"model":"fixed","state":{"entities":[{"id":"i1","name":"Ada","kind":"person"},{"id":"i2","name":"Grace","kind":"person"}]},"questions":{"q1":{"type":"noul","instructions":"Is it true that i1 follows i2?"},"q2":{"type":"noul","instructions":"Is it true that i2 follows i1?"}}})
    );
    assert_eq!(
        live.value().result().answer_id(),
        held.value().result().answer_id()
    );
    assert_expansion(&held);
    schema::call(&live, "completeRelation");
    schema::call(&held, "completeRelation");
    drop(engine);
    drop(replay);
    assert_eq!(
        held.value().result().source_edges().unwrap()[1]
            .target()
            .location()
            .first_line(),
        Some(3)
    );
}
#[test]
fn source_inventory_survives_rejected_edges_and_zero_relation_questions() {
    for same_entity in [false, true] {
        let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.1},"q2":{"type":"noul","noul":0.1}}}"#)).unwrap();
        let engine = engine(&listener);
        let call = engine
            .relate_records_complete_with(
                &ask(),
                [
                    item("Ada", 1, "sources.jsonl", "false"),
                    item(
                        if same_entity { "Ada" } else { "Grace" },
                        3,
                        "sources.jsonl",
                        "null",
                    ),
                    item("Ada", 5, "sources.jsonl", "[1,2]"),
                ],
                CallOptions::new(),
            )
            .unwrap();
        let inventory = call
            .value()
            .result()
            .input_sources()
            .unwrap()
            .collect::<Vec<_>>();
        assert_eq!(inventory.len(), 3);
        for (index, (ordinal, source)) in inventory.iter().enumerate() {
            assert_eq!(*ordinal, index);
            assert_eq!(source.file(), "sources.jsonl");
            assert_eq!(source.first_line(), Some(1 + index * 2));
        }
        let document = serde_json::to_value(call.value()).unwrap();
        assert_eq!(document["value"], json!([]));
        assert_eq!(
            document["input_sources"],
            json!([
                {"index":0,"source":{"file":"sources.jsonl","first_line":1,"last_line":1}},
                {"index":1,"source":{"file":"sources.jsonl","first_line":3,"last_line":3}},
                {"index":2,"source":{"file":"sources.jsonl","first_line":5,"last_line":5}}
            ])
        );
        assert_eq!(document["input"][1]["private"], Value::Null);
        assert_eq!(listener.count(), usize::from(!same_entity));
        assert_eq!(
            call.value().result().members().len(),
            if same_entity { 0 } else { 2 }
        );
        assert_eq!(
            call.value().result().identity().observations().len(),
            if same_entity { 0 } else { 2 }
        );
    }
}
#[test]
fn source_inventory_counts_occurrences_before_deduplication_and_leaves_the_refused_tail_unread() {
    let listener = Listener::answering(|_| Canned::ok("{}")).unwrap();
    let engine = engine(&listener);
    let call = engine
        .relate_records_complete_with(
            &ask(),
            (1..=255).map(|at| item("Ada", at, "source.jsonl", "null")),
            CallOptions::new(),
        )
        .unwrap();
    assert_eq!(call.value().original().len(), 255);
    assert_eq!(call.value().result().input_sources().unwrap().count(), 255);
    let pulls = AtomicUsize::new(0);
    let rows = std::iter::from_fn(|| {
        let at = pulls.fetch_add(1, Ordering::Relaxed) + 1;
        assert!(at <= 256, "refusal must leave the suffix unread");
        Some(Ok(item("Ada", at, "source.jsonl", "null")))
    });
    let error = engine
        .try_relate_records_complete_with(&ask(), rows, CallOptions::new())
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(
        error.detail().message(),
        "source relate takes at most 255 source records"
    );
    assert!(error.facts().is_none());
    assert_eq!(pulls.load(Ordering::Relaxed), 256);
    assert_eq!(listener.count(), 0);
}
#[test]
fn source_relations_keep_nonserializable_nonclone_originals_and_refuse_mixed_sources_before_sending()
 {
    struct Original {
        value: RecordEvidence,
        marker: std::rc::Rc<()>,
    }
    impl InputEvidence for Original {
        fn question_input(&self) -> thinkthen::QuestionInput {
            self.value.question_input()
        }
    }
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.1}}}"#)).unwrap();
    let engine = engine(&listener);
    let make = |name, at| RecordInput {
        examples: None,
        seed_spans: None,
        original: Original {
            value: item(name, at, "source.txt", "false").original,
            marker: std::rc::Rc::new(()),
        },
        context: None,
        options: None,
    };
    let call = engine
        .relate_records_complete_with(
            &ask(),
            [make("Ada", 1), make("Grace", 2)],
            CallOptions::new(),
        )
        .unwrap();
    assert_eq!(
        std::rc::Rc::strong_count(&call.value().original()[0].marker),
        1
    );
    assert_eq!(
        call.value().result().source_edges().unwrap()[0]
            .source()
            .record()
            .content()
            .unwrap()
            .to_json()
            .unwrap(),
        r#"{"name":"Ada","kind":"person","private":false}"#
    );
    let bare = RecordReading::new(&[], None, None)
        .unwrap()
        .compose(RawRecord::json(r#"{"name":"Grace","kind":"person"}"#).unwrap())
        .unwrap();
    let error = engine
        .relate_records_complete_with(
            &ask(),
            [item("Ada", 1, "source.txt", "false"), bare],
            CallOptions::new(),
        )
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert!(error.facts().is_none());
    assert_eq!(listener.count(), 1);
}
#[test]
#[ignore = "release-only large-input boundary; run sdlc/scripts/test-full-cases --run"]
fn release_only_expanded_source_edges_enforce_escaped_byte_limit_and_preserve_started_facts_without_resending()
 {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.1}}}"#)).unwrap();
    let engine = engine(&listener);
    let payload = serde_json::to_string(&"\"".repeat(400_000)).unwrap();
    let records = (1..=8).map(|at| {
        item(
            if at <= 4 { "Ada" } else { "Grace" },
            at,
            "source.jsonl",
            &payload,
        )
    });
    let error = engine
        .relate_records_complete_with(&ask(), records, CallOptions::new())
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(
        error.detail().message(),
        "source relate output exceeds 16 MiB"
    );
    assert_eq!(error.facts().unwrap().requests_sent(), 1);
    assert_eq!(listener.count(), 1);
}

#[cfg(test)]
fn assert_expansion(
    held: &thinkthen::Call<
        thinkthen::CompleteRecord<Vec<RecordEvidence>, thinkthen::CompleteRelated>,
    >,
) {
    let value = held.value().result();
    assert_eq!(value.value().len(), 1);
    assert_eq!(value.members().len(), 2);
    assert_eq!(value.reported_usage().unwrap().input_tokens(), Some(887));
    assert_eq!(value.reported_usage().unwrap().output_tokens(), None);
    let edges = value.source_edges().unwrap();
    assert_eq!(edges.len(), 2);
    assert_eq!(
        edges
            .iter()
            .map(|edge| (edge.source().ordinal(), edge.target().ordinal()))
            .collect::<Vec<_>>(),
        [(0, 1), (2, 1)]
    );
    assert_eq!(edges[1].source().entity().name(), "Ada");
    assert_eq!(edges[1].source().location().first_line(), Some(5));
    assert_eq!(edges[1].source().location().file(), "renamed.jsonl");
    let written = serde_json::to_value(held.value()).unwrap();
    assert_eq!(
        written["input_sources"],
        json!([
            {"index":0,"source":{"file":"renamed.jsonl","first_line":1,"last_line":1}},
            {"index":1,"source":{"file":"renamed.jsonl","first_line":3,"last_line":3}},
            {"index":2,"source":{"file":"renamed.jsonl","first_line":5,"last_line":5}}
        ])
    );
    assert_eq!(written["value"][0]["source"]["record"]["private"], false);
    assert_eq!(
        written["value"][1]["source"]["record"]["private"],
        json!([1, 2])
    );
    assert_eq!(
        written["value"][0]["target"]["record"]["private"],
        Value::Null
    );
    for (edge, native) in written["value"].as_array().unwrap().iter().zip(edges) {
        assert_eq!(edge["source"]["ordinal"], native.source().ordinal());
        assert_eq!(edge["target"]["ordinal"], native.target().ordinal());
    }
    assert_eq!(written["value"][1]["source"]["first_line"], 5);
    assert_eq!(written["value"][1]["source"]["file"], "renamed.jsonl");
    assert!(!format!("{edges:?}").contains("renamed.jsonl"));
}

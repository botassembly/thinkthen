//! Native source relations deduplicate wire evidence and preserve physical occurrences.
use super::*;
#[path = "../support/source_relations.rs"]
mod fixture;
use fixture::{ask, item};
#[test]
fn equal_source_entities_share_wire_questions_and_expand_original_occurrences_after_zero_send_replay()
 {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.1}},"usage":{"input_tokens":887}}"#)).unwrap();
    let root = folder();
    let build = || crate::api::fixed(listener.base(), "complete-private").max_retries(0);
    let engine = build().record(&root).unwrap().build().unwrap();
    let records = |file| {
        [
            item("Ada", 1, file, "false"),
            item("Grace", 3, file, "null"),
            item("Ada", 5, file, "[1,2]"),
        ]
    };
    let live = crate::api::relation(
        &engine,
        &ask(),
        (records("first.jsonl")).into_iter().map(Ok),
        CallOptions::new().attempts(true),
    )
    .unwrap();
    let live_value = crate::api::related(&live).unwrap();
    let replay = build().replay(&root).unwrap().build().unwrap();
    let held = crate::api::relation(
        &replay,
        &ask(),
        (records("renamed.jsonl")).into_iter().map(Ok),
        CallOptions::new().attempts(true),
    )
    .unwrap();
    let held_value = crate::api::related(&held).unwrap();
    assert_eq!(listener.count(), 1);
    assert_eq!(listener.questions(), 2);
    assert_eq!(held.facts().requests_sent(), 0);
    assert_eq!(live_value.original().len(), 3);
    let actual = serde_json::from_slice::<Value>(&listener.requests()[0].body).unwrap();
    assert_eq!(
        actual,
        json!({"model":"fixed","state":{"entities":[{"id":"i1","name":"Ada","kind":"person"},{"id":"i2","name":"Grace","kind":"person"}]},"questions":{"q1":{"type":"noul","instructions":"Is it true that i1 follows i2?"},"q2":{"type":"noul","instructions":"Is it true that i2 follows i1?"}}})
    );
    assert_eq!(
        live_value.result().answer_id(),
        held_value.result().answer_id()
    );
    assert_expansion(&held);
    schema::call(&live, "completeRelation");
    schema::call(&held, "completeRelation");
    drop(engine);
    drop(replay);
    assert_eq!(
        held_value.result().source_edges().unwrap()[1]
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
        let call = crate::api::relation(
            &engine,
            &ask(),
            ([
                item("Ada", 1, "sources.jsonl", "false"),
                item(
                    if same_entity { "Ada" } else { "Grace" },
                    3,
                    "sources.jsonl",
                    "null",
                ),
                item("Ada", 5, "sources.jsonl", "[1,2]"),
            ])
            .into_iter()
            .map(Ok),
            CallOptions::new(),
        )
        .unwrap();
        let call_value = crate::api::related(&call).unwrap();
        let inventory = call_value
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
        let document = serde_json::to_value(call_value).unwrap();
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
            call_value.result().members().len(),
            if same_entity { 0 } else { 2 }
        );
        assert_eq!(
            call_value.result().identity().observations().len(),
            if same_entity { 0 } else { 2 }
        );
    }
}
#[test]
fn source_inventory_counts_occurrences_before_deduplication_and_leaves_the_refused_tail_unread() {
    let listener = Listener::answering(|_| Canned::ok("{}")).unwrap();
    let engine = engine(&listener);
    let call = crate::api::relation(
        &engine,
        &ask(),
        (1..=255)
            .map(|at| item("Ada", at, "source.jsonl", "null"))
            .map(Ok),
        CallOptions::new(),
    )
    .unwrap();
    let call_value = crate::api::related(&call).unwrap();
    assert_eq!(call_value.original().len(), 255);
    assert_eq!(call_value.result().input_sources().unwrap().count(), 255);
    let pulls = AtomicUsize::new(0);
    let rows = std::iter::from_fn(|| {
        let at = pulls.fetch_add(1, Ordering::Relaxed) + 1;
        assert!(at <= 256, "refusal must leave the suffix unread");
        Some(Ok(item("Ada", at, "source.jsonl", "null")))
    });
    let error = crate::api::relation(&engine, &ask(), rows, CallOptions::new()).unwrap_err();
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
    let error = crate::api::relation(
        &engine,
        &ask(),
        records.into_iter().map(Ok),
        CallOptions::new(),
    )
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
fn assert_expansion(held: &thinkthen::Call<thinkthen::RequestValue>) {
    let held_value = crate::api::related(held).unwrap();
    let value = held_value.result();
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
    let written = serde_json::to_value(held_value).unwrap();
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

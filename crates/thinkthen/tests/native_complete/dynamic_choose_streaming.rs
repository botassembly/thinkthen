use super::*;
use std::cell::Cell;
use std::num::NonZeroUsize;
use thinkthen::{
    BatchSetting, CancelToken, InputFileReader, InputReaderOptions, RawRecord,
    RecordChooseQuestion, RecordInput, RecordOptions, RecordReading,
};

#[cfg(test)]
fn one() -> CallOptions<'static> {
    CallOptions::new()
        .batch(BatchSetting::Records(NonZeroUsize::new(1).unwrap()))
        .attempts(true)
}

#[cfg(test)]
fn candidates(text: &str) -> RecordOptions {
    RecordOptions::project(text, "").unwrap()
}

struct Original(usize, std::rc::Rc<()>);
impl thinkthen::Evidence for Original {
    fn evidence(&self) -> &str {
        "Same."
    }
}

#[test]
fn empty_dynamic_choose_finishes_without_candidates_and_still_validates_batch_controls() {
    let listener = Listener::answering(|_| Canned::ok("unused")).unwrap();
    let engine = engine(&listener);
    let invalid = RecordChooseQuestion::from_json(r#"{"choose":"Which?","batch":0}"#).unwrap();
    let error = engine
        .try_choose_dynamic_records_complete_with(
            &invalid,
            std::iter::empty::<Result<RecordInput<String>, thinkthen::Error>>(),
            CallOptions::new(),
        )
        .into_call()
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert!(error.facts().is_none());
    let call = engine
        .try_choose_dynamic_records_complete_with(
            &invalid,
            std::iter::empty::<Result<RecordInput<String>, thinkthen::Error>>(),
            CallOptions::new().batch(BatchSetting::Max).attempts(true),
        )
        .into_call()
        .unwrap();
    assert!(call.value().is_empty());
    assert_eq!(call.facts().records(), 0);
    assert_eq!(call.facts().requests_sent(), 0);
    assert!(call.facts().attempts().unwrap().is_empty());
    assert!(call.facts().call_id().is_some());
    assert!(call.complete().is_some());
    assert_eq!(listener.count(), 0);
}

#[test]
fn dynamic_reader_failure_preserves_the_located_prefix_and_never_inherits_candidates() {
    for second in [
        "{not-valid-private",
        r#"{"body":"Missing.","context":"Guide."}"#,
    ] {
        let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"choice","probabilities":{"b":0.8,"a":0.2}}},"usage":{"input_tokens":887}}"#)).unwrap();
        let engine = engine(&listener);
        let question = Question::choose_records("Which?").unwrap();
        let reading = RecordReading::new(&["/body"], Some("/context"), Some("/options")).unwrap();
        let first = r#"{"z":false,"body":"First.","context":"Guide.","options":["b","a"]}"#;
        let text = format!("{first}\n{second}\n{{\"body\":\"Unread.\"}}\n");
        let reader = InputFileReader::new(
            "private.jsonl",
            std::io::Cursor::new(text),
            InputReaderOptions::default(),
        )
        .unwrap();
        let pulled = Cell::new(0);
        let records = reader
            .inspect(|_| pulled.set(pulled.get() + 1))
            .map(|source| source.and_then(|source| reading.compose_source(source)));
        let mut batch = engine.try_choose_dynamic_records_complete_with(&question, records, one());
        let row = batch.next().unwrap().unwrap();
        assert_eq!(row.ordinal(), 0);
        assert_eq!(row.result().value(), Some("b"));
        assert_eq!(pulled.get(), 1);
        let thinkthen::QuestionInput::Record(original) = row.original() else {
            panic!("record")
        };
        assert_eq!(original.location().unwrap().file(), "private.jsonl");
        assert_eq!(original.location().unwrap().first_line(), Some(1));
        assert_eq!(
            original.original().content().unwrap().to_json().unwrap(),
            first
        );
        let error = batch.next().unwrap().unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Usage);
        assert_eq!(error.stopped().at(), Some(2));
        assert_eq!(pulled.get(), 2);
        assert!(batch.next().is_none());
        let facts = error.facts().unwrap();
        assert_eq!(facts.records(), 1);
        assert_eq!(facts.requests_sent(), 1);
        assert_eq!(facts.input_tokens(), Some(887));
        assert_eq!(facts.call_id(), batch.facts().unwrap().call_id());
        assert_eq!(facts.attempts().unwrap().len(), 1);
        assert_eq!(listener.count(), 1);
        assert_eq!(
            std::str::from_utf8(&listener.requests()[0].body).unwrap(),
            r#"{"state":"Guide.","model":"fixed","questions":{"q1":{"type":"choice","instructions":"The text is \"First.\". Which?","criteria":{"b":null,"a":null}}}}"#
        );
        assert!(!format!("{error:?} {error}").contains("not-valid-private"));
    }
}

#[test]
fn missing_first_or_later_dynamic_candidates_stop_at_their_original_without_reading_a_suffix() {
    for prefix in [false, true] {
        let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"choice","probabilities":{"b":0.8,"a":0.2}}}}"#)).unwrap();
        let engine = engine(&listener);
        let question = Question::choose_records("Which?").unwrap();
        let pulled = Cell::new(0);
        let records = (0..3).map(|at| {
            pulled.set(pulled.get() + 1);
            Ok(RecordInput {
                examples: None,
                original: "Same.",
                context: None,
                options: (prefix && at == 0).then(|| candidates(r#"["b","a"]"#)),
            })
        });
        let mut batch = engine.try_choose_dynamic_records_complete_with(&question, records, one());
        if prefix {
            let row = batch.next().unwrap().unwrap();
            assert_eq!(row.ordinal(), 0);
            assert_eq!(row.result().value(), Some("b"));
        }
        let error = batch.next().unwrap().unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Usage);
        assert_eq!(
            error.detail().message(),
            "record choose requires candidates on every original"
        );
        assert_eq!(error.stopped().at(), Some(if prefix { 2 } else { 1 }));
        assert_eq!(pulled.get(), if prefix { 2 } else { 1 });
        assert!(batch.next().is_none());
        assert_eq!(error.facts().unwrap().records(), u64::from(prefix));
        assert_eq!(error.facts().unwrap().requests_sent(), u64::from(prefix));
        assert_eq!(
            error.facts().unwrap().call_id(),
            batch.facts().unwrap().call_id()
        );
        assert_eq!(listener.count(), usize::from(prefix));
    }
}

#[test]
fn dynamic_candidate_order_retains_originals_probabilities_and_distinct_replay_identity() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"saved-model","answers":{"q1":{"type":"choice","probabilities":{"a":0.2,"b":0.8}}}}"#)).unwrap();
    let folder = folder();
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
    let engine = build().cache_at(&folder).unwrap().build().unwrap();
    let question = RecordChooseQuestion::from_json(
        r#"{"choose":"Which?","model":"saved-model","threshold":0.75,"batch":2}"#,
    )
    .unwrap();
    let lists = [
        r#"{"b":{"z":"B","a":null},"a":null}"#,
        r#"{"a":null,"b":{"z":"B","a":null}}"#,
        r#"{"b":{"z":"B","a":null},"a":null}"#,
    ];
    let records = || {
        lists.iter().enumerate().map(|(at, list)| {
            Ok(RecordInput {
                examples: None,
                original: Original(at, std::rc::Rc::new(())),
                context: Some("Guide.".into()),
                options: Some(candidates(list)),
            })
        })
    };
    let call = engine
        .try_choose_dynamic_records_complete_with(&question, records(), one())
        .into_call()
        .unwrap();
    assert_eq!(call.facts().model(), Some("saved-model"));
    assert_eq!(call.facts().records(), 3);
    assert_eq!(call.facts().requests_sent(), 2);
    for (at, row) in call.value().iter().enumerate() {
        assert_eq!(row.ordinal(), at);
        assert_eq!(row.original().0, at);
        assert_eq!(std::rc::Rc::strong_count(&row.original().1), 1);
        assert_eq!(row.result().value(), Some("b"));
        let thinkthen::Probabilities::Named(probabilities) = row.result().probabilities() else {
            panic!("ordered probabilities")
        };
        let expected = if at == 1 {
            [("a", 0.2), ("b", 0.8)]
        } else {
            [("b", 0.8), ("a", 0.2)]
        };
        assert_eq!(
            probabilities
                .iter()
                .map(|p| (p.name(), p.probability()))
                .collect::<Vec<_>>(),
            expected
        );
    }
    assert_eq!(
        call.value()[0].result().identity().observations(),
        call.value()[2].result().identity().observations()
    );
    assert_ne!(
        call.value()[0].result().identity().observations(),
        call.value()[1].result().identity().observations()
    );
    assert_eq!(listener.count(), 2);
    let requests = listener.requests();
    assert_eq!(
        std::str::from_utf8(&requests[0].body).unwrap(),
        r#"{"state":"Guide.","model":"saved-model","questions":{"q1":{"type":"choice","instructions":"The text is \"Same.\". Which?","criteria":{"b":{"z":"B","a":null},"a":null}}}}"#
    );
    assert_eq!(
        std::str::from_utf8(&requests[1].body).unwrap(),
        r#"{"state":"Guide.","model":"saved-model","questions":{"q1":{"type":"choice","instructions":"The text is \"Same.\". Which?","criteria":{"a":null,"b":{"z":"B","a":null}}}}}"#
    );
    let replay = build().replay(&folder).unwrap().build().unwrap();
    let repeated = replay
        .try_choose_dynamic_records_complete_with(&question, records(), one())
        .into_call()
        .unwrap();
    assert_eq!(repeated.facts().records(), 3);
    assert_eq!(repeated.facts().requests_sent(), 0);
    assert_eq!(listener.count(), 2);
    drop(replay);
    drop(engine);
    std::fs::remove_dir_all(folder).unwrap();
}

#[test]
fn dynamic_choose_cancellation_stops_before_polling_and_after_a_delivered_row() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"choice","probabilities":{"b":0.8,"a":0.2}}}}"#)).unwrap();
    let engine = engine(&listener);
    let question = Question::choose_records("Which?").unwrap();
    for pre_cancelled in [true, false] {
        let token = CancelToken::new();
        if pre_cancelled {
            token.cancel();
        }
        let pulled = Cell::new(0);
        let records = (0..2).map(|_| {
            pulled.set(pulled.get() + 1);
            Ok(RecordInput {
                examples: None,
                original: "Same.",
                context: None,
                options: Some(candidates(r#"["b","a"]"#)),
            })
        });
        let mut batch = engine.try_choose_dynamic_records_complete_with(
            &question,
            records,
            one().cancel(&token),
        );
        if !pre_cancelled {
            assert_eq!(batch.next().unwrap().unwrap().ordinal(), 0);
            assert_eq!(pulled.get(), 1);
            token.cancel();
        }
        let error = batch.next().unwrap().unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Cancelled);
        assert!(batch.next().is_none());
        if !pre_cancelled {
            assert_eq!(error.facts().unwrap().records(), 1);
            assert_eq!(error.facts().unwrap().requests_sent(), 1);
            assert_eq!(
                error.facts().unwrap().call_id(),
                batch.facts().unwrap().call_id()
            );
        } else {
            assert_eq!(pulled.get(), 0);
            assert!(error.facts().is_none());
        }
    }
    assert_eq!(listener.count(), 1);
}

#[test]
fn dynamic_choose_declarations_admit_the_staged_batch_before_replay_lookup_or_send() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"choice","probabilities":{"b":0.8,"a":0.2}}}}"#)).unwrap();
    let folder = folder();
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
    let engine = build().no_cache().record(&folder).unwrap().build().unwrap();
    let question = RecordChooseQuestion::from_json(
        r#"{"choose":"Which?","item_schema":{"type":"string"},"context_schema":{"type":"string"}}"#,
    )
    .unwrap();
    let reading = RecordReading::new(&["/body"], Some("/context"), Some("/options")).unwrap();
    let input = |text| reading.compose(RawRecord::json(text).unwrap()).unwrap();
    engine
        .try_choose_dynamic_records_complete_with(
            &question,
            [Ok(input(
                r#"{"body":"Primed.","context":"Guide.","options":["b","a"]}"#,
            ))],
            one(),
        )
        .into_call()
        .unwrap();
    let replay = build().replay(&folder).unwrap().build().unwrap();
    for (second, invalid_context, message) in [
        (
            r#"{"body":12,"context":"Guide.","options":["b","a"]}"#,
            false,
            "the item does not match item_schema",
        ),
        (
            r#"{"body":"Valid.","context":"Guide.","options":["b","a"]}"#,
            true,
            "the per-item context does not match context_schema",
        ),
    ] {
        for selected in [&engine, &replay] {
            let mut invalid = input(second);
            if invalid_context {
                invalid.context = Some(thinkthen::RecordContext::Object(
                    thinkthen::ObjectContext::new(&RawRecord::json("{}").unwrap()).unwrap(),
                ));
            }
            let mut batch = selected.try_choose_dynamic_records_complete_with(
                &question,
                [
                    Ok(input(
                        r#"{"body":"Unstored.","context":"Guide.","options":["b","a"]}"#,
                    )),
                    Ok(invalid),
                ],
                CallOptions::new().batch(BatchSetting::Records(NonZeroUsize::new(2).unwrap())),
            );
            let error = batch.next().unwrap().unwrap_err();
            assert_eq!(error.kind(), ErrorKind::Usage);
            assert_eq!(error.detail().message(), message);
            assert_eq!(error.stopped().at(), Some(2));
            assert_eq!(error.facts().unwrap().records(), 0);
            assert_eq!(error.facts().unwrap().requests_sent(), 0);
            assert!(batch.next().is_none());
            assert_eq!(listener.count(), 1);
        }
    }
    drop(replay);
    drop(engine);
    std::fs::remove_dir_all(folder).unwrap();
}

//! Focused public boundary cases retained in the parent test target.

use super::*;

#[test]
fn typed_bulk_verbs_share_one_request_and_keep_input_order() {
    let _serial = serial();
    let backend = Backend::start().expect("backend");
    let engine = engine(&format!("{}/generic/v1", backend.origin()));
    let records = ["alpha", "beta"];
    let setting = BatchSetting::Records(std::num::NonZeroUsize::new(2).expect("two"));

    let choose = Question::choose::<BulkLabel>("Which label?")
        .and_then(|builder| builder.option(BulkLabel::First, None))
        .and_then(|builder| builder.option(BulkLabel::Second, None))
        .and_then(thinkthen::ChooseBuilder::build)
        .expect("choose");
    let rows = engine
        .choose_many_with(&choose, records, CallOptions::new().batch(setting))
        .collect::<Result<Vec<_>, _>>()
        .expect("choices");
    assert_eq!(
        rows.iter().map(|row| *row.input()).collect::<Vec<_>>(),
        records
    );
    assert!(
        rows.iter()
            .all(|row| *row.value() == Some(BulkLabel::First))
    );
    assert_eq!(backend.count(), 1);

    let score = Question::score("How high?")
        .and_then(|builder| builder.level("low", None))
        .and_then(|builder| builder.level("high", None))
        .and_then(thinkthen::ScoreBuilder::build)
        .expect("score");
    let rows = engine
        .score_many_with(&score, records, CallOptions::new().batch(setting))
        .collect::<Result<Vec<_>, _>>()
        .expect("scores");
    assert_eq!(
        rows.iter().map(|row| *row.input()).collect::<Vec<_>>(),
        records
    );
    assert!(
        rows.iter()
            .all(|row| (*row.value() - 0.1).abs() < 0.000_001)
    );
    assert_eq!(backend.count(), 2);

    let tag = Question::tag::<BulkLabel>("Which labels?")
        .and_then(|builder| builder.label(BulkLabel::First, None))
        .and_then(|builder| builder.label(BulkLabel::Second, None))
        .and_then(thinkthen::TagBuilder::cut)
        .expect("tag");
    let rows = engine
        .tag_many_with(&tag, records, CallOptions::new().batch(setting))
        .collect::<Result<Vec<_>, _>>()
        .expect("tags");
    assert_eq!(
        rows.iter().map(|row| *row.input()).collect::<Vec<_>>(),
        records
    );
    assert!(
        rows.iter()
            .all(|row| row.value() == &vec![BulkLabel::First, BulkLabel::Second])
    );
    assert_eq!(backend.count(), 3);
}

#[test]
fn a_later_invalid_annotation_keeps_the_valid_record_prefix() {
    let _serial = serial();
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.9}}}"#)
    })
    .expect("listener");
    let engine = engine(listener.base());
    let set = QuestionSet::from_json(
        r#"{"version":1,"questions":{"left":{"decide":"Left?","on":"/left"},"right":{"decide":"Right?","on":"/right"}}}"#,
    )
    .expect("set");
    let records = [r#"{"left":"a","right":"b"}"#, r#"{"left":"c"}"#];
    let mut rows = engine.annotate(&set, records);
    assert_eq!(
        rows.next().expect("first").expect("valid prefix").input(),
        &records[0]
    );
    assert_eq!(
        rows.next()
            .expect("refusal")
            .expect_err("missing part")
            .kind(),
        ErrorKind::Usage
    );
    assert!(rows.next().is_none());
    assert_eq!(
        rows.facts()
            .map(|facts| (facts.records(), facts.requests_sent())),
        Some((1, 1))
    );
    assert_eq!(
        listener.requests().len(),
        1,
        "both groups share one request"
    );
}

#[test]
fn typed_choice_keeps_null_distinct_from_a_failed_later_row() {
    let _serial = serial();
    let null = r#"{"model":"jev-latest","answers":{"q1":{"type":"choice","probabilities":{"first":1.0,"second":0.0}},"q2":{"type":"choice","probabilities":{"first":0.5,"second":0.5}}}}"#;
    let failed = r#"{"model":"jev-latest","answers":{"q1":{"type":"choice","probabilities":{"first":1.0,"second":0.0}}}}"#;
    let listener =
        Listener::serving(vec![Canned::ok(null), Canned::ok(failed)]).expect("scripted listener");
    let engine = engine(listener.base());
    let choose = Question::choose::<BulkLabel>("Which label?")
        .and_then(|builder| builder.option(BulkLabel::First, None))
        .and_then(|builder| builder.option(BulkLabel::Second, None))
        .and_then(thinkthen::ChooseBuilder::build)
        .expect("choose");
    let setting = BatchSetting::Records(std::num::NonZeroUsize::new(2).expect("two"));
    let rows = engine
        .choose_many_with(
            &choose,
            ["alpha", "beta"],
            CallOptions::new().batch(setting),
        )
        .collect::<Result<Vec<_>, _>>()
        .expect("a tie is a null choice");
    assert_eq!(rows.len(), 2);
    assert_eq!(*rows[0].value(), Some(BulkLabel::First));
    assert_eq!(*rows[1].value(), None);

    let seen = Mutex::new(Vec::new());
    let observe = |event: RecordObservation<'_>| match event {
        RecordObservation::Question { index, detail, .. } => {
            seen.lock()
                .expect("observations")
                .push((index, detail.failure()));
        }
        RecordObservation::Row { .. } => {}
    };
    let mut rows = engine.choose_many_with(
        &choose,
        ["gamma", "delta"],
        CallOptions::new().batch(setting).observe(&observe),
    );
    assert_eq!(
        rows.next()
            .expect("first row")
            .expect("first choice")
            .value(),
        &Some(BulkLabel::First)
    );
    let error = rows
        .next()
        .expect("failed row")
        .expect_err("missing answer");
    assert_eq!(error.kind(), ErrorKind::Backend);
    assert!(rows.next().is_none());
    let facts = rows.facts().expect("finished failure facts");
    assert_eq!((facts.records(), facts.requests_sent()), (1, 1));
    assert_eq!(error.facts().map(|facts| facts.records()), Some(1));
    assert_eq!(
        *seen.lock().expect("observations"),
        [(0, None), (1, Some(thinkthen::FailureCause::MissingAnswer)),]
    );
    assert_eq!(listener.requests().len(), 2);
}

#[test]
fn eager_observer_reads_complete_question_then_row_on_caller_thread() {
    let _serial = serial();
    let listener = Listener::answering(|_| Canned::ok(DECIDED)).expect("listener");
    let engine = engine(listener.base());
    let caller = thread::current().id();
    let seen = Mutex::new(Vec::new());
    let observe = |event: RecordObservation<'_>| {
        assert_eq!(thread::current().id(), caller);
        match event {
            RecordObservation::Question { index, detail, .. } => {
                assert_eq!(index, 0);
                assert_eq!(detail.question_sha256().len(), 64);
                assert_eq!(detail.requests().len(), 1);
                assert_eq!(detail.requests_sent(), 1);
                assert_eq!(detail.usage().map(|use_| use_.input_tokens()), Some(3));
                seen.lock().expect("observation").push("question");
            }
            RecordObservation::Row { index, value } => {
                assert_eq!(index, 0);
                assert!(matches!(
                    value,
                    ObservedRow::Judgment(Judgment::Decision(Answer::Yes))
                ));
                seen.lock().expect("observation").push("row");
            }
        }
    };
    let call = engine
        .decide_with(&question(), "alpha", CallOptions::new().observe(&observe))
        .expect("observed decision");
    assert_eq!(*call.value(), Answer::Yes);
    assert_eq!(*seen.lock().expect("observation"), ["question", "row"]);
    assert_eq!(listener.count(), 1);
}

#[test]
fn batch_observer_keeps_filtered_row_and_even_request_shares() {
    let _serial = serial();
    let reply = r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.1},"q2":{"type":"noul","noul":0.9}},"usage":{"input_tokens":3,"output_tokens":1}}"#;
    let listener = Listener::answering(move |_| Canned::ok(reply)).expect("listener");
    let engine = engine(listener.base());
    let seen = Mutex::new(Vec::new());
    let caller = thread::current().id();
    let observe = |event: RecordObservation<'_>| {
        assert_eq!(thread::current().id(), caller);
        match event {
            RecordObservation::Question { index, detail, .. } => {
                assert_eq!(detail.question_sha256().len(), 64);
                assert_eq!(detail.requests().len(), 1);
                assert_eq!(detail.model(), "jev-latest");
                assert_eq!(detail.url(), listener.url());
                assert_eq!(detail.requests_sent(), u64::from(index == 0));
                let usage = detail.usage().expect("reported usage share");
                assert_eq!(usage.input_tokens(), if index == 0 { 2 } else { 1 });
                assert_eq!(usage.output_tokens(), u64::from(index == 0));
                seen.lock().expect("observations").push((index, "question"));
            }
            RecordObservation::Row { index, value } => {
                assert!(matches!(
                    value,
                    ObservedRow::Judgment(Judgment::Decision(_))
                ));
                seen.lock().expect("observations").push((index, "row"));
            }
        }
    };
    let setting = BatchSetting::Records(std::num::NonZeroUsize::new(2).expect("two"));
    let asked = question();
    let mut batch = engine.filter_with(
        &asked,
        ["alpha", "beta"],
        CallOptions::new().batch(setting).observe(&observe),
    );
    let kept = batch
        .by_ref()
        .collect::<Result<Vec<_>, _>>()
        .expect("filtered");
    assert_eq!(kept, ["beta"]);
    assert_eq!(batch.facts().map(|facts| facts.records()), Some(2));
    assert_eq!(listener.count(), 1);
    assert_eq!(
        *seen.lock().expect("observations"),
        [(0, "question"), (0, "row"), (1, "question"), (1, "row")]
    );
}

#[test]
fn find_observer_names_its_actual_question_and_selected_unit() {
    let _serial = serial();
    let backend = Backend::start().expect("backend");
    let engine = engine(&format!("{}/generic/v1", backend.origin()));
    let seen = Mutex::new(Vec::new());
    let observe = |event: RecordObservation<'_>| match event {
        RecordObservation::Question { detail, .. } => {
            assert_eq!(detail.question_sha256().len(), 64);
            assert_eq!(detail.requests().len(), 1);
            assert!(matches!(detail.value(), Some(Judgment::Choice(_))));
            seen.lock().expect("observations").push("question");
        }
        RecordObservation::Row { value, .. } => {
            assert!(matches!(value, ObservedRow::Find(Some(0))));
            seen.lock().expect("observations").push("row");
        }
    };
    let asked = Question::find("Which unit?").expect("find");
    let found = engine
        .find_with(
            &asked,
            ["first", "second"],
            CallOptions::new().observe(&observe),
        )
        .expect("found");
    assert_eq!(found.value().selected(), Some(&"first"));
    assert_eq!(backend.count(), 1);
    assert_eq!(*seen.lock().expect("observations"), ["question", "row"]);
}

#[test]
fn eager_observer_panic_returns_its_payload_after_the_call() {
    let _serial = serial();
    let listener = Listener::answering(|_| Canned::ok(DECIDED)).expect("listener");
    let engine = engine(listener.base());
    let calls = AtomicUsize::new(0);
    let observe = |_: RecordObservation<'_>| {
        calls.fetch_add(1, Ordering::SeqCst);
        std::panic::resume_unwind(Box::new("observer payload"));
    };
    let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        engine.decide_with(&question(), "alpha", CallOptions::new().observe(&observe))
    }));
    let Err(payload) = caught else {
        panic!("observer panic did not reach its caller");
    };
    assert_eq!(payload.downcast_ref::<&str>(), Some(&"observer payload"));
    assert_eq!(
        calls.load(Ordering::SeqCst),
        1,
        "no Row callback after Question panic"
    );
    assert_eq!(listener.count(), 1);
    assert_eq!(
        engine
            .decide(&question(), "beta")
            .expect("next call")
            .value(),
        &Answer::Yes
    );
    assert_eq!(listener.count(), 2);
}

#[test]
fn annotation_observer_keeps_named_success_and_failure_in_set_order() {
    let _serial = serial();
    let reply = r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9}},"usage":{"input_tokens":3,"output_tokens":1}}"#;
    let listener = Listener::answering(move |_| Canned::ok(reply)).expect("listener");
    let engine = engine(listener.base());
    let set = QuestionSet::from_json(
        r#"{"version":1,"questions":{"first":{"decide":"First?"},"second":{"decide":"Second?"}}}"#,
    )
    .expect("set");
    let seen = Mutex::new(Vec::new());
    let observe = |event: RecordObservation<'_>| match event {
        RecordObservation::Question {
            index,
            member,
            detail,
            ..
        } => {
            assert_eq!(index, 0);
            assert_eq!(detail.question_sha256().len(), 64);
            assert_eq!(detail.requests().len(), 1);
            assert_eq!(detail.model(), "jev-latest");
            assert_eq!(detail.url(), listener.url());
            seen.lock().expect("observations").push((
                member.expect("member").to_owned(),
                detail.failure(),
                detail.requests_sent(),
                detail.usage().expect("usage share").input_tokens(),
            ));
        }
        RecordObservation::Row { index, value } => {
            assert_eq!(index, 0);
            let ObservedRow::Annotated(values) = value else {
                panic!("an annotated row");
            };
            assert_eq!(
                values.iter().map(|one| one.name()).collect::<Vec<_>>(),
                ["first", "second"]
            );
            assert!(matches!(values[1].value(), thinkthen::Annotated::Failed(_)));
        }
    };
    let rows = engine
        .annotate_with(&set, ["alpha"], CallOptions::new().observe(&observe))
        .collect::<Result<Vec<_>, _>>()
        .expect("one partly answered annotation");
    assert_eq!(rows.len(), 1);
    assert_eq!(listener.count(), 1);
    assert_eq!(
        *seen.lock().expect("observations"),
        [
            ("first".to_owned(), None, 1, 2),
            (
                "second".to_owned(),
                Some(thinkthen::FailureCause::MissingAnswer),
                0,
                1,
            ),
        ]
    );
}

#[test]
fn recognition_observer_names_boundary_then_kind_requests() {
    let _serial = serial();
    let listener = Listener::answering(|body| {
        let request: serde_json::Value = serde_json::from_slice(body).expect("request");
        let mut answers = serde_json::Map::new();
        for (name, question) in request["questions"].as_object().expect("questions") {
            let labels = question["criteria"].as_object().expect("labels");
            let picked = if labels.contains_key("SINGLE") {
                "SINGLE"
            } else {
                "person"
            };
            let probabilities = labels
                .keys()
                .map(|label| (label.clone(), serde_json::json!(u8::from(label == picked))))
                .collect::<serde_json::Map<_, _>>();
            answers.insert(
                name.clone(),
                serde_json::json!({"type":"choice","choice":picked,"probabilities":probabilities}),
            );
        }
        Canned::ok(
            &serde_json::json!({"model":"jev-latest","answers":answers,"usage":{"input_tokens":3,"output_tokens":1}})
                .to_string(),
        )
    })
    .expect("listener");
    let engine = engine(listener.base());
    let asked = thinkthen::Recognize::builder()
        .kind(thinkthen::Kind::new("person", None).expect("kind"))
        .and_then(thinkthen::RecognizeBuilder::build)
        .expect("recognize");
    let seen = Mutex::new(Vec::new());
    let observe = |event: RecordObservation<'_>| match event {
        RecordObservation::Question {
            stage,
            position,
            detail,
            ..
        } => {
            assert_eq!(detail.question_sha256().len(), 64);
            assert_eq!(detail.requests().len(), 1);
            assert_eq!(detail.requests_sent(), 1);
            seen.lock()
                .expect("observations")
                .push((stage.expect("stage"), position));
        }
        RecordObservation::Row { value, .. } => {
            assert!(matches!(value, ObservedRow::Recognized(_)));
            seen.lock().expect("observations").push(("row", 0));
        }
    };
    let recognized = engine
        .recognize_with(&asked, "Ada", CallOptions::new().observe(&observe))
        .expect("recognized");
    assert_eq!(recognized.value().entities().len(), 1);
    assert_eq!(listener.count(), 2);
    assert_eq!(
        *seen.lock().expect("observations"),
        [("boundary", 0), ("kind", 0), ("row", 0)]
    );
}

#[test]
fn relation_observer_names_actual_pair_before_final_edges() {
    let _serial = serial();
    let backend = Backend::start().expect("backend");
    let engine = engine(&format!("{}/generic/v1", backend.origin()));
    let asked = thinkthen::Relate::builder()
        .relation(
            thinkthen::RelationRule::one_way("works_with", "person", "organization").expect("rule"),
        )
        .and_then(thinkthen::RelateBuilder::build)
        .expect("relate");
    let entities = [
        thinkthen::Entity::new("Ada", "person").expect("person"),
        thinkthen::Entity::new("Acme", "organization").expect("organization"),
    ];
    let seen = Mutex::new(Vec::new());
    let observe = |event: RecordObservation<'_>| match event {
        RecordObservation::Question {
            stage,
            position,
            detail,
            ..
        } => {
            assert_eq!(stage, Some("relation"));
            assert_eq!(position, 0);
            assert_eq!(detail.question_sha256().len(), 64);
            assert_eq!(detail.requests().len(), 1);
            assert_eq!(detail.requests_sent(), 1);
            seen.lock().expect("observations").push("question");
        }
        RecordObservation::Row { value, .. } => {
            let ObservedRow::Relations(edges) = value else {
                panic!("relation row");
            };
            assert_eq!(edges.len(), 1);
            assert_eq!(edges[0].relation(), "works_with");
            seen.lock().expect("observations").push("row");
        }
    };
    let edges = engine
        .relate_with(&asked, entities, CallOptions::new().observe(&observe))
        .expect("related");
    assert_eq!(edges.value().len(), 1);
    assert_eq!(backend.count(), 1);
    assert_eq!(*seen.lock().expect("observations"), ["question", "row"]);
}

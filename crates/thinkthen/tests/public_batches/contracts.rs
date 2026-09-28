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

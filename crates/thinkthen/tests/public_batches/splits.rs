//! Refused requests and their one split at the public boundary.

use super::*;
fn packed() -> CallOptions<'static> {
    CallOptions::new().batch(BatchSetting::Records(
        std::num::NonZeroUsize::new(2).expect("two"),
    ))
}

#[test]
fn named_groups_pack_two_rows_and_keep_ordered_observations() {
    let _serial = serial();
    let reply = r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.8},"q3":{"type":"noul","noul":0.7},"q4":{"type":"noul","noul":0.6}},"usage":{"input_tokens":4,"output_tokens":2}}"#;
    let listener = Listener::serving(vec![Canned::ok(reply)]).expect("scripted listener");
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
        } => seen.lock().expect("events").push((
            index,
            member.expect("name").to_owned(),
            detail.requests_sent(),
        )),
        RecordObservation::Row { .. } => {}
    };
    let mut rows = engine.annotate_with(&set, ["alpha", "beta"], packed().observe(&observe));
    let values = rows.by_ref().collect::<Result<Vec<_>, _>>().expect("rows");
    assert_eq!(values.len(), 2);
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    assert_eq!(
        rows.facts()
            .map(|facts| (facts.records(), facts.requests_sent())),
        Some((2, 1))
    );
    assert_eq!(
        *seen.lock().expect("events"),
        [
            (0, "first".to_owned(), 1),
            (0, "second".to_owned(), 0),
            (1, "first".to_owned(), 0),
            (1, "second".to_owned(), 0),
        ]
    );
    let body: serde_json::Value = serde_json::from_slice(&requests[0].body).expect("body");
    assert_eq!(
        body["state"],
        "Each question quotes the text it asks about."
    );
    assert_eq!(
        body["questions"],
        serde_json::json!({
            "q1": {"type": "noul", "instructions": "The text is \"alpha\". First?"},
            "q2": {"type": "noul", "instructions": "The text is \"alpha\". Second?"},
            "q3": {"type": "noul", "instructions": "The text is \"beta\". First?"},
            "q4": {"type": "noul", "instructions": "The text is \"beta\". Second?"},
        })
    );
}

#[test]
fn a_record_with_no_usable_question_stops_after_the_usable_prefix() {
    let _serial = serial();
    // One request holds every record's two groups: q1 and q2 for the first
    // record, q3 and q4 for the second, q5 and q6 for the third.
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"jev-latest","answers":{"q2":{"type":"noul","noul":0.9},"q5":{"type":"noul","noul":0.9},"q6":{"type":"noul","noul":0.9}}}"#)
    })
    .expect("listener");
    let engine = engine(listener.base());
    let set = QuestionSet::from_json(
        r#"{"version":1,"questions":{"a":{"decide":"Group A?","on":"/a"},"b":{"decide":"Group B?","on":"/b"}}}"#,
    )
    .expect("set");
    let seen = Mutex::new(Vec::new());
    let observe = |event: RecordObservation<'_>| match event {
        RecordObservation::Question {
            index,
            member,
            detail,
            ..
        } => seen.lock().expect("events").push((
            index,
            member.expect("name").to_owned(),
            detail.failure().is_some(),
        )),
        RecordObservation::Row { index, .. } => {
            seen.lock()
                .expect("events")
                .push((index, "row".to_owned(), false))
        }
    };
    let records = [
        r#"{"a":"a1","b":"b1"}"#,
        r#"{"a":"a2","b":"b2"}"#,
        r#"{"a":"a3","b":"b3"}"#,
    ];
    let mut rows = engine.annotate_with(&set, records, CallOptions::new().observe(&observe));
    let first = rows.next().expect("row one").expect("usable sibling");
    assert_eq!(first.input(), &records[0]);
    let error = rows
        .next()
        .expect("row two stop")
        .expect_err("no usable sibling");
    assert_eq!(error.kind(), ErrorKind::Backend);
    assert!(rows.next().is_none());
    assert_eq!(
        rows.facts()
            .map(|facts| (facts.records(), facts.requests_sent())),
        Some((1, 1))
    );
    assert_eq!(listener.requests().len(), 1);
    assert_eq!(
        *seen.lock().expect("events"),
        [
            (0, "a".to_owned(), true),
            (0, "b".to_owned(), false),
            (0, "row".to_owned(), false),
            (1, "a".to_owned(), true),
            (1, "b".to_owned(), true),
        ]
    );
}

#[test]
fn a_halved_request_keeps_one_question_key_per_member() {
    let _serial = serial();
    let reply = r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.8}}}"#;
    let listener = Listener::serving(vec![
        Canned::status(413, "too large"),
        Canned::ok(reply),
        Canned::ok(reply),
    ])
    .expect("listener");
    let engine = engine(listener.base());
    let set = QuestionSet::from_json(
        r#"{"version":1,"questions":{"first":{"decide":"First?"},"second":{"decide":"Second?"}}}"#,
    )
    .expect("set");
    let seen = Mutex::new(Vec::new());
    let observe = |event: RecordObservation<'_>| {
        if let RecordObservation::Question {
            index,
            member,
            detail,
            ..
        } = event
        {
            seen.lock().expect("events").push((
                index,
                member.expect("name").to_owned(),
                detail.requests().to_vec(),
                detail.requests_sent(),
            ));
        }
    };
    let mut rows = engine.annotate_with(&set, ["alpha", "beta"], packed().observe(&observe));
    assert_eq!(
        rows.by_ref()
            .collect::<Result<Vec<_>, _>>()
            .expect("rows")
            .len(),
        2
    );
    assert_eq!(
        rows.facts()
            .map(|facts| (facts.records(), facts.requests_sent())),
        Some((2, 3))
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 3);
    let keys = super::identity::question_keys(listener.url(), &requests[0].body);
    let member = |index: usize, name: &str, key: usize, sent: u64| {
        (index, name.to_owned(), vec![keys[key].clone()], sent)
    };
    assert_eq!(
        *seen.lock().expect("events"),
        [
            member(0, "first", 0, 1),
            member(0, "second", 1, 1),
            member(1, "first", 2, 1),
            member(1, "second", 3, 0),
        ]
    );
}

#[test]
fn profile_splits_each_incompatible_named_group_before_sending() {
    let _serial = serial();
    let (alpha_sender, alpha_answered) = std::sync::mpsc::channel();
    let (beta_sender, beta_answered) = std::sync::mpsc::channel();
    let listener = Listener::answering(move |body| {
        let request: serde_json::Value = serde_json::from_slice(body).expect("request");
        assert_eq!(
            request["questions"].as_object().expect("questions").len(),
            1
        );
        let text = String::from_utf8_lossy(body);
        if text.contains("alpha") && text.contains("First?") {
            Canned::ok(DECIDED)
                .after(150)
                .notifying(alpha_sender.clone())
        } else if text.contains("beta") && text.contains("First?") {
            Canned::ok(DECIDED).notifying(beta_sender.clone())
        } else {
            Canned::ok(DECIDED)
        }
    })
    .expect("listener");
    let completion = thread::spawn(move || {
        beta_answered
            .recv_timeout(Duration::from_secs(2))
            .expect("later row answered");
        alpha_answered.try_recv().is_err()
    });
    let engine = Engine::builder()
        .base_url(listener.base())
        .expect("base")
        .api_key("sk-public-batches")
        .expect("key")
        .throttle(THROTTLE)
        .expect("throttle")
        .profile_json(
            r#"{"schema":"thinkthen.backend-profile/1","name":"one-question","max_questions":1}"#,
        )
        .expect("profile")
        .no_cache()
        .build()
        .expect("engine");
    let set = QuestionSet::from_json(
        r#"{"version":1,"questions":{"first":{"decide":"First?"},"second":{"decide":"Second?"}}}"#,
    )
    .expect("set");
    let mut rows = engine.annotate_with(&set, ["alpha", "beta"], packed());
    assert_eq!(
        rows.by_ref()
            .collect::<Result<Vec<_>, _>>()
            .expect("rows")
            .len(),
        2
    );
    assert_eq!(
        rows.facts()
            .map(|facts| (facts.records(), facts.requests_sent())),
        Some((2, 4))
    );
    assert_eq!(listener.requests().len(), 4);
    assert!(
        completion.join().expect("completion order"),
        "later row answered before the held earlier chunk"
    );
}

#[test]
fn compatible_profile_group_still_packs_multiple_rows() {
    let _serial = serial();
    let reply = r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.9},"q3":{"type":"noul","noul":0.9},"q4":{"type":"noul","noul":0.9}}}"#;
    let listener = Listener::serving(vec![Canned::ok(reply)]).expect("listener");
    let engine = Engine::builder()
        .base_url(listener.base())
        .expect("base")
        .api_key("sk-public-batches")
        .expect("key")
        .profile_json(r#"{"schema":"thinkthen.backend-profile/1","name":"four","max_questions":4}"#)
        .expect("profile")
        .no_cache()
        .build()
        .expect("engine");
    let set = QuestionSet::from_json(
        r#"{"version":1,"questions":{"first":{"decide":"First?"},"second":{"decide":"Second?"}}}"#,
    )
    .expect("set");
    let mut rows = engine.annotate(&set, ["alpha", "beta"]);
    assert_eq!(
        rows.by_ref()
            .collect::<Result<Vec<_>, _>>()
            .expect("rows")
            .len(),
        2
    );
    assert_eq!(rows.facts().map(|facts| facts.requests_sent()), Some(1));
    assert_eq!(listener.requests().len(), 1);
}

#[test]
fn a_later_profile_group_refuses_before_any_request() {
    let _serial = serial();
    let listener = Listener::answering(|_| Canned::ok(DECIDED)).expect("listener");
    let engine = Engine::builder()
        .base_url(listener.base())
        .expect("base")
        .api_key("sk-public-batches")
        .expect("key")
        .profile_json(
            r#"{"schema":"thinkthen.backend-profile/1","name":"tiny","max_evidence_bytes":1}"#,
        )
        .expect("profile")
        .no_cache()
        .build()
        .expect("engine");
    let set = QuestionSet::from_json(
        r#"{"version":1,"questions":{"first":{"decide":"First?","on":"/a"},"second":{"decide":"Second?","on":"/b"}}}"#,
    ).expect("set");
    let mut rows = engine.annotate(&set, [r#"{"a":"x","b":"long"}"#]);
    assert_eq!(
        rows.next().expect("refusal").expect_err("profile").kind(),
        ErrorKind::Usage
    );
    assert!(rows.next().is_none());
    assert_eq!(listener.count(), 0);
}

#[test]
fn a_refused_request_and_both_halves_keep_exact_send_shares() {
    let _serial = serial();
    let listener = Listener::serving(vec![
        Canned::status(413, "too large"),
        Canned::ok(DECIDED),
        Canned::ok(DECIDED),
    ])
    .expect("scripted listener");
    let engine = engine(listener.base());
    let seen = Mutex::new(Vec::new());
    let observe = |event: RecordObservation<'_>| {
        if let RecordObservation::Question { index, detail, .. } = event {
            seen.lock().expect("observations").push((
                index,
                detail.requests().to_vec(),
                detail.requests_sent(),
            ));
        }
    };
    let question = question();
    let mut rows =
        engine.decide_many_with(&question, ["alpha", "beta"], packed().observe(&observe));
    let values = rows
        .by_ref()
        .collect::<Result<Vec<_>, _>>()
        .expect("halves");
    assert_eq!(values.len(), 2);
    assert!(values.iter().all(|row| *row.value() == Answer::Yes));
    assert_eq!(
        rows.facts()
            .map(|facts| (facts.records(), facts.requests_sent())),
        Some((2, 3))
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 3);
    let sizes = requests
        .iter()
        .map(|request| {
            let body: serde_json::Value = serde_json::from_slice(&request.body).expect("body");
            body["questions"].as_object().expect("questions").len()
        })
        .collect::<Vec<_>>();
    assert_eq!(sizes, [2, 1, 1]);
    let keys = super::identity::question_keys(listener.url(), &requests[0].body);
    let seen = seen.lock().expect("observations");
    assert_eq!(
        seen.as_slice(),
        [(0, vec![keys[0].clone()], 2), (1, vec![keys[1].clone()], 1)]
    );
}

#[test]
fn a_failed_left_half_prevents_a_right_send_and_row() {
    let _serial = serial();
    let missing = r#"{"model":"jev-latest","answers":{}}"#;
    let listener = Listener::serving(vec![
        Canned::status(413, "too large"),
        Canned::ok(missing),
        Canned::ok(DECIDED),
    ])
    .expect("scripted listener");
    let engine = engine(listener.base());
    let seen = Mutex::new(Vec::new());
    let observe = |event: RecordObservation<'_>| {
        if let RecordObservation::Question { index, detail, .. } = event {
            seen.lock().expect("observations").push((
                index,
                detail.failure(),
                detail.requests().to_vec(),
                detail.requests_sent(),
            ));
        }
    };
    let question = question();
    let mut rows =
        engine.decide_many_with(&question, ["alpha", "beta"], packed().observe(&observe));
    let error = rows
        .next()
        .expect("terminal error")
        .expect_err("failed left");
    assert_eq!(error.kind(), ErrorKind::Backend);
    assert!(rows.next().is_none());
    assert_eq!(
        rows.facts()
            .map(|facts| (facts.records(), facts.requests_sent())),
        Some((0, 2))
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 2, "the right half was not sent");
    assert!(seen.lock().expect("observations").is_empty());
}

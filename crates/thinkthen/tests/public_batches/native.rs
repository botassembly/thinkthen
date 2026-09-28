//! Recoverable native details beside the stopping public stream.

use super::splits::digest;
use super::*;
use thinkthen::SendBudget;

const SPLIT_ORIGINAL: &str = r#"{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"The text is \"alpha\". Does this ask for a refund?"},"q2":{"type":"noul","instructions":"The text is \"beta\". Does this ask for a refund?"},"q3":{"type":"noul","instructions":"The text is \"gamma\". Does this ask for a refund?"},"q4":{"type":"noul","instructions":"The text is \"delta\". Does this ask for a refund?"}}}"#;
const SPLIT_LEFT: &str = r#"{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"The text is \"alpha\". Does this ask for a refund?"},"q2":{"type":"noul","instructions":"The text is \"beta\". Does this ask for a refund?"}}}"#;

#[test]
#[allow(
    clippy::cognitive_complexity,
    clippy::too_many_lines,
    reason = "one exact split fixture compares native continuation and stopping on both left failure forms"
)]
fn native_recovery_continues_after_one_failed_left_member_but_stopping_rows_do_not() {
    let _serial = serial();
    let left = r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9}}}"#;
    let right = r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.8},"q2":{"type":"noul","noul":0.7}}}"#;
    let question = question();
    let texts = ["alpha", "beta", "gamma", "delta"];
    let options = CallOptions::new().batch(BatchSetting::Records(
        std::num::NonZeroUsize::new(4).expect("four"),
    ));

    let listener = Listener::serving(vec![
        Canned::status(413, "too large"),
        Canned::ok(left),
        Canned::ok(right),
    ])
    .expect("native listener");
    let native_engine = engine(listener.base());
    let result = native_engine
        .details_many_recoverable_with(&question, &texts.map(str::to_owned), options)
        .expect("recoverable native vector");
    assert_eq!(
        (result.facts().records(), result.facts().requests_sent()),
        (4, 3)
    );
    assert!(matches!(
        result.value()[0],
        thinkthen::RecoverableDetails::Answered(_)
    ));
    assert!(matches!(
        result.value()[1],
        thinkthen::RecoverableDetails::Failed {
            kind: ErrorKind::Backend,
            retryable: false,
            cause: Some(_)
        }
    ));
    assert!(matches!(
        result.value()[2],
        thinkthen::RecoverableDetails::Answered(_)
    ));
    assert!(matches!(
        result.value()[3],
        thinkthen::RecoverableDetails::Answered(_)
    ));
    let requests = listener.requests();
    assert_eq!(requests.len(), 3);
    let expected = [
        SPLIT_ORIGINAL,
        SPLIT_LEFT,
        r#"{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"The text is \"gamma\". Does this ask for a refund?"},"q2":{"type":"noul","instructions":"The text is \"delta\". Does this ask for a refund?"}}}"#,
    ];
    for (actual, expected) in requests.iter().zip(expected) {
        assert_eq!(actual.body, expected.as_bytes());
    }
    let digests = expected.map(|body| digest(listener.url(), body.as_bytes()));
    for (place, half) in [(0, 1), (2, 2), (3, 2)] {
        let thinkthen::RecoverableDetails::Answered(details) = &result.value()[place] else {
            panic!("answered member")
        };
        assert_eq!(
            details.requests(),
            &[digests[0].clone(), digests[half].clone()]
        );
    }

    let listener = Listener::serving(vec![
        Canned::status(413, "too large"),
        Canned::ok(left),
        Canned::ok(right),
    ])
    .expect("stopping listener");
    let stopping_engine = engine(listener.base());
    let mut rows = stopping_engine.decide_many_with(&question, texts, options);
    assert_eq!(
        *rows.next().expect("first").expect("answered").value(),
        Answer::Yes
    );
    assert_eq!(
        rows.next()
            .expect("stop")
            .expect_err("failed member")
            .kind(),
        ErrorKind::Backend
    );
    assert!(rows.next().is_none());
    assert_eq!(
        rows.facts()
            .map(|facts| (facts.records(), facts.requests_sent())),
        Some((1, 2))
    );
    assert_eq!(listener.requests().len(), 2);

    let listener = Listener::serving(vec![
        Canned::status(413, "too large"),
        Canned::status(400, "bad request"),
        Canned::ok(right),
    ])
    .expect("request failure listener");
    let native_engine = engine(listener.base());
    let result = native_engine
        .details_many_recoverable_with(&question, &texts.map(str::to_owned), options)
        .expect("recoverable left request");
    assert_eq!(
        (result.facts().records(), result.facts().requests_sent()),
        (4, 3)
    );
    for place in [0, 1] {
        assert!(matches!(
            result.value()[place],
            thinkthen::RecoverableDetails::Failed {
                kind: ErrorKind::Backend,
                retryable: false,
                cause: None
            }
        ));
    }
    assert!(
        result.value()[2..]
            .iter()
            .all(|row| matches!(row, thinkthen::RecoverableDetails::Answered(_)))
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 3);
    for (actual, expected) in requests.iter().zip(expected) {
        assert_eq!(actual.body, expected.as_bytes());
    }

    let listener = Listener::serving(vec![
        Canned::status(413, "too large"),
        Canned::status(400, "bad request"),
        Canned::ok(right),
    ])
    .expect("stopping request listener");
    let stopping_engine = engine(listener.base());
    let mut rows = stopping_engine.decide_many_with(&question, texts, options);
    assert_eq!(
        rows.next().expect("stop").expect_err("left failed").kind(),
        ErrorKind::Backend
    );
    assert!(rows.next().is_none());
    assert_eq!(
        rows.facts()
            .map(|facts| (facts.records(), facts.requests_sent())),
        Some((0, 2))
    );
    assert_eq!(listener.requests().len(), 2);
}

#[test]
fn native_denied_left_fills_unsent_half_and_keeps_stopping_stream_terminal() {
    let _serial = serial();
    let texts = ["alpha", "beta", "gamma", "delta"].map(str::to_owned);
    let question = question();
    let setting = BatchSetting::Records(std::num::NonZeroUsize::new(4).expect("four"));

    let budget = SendBudget::new();
    let listener = Listener::serving(vec![Canned::status(413, "too large")]).expect("listener");
    let result = engine(listener.base())
        .details_many_recoverable_with(
            &question,
            &texts,
            CallOptions::new()
                .batch(setting)
                .send_budget(&budget, Some(1)),
        )
        .expect("safe denied left and unsent right");
    assert_eq!(
        (result.facts().records(), result.facts().requests_sent()),
        (4, 1)
    );
    assert!(result.value().iter().all(|row| matches!(
        row,
        thinkthen::RecoverableDetails::Failed {
            kind: ErrorKind::Usage,
            retryable: false,
            cause: None
        }
    )));
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].body, SPLIT_ORIGINAL.as_bytes());

    let budget = SendBudget::new();
    let listener = Listener::serving(vec![Canned::status(413, "too large")]).expect("listener");
    let stopping_engine = engine(listener.base());
    let mut stopping = stopping_engine.decide_many_with(
        &question,
        texts,
        CallOptions::new()
            .batch(setting)
            .send_budget(&budget, Some(1)),
    );
    assert_eq!(
        stopping
            .next()
            .expect("left stop")
            .expect_err("denied left")
            .kind(),
        ErrorKind::Usage
    );
    assert!(stopping.next().is_none());
    assert_eq!(
        stopping
            .facts()
            .map(|facts| (facts.records(), facts.requests_sent())),
        Some((0, 1))
    );
    assert_eq!(listener.requests().len(), 1);
}

#[test]
fn native_denied_right_keeps_answered_left_with_both_request_receipts() {
    let _serial = serial();
    let texts = ["alpha", "beta", "gamma", "delta"].map(str::to_owned);
    let question = question();
    let setting = BatchSetting::Records(std::num::NonZeroUsize::new(4).expect("four"));
    let budget = SendBudget::new();
    let left = r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.8}}}"#;
    let listener = Listener::serving(vec![Canned::status(413, "too large"), Canned::ok(left)])
        .expect("listener");
    let result = engine(listener.base())
        .details_many_recoverable_with(
            &question,
            &texts,
            CallOptions::new()
                .batch(setting)
                .send_budget(&budget, Some(2)),
        )
        .expect("answered left and safe denied right");
    assert_eq!(
        (result.facts().records(), result.facts().requests_sent()),
        (4, 2)
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 2);
    assert_eq!(requests[0].body, SPLIT_ORIGINAL.as_bytes());
    assert_eq!(requests[1].body, SPLIT_LEFT.as_bytes());
    let digests = [SPLIT_ORIGINAL, SPLIT_LEFT].map(|body| digest(listener.url(), body.as_bytes()));
    for row in &result.value()[..2] {
        let thinkthen::RecoverableDetails::Answered(details) = row else {
            panic!("answered left member")
        };
        assert_eq!(details.requests(), &digests);
    }
    assert!(result.value()[2..].iter().all(|row| matches!(
        row,
        thinkthen::RecoverableDetails::Failed {
            kind: ErrorKind::Usage,
            retryable: false,
            cause: None
        }
    )));
}

#[test]
fn native_denied_retry_is_usage_without_a_second_send() {
    let _serial = serial();
    let budget = SendBudget::new();
    let listener =
        Listener::answering(|_| Canned::status(503, "busy").asking("retry-after-ms", "0"))
            .expect("listener");
    let engine = Engine::builder()
        .base_url(listener.base())
        .and_then(|builder| builder.api_key("sk-public-batches"))
        .and_then(|builder| builder.throttle(THROTTLE))
        .map(EngineBuilder::no_cache)
        .map(|builder| builder.max_retries(1))
        .and_then(EngineBuilder::build)
        .expect("engine");
    let result = engine
        .details_many_recoverable_with(
            &question(),
            &["alpha".to_owned()],
            CallOptions::new().send_budget(&budget, Some(1)),
        )
        .expect("safe denied retry");
    assert_eq!(
        (result.facts().records(), result.facts().requests_sent()),
        (1, 1)
    );
    assert!(matches!(
        result.value()[0],
        thinkthen::RecoverableDetails::Failed {
            kind: ErrorKind::Usage,
            retryable: false,
            cause: None
        }
    ));
    assert_eq!(listener.requests().len(), 1);
}

#[test]
fn native_recovery_keeps_closed_prefix_before_one_invalid_member() {
    let _serial = serial();
    let listener =
        Listener::serving(vec![Canned::ok(DECIDED), Canned::ok(DECIDED)]).expect("listener");
    let engine = engine(listener.base());
    let result = engine
        .details_many_recoverable_with(
            &question(),
            &["alpha".to_owned(), " ".to_owned(), "beta".to_owned()],
            CallOptions::new(),
        )
        .expect("recoverable rows");
    assert!(matches!(
        result.value()[0],
        thinkthen::RecoverableDetails::Answered(_)
    ));
    assert!(matches!(
        result.value()[1],
        thinkthen::RecoverableDetails::Failed {
            kind: ErrorKind::Usage,
            retryable: false,
            cause: None
        }
    ));
    assert!(matches!(
        result.value()[2],
        thinkthen::RecoverableDetails::Answered(_)
    ));
    assert_eq!(
        (result.facts().records(), result.facts().requests_sent()),
        (3, 2)
    );
    assert_eq!(listener.requests().len(), 2);
}

//! Recoverable native outcomes beside the stopping public stream.

use super::identity::question_keys;
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
    let result = crate::api::recover_decisions(
        &native_engine,
        &question,
        &texts.map(str::to_owned),
        options,
    )
    .expect("recoverable native vector");
    assert_eq!(
        (result.facts().records(), result.facts().requests_sent()),
        (4, 3)
    );
    assert!(result.value()[0].outcome().is_ok());
    assert_failed(
        &result.value()[1],
        ErrorKind::Backend,
        thinkthen::StopCause::Reply,
    );
    assert!(result.value()[2].outcome().is_ok());
    assert!(result.value()[3].outcome().is_ok());
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
    let keys = question_keys(listener.url(), SPLIT_ORIGINAL.as_bytes(), "jev-latest");
    for place in [0, 2, 3] {
        let thinkthen::RequestValue::Decisions(rows) =
            result.value()[place].outcome().expect("answered member")
        else {
            panic!("decision rows");
        };
        assert_eq!(
            rows[0].result().meta().requests(),
            std::slice::from_ref(&keys[place])
        );
    }

    let listener = Listener::serving(vec![
        Canned::status(413, "too large"),
        Canned::ok(left),
        Canned::ok(right),
    ])
    .expect("stopping listener");
    let stopping_engine = engine(listener.base());
    let mut rows = crate::api::pull_decisions(
        &stopping_engine,
        &question,
        crate::api::plain_records(texts),
        options,
    );
    assert_eq!(
        rows.next()
            .expect("first")
            .expect("answered")
            .result()
            .value(),
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
    // A halved request sends its first half, then its second, in one slot,
    // by ADR 0111 section 6. The failed row stops only later requests.
    assert_eq!(
        rows.facts()
            .map(|facts| (facts.records(), facts.requests_sent())),
        Some((1, 3))
    );
    assert_eq!(listener.requests().len(), 3);

    let listener = Listener::serving(vec![
        Canned::status(413, "too large"),
        Canned::status(400, "bad request"),
        Canned::ok(right),
    ])
    .expect("request failure listener");
    let native_engine = engine(listener.base());
    let result = crate::api::recover_decisions(
        &native_engine,
        &question,
        &texts.map(str::to_owned),
        options,
    )
    .expect("recoverable left request");
    assert_eq!(
        (result.facts().records(), result.facts().requests_sent()),
        (4, 3)
    );
    for place in [0, 1] {
        assert_failed(
            &result.value()[place],
            ErrorKind::Backend,
            thinkthen::StopCause::Status,
        );
    }
    assert!(result.value()[2..].iter().all(|row| row.outcome().is_ok()));
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
    let mut rows = crate::api::pull_decisions(
        &stopping_engine,
        &question,
        crate::api::plain_records(texts),
        options,
    );
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
    let result = crate::api::recover_decisions(
        &engine(listener.base()),
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
    for row in result.value().iter() {
        assert_failed(row, ErrorKind::Usage, thinkthen::StopCause::Usage);
    }
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].body, SPLIT_ORIGINAL.as_bytes());

    let budget = SendBudget::new();
    let listener = Listener::serving(vec![Canned::status(413, "too large")]).expect("listener");
    let stopping_engine = engine(listener.base());
    let mut stopping = crate::api::pull_decisions(
        &stopping_engine,
        &question,
        crate::api::plain_records(texts),
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
fn native_denied_right_keeps_answered_left_with_its_question_keys() {
    let _serial = serial();
    let texts = ["alpha", "beta", "gamma", "delta"].map(str::to_owned);
    let question = question();
    let setting = BatchSetting::Records(std::num::NonZeroUsize::new(4).expect("four"));
    let budget = SendBudget::new();
    let left = r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.8}}}"#;
    let listener = Listener::serving(vec![Canned::status(413, "too large"), Canned::ok(left)])
        .expect("listener");
    let result = crate::api::recover_decisions(
        &engine(listener.base()),
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
    let keys = question_keys(listener.url(), SPLIT_ORIGINAL.as_bytes(), "jev-latest");
    for (row, key) in result.value()[..2].iter().zip(&keys) {
        let thinkthen::RequestValue::Decisions(rows) = row.outcome().expect("answered left member")
        else {
            panic!("decision rows");
        };
        assert_eq!(
            rows[0].result().meta().requests(),
            std::slice::from_ref(key)
        );
    }
    for row in result.value()[2..].iter() {
        assert_failed(row, ErrorKind::Usage, thinkthen::StopCause::Usage);
    }
}

#[test]
fn native_denied_retry_is_usage_without_a_second_send() {
    let _serial = serial();
    let budget = SendBudget::new();
    let listener =
        Listener::answering(|_| Canned::status(503, "busy").asking("retry-after-ms", "0"))
            .expect("listener");
    let result = crate::api::recover_decisions(
        &retrying(listener.base()),
        &question(),
        &["alpha".to_owned()],
        CallOptions::new().send_budget(&budget, Some(1)),
    )
    .expect("safe denied retry");
    assert_eq!(
        (result.facts().records(), result.facts().requests_sent()),
        (1, 1)
    );
    assert_failed(
        &result.value()[0],
        ErrorKind::Usage,
        thinkthen::StopCause::Usage,
    );
    assert_eq!(listener.requests().len(), 1);
}

/// An engine with one retry, no cache and the shared throttle.
fn retrying(base: &str) -> Engine {
    Engine::builder()
        .base_url(base)
        .and_then(|builder| builder.api_key("sk-public-batches"))
        .and_then(|builder| builder.throttle(THROTTLE))
        .map(EngineBuilder::no_cache)
        .map(|builder| builder.max_retries(1))
        .and_then(EngineBuilder::build)
        .expect("engine")
}

/// Ticket 0240: a retry denied by the send budget fails only its own
/// request's rows. A later request that answered first keeps its answers.
#[test]
fn native_denied_retry_keeps_a_later_request_answered_first() {
    let _serial = serial();
    let answered = std::sync::Arc::new((std::sync::Mutex::new(false), std::sync::Condvar::new()));
    let held = std::sync::Arc::clone(&answered);
    let listener = Listener::answering(move |body| {
        let (done, signal) = &*held;
        let mut done = done.lock().expect("flag");
        if String::from_utf8_lossy(body).contains("alpha") {
            let _waited = signal
                .wait_timeout_while(done, std::time::Duration::from_secs(30), |done| !*done)
                .expect("wait");
            return Canned::status(503, "busy").asking("retry-after-ms", "0");
        }
        *done = true;
        signal.notify_all();
        Canned::ok(r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.8}}}"#)
    })
    .expect("listener");
    let budget = SendBudget::new();
    let texts = ["alpha", "beta", "gamma", "delta"].map(str::to_owned);
    let result = crate::api::recover_decisions(
        &retrying(listener.base()),
        &question(),
        &texts,
        CallOptions::new()
            .batch(BatchSetting::Records(
                std::num::NonZeroUsize::new(2).expect("two"),
            ))
            .send_budget(&budget, Some(2)),
    )
    .expect("safe denied retry beside a later answer");
    let answered: Vec<bool> = result
        .value()
        .iter()
        .map(|row| row.outcome().is_ok())
        .collect();
    assert_eq!(answered, [false, false, true, true]);
    for row in result.value()[..2].iter() {
        assert_failed(row, ErrorKind::Usage, thinkthen::StopCause::Usage);
    }
    assert_eq!(listener.requests().len(), 2);
}

#[test]
fn native_recovery_keeps_one_invalid_member_out_of_one_request() {
    let _serial = serial();
    let listener = Listener::serving(vec![Canned::ok(
        r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.9}}}"#,
    )])
    .expect("listener");
    let engine = engine(listener.base());
    let result = crate::api::recover_decisions(
        &engine,
        &question(),
        &["alpha".to_owned(), " ".to_owned(), "beta".to_owned()],
        CallOptions::new(),
    )
    .expect("recoverable rows");
    assert!(result.value()[0].outcome().is_ok());
    assert_failed(
        &result.value()[1],
        ErrorKind::Usage,
        thinkthen::StopCause::Usage,
    );
    assert!(result.value()[2].outcome().is_ok());
    assert_eq!(
        (result.facts().records(), result.facts().requests_sent()),
        (3, 1)
    );
    assert_eq!(listener.requests().len(), 1);
}

#[cfg(test)]
fn assert_failed(
    row: &thinkthen::RequestRecoveredRow,
    kind: ErrorKind,
    cause: thinkthen::StopCause,
) {
    let error = row.outcome().expect_err("failed row");
    assert_eq!(error.kind(), kind);
    assert!(!error.retryable());
    assert_eq!(error.stopped().cause(), cause);
}

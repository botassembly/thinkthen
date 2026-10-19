//! A named call owns pull failures and caller stopping controls.
use conformance_backend::{CASES, Canned, Listener, Rendezvous};
use std::{
    error::Error as _,
    sync::{Arc, mpsc},
    time::Duration,
};
use thinkthen::{
    CallOptions, CancelToken, Engine, Error, ErrorKind, Request, RequestEnvironment, RequestFeed,
    RequestInput,
};

fn arguments(feed: bool) -> Result<thinkthen::RequestArguments, Error> {
    let mut request = Request::named_from_json(
        "decide",
        r#"{"kind":"text","text":"Does it pass?"}"#,
        r#"{"kind":"text","text":"one"}"#,
        "{}",
    )?;
    if feed {
        request.call.arguments.input = RequestInput::Feed {
            name: "caller".into(),
            framing: Default::default(),
            reading: Default::default(),
            images: vec![],
        };
    }
    Ok(request.call.arguments)
}
fn engine(listener: &Listener) -> Result<Engine, Error> {
    Engine::builder()
        .base_url(listener.base())?
        .api_key("sk-call-loopback")?
        .no_cache()
        .max_retries(0)
        .build()
}
fn behavior(
    error: &Error,
    id: &str,
    requests_sent: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let cases: serde_json::Value = serde_json::from_str(CASES)?;
    let expected = cases
        .get("call_behaviors")
        .and_then(|cases| cases.get(id))
        .ok_or("missing shared call behavior")?;
    assert_eq!(
        Some(error.kind().name()),
        expected.get("kind").and_then(serde_json::Value::as_str)
    );
    assert_eq!(
        Some(error.to_string().as_str()),
        expected.get("message").and_then(serde_json::Value::as_str)
    );
    assert_eq!(
        Some(requests_sent as u64),
        expected
            .get("requests_sent")
            .and_then(serde_json::Value::as_u64)
    );
    if let Some(sent) = expected
        .get("facts_requests_sent")
        .and_then(serde_json::Value::as_u64)
    {
        assert_eq!(
            error.facts().ok_or("missing final facts")?.requests_sent(),
            sent
        );
    }
    Ok(())
}
#[test]
fn producer_failure_keeps_its_cause_and_sends_nothing() -> Result<(), Box<dyn std::error::Error>> {
    let listener = Listener::answering(|_| Canned::ok("{}"))?;
    let engine = engine(&listener)?;
    let error = engine
        .call(
            "decide",
            arguments(true)?,
            RequestEnvironment {
                controls: Default::default(),
                feed: Some(RequestFeed::new(
                    "caller",
                    std::iter::once(Err(Error::new(ErrorKind::Local, "producer marker"))),
                )),
            },
        )
        .err()
        .ok_or("producer succeeded")?;
    behavior(&error, "producer", listener.count())?;
    assert_eq!(
        error.source().ok_or("lost producer cause")?.to_string(),
        "producer marker"
    );
    assert!(!format!("{error:?}").contains("producer marker"));
    Ok(())
}
#[test]
fn feed_without_producer_is_usage_before_send() -> Result<(), Box<dyn std::error::Error>> {
    let listener = Listener::answering(|_| Canned::ok("{}"))?;
    let error = engine(&listener)?
        .call("decide", arguments(true)?, RequestEnvironment::default())
        .err()
        .ok_or("missing producer succeeded")?;
    behavior(&error, "missing-producer", listener.count())?;
    Ok(())
}
#[test]
fn cancellation_while_reply_is_held_returns_settled_failure()
-> Result<(), Box<dyn std::error::Error>> {
    let release = Arc::new(Rendezvous::new(2));
    let held = Arc::clone(&release);
    let (sent, arrival) = mpsc::channel();
    let listener = Listener::answering_with_events(
        move |_| {
            Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}}}"#)
                .after_release(Arc::clone(&held))
        },
        sent,
    )?;
    let engine = engine(&listener)?;
    let cancel = CancelToken::new();
    let stop = cancel.clone();
    let worker = std::thread::spawn(move || {
        engine.call(
            "decide",
            arguments(false)?,
            RequestEnvironment {
                controls: CallOptions::new().cancel(&stop),
                feed: None,
            },
        )
    });
    let arrived = arrival.recv_timeout(Duration::from_secs(5));
    cancel.cancel();
    if arrived.is_ok() {
        release.wait();
    }
    let outcome = worker.join().map_err(|_| "call worker panicked")?;
    arrived?;
    let error = outcome.err().ok_or("cancelled call succeeded")?;
    behavior(&error, "held-cancel", listener.count())?;
    Ok(())
}
#[test]
fn expired_deadline_sends_nothing() -> Result<(), Box<dyn std::error::Error>> {
    let listener = Listener::answering(|_| Canned::ok("{}"))?;
    let error = engine(&listener)?
        .call(
            "decide",
            arguments(false)?,
            RequestEnvironment {
                controls: CallOptions::new().deadline_after(Duration::ZERO)?,
                feed: None,
            },
        )
        .err()
        .ok_or("expired call succeeded")?;
    behavior(&error, "expired-deadline", listener.count())?;
    Ok(())
}

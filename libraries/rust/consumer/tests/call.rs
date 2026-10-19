//! A named call owns pull failures and caller stopping controls.
use conformance_backend::{Canned, Listener, Rendezvous, CASES};
use std::{error::Error as _, sync::{Arc, mpsc}, time::{Duration}};
use thinkthen::{CallOptions, CancelToken, Engine, Error, ErrorKind, Request, RequestEnvironment, RequestFeed, RequestInput};

fn arguments(feed: bool) -> Result<thinkthen::RequestArguments, Error> {
    let mut request = Request::named_from_json("decide", r#"{"kind":"text","text":"Does it pass?"}"#, r#"{"kind":"text","text":"one"}"#, "{}")?;
    if feed {
        request.call.arguments.input = RequestInput::Feed { name: "caller".into(), framing: Default::default(), reading: Default::default(), images: vec![] };
    }
    Ok(request.call.arguments)
}
fn engine(listener: &Listener) -> Result<Engine, Error> {
    Engine::builder().base_url(listener.base())?.api_key("sk-call-loopback")?.no_cache().max_retries(0).build()
}
fn kind(error: &Error, expected: ErrorKind, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let cases: serde_json::Value = serde_json::from_str(CASES)?;
    assert!(cases.get("error_kinds").and_then(serde_json::Value::as_array).ok_or("missing shared kinds")?.iter().any(|kind| kind.as_str() == Some(error.kind().name())));
    assert_eq!(error.kind(), expected);
    assert_eq!(error.to_string(), message);
    Ok(())
}
#[test]
fn producer_failure_keeps_its_cause_and_sends_nothing() -> Result<(), Box<dyn std::error::Error>> {
    let listener = Listener::answering(|_| Canned::ok("{}"))?;
    let engine = engine(&listener)?;
    let error = engine.call("decide", arguments(true)?, RequestEnvironment { controls: Default::default(), feed: Some(RequestFeed::new("caller", std::iter::once(Err(Error::new(ErrorKind::Local, "producer marker"))))) }).err().ok_or("producer succeeded")?;
    kind(&error, ErrorKind::Local, "session producer raised an exception")?;
    assert_eq!(error.source().ok_or("lost producer cause")?.to_string(), "producer marker");
    assert!(!format!("{error:?}").contains("producer marker"));
    assert_eq!(listener.count(), 0);
    Ok(())
}
#[test]
fn feed_without_producer_is_usage_before_send() -> Result<(), Box<dyn std::error::Error>> {
    let listener = Listener::answering(|_| Canned::ok("{}"))?;
    let error = engine(&listener)?.call("decide", arguments(true)?, RequestEnvironment::default()).err().ok_or("missing producer succeeded")?;
    kind(&error, ErrorKind::Usage, "the request requires its named caller-supplied feed")?;
    assert_eq!(listener.count(), 0);
    Ok(())
}
#[test]
fn cancellation_while_reply_is_held_returns_settled_failure() -> Result<(), Box<dyn std::error::Error>> {
    let release = Arc::new(Rendezvous::new(2));
    let held = Arc::clone(&release);
    let (sent, arrival) = mpsc::channel();
    let listener = Listener::answering_with_events(move |_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}}}"#).after_release(Arc::clone(&held)), sent)?;
    let engine = engine(&listener)?;
    let cancel = CancelToken::new();
    let stop = cancel.clone();
    let worker = std::thread::spawn(move || engine.call("decide", arguments(false)?, RequestEnvironment { controls: CallOptions::new().cancel(&stop), feed: None }));
    let arrived = arrival.recv_timeout(Duration::from_secs(5));
    cancel.cancel();
    if arrived.is_ok() {
        release.wait();
    }
    let outcome = worker.join().map_err(|_| "call worker panicked")?;
    arrived?;
    let error = outcome.err().ok_or("cancelled call succeeded")?;
    kind(&error, ErrorKind::Cancelled, "the call was cancelled")?;
    assert_eq!(listener.count(), 1);
    assert_eq!(error.facts().ok_or("missing final facts")?.requests_sent(), 1);
    Ok(())
}
#[test]
fn expired_deadline_sends_nothing() -> Result<(), Box<dyn std::error::Error>> {
    let listener = Listener::answering(|_| Canned::ok("{}"))?;
    let error = engine(&listener)?.call("decide", arguments(false)?, RequestEnvironment { controls: CallOptions::new().deadline_after(Duration::ZERO)?, feed: None }).err().ok_or("expired call succeeded")?;
    kind(&error, ErrorKind::Deadline, "the deadline of 0 s passed before the call answered")?;
    assert_eq!(listener.count(), 0);
    Ok(())
}

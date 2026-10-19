//! Narrow ordinary continuation retains native annotation decoding and settlement.
#![allow(clippy::unwrap_used, clippy::panic, clippy::indexing_slicing, reason = "isolated public session fixtures stop on malformed setup")]
use super::*;
use std::sync::Mutex;

type Policy = Arc<dyn Fn(usize, &str) -> bool + Send + Sync>;
fn annotation() -> Request {
    let question = QuestionSet::from_json(r#"{"version":1,"questions":{"fit":{"decide":"Fits?","on":["/body"]}}}"#).unwrap();
    Request::named("annotate", RequestArguments {
        question: RequestQuestion::Definition { value: question.into() }, input: feed(),
        options: RequestOptions { batch: Some(RequestBatch::Count(1)), ..Default::default() },
    }).unwrap()
}
fn original(missing: bool) -> RequestSessionDescriptor {
    RequestSessionDescriptor::from_json(&json!({"item":{"original":{"kind":"json","value":
        if missing { json!({"private-marker":"withheld"}) } else { json!({"body":"Fits."}) }
    }}}).to_string()).unwrap()
}
fn distinct_original(missing: bool, ordinal: usize) -> RequestSessionDescriptor {
    let mut value = original(missing);
    if !missing {
        value.item.original = Some(RequestOriginal::Json { value: RawRecord::json(
            &json!({"body": format!("Fits {ordinal}.")}).to_string()).unwrap() });
    }
    value
}
fn options(policy: Option<Policy>) -> RequestSessionFeedOptions {
    RequestSessionFeedOptions { on_missing_annotation_pointer: policy, ..Default::default() }
}
fn terminal(packets: &[RequestSessionResult]) -> &RequestSessionTerminal {
    let Some(RequestSessionResult::Terminal(terminal)) = packets.last() else { panic!("terminal") };
    terminal
}
fn ordinals(packets: &[RequestSessionResult]) -> Vec<usize> {
    packets.iter().filter_map(|packet| match packet {
        RequestSessionResult::Row(RequestSessionRow::Annotation(row)) => Some(row.ordinal()), _ => None,
    }).collect()
}

#[test]
fn ordinary_annotation_policy_only_skips_the_actual_missing_question_pointer() {
    for missing in 0..3 {
        for permit in [None, Some(false), Some(true)] {
            let listener = Listener::answering(response).unwrap();
            let observed = Arc::new(Mutex::new(Vec::new()));
            let callback = permit.map(|permit| {
                let observed = observed.clone();
                Arc::new(move |ordinal: usize, pointer: &str| {
                    observed.lock().unwrap().push((ordinal, pointer.to_owned())); permit
                }) as Policy
            });
            let session = engine(&listener).request_session_with_feed_options(annotation(), Surface::Rust, options(callback)).unwrap();
            for at in 0..3 { let _status = session.push_with(|| Ok::<_, Error>(distinct_original(at == missing, at))).unwrap(); }
            session.finish(None).unwrap();
            let packets = drain(&session);
            let terminal = terminal(&packets);
            if permit == Some(true) {
                assert!(terminal.error.is_none());
                assert_eq!(ordinals(&packets), (0..3).filter(|at| *at != missing).collect::<Vec<_>>());
                assert_eq!(terminal.facts.as_ref().unwrap().records(), 3);
                assert_eq!(terminal.facts.as_ref().unwrap().requests_sent(), 2);
                assert_eq!(listener.count(), 2);
            } else {
                let error = terminal.error.as_ref().unwrap();
                assert!(matches!(error.diagnostic(), Some(ErrorDiagnostic::MissingPointer { pointer: "/body" })));
                assert_eq!(ordinals(&packets), (0..missing).collect::<Vec<_>>());
                assert_eq!(terminal.facts.as_ref().unwrap().records(), missing as u64);
                assert_eq!(listener.count(), missing);
                assert!(!format!("{error:?} {} {:?}", error, error.diagnostic()).contains("private-marker"));
            }
            let observed = observed.lock().unwrap();
            assert_eq!(*observed, if permit.is_some() { vec![(missing, "/body".to_owned())] } else { Vec::new() });
        }
    }
}

#[test]
fn command_projection_and_provider_failures_never_invoke_the_pointer_policy() {
    for case in 0..3 {
        let listener = Listener::answering(move |_| if case == 1 { Canned::status(500, "private-provider-marker") } else { Canned::ok(r#"{"model":"fixed","answers":{}}"#) }).unwrap();
        let called = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let calls = called.clone();
        let callback = Arc::new(move |_: usize, _: &str| { calls.fetch_add(1, std::sync::atomic::Ordering::Relaxed); true });
        let mut request = annotation();
        if case == 0 {
            let mut arguments = request.call.arguments().clone();
            arguments.options.field = Some(vec!["/command".to_owned()]);
            request = Request::for_function(RequestFunction::Annotate, arguments);
        }
        let session = engine(&listener).request_session_with_feed_options(request, Surface::Rust, options(Some(callback))).unwrap();
        let _status = session.push_with(|| Ok::<_, Error>(original(false))).unwrap();
        session.finish(None).unwrap();
        let packets = drain(&session);
        let terminal = terminal(&packets);
        assert!(terminal.error.is_some());
        assert!(ordinals(&packets).is_empty());
        assert_eq!(called.load(std::sync::atomic::Ordering::Relaxed), 0);
        assert_eq!(listener.count(), usize::from(case != 0));
        if case == 0 {
            assert!(matches!(terminal.error.as_ref().unwrap().diagnostic(), Some(ErrorDiagnostic::MissingPointer { pointer: "/command" })));
        } else {
            assert_eq!(terminal.facts.as_ref().unwrap().requests_sent(), 1);
        }
    }
}

#[test]
fn callback_panic_declines_and_incompatible_modes_refuse_before_input() {
    let listener = Listener::answering(response).unwrap();
    let engine = engine(&listener);
    let callback = Arc::new(|_: usize, _: &str| -> bool { panic!("fixture callback panic") });
    let session = engine.request_session_with_feed_options(annotation(), Surface::Rust, options(Some(callback))).unwrap();
    session.push_with(|| Ok::<_, Error>(original(true))).unwrap();
    session.push_with(|| Ok::<_, Error>(distinct_original(false, 1))).unwrap();
    session.finish(None).unwrap();
    let packets = drain(&session);
    assert!(matches!(terminal(&packets).error.as_ref().unwrap().diagnostic(), Some(ErrorDiagnostic::MissingPointer { pointer: "/body" })));
    assert!(terminal(&packets).facts.is_some());
    assert_eq!(listener.count(), 0, "callback panic admits no later request");
    let callback: Policy = Arc::new(|_: usize, _: &str| true);
    assert_eq!(engine.request_session_with_feed_options(request(feed()), Surface::Rust, options(Some(callback.clone()))).unwrap_err().kind(), ErrorKind::Usage);
    assert_eq!(engine.request_recovering_session_with_feed_options(annotation(), Surface::Rust, options(Some(callback))).unwrap_err().kind(), ErrorKind::Usage);
    assert!(listener.requests().is_empty());
    assert_eq!(listener.count(), 0);
}

#[test]
fn synchronous_call_uses_the_same_narrow_policy_and_refuses_other_controls_before_sources() {
    let listener = Listener::answering(response).unwrap();
    let engine = engine(&listener);
    let seen = Mutex::new(Vec::new());
    let callback = |ordinal: usize, pointer: &str| {
        seen.lock().unwrap().push((ordinal, pointer.to_owned())); true
    };
    let mut arguments = annotation().call.arguments().clone();
    arguments.input = RequestInput::Records { items: vec![original(false).item, original(true).item, original(false).item] };
    let call = engine.call("annotate", arguments, RequestEnvironment {
        controls: CallOptions::new().on_missing_annotation_pointer(&callback), feed: None,
    }).unwrap();
    let RequestValue::Annotations(rows) = call.value() else { panic!("annotations") };
    assert_eq!(rows.iter().map(CompleteRecord::ordinal).collect::<Vec<_>>(), [0, 2]);
    assert_eq!(*seen.lock().unwrap(), [(1, "/body".to_owned())]);
    assert_eq!(call.facts().records(), 3);
    assert_eq!(call.facts().requests_sent(), 2);
    let mut arguments = request(feed()).call.arguments().clone();
    arguments.question = RequestQuestion::File { path: std::env::temp_dir().join("thinkthen-no-annotation-policy-file/missing.json") };
    let error = engine.call("decide", arguments, RequestEnvironment {
        controls: CallOptions::new().on_missing_annotation_pointer(&callback), feed: None,
    }).unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(error.detail().message(), "missing annotation pointer continuation requires an annotation request");
    assert_eq!(listener.count(), 2);
}

#[path = "annotation_missing/ack.rs"]
mod ack;

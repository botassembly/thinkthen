//! The explicit command mode publishes native recognition before input closure.
use super::*;

fn start(listener: &Listener, input: RequestInput, eager: bool) -> RequestSession {
    engine(listener).request_session_with_feed_options(
        super::recognize::recognition_request(input), Surface::Rust,
        RequestSessionFeedOptions { incremental_recognition: true, eager, ..Default::default() },
    ).unwrap()
}
fn rows(packets: &[RequestSessionResult]) -> Vec<usize> {
    packets.iter().filter_map(|packet| match packet {
        RequestSessionResult::Aggregate(RequestValue::Recognized(rows)) => {
            assert!(!rows.is_empty());
            Some(rows.iter().map(CompleteRecord::ordinal))
        }
        _ => None,
    }).flatten().collect()
}

#[test]
fn recognition_can_publish_before_feed_eof_without_replaying_its_prefix() {
    for later_failure in [false, true] {
        let listener = Listener::answering(|body| recognition_response(body, None)).unwrap();
        let session = start(&listener, feed(), false);
        assert!(matches!(session.try_push(descriptor("first")).unwrap(), RequestSessionPush::Accepted));
        let mut packets = Vec::new();
        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            assert!(Instant::now() < deadline, "first recognition must precede input EOF");
            match session.try_read() {
                RequestSessionRead::Result(packet) => packets.push(packet),
                RequestSessionRead::Pending => {
                    let ready = session.wait(RequestSessionWaitInterest::Result, Duration::from_millis(50));
                    assert_ne!(ready, RequestSessionReady::End);
                }
                RequestSessionRead::End => panic!("session ended before input EOF"),
            }
            if !rows(&packets).is_empty() { break; }
        }
        assert_eq!(rows(&packets), [0]);
        if later_failure { session.finish(Some(RequestReaderFailure::Producer)).unwrap(); }
        else {
            assert!(matches!(session.try_push(descriptor("second")).unwrap(), RequestSessionPush::Accepted));
            session.finish(None).unwrap();
        }
        packets.extend(drain(&session));
        assert_eq!(rows(&packets), if later_failure { vec![0] } else { vec![0, 1] });
        let RequestSessionResult::Terminal(terminal) = packets.last().unwrap() else { panic!("terminal") };
        assert_eq!(terminal.error.is_some(), later_failure);
        let Some(SettledAnswer::Recognize(answer)) = &terminal.answer else { panic!("recognition settlement") };
        let AnswerShape::Collection(values) = &answer.value else { panic!("feed keeps a collection") };
        let AnswerShape::Collection(probabilities) = &answer.probability else { panic!("aligned probabilities") };
        let expected = if later_failure { 1 } else { 2 };
        assert_eq!(values.len(), expected, "terminal prefix contains every emitted row once");
        assert_eq!(probabilities.len(), expected);
        assert!(values.iter().all(Option::is_some));
        assert_eq!(terminal.facts.as_ref().unwrap().records(), if later_failure { 1 } else { 2 });
        assert_eq!(terminal.facts.as_ref().unwrap().requests_sent(), listener.requests().len() as u64);
    }
}

#[test]
fn incremental_empty_eager_and_cancellation_keep_actual_settlement() {
    let listener = Listener::answering(|body| recognition_response(body, None)).unwrap();
    let inline = start(&listener, RequestInput::Records { items: Vec::new() }, false);
    let inline_packets = drain(&inline);
    assert!(rows(&inline_packets).is_empty());
    assert!(!inline_packets.iter().any(|packet| matches!(packet, RequestSessionResult::Aggregate(_))));
    let RequestSessionResult::Terminal(inline_terminal) = inline_packets.last().unwrap() else { panic!("terminal") };
    assert!(inline_terminal.error.is_none());
    assert_eq!(listener.count(), 0);
    for eager in [false, true] {
        let listener = Listener::answering(|body| recognition_response(body, None)).unwrap();
        let session = start(&listener, feed(), eager);
        session.finish(None).unwrap();
        let packets = drain(&session);
        assert!(rows(&packets).is_empty());
        assert!(!packets.iter().any(|packet| matches!(packet, RequestSessionResult::Aggregate(_))));
        let RequestSessionResult::Terminal(terminal) = packets.last().unwrap() else { panic!("terminal") };
        assert!(terminal.error.is_none());
        assert_eq!(listener.count(), 0);
        let session = start(&listener, feed(), eager);
        session.cancel();
        let packets = drain(&session);
        assert!(rows(&packets).is_empty());
        let RequestSessionResult::Terminal(terminal) = packets.last().unwrap() else { panic!("terminal") };
        assert_eq!(terminal.error.as_ref().unwrap().kind(), ErrorKind::Cancelled);
        assert_eq!(listener.count(), 0);
    }
}

#[test]
fn incremental_recognition_refuses_other_functions_before_input() {
    let listener = Listener::answering(response).unwrap();
    let refused = engine(&listener).request_session_with_feed_options(request(RequestInput::Source {
        source: RequestSource { paths: vec!["absent-incremental-source".into()], framing: None,
            reading: ReaderOptions::default(), media: ReaderMedia::Text },
    }), Surface::Rust, RequestSessionFeedOptions { incremental_recognition: true, ..Default::default() }).unwrap_err();
    assert_eq!(refused.kind(), ErrorKind::Usage);
    assert_eq!(listener.count(), 0);
}

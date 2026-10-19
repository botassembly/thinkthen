//! A caller acknowledgement never blocks the session queue or overtakes output.
use super::*;
use std::sync::mpsc;

#[test]
fn held_acknowledgement_allows_prior_output_to_drain_and_blocks_later_sends() {
    let listener = Listener::answering(response).unwrap();
    let (offered, events) = mpsc::sync_channel(1);
    let (acknowledge, acknowledgements) = mpsc::sync_channel(1);
    let acknowledgements = Mutex::new(acknowledgements);
    let policy = Arc::new(move |ordinal: usize, pointer: &str| {
        if offered.send((ordinal, pointer.to_owned())).is_err() { return false; }
        acknowledgements.lock().unwrap().recv().unwrap_or(false)
    });
    let session = engine(&listener).request_session_with_feed_options(annotation(), Surface::Rust, options(Some(policy))).unwrap();
    session.push_with(|| Ok::<_, Error>(distinct_original(false, 0))).unwrap();
    session.push_with(|| Ok::<_, Error>(original(true))).unwrap();
    let mut packets = Vec::new();
    loop {
        let RequestSessionRead::Result(packet) = next(&session) else { panic!("first row") };
        let row = matches!(packet, RequestSessionResult::Row(_));
        packets.push(packet);
        if row { break; }
    }
    // Native observation and row publication use one bounded packet cell.
    // Drain prior output before expecting the next preparation's policy offer.
    assert_eq!(ordinals(&packets), [0]);
    assert_eq!(events.recv_timeout(Duration::from_secs(5)).unwrap(), (1, "/body".to_owned()));
    session.push_with(|| Ok::<_, Error>(distinct_original(false, 2))).unwrap();
    session.finish(None).unwrap();
    assert!(matches!(session.try_read(), RequestSessionRead::Pending));
    assert_eq!(listener.count(), 1);
    acknowledge.send(true).unwrap();
    packets.extend(drain(&session));
    assert_eq!(ordinals(&packets), [0, 2]);
    assert!(terminal(&packets).error.is_none());
    assert_eq!(terminal(&packets).facts.as_ref().unwrap().records(), 3);
    assert_eq!(listener.count(), 2);
}

#[test]
fn cancellation_and_a_dropped_ack_receiver_release_the_native_failure_path() {
    for cancel in [false, true] {
        let listener = Listener::answering(response).unwrap();
        let (offered, events) = mpsc::sync_channel(1);
        let (acknowledge, acknowledgements) = mpsc::sync_channel(1);
        let acknowledgements = Mutex::new(acknowledgements);
        let policy = Arc::new(move |_: usize, _: &str| {
            if offered.send(()).is_err() { return false; }
            acknowledgements.lock().unwrap().recv().unwrap_or(false)
        });
        let session = engine(&listener).request_session_with_feed_options(annotation(), Surface::Rust, options(Some(policy))).unwrap();
        session.try_push(original(true)).unwrap();
        events.recv_timeout(Duration::from_secs(5)).unwrap();
        session.push_with(|| Ok::<_, Error>(distinct_original(false, 1))).unwrap();
        session.finish(None).unwrap();
        if cancel { session.cancel(); }
        drop(acknowledge);
        let packets = drain(&session);
        let error = terminal(&packets).error.as_ref().unwrap();
        assert_eq!(error.kind(), if cancel { ErrorKind::Cancelled } else { ErrorKind::Usage });
        assert!(terminal(&packets).facts.is_some());
        assert!(ordinals(&packets).is_empty());
        assert_eq!(listener.count(), 0);
    }
}

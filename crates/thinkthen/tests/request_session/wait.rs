//! Native wait readiness, ownership and cancellation behavior.
use super::*;

#[test]
fn result_wait_ignores_input_room_and_timeout_consumes_nothing() {
    let listener = Listener::answering(response).unwrap();
    let session = engine(&listener).request_session(request(feed())).unwrap();
    assert_eq!(
        session.wait(RequestSessionWaitInterest::Input, Duration::ZERO),
        RequestSessionReady::Input
    );
    let started = Instant::now();
    assert_eq!(
        session.wait(
            RequestSessionWaitInterest::Result,
            Duration::from_millis(30)
        ),
        RequestSessionReady::Timeout
    );
    assert!(started.elapsed() >= Duration::from_millis(30));
    assert!(matches!(session.try_read(), RequestSessionRead::Pending));
    session.finish(None).unwrap();
    assert_eq!(listener.count(), 0);
}

#[test]
fn held_provider_bounds_intake_and_cancel_waits_for_actual_terminal_facts() {
    for framing in [RequestFraming::Document, RequestFraming::Csv] {
        let release = Arc::new(Rendezvous::new(2));
        let held = Arc::clone(&release);
        let (sent, received) = mpsc::channel();
        let listener = Listener::answering_with_events(
            move |body| response(body).after_release(Arc::clone(&held)),
            sent,
        )
        .unwrap();
        let engine = engine(&listener);
        let arguments = framing::table_request(framing).call.arguments().clone();
        let session = Arc::new(
            engine
                .request_session(Request::new(RequestCall::Decide(arguments)))
                .unwrap(),
        );
        if matches!(framing, RequestFraming::Csv) {
            framing::push(&session, descriptor("body\n"));
        }
        framing::push(&session, descriptor("first"));
        received.recv_timeout(Duration::from_secs(5)).unwrap();
        assert!(matches!(
            session.try_push(descriptor("second")).unwrap(),
            RequestSessionPush::Accepted
        ));
        // Native scheduling may own additional in-flight inputs; the session bounds
        // waiting transfers rather than changing those existing scheduler limits.
        let retry = (0..100)
            .find_map(|_| match session.try_push(descriptor("next")).unwrap() {
                RequestSessionPush::Full(value) => Some(value),
                RequestSessionPush::Accepted => None,
                other => panic!("feed closed before cancellation: {other:?}"),
            })
            .expect("held native work eventually backpressures the one input cell");
        session.finish(None).unwrap();
        assert_eq!(
            session.wait(RequestSessionWaitInterest::Input, Duration::ZERO),
            RequestSessionReady::Closed
        );
        session.finish(None).unwrap();
        assert_eq!(
            session
                .finish(Some(RequestReaderFailure::Io { location: None }))
                .unwrap_err()
                .kind(),
            ErrorKind::Usage
        );
        cancel_and_settle(&session, &release, retry);
        assert_eq!(listener.count(), 1);
    }
}

#[allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "owned wait fixtures must settle and retain the exact terminal packet"
)]
fn cancel_and_settle(
    session: &Arc<RequestSession>,
    release: &Rendezvous,
    retry: RequestSessionDescriptor,
) {
    let waiting = Arc::clone(session);
    let (entered, started) = mpsc::channel();
    let (woke, wake) = mpsc::channel();
    let waiter = std::thread::spawn(move || {
        entered.send(()).unwrap();
        woke.send(waiting.wait(RequestSessionWaitInterest::Result, Duration::from_secs(5)))
            .unwrap();
    });
    started.recv_timeout(Duration::from_secs(1)).unwrap();
    let began = Instant::now();
    session.cancel();
    assert_eq!(
        wake.recv_timeout(Duration::from_secs(1)).unwrap(),
        RequestSessionReady::Cancelled
    );
    waiter.join().unwrap();
    assert!(began.elapsed() < Duration::from_secs(1));
    assert_eq!(
        session.wait(
            RequestSessionWaitInterest::Settlement,
            Duration::from_millis(30)
        ),
        RequestSessionReady::Timeout
    );
    assert!(matches!(
        session.try_push(retry).unwrap(),
        RequestSessionPush::Closed(_)
    ));
    assert!(matches!(session.try_read(), RequestSessionRead::Pending));
    let waiting = Arc::clone(session);
    let result_waiter = std::thread::spawn(move || {
        waiting.wait(
            RequestSessionWaitInterest::Settlement,
            Duration::from_secs(5),
        )
    });
    assert!(release.wait());
    assert_eq!(result_waiter.join().unwrap(), RequestSessionReady::Result);
    let packets = drain(session);
    let RequestSessionResult::Terminal(terminal) = packets.last().unwrap() else {
        panic!("terminal comes last")
    };
    assert_eq!(
        terminal.error.as_ref().unwrap().kind(),
        ErrorKind::Cancelled
    );
    assert_eq!(terminal.facts.as_ref().unwrap().requests_sent(), 1);
    assert_eq!(terminal.facts.as_ref().unwrap().input_tokens(), None);
    assert_eq!(
        packets
            .iter()
            .filter(|p| matches!(p, RequestSessionResult::Terminal(_)))
            .count(),
        1
    );
    assert!(matches!(session.try_read(), RequestSessionRead::End));
    assert_eq!(
        session.wait(RequestSessionWaitInterest::Settlement, Duration::ZERO),
        RequestSessionReady::End
    );
}

#[test]
fn cancel_and_free_return_while_the_provider_is_held_and_the_engine_owner_is_gone() {
    let release = Arc::new(Rendezvous::new(2));
    let held = Arc::clone(&release);
    let (sent, received) = mpsc::channel();
    let listener = Listener::answering_with_events(
        move |body| response(body).after_release(Arc::clone(&held)),
        sent,
    )
    .unwrap();
    let engine = engine(&listener);
    let session = engine
        .request_session_with_feed_options(
            request(feed()),
            Surface::Rust,
            RequestSessionFeedOptions {
                eager: true,
                ..Default::default()
            },
        )
        .unwrap();
    session.try_push(descriptor("first")).unwrap();
    session.finish(None).unwrap();
    received.recv_timeout(Duration::from_secs(5)).unwrap();
    drop(engine);
    let began = Instant::now();
    session.cancel();
    drop(session);
    assert!(began.elapsed() < Duration::from_secs(1));
    assert!(release.wait());
}

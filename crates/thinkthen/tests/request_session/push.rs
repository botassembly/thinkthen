//! Producer transfer ignores pending output and releases foreign payloads on close.
use super::*;

#[test]
fn blocking_push_ignores_pending_output_and_wakes_for_room_or_cancel() {
    for cancel in [false, true] {
        let listener = Listener::answering(response).unwrap();
        let mut arguments = request(feed()).call.arguments().clone();
        arguments.options.batch = Some(RequestBatch::Count(1));
        let request = Request::new(RequestCall::new(RequestFunction::Decide, arguments));
        let session = Arc::new(engine(&listener).request_session(request).unwrap());
        loop {
            match session.try_push(descriptor("queued")).unwrap() {
                RequestSessionPush::Accepted => {}
                RequestSessionPush::Full(_) => break,
                other => panic!("intake closed: {other:?}"),
            }
        }
        assert_eq!(
            session.wait(RequestSessionWaitInterest::Result, Duration::from_secs(5)),
            RequestSessionReady::Result
        );
        // Once the worker publishes its output it can consume one queued input.
        // Fill that room too, leaving both cells occupied for the producer wait.
        while matches!(
            session.try_push(descriptor("queued")).unwrap(),
            RequestSessionPush::Accepted
        ) {}
        let waiting = Arc::clone(&session);
        let (entered, started) = mpsc::channel();
        let (sent, received) = mpsc::channel();
        let producer = std::thread::spawn(move || {
            entered.send(()).unwrap();
            sent.send(waiting.push_with(|| Ok::<_, Error>(descriptor("transferred"))))
                .unwrap();
        });
        started.recv_timeout(Duration::from_secs(1)).unwrap();
        assert!(matches!(
            received.recv_timeout(Duration::from_millis(30)),
            Err(mpsc::RecvTimeoutError::Timeout)
        ));
        if cancel {
            session.cancel();
            assert_eq!(
                received
                    .recv_timeout(Duration::from_secs(1))
                    .unwrap()
                    .unwrap(),
                RequestSessionPushStatus::Closed
            );
            assert_eq!(
                session
                    .push_with::<Error>(|| panic!("closed intake decoded payload"))
                    .unwrap(),
                RequestSessionPushStatus::Closed
            );
        } else {
            while !matches!(
                session
                    .next_packet(RequestSessionWaitInterest::Result)
                    .unwrap(),
                RequestSessionRead::Result(RequestSessionResult::Row(_))
            ) {}
            assert_eq!(
                received
                    .recv_timeout(Duration::from_secs(5))
                    .unwrap()
                    .unwrap(),
                RequestSessionPushStatus::Accepted
            );
            session.cancel();
        }
        producer.join().unwrap();
        let _ = drain(&session);
    }
}

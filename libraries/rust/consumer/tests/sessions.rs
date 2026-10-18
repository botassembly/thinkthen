//! Session behavior through the installed Rust crate, in both execution modes.
use conformance_backend::{Canned, Listener, Rendezvous};
use serde_json::{Value, json};
use std::sync::{Arc, mpsc};
use std::time::Duration;
use thinkthen::{
    BatchSetting, Engine, ErrorKind, ReaderOptions, Request, RequestArguments, RequestCall,
    RequestFraming, RequestFunction, RequestInput, RequestOptions, RequestQuestion,
    RequestRecoveringRead, RequestRecoveringResult, RequestRecoveringSession, RequestSession,
    RequestSessionDescriptor, RequestSessionPushStatus, RequestSessionRead, RequestSessionReady,
    RequestSessionResult, RequestSessionRow, RequestSessionTerminal, RequestSessionWaitInterest,
    Surface,
};

enum Session {
    Ordinary(RequestSession),
    Recovering(RequestRecoveringSession),
}
macro_rules! dispatch {
    ($self:expr, $session:ident, $body:expr) => {
        match $self {
            Session::Ordinary($session) => $body,
            Session::Recovering($session) => $body,
        }
    };
}
impl Session {
    #[expect(
        clippy::unwrap_used,
        reason = "the fixture must admit before session behavior is exercised"
    )]
    fn open(engine: &Engine, recovering: bool) -> Self {
        let request = Request::new(RequestCall::new(
            RequestFunction::Decide,
            RequestArguments {
                question: RequestQuestion::Text {
                    text: "Does it pass?".into(),
                },
                input: RequestInput::Feed {
                    name: "records".into(),
                    framing: RequestFraming::Document,
                    reading: ReaderOptions::default(),
                    images: vec![],
                },
                options: RequestOptions::default(),
            },
        ));
        if recovering {
            Self::Recovering(
                engine
                    .start_request_recovering_session(request, Surface::Rust)
                    .unwrap(),
            )
        } else {
            Self::Ordinary(engine.request_session(request).unwrap())
        }
    }
    fn wait(&self, interest: RequestSessionWaitInterest, timeout: Duration) -> RequestSessionReady {
        dispatch!(self, s, s.wait(interest, timeout))
    }
    #[expect(
        clippy::unwrap_used,
        reason = "fixture conversion and admission failures stop the caller check"
    )]
    fn push(&self, index: usize, blocking: bool) -> RequestSessionPushStatus {
        let descriptor = || {
            RequestSessionDescriptor::from_json(
                &json!({"item":{"original":{"kind":"text","text":format!("row {index}")}}})
                    .to_string(),
            )
        };
        if blocking {
            dispatch!(self, s, s.push_with(descriptor).unwrap())
        } else {
            dispatch!(self, s, s.try_push_with(descriptor).unwrap())
        }
    }
    fn push_until_accepted(&self, index: usize, blocking: bool) {
        loop {
            let status = self.push(index, blocking);
            if status == RequestSessionPushStatus::Accepted {
                return;
            }
            assert_eq!(status, RequestSessionPushStatus::Full);
            assert_ne!(
                self.wait(RequestSessionWaitInterest::Input, Duration::from_secs(5)),
                RequestSessionReady::Timeout
            );
        }
    }
    fn cancel(&self) {
        dispatch!(self, s, s.cancel());
    }
    #[expect(
        clippy::unwrap_used,
        reason = "fixture EOF must be accepted before settlement is checked"
    )]
    fn finish(&self) {
        dispatch!(self, s, s.finish(None).unwrap());
    }
    #[expect(
        clippy::unwrap_used,
        clippy::panic,
        reason = "caller reads must retain their ordered rows and exactly one terminal"
    )]
    fn drain(&self) -> (Vec<usize>, RequestSessionTerminal) {
        let mut rows = Vec::new();
        let terminal = loop {
            match self {
                Self::Ordinary(s) => match s
                    .next_packet(RequestSessionWaitInterest::Settlement)
                    .unwrap()
                {
                    RequestSessionRead::Result(RequestSessionResult::Row(
                        RequestSessionRow::Decision(row),
                    )) => rows.push(row.ordinal()),
                    RequestSessionRead::Result(RequestSessionResult::Terminal(terminal)) => {
                        break terminal;
                    }
                    RequestSessionRead::Result(_) => {}
                    other => panic!("missing ordinary terminal: {other:?}"),
                },
                Self::Recovering(s) => match s
                    .next_packet(RequestSessionWaitInterest::Settlement)
                    .unwrap()
                {
                    RequestRecoveringRead::Result(RequestRecoveringResult::Row(row)) => {
                        assert!(row.outcome().is_ok());
                        rows.push(row.ordinal());
                    }
                    RequestRecoveringRead::Result(RequestRecoveringResult::Terminal(terminal)) => {
                        break terminal;
                    }
                    RequestRecoveringRead::Result(_) => {}
                    other => panic!("missing recovering terminal: {other:?}"),
                },
            }
        };
        assert_eq!(
            self.wait(RequestSessionWaitInterest::Settlement, Duration::ZERO),
            RequestSessionReady::End
        );
        (rows, terminal)
    }
}
#[expect(
    clippy::unwrap_used,
    reason = "the fixture engine names only its owned listener and fake key"
)]
fn engine(listener: &Listener) -> Engine {
    Engine::builder()
        .base_url(listener.base())
        .unwrap()
        .api_key("sk-session-fixture")
        .unwrap()
        .model("fixed")
        .unwrap()
        .no_cache()
        .max_retries(0)
        .throttle(1)
        .unwrap()
        .batch(BatchSetting::Records(
            std::num::NonZeroUsize::new(1).unwrap(),
        ))
        .build()
        .unwrap()
}
#[expect(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "the local fixture answers the actual native request or fails the caller check"
)]
fn response(body: &[u8]) -> Canned {
    let body: Value = serde_json::from_slice(body).unwrap();
    let answers = body["questions"]
        .as_object()
        .unwrap()
        .keys()
        .map(|name| (name.clone(), json!({"type":"noul","noul":0.9})))
        .collect::<serde_json::Map<_, _>>();
    Canned::ok(&json!({"model":"fixed","answers":answers}).to_string())
}
#[test]
fn push_all_then_finish_preserves_order_with_blocking_and_try_push() {
    for (recovering, blocking) in [(false, false), (false, true), (true, false), (true, true)] {
        let listener = Listener::answering(response).unwrap();
        let session = Session::open(&engine(&listener), recovering);
        for index in 0..24 {
            session.push_until_accepted(index, blocking);
        }
        session.finish();
        let (rows, terminal) = session.drain();
        assert_eq!(rows, (0..24).collect::<Vec<_>>());
        assert!(terminal.error.is_none());
        assert_eq!(terminal.facts.unwrap().requests_sent(), 24);
    }
}

#[test]
fn cancel_before_wait_closes_intake_and_settles_without_a_send() {
    for recovering in [false, true] {
        let listener = Listener::answering(response).unwrap();
        let session = Session::open(&engine(&listener), recovering);
        session.cancel();
        assert!(matches!(
            session.wait(RequestSessionWaitInterest::Result, Duration::from_secs(5)),
            RequestSessionReady::Cancelled | RequestSessionReady::Result
        ));
        assert_eq!(session.push(0, false), RequestSessionPushStatus::Closed);
        let (_, terminal) = session.drain();
        assert_eq!(terminal.error.unwrap().kind(), ErrorKind::Cancelled);
        assert_eq!(listener.count(), 0);
    }
}
#[test]
fn held_provider_cancel_interrupts_armed_wait_and_retains_actual_settlement() {
    for recovering in [false, true] {
        let release = Arc::new(Rendezvous::new(2));
        let held = Arc::clone(&release);
        let (sent, arrival) = mpsc::channel();
        let listener = Listener::answering_with_events(
            move |body| response(body).after_release(Arc::clone(&held)),
            sent,
        )
        .unwrap();
        let session = Arc::new(Session::open(&engine(&listener), recovering));
        assert_eq!(session.push(0, true), RequestSessionPushStatus::Accepted);
        arrival.recv_timeout(Duration::from_secs(5)).unwrap();
        // Fill the bounded intake without consuming any output.
        assert!((1..100).any(|index| session.push(index, false) == RequestSessionPushStatus::Full));
        session.cancel();
        assert_eq!(
            session.wait(RequestSessionWaitInterest::SettlementArm, Duration::ZERO),
            RequestSessionReady::Timeout
        );
        let waiting = Arc::clone(&session);
        let (entered, armed) = mpsc::sync_channel(0);
        let (done, wake) = mpsc::channel();
        let worker = std::thread::spawn(move || {
            entered.send(()).unwrap();
            done.send(waiting.wait(
                RequestSessionWaitInterest::InterruptibleSettlement,
                Duration::from_secs(5),
            ))
            .unwrap();
        });
        armed.recv_timeout(Duration::from_secs(1)).unwrap();
        session.cancel();
        assert_eq!(
            wake.recv_timeout(Duration::from_secs(1)).unwrap(),
            RequestSessionReady::Cancelled
        );
        worker.join().unwrap();
        assert_eq!(
            session.wait(RequestSessionWaitInterest::Settlement, Duration::ZERO),
            RequestSessionReady::Timeout
        );
        assert!(release.wait());
        let (_, terminal) = session.drain();
        assert_eq!(terminal.error.unwrap().kind(), ErrorKind::Cancelled);
        assert_eq!(terminal.facts.unwrap().requests_sent(), 1);
    }
}
#[test]
fn dropping_the_caller_owner_returns_during_an_active_wait() {
    for recovering in [false, true] {
        let listener = Listener::answering(response).unwrap();
        let session = Arc::new(Session::open(&engine(&listener), recovering));
        let waiting = Arc::clone(&session);
        let canceller = Arc::clone(&session);
        let (sent, entered) = mpsc::sync_channel(0);
        let (completed, wake) = mpsc::channel();
        let worker = std::thread::spawn(move || {
            sent.send(()).unwrap();
            completed
                .send(waiting.wait(RequestSessionWaitInterest::Result, Duration::from_secs(5)))
                .unwrap();
        });
        entered.recv_timeout(Duration::from_secs(1)).unwrap();
        assert!(matches!(
            wake.recv_timeout(Duration::from_millis(20)),
            Err(mpsc::RecvTimeoutError::Timeout)
        ));
        drop(session);
        assert!(matches!(
            wake.recv_timeout(Duration::from_millis(20)),
            Err(mpsc::RecvTimeoutError::Timeout)
        ));
        canceller.cancel();
        assert!(matches!(
            wake.recv_timeout(Duration::from_secs(1)).unwrap(),
            RequestSessionReady::Cancelled | RequestSessionReady::Result
        ));
        worker.join().unwrap();
    }
}

#[cfg(feature = "session-stress")]
#[test]
fn repeated_session_contention_never_loses_wake_or_settlement() {
    for _ in 0..12 {
        push_all_then_finish_preserves_order_with_blocking_and_try_push();
        cancel_before_wait_closes_intake_and_settles_without_a_send();
        held_provider_cancel_interrupts_armed_wait_and_retains_actual_settlement();
        dropping_the_caller_owner_returns_during_an_active_wait();
    }
}

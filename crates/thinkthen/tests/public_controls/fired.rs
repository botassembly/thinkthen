//! Ticket 0166: a token fired while a send is in flight ends the call cancelled.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;

use conformance_backend::{Canned, Listener};
use thinkthen::{Answer, CallOptions, CancelToken, ErrorKind};

use super::{DECIDED, engine, kind, question, serial};

/// A token fired during a send ends the call cancelled, whatever
/// the reply, and a retry the reply asks for never goes.
#[test]
fn a_token_fired_during_a_send_ends_the_call_cancelled() {
    let _serial = serial();
    for status in [503, 422] {
        let release = Arc::new(Barrier::new(2));
        let held = Arc::clone(&release);
        let first = AtomicUsize::new(0);
        let listener = Listener::answering(move |_| {
            if first.fetch_add(1, Ordering::SeqCst) > 0 {
                return Canned::ok(DECIDED);
            }
            let refused = Canned::status(status, "").after_release(Arc::clone(&held));
            refused.asking("retry-after-ms", "0")
        })
        .expect("listener");
        let (token, engine) = (CancelToken::new(), engine(listener.base()));
        let result = thread::scope(|scope| {
            scope.spawn(|| fire_on_arrival(&listener, &token, &release));
            let options = CallOptions::new().cancel(&token);
            engine.decide_with(&question(), "Refund me.", options)
        });
        assert_eq!(kind(&result), Some(ErrorKind::Cancelled), "{status}");
        assert_eq!(
            listener.count(),
            1,
            "{status}: nothing was sent after the fire"
        );
    }
}

/// A token fired after a batch's last row ends the batch cancelled.
#[test]
fn a_token_fired_before_a_batch_ends_ends_it_cancelled() {
    let _serial = serial();
    let listener = Listener::answering(|_| Canned::ok(DECIDED)).expect("listener");
    let engine = engine(listener.base());
    let (token, asked) = (CancelToken::new(), question());
    let options = CallOptions::new().cancel(&token);
    let mut rows = engine.decide_many_with(&asked, ["Refund me."], options);
    assert_eq!(
        rows.next().map(|row| row.map(|row| *row.value()).ok()),
        Some(Some(Answer::Yes))
    );
    token.cancel();
    assert_eq!(
        rows.next().as_ref().and_then(kind),
        Some(ErrorKind::Cancelled)
    );
    assert!(rows.next().is_none());
    assert_eq!(listener.count(), 1);
}

/// Fire the token once the listener has the request, then let its reply go.
fn fire_on_arrival(listener: &Listener, token: &CancelToken, release: &Barrier) {
    let start = std::time::Instant::now();
    while listener.count() < 1 {
        assert!(start.elapsed() < super::BOUND, "no request arrived");
        thread::sleep(Duration::from_millis(5));
    }
    token.cancel();
    release.wait();
}

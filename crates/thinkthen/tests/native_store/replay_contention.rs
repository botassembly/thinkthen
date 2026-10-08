//! A replay's snapshot admission obeys public stop controls.
use super::*;
use std::time::{Duration, Instant};

#[test]
fn replay_waiting_for_a_writer_obeys_the_call_deadline_and_cancellation() {
    let listener = Listener::answering(|_| Canned::ok(REPLY)).unwrap();
    let place = folder();
    let question = Question::decide("Refund?").unwrap().cut();
    let record = build(&listener).record(&place).unwrap().build().unwrap();
    record.details(&question, "Refund me.").unwrap();
    drop(record);
    let holder = Connection::open(place.join("thinkthen.sqlite")).unwrap();
    holder.execute_batch("BEGIN EXCLUSIVE").unwrap();
    let mut kinds = Vec::new();
    for deadline in [true, false] {
        let replay = build(&listener).replay(&place).unwrap().build().unwrap();
        let started = Instant::now();
        let cancel = || started.elapsed() >= Duration::from_millis(50);
        let options = if deadline {
            CallOptions::new()
                .deadline_after(Duration::from_millis(50))
                .unwrap()
        } else {
            CallOptions::new().interrupt(&cancel)
        };
        let error = replay
            .details_with(&question, "Refund me.", options)
            .unwrap_err();
        kinds.push(error.kind());
        assert!(
            started.elapsed() < Duration::from_secs(10),
            "stop bounded the wait"
        );
    }
    assert_eq!(listener.count(), 1, "neither replay sent a request");
    holder.execute_batch("ROLLBACK").unwrap();
    drop(holder);
    std::fs::remove_dir_all(place).unwrap();
    assert_eq!(kinds, [ErrorKind::Deadline, ErrorKind::Cancelled]);
}

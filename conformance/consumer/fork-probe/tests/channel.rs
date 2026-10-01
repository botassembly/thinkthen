//! Ticket 0365: every wait on the crate's fork-safe channel answers in a
//! forked child.
//!
//! On macOS a std channel wait parks the thread on a semaphore a forked child
//! cannot use, so a child whose thread waited once before the fork crashed on
//! its next wait. The parent's test thread waits first here, and each wait
//! truly blocks, because its sender acts only after a pause.

use std::thread;
use std::time::Duration;

use fork_probe::in_child;
use thinkthen::fork_safe::{Receiver, RecvError, RecvTimeoutError, channel};

const PAUSE: Duration = Duration::from_millis(30);
const BOUND: Duration = Duration::from_secs(10);

/// A receiver whose one sender sends `value` after the pause, or ends unsent.
fn later(value: Option<u8>) -> Receiver<u8> {
    let (sender, receiver) = channel();
    thread::spawn(move || {
        thread::sleep(PAUSE);
        if let Some(value) = value {
            let _sent = sender.send(value);
        }
    });
    receiver
}

fn every_wait_answers() -> bool {
    let (_held, quiet) = channel::<u8>();
    later(Some(1)).recv() == Ok(1)
        && later(Some(2)).recv_timeout(BOUND) == Ok(2)
        && later(None).recv() == Err(RecvError)
        && later(None).recv_timeout(BOUND) == Err(RecvTimeoutError::Disconnected)
        && quiet.recv_timeout(PAUSE) == Err(RecvTimeoutError::Timeout)
}

#[test]
fn every_channel_wait_answers_in_a_forked_child() {
    assert!(every_wait_answers(), "the parent's waits answer");
    in_child(every_wait_answers).expect("the child's waits answer");
}

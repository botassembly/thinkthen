#![cfg(test)]
//! A kept thread's end has no outside signal, so these drive the kept threads directly.
use super::threads::Threads;
use std::cell::RefCell;
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender, TryRecvError, channel};
use std::thread::{self, ThreadId};
use std::time::Duration;

thread_local! {
    // Dropped when its thread ends, which disconnects the test's receiver.
    static ENDS: RefCell<Option<Sender<()>>> = const { RefCell::new(None) };
}

#[test]
fn a_thread_given_no_work_ends() {
    let threads = Threads::ending_after(Duration::from_millis(1));
    let (end, ended) = channel();
    threads.run(move || ENDS.set(Some(end))).unwrap();
    assert_eq!(
        ended.recv_timeout(Duration::from_secs(60)),
        Err(RecvTimeoutError::Disconnected)
    );
}

#[test]
fn releasing_the_client_ends_its_threads() {
    let threads = Threads::default();
    let (end, ended) = channel();
    threads.run(move || ENDS.set(Some(end))).unwrap();
    drop(threads);
    assert_eq!(ended.try_recv(), Err(TryRecvError::Disconnected));
}

#[test]
fn a_thread_whose_work_panicked_takes_the_next_work() {
    let threads = Threads::default();
    let (first, ran) = channel::<ThreadId>();
    threads
        .run(move || {
            first.send(thread::current().id()).unwrap();
            // The payload is dropped as a returned value is: once the thread is free.
            std::panic::panic_any(first)
        })
        .unwrap();
    let kept = ran.recv().unwrap();
    assert!(ran.recv().is_err());
    let (second, ran) = channel();
    threads
        .run(move || second.send(thread::current().id()).unwrap())
        .unwrap();
    assert_eq!(ran.recv().unwrap(), kept);
}

#[test]
fn work_at_once_starts_threads_and_later_work_takes_the_last_freed() {
    let threads = Threads::default();
    let (ran, ids) = channel::<ThreadId>();
    // Work that holds its thread until `gate` closes; the receiver closes once the thread is free.
    let start = |gate: Receiver<()>| {
        let (free, freed) = channel::<()>();
        let ran = ran.clone();
        threads
            .run(move || {
                ran.send(thread::current().id()).unwrap();
                let _ = gate.recv();
                free
            })
            .unwrap();
        freed
    };
    let (open_first, gate) = channel();
    let first_freed = start(gate);
    let first = ids.recv().unwrap();
    let (open_second, gate) = channel();
    let second_freed = start(gate);
    let second = ids.recv().unwrap();
    assert_ne!(first, second);
    drop(open_first);
    assert!(first_freed.recv().is_err());
    drop(open_second);
    assert!(second_freed.recv().is_err());
    // The longest idle thread is left alone, so a surplus thread reaches its idle end.
    let (_open_third, gate) = channel();
    let _third_freed = start(gate);
    assert_eq!(ids.recv().unwrap(), second);
    let (_open_fourth, gate) = channel();
    let _fourth_freed = start(gate);
    assert_eq!(ids.recv().unwrap(), first);
}

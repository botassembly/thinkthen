//! The rebuild rule for process-owned state, as an edge-case table and one
//! race. A forked child is the same memory under another process ID, so each
//! row publishes state as one process and then calls as another.
//!
//! Every call runs on its own thread and fails on `recv_timeout`, so a rule
//! that waits on an inherited marker fails the row and never hangs the gate.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::channel;
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;

use super::Guarded;

const PARENT: u32 = 41;
const CHILD: u32 = 42;
const OTHER: u32 = 43;
const BOUND: Duration = Duration::from_secs(30);

type Built = Result<&'static str, &'static str>;
type Called = (Result<Arc<&'static str>, &'static str>, usize);
type Row = (&'static str, u32, u32, Built, Built, usize);

/// Call `current` as `pid` on its own thread: the answer and how many builds ran.
fn call(guarded: &Arc<Guarded<&'static str>>, pid: u32, build: Built) -> Called {
    let (sent, received) = channel();
    let guarded = Arc::clone(guarded);
    thread::spawn(move || {
        let builds = AtomicUsize::new(0);
        let answer = guarded.current(
            pid,
            || Err("waited on a rebuild"),
            || {
                builds.fetch_add(1, Ordering::SeqCst);
                build
            },
        );
        drop(guarded);
        let _sent = sent.send((answer, builds.into_inner()));
    });
    received
        .recv_timeout(BOUND)
        .unwrap_or_else(|_| panic!("process {pid} is still waiting"))
}

#[test]
fn each_owner_and_marker_gives_one_answer() {
    // (row, rebuild marker left by the parent's memory, caller, build result,
    //  answer, builds)
    let rows: [Row; 7] = [
        (
            "the owner keeps its state",
            0,
            PARENT,
            Ok("new"),
            Ok("parent"),
            0,
        ),
        ("a child builds", 0, CHILD, Ok("child"), Ok("child"), 1),
        (
            "a child replaces the parent's rebuild",
            PARENT,
            CHILD,
            Ok("child"),
            Ok("child"),
            1,
        ),
        (
            "a grandchild replaces an older rebuild",
            OTHER,
            CHILD,
            Ok("child"),
            Ok("child"),
            1,
        ),
        (
            "a failed build is returned",
            0,
            CHILD,
            Err("refused"),
            Err("refused"),
            1,
        ),
        (
            "the owner never waits on a rebuild",
            CHILD,
            PARENT,
            Ok("new"),
            Ok("parent"),
            0,
        ),
        (
            "a child waits on its own rebuild",
            CHILD,
            CHILD,
            Ok("new"),
            Err("waited on a rebuild"),
            0,
        ),
    ];
    for (row, marker, caller, build, answer, builds) in rows {
        let guarded = Arc::new(Guarded::empty());
        let parent = call(&guarded, PARENT, Ok("parent"))
            .0
            .expect("parent state");
        guarded.rebuilding.store(marker, Ordering::SeqCst);

        let (got, built) = call(&guarded, caller, build);

        assert_eq!((got.map(|state| *state), built), (answer, builds), "{row}");
        assert_eq!(
            Arc::strong_count(&parent),
            2,
            "{row}: the parent state is never dropped"
        );
        if builds == 1 {
            assert_eq!(
                guarded.rebuilding.load(Ordering::SeqCst),
                0,
                "{row}: the marker is cleared"
            );
        }
    }
}

#[test]
fn a_failed_build_leaves_the_next_call_to_build() {
    let guarded = Arc::new(Guarded::empty());
    let _parent = call(&guarded, PARENT, Ok("parent"));
    assert_eq!(call(&guarded, CHILD, Err("refused")).0, Err("refused"));

    let (got, built) = call(&guarded, CHILD, Ok("child"));

    assert_eq!((got.map(|state| *state), built), (Ok("child"), 1));
    assert_eq!(
        call(&guarded, CHILD, Ok("again")).1,
        0,
        "the child now owns its state"
    );
}

#[test]
fn a_drop_frees_only_the_state_this_process_built() {
    for (owner, count) in [(std::process::id(), 1), (PARENT, 2)] {
        let guarded = Arc::new(Guarded::empty());
        let state = call(&guarded, owner, Ok("state")).0.expect("state");
        drop(guarded);
        assert_eq!(Arc::strong_count(&state), count, "owner {owner}");
    }
}

#[test]
fn racing_children_publish_one_state() {
    let guarded = Arc::new(Guarded::empty());
    let _parent = call(&guarded, PARENT, Ok("parent"));
    let callers = 8;
    let start = Arc::new(Barrier::new(callers));
    let builds = Arc::new(AtomicUsize::new(0));
    let (sent, received) = channel();
    for _ in 0..callers {
        let (guarded, start, builds, sent) = (
            Arc::clone(&guarded),
            Arc::clone(&start),
            Arc::clone(&builds),
            sent.clone(),
        );
        thread::spawn(move || {
            start.wait();
            let state = guarded.current(
                CHILD,
                || {
                    thread::sleep(Duration::from_millis(1));
                    Ok::<(), ()>(())
                },
                || {
                    builds.fetch_add(1, Ordering::SeqCst);
                    thread::sleep(Duration::from_millis(20));
                    Ok("child")
                },
            );
            let _sent = sent.send(state);
        });
    }
    let states: Vec<_> = (0..callers)
        .map(|_| {
            received
                .recv_timeout(BOUND)
                .expect("a racing child finished")
                .expect("state")
        })
        .collect();

    assert_eq!(builds.load(Ordering::SeqCst), 1);
    assert!(states.iter().all(|state| Arc::ptr_eq(state, &states[0])));
}

//! A host stop while relate's sends overlap, run only under stress.

use std::sync::{Mutex, PoisonError};
use std::thread;

use conformance_backend::Backend;
use thinkthen::{CallOptions, Entity, ErrorKind, Relate};

use super::{Runs, engine, kind, serial};

/// Shared pair requests fill the default throttle of 4. The host check runs
/// only while no send is out, so it fires at the first such moment after
/// four sends, and nothing is sent after it fires. Replies that overlap
/// until the call ends can leave no such moment, so a loaded machine can
/// fail this row. The routine row at throttle 1 proves the stop by order
/// (ticket 0352).
#[test]
#[ignore = "overlapping sends on a loaded machine; run sdlc/scripts/test-stress --run"]
fn a_host_interrupt_while_relate_sends_overlap_stops_between_four_and_eight() {
    let _serial = serial();
    let entities = (0..60)
        .map(|n| Entity::new(&format!("service {n}"), "service").expect("entity"))
        .collect::<Vec<_>>();
    let backend = Backend::start().expect("backend");
    let held = engine(&format!("{}/arm/held/v1", backend.origin()));
    let ask = Relate::from_json(
        r#"{"version":1,"relate":{"relations":[{"name":"knows","source":"service","target":"service"}]}}"#,
    )
    .expect("relate file");
    let runs = Runs::default();
    let stopped_at = Mutex::new(None);
    let check = || {
        let sent = backend.count();
        runs.record(sent);
        let mut held = stopped_at.lock().unwrap_or_else(PoisonError::into_inner);
        if sent >= 4 {
            held.get_or_insert(sent);
        }
        held.is_some()
    };
    let result = thread::scope(|scope| {
        scope.spawn(|| {
            backend.wait(4);
            backend.release();
        });
        held.relate_with(&ask, entities, CallOptions::new().interrupt(&check))
    });
    assert!(runs.all_on(thread::current().id()), "a check ran on a worker");
    assert_eq!(kind(&result), Some(ErrorKind::Cancelled));
    let stopped_at = stopped_at
        .into_inner()
        .unwrap_or_else(PoisonError::into_inner)
        .expect("the check fired");
    assert_eq!(backend.count(), stopped_at, "nothing new was sent");
    assert!((4..=8).contains(&stopped_at), "stopped at {stopped_at}");
}

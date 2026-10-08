//! Conversion and replay while another caller has the live store open.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, mpsc};
use std::time::Duration;

use conformance_backend::Rendezvous;

use crate::harness::{Canned, Listener, finish, spawn, start};

const REPLY: &str = r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}}}"#;

fn folder(name: &str) -> PathBuf {
    let folder = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("cache-coexistence-{name}-{}", std::process::id()));
    fs::create_dir(&folder).expect("owned folder");
    folder
}

fn arguments<'a>(base: &'a str, path: &'a str, mode: &'a str) -> [&'a str; 8] {
    [
        "decide", "Refund?", "--url", base, "--model", "fixed", mode, path,
    ]
}

#[test]
#[cfg(unix)]
fn a_writer_opened_before_conversion_fails_clearly_and_retained_answers_replay() {
    let folder = folder("opened-writer");
    let path = folder.to_str().expect("folder path");
    let gate = Arc::new(Rendezvous::new(2));
    let held = Arc::clone(&gate);
    let (arrived, requests) = mpsc::channel();
    let listener = Listener::answering(move |_| {
        arrived.send(()).expect("request arrival");
        Canned::ok(REPLY).after_release(Arc::clone(&held))
    })
    .expect("loopback listener");
    let record = arguments(listener.base(), path, "--record");
    let seed = start(&record, &[], b"Refund the first order.").expect("seed writer");
    requests
        .recv_timeout(Duration::from_secs(10))
        .expect("seed request");
    assert!(gate.wait());
    let seed = finish(seed, "seed writer").expect("seed result");
    assert_eq!(seed.status.code(), Some(0));
    assert_eq!(seed.stdout, b"true\n");

    let writer = start(&record, &[], b"Refund the second order.").expect("open writer");
    requests
        .recv_timeout(Duration::from_secs(10))
        .expect("writer opened the existing store before sending");
    let converted = spawn(&["cache", "convert", path], &[], b"").expect("converter");
    assert!(gate.wait());
    let writer = finish(writer, "writer after conversion").expect("writer result");
    assert_eq!(converted.status.code(), Some(0), "{converted:?}");
    assert_eq!(writer.status.code(), Some(5), "{writer:?}");
    assert!(writer.stdout.is_empty(), "{writer:?}");
    assert!(!folder.join("thinkthen.sqlite").exists());

    let replay = arguments(listener.base(), path, "--replay");
    let retained = spawn(&replay, &[], b"Refund the first order.").expect("fresh replay");
    assert_eq!(retained.status.code(), Some(0), "{retained:?}");
    assert_eq!(retained.stdout, b"true\n");
    let absent = spawn(&replay, &[], b"Refund the second order.").expect("failed answer replay");
    assert_eq!(absent.status.code(), Some(5), "{absent:?}");
    assert!(absent.stdout.is_empty());
    assert_eq!(listener.count(), 2);
    fs::remove_dir_all(folder).expect("owned folder cleanup");
}

#[test]
fn replay_waits_for_an_exclusive_writer_and_reads_the_committed_answer_without_sending() {
    let folder = folder("replay-writer");
    let path = folder.to_str().expect("folder path");
    let listener = Listener::answering(|_| Canned::ok(REPLY)).expect("loopback listener");
    let seed = spawn(
        &arguments(listener.base(), path, "--record"),
        &[],
        b"Refund the first order.",
    )
    .expect("seed writer");
    assert_eq!(seed.status.code(), Some(0), "{seed:?}");
    let database = folder.join("thinkthen.sqlite");
    let (locked, holding) = mpsc::channel();
    let writer = std::thread::spawn(move || {
        let connection = rusqlite::Connection::open(database).expect("writer database");
        connection
            .execute_batch("BEGIN EXCLUSIVE")
            .expect("writer lock");
        connection
            .execute(
                "UPDATE answers SET answer=?1",
                [r#"{"type":"noul","noul":0.1}"#],
            )
            .expect("pending answer");
        locked.send(()).expect("lock signal");
        std::thread::sleep(Duration::from_millis(300));
        connection.execute_batch("COMMIT").expect("writer commit");
    });
    holding
        .recv_timeout(Duration::from_secs(10))
        .expect("writer holds lock");
    let replay = spawn(
        &arguments(listener.base(), path, "--replay"),
        &[],
        b"Refund the first order.",
    )
    .expect("replay process");
    writer.join().expect("writer finished");
    assert_eq!(listener.count(), 1, "replay sends no request");
    fs::remove_dir_all(folder).expect("owned folder cleanup");
    assert_eq!(replay.status.code(), Some(1), "{replay:?}");
    assert_eq!(replay.stdout, b"false\n");
}

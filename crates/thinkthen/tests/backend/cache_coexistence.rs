//! Conversion and replay while another caller has the live store open.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
#[cfg(unix)]
use std::sync::Arc;
use std::sync::mpsc;
use std::time::Duration;

#[cfg(unix)]
use conformance_backend::Rendezvous;

use crate::harness::{Canned, Listener, spawn};
#[cfg(unix)]
use crate::harness::{finish, start};

const REPLY: &str = r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}}}"#;

fn folder(name: &str) -> io::Result<PathBuf> {
    let folder = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("cache-coexistence-{name}-{}", std::process::id()));
    fs::create_dir(&folder)?;
    Ok(folder)
}

fn arguments<'a>(base: &'a str, path: &'a str, mode: &'a str) -> [&'a str; 8] {
    [
        "decide", "Refund?", "--url", base, "--model", "fixed", mode, path,
    ]
}

#[test]
#[cfg(unix)]
fn an_open_writer_and_conversion_preserve_successful_answers_or_refuse_the_writer() {
    for convert_first in [true, false] {
        let folder = folder(&format!("opened-writer-{convert_first}")).expect("owned folder");
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
        let (converted, writer) = if convert_first {
            let converted = spawn(&["cache", "convert", path], &[], b"").expect("converter");
            assert!(gate.wait());
            (
                converted,
                finish(writer, "writer after conversion").expect("writer result"),
            )
        } else {
            assert!(gate.wait());
            let writer = finish(writer, "writer before conversion").expect("writer result");
            (
                spawn(&["cache", "convert", path], &[], b"").expect("converter"),
                writer,
            )
        };
        assert_eq!(converted.status.code(), Some(0), "{converted:?}");
        let (code, output) = if convert_first {
            (5, b"".as_slice())
        } else {
            (0, b"true\n".as_slice())
        };
        assert_eq!(writer.status.code(), Some(code), "{writer:?}");
        assert_eq!(writer.stdout, output, "{writer:?}");
        assert!(!folder.join("thinkthen.sqlite").exists());

        let replay = arguments(listener.base(), path, "--replay");
        let retained = spawn(&replay, &[], b"Refund the first order.").expect("fresh replay");
        assert_eq!(retained.status.code(), Some(0), "{retained:?}");
        assert_eq!(retained.stdout, b"true\n");
        let absent =
            spawn(&replay, &[], b"Refund the second order.").expect("failed answer replay");
        assert_eq!(absent.status.code(), Some(code), "{absent:?}");
        assert_eq!(absent.stdout, output);
        assert_eq!(listener.count(), 2);
        fs::remove_dir_all(folder).expect("owned folder cleanup");
    }
}

#[test]
fn replay_waits_for_an_exclusive_writer_and_reads_the_committed_answer_without_sending() {
    let folder = folder("replay-writer").expect("owned folder");
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

#[test]
#[cfg(windows)]
fn conversion_refuses_windows_peer_handles_and_a_retry_preserves_every_answer() {
    for read_only in [false, true] {
        for previous_fixture in [false, true] {
            let folder = folder(&format!("windows-peer-{read_only}-{previous_fixture}"))
                .expect("owned folder");
            let path = folder.to_str().expect("folder path");
            let listener = Listener::answering(|_| Canned::ok(REPLY)).expect("loopback listener");
            let seed = spawn(
                &arguments(listener.base(), path, "--record"),
                &[],
                b"Refund the first order.",
            )
            .expect("seed writer");
            assert_eq!(seed.status.code(), Some(0), "{seed:?}");
            let snapshot = folder.join("snapshot");
            fs::create_dir(&snapshot).expect("owned snapshot folder");
            fs::copy(
                folder.join("thinkthen.sqlite"),
                snapshot.join("thinkthen.sqlite"),
            )
            .expect("snapshot");
            let converted = spawn(
                &[
                    "cache",
                    "convert",
                    snapshot.to_str().expect("snapshot path"),
                ],
                &[],
                b"",
            )
            .expect("snapshot converter");
            assert_eq!(converted.status.code(), Some(0), "{converted:?}");
            let original = fs::read(snapshot.join("thinkthen.jsonl")).expect("snapshot fixture");
            if previous_fixture {
                fs::write(folder.join("thinkthen.jsonl"), &original).expect("existing fixture");
            }
            let flags = if read_only {
                rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY
            } else {
                rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE
            };
            let peer =
                rusqlite::Connection::open_with_flags(folder.join("thinkthen.sqlite"), flags)
                    .expect("peer handle");
            let refused =
                spawn(&["cache", "convert", path], &[], b"").expect("converter with peer");
            assert_eq!(refused.status.code(), Some(5), "{refused:?}");
            assert!(refused.stdout.is_empty());
            assert_eq!(
                String::from_utf8(refused.stderr).expect("diagnostic"),
                "thinkthen: the recording folder could not be read or written; check its permissions and free space\n"
            );
            assert!(folder.join("thinkthen.sqlite").exists());
            if previous_fixture {
                assert_eq!(
                    fs::read(folder.join("thinkthen.jsonl")).expect("restored fixture"),
                    original
                );
            } else {
                assert!(
                    !folder.join("thinkthen.jsonl").exists(),
                    "refusal leaves no competing fixture"
                );
            }
            drop(peer);
            let retried =
                spawn(&["cache", "convert", path], &[], b"").expect("retry after peer closes");
            assert_eq!(retried.status.code(), Some(0), "{retried:?}");
            assert!(!folder.join("thinkthen.sqlite").exists());
            let retained = spawn(
                &arguments(listener.base(), path, "--replay"),
                &[],
                b"Refund the first order.",
            )
            .expect("fresh replay");
            assert_eq!(retained.status.code(), Some(0), "{retained:?}");
            assert_eq!(retained.stdout, b"true\n");
            assert_eq!(listener.count(), 1, "replay sends nothing");
            fs::remove_dir_all(folder).expect("owned folder cleanup");
        }
    }
}

#![cfg(test)]
#![cfg(unix)]
//! Interrupt the real command at observable file and backend boundaries.

use super::support;
use loopback::{HeldReply, KEY, Loopback};
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc;
use std::time::{Duration, Instant};

fn interrupt(pid: u32) {
    assert!(
        Command::new("kill")
            .args(["-INT", &pid.to_string()])
            .status()
            .unwrap()
            .success()
    );
}

#[test]
fn cancellation_during_an_open_request_publishes_nothing() {
    for in_place in [false, true] {
        let directory = tempfile::tempdir().unwrap();
        let input = directory.path().join("input.csv");
        let original = b"Body\nalpha\nbeta\n";
        fs::write(&input, original).unwrap();
        let (server, received, closed) = Loopback::holding(HeldReply::Headers).unwrap();
        let mut command = support::command();
        command
            .args(["decide", "Ready?", "--csv", "--input"])
            .arg(&input)
            .args([
                "--column",
                "Body",
                "--into",
                "ready",
                "--batch",
                "1",
                "--url",
                server.base(),
            ])
            .env("THINKTHEN_API_KEY", KEY)
            .env("TMPDIR", directory.path())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        if in_place {
            command.arg("--in-place");
        }
        let child = command.spawn().unwrap();
        received.recv().unwrap();
        interrupt(child.id());
        let output = child.wait_with_output().unwrap();
        assert_eq!(output.status.code(), Some(130));
        assert!(output.stdout.is_empty());
        assert!(closed.recv().unwrap().is_ok());
        assert_eq!(fs::read(&input).unwrap(), original);
        assert_eq!(fs::read_dir(directory.path()).unwrap().count(), 1);
    }
}

#[test]
fn cancellation_after_backup_removes_only_the_pending_publication() {
    for (iteration, forced) in [false, true].into_iter().enumerate() {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path();
        let input = root.join("input.csv");
        let saved = root.join("source.csv");
        let original = b"Body\nalpha\n";
        fs::write(&input, original).unwrap();
        let prior_backup = root.join("input.backup.csv");
        fs::write(&prior_backup, b"older backup").unwrap();
        let (changed, received) = std::sync::mpsc::channel();
        let swapped = input.clone();
        let preserved = saved.clone();
        let server = Loopback::answering(move |_| {
            // The snapshot is complete. Hold the final source hash on a FIFO.
            fs::rename(&swapped, &preserved).unwrap();
            assert!(
                Command::new("mkfifo")
                    .arg(&swapped)
                    .status()
                    .unwrap()
                    .success()
            );
            changed.send(()).unwrap();
            serde_json::json!({"type":"noul","noul":0.99})
        })
        .unwrap();
        let child = support::command()
            .args(["decide", "Ready?", "--csv", "--input"])
            .arg(&input)
            .args([
                "--column",
                "Body",
                "--into",
                "ready",
                "--in-place",
                "--url",
                server.base(),
            ])
            .env("THINKTHEN_API_KEY", KEY)
            .env("TMPDIR", root)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let mut child = Some(child);
        let mut status = None;
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let process = child.as_mut().unwrap();
            wait_for(process, || received.try_recv().ok());
            wait_for(process, || {
                root.join("input.backup-2.csv").exists().then_some(())
            });
            let (opened, writer) = mpsc::channel();
            let fifo = input.clone();
            std::thread::spawn(move || {
                let _ = opened.send(OpenOptions::new().write(true).open(fifo));
            });
            let mut feed = wait_for(process, || writer.try_recv().ok()).unwrap();
            // Keep the FIFO path until cancellation acknowledges the held operation.
            interrupt(process.id());
            let stderr = process.stderr.take().unwrap();
            let (acknowledged, notice) = mpsc::channel();
            std::thread::spawn(move || {
                let mut line = String::new();
                let read = BufReader::new(stderr).read_line(&mut line);
                let _ = acknowledged.send(read.map(|_| line));
            });
            let notice = wait_for(process, || notice.try_recv().ok()).unwrap();
            assert_eq!(
                notice,
                "thinkthen: cancelling; press Ctrl-C again to exit\n"
            );
            fs::rename(&saved, &input).unwrap();
            if forced {
                interrupt(process.id());
            } else if let Err(error) = feed.write_all(original) {
                assert_eq!(error.kind(), std::io::ErrorKind::BrokenPipe);
            }
            drop(feed);
            let deadline = Instant::now() + Duration::from_secs(30);
            while process.try_wait().unwrap().is_none() {
                assert!(
                    Instant::now() < deadline,
                    "child did not exit after cancellation"
                );
                std::thread::park_timeout(Duration::from_millis(1));
            }
            let output = child.take().unwrap().wait_with_output().unwrap();
            status = Some(output.status);
            assert_eq!(output.status.code(), Some(130));
            assert!(output.stdout.is_empty());
            assert_eq!(fs::read(&input).unwrap(), original);
            assert_eq!(fs::read(prior_backup).unwrap(), b"older backup");
            assert_eq!(
                fs::read_dir(root).unwrap().count(),
                2,
                "an output, provisional backup, lock or spool survived cancellation"
            );
        }));
        if let Err(failure) = result {
            let child_status = child.as_mut().map(|process| process.try_wait());
            if let Some(process) = child.as_mut() {
                let _ = process.kill();
                let _ = process.wait();
            }
            let files: Vec<_> = fs::read_dir(root)
                .unwrap()
                .map(|entry| entry.unwrap().file_name())
                .collect();
            let path = directory.keep();
            let reason = failure
                .downcast_ref::<String>()
                .map(String::as_str)
                .or_else(|| failure.downcast_ref::<&str>().copied());
            panic!(
                "iteration={iteration} forced={forced} child={child_status:?} completed={status:?} files={files:?} retained={} failure={reason:?}",
                path.display()
            );
        }
    }
}

fn wait_for<T>(child: &mut Child, mut observe: impl FnMut() -> Option<T>) -> T {
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        assert!(
            child.try_wait().unwrap().is_none(),
            "child exited before fixture event"
        );
        if let Some(value) = observe() {
            return value;
        }
        assert!(Instant::now() < deadline, "fixture event did not arrive");
        std::thread::park_timeout(Duration::from_millis(1));
    }
}

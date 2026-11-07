#![cfg(test)]
//! A failed final publication keeps the finished output and names it.

use super::support;
use crate::loopback::{KEY, Loopback};
use serde_json::json;
use std::fs;
use std::path::Path;
use std::process::Output;

const ORIGINAL: &[u8] = b"Body\nalpha\n";

fn assert_kept(result: &Output, kept: &Path) {
    assert_eq!(result.status.code(), Some(5), "{result:?}");
    assert_eq!(support::reported(result), "local.io");
    let diagnostic = String::from_utf8_lossy(&result.stderr);
    assert!(diagnostic.contains(kept.to_str().unwrap()), "{diagnostic}");
    assert_eq!(
        fs::read_to_string(kept).unwrap(),
        "Body,ready\nalpha,TRUE\n"
    );
}

#[test]
fn an_output_folder_that_cannot_take_the_file_stops_before_any_call() {
    let directory = tempfile::tempdir().unwrap();
    let input = directory.path().join("input.csv");
    fs::write(&input, ORIGINAL).unwrap();
    let server = Loopback::noul(0.99).unwrap();
    let result = support::command()
        .args(["decide", "Ready?", "--from", "csv"])
        .arg(&input)
        .args(["--column", "Body", "--into", "ready", "--output"])
        .arg(directory.path().join("no-such-folder").join("result.csv"))
        .args(["--url", server.base()])
        .env("THINKTHEN_API_KEY", KEY)
        .output()
        .unwrap();
    assert_eq!(result.status.code(), Some(5), "{result:?}");
    assert_eq!(support::reported(&result), "local.io");
    assert_eq!(server.count(), 0);
}

#[cfg(unix)]
#[test]
fn a_kept_name_that_cannot_exist_leaves_the_output_under_its_temporary_name() {
    let directory = tempfile::tempdir().unwrap();
    let input = directory.path().join("input.csv");
    fs::write(&input, ORIGINAL).unwrap();
    let elsewhere = directory.path().join("elsewhere");
    fs::create_dir(&elsewhere).unwrap();
    // The longest name a folder takes, so no longer sibling name can exist.
    let output = elsewhere.join(format!("{}.csv", "a".repeat(251)));
    let taken = output.clone();
    let server = Loopback::answering(move |_| {
        fs::write(&taken, b"another program's file").unwrap();
        json!({"type":"noul","noul":0.99})
    })
    .unwrap();
    let result = support::command()
        .args(["decide", "Ready?", "--from", "csv"])
        .arg(&input)
        .args(["--column", "Body", "--into", "ready", "--output"])
        .arg(&output)
        .args(["--url", server.base()])
        .env("THINKTHEN_API_KEY", KEY)
        .output()
        .unwrap();
    let kept = fs::read_dir(&elsewhere)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .find(|path| *path != output);
    assert_kept(&result, &kept.expect("the finished output survives"));
    assert_eq!(fs::read(&output).unwrap(), b"another program's file");
}

#[cfg(target_os = "linux")]
#[test]
fn a_file_name_that_is_not_utf8_stops_before_any_call() {
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt;

    let directory = tempfile::tempdir().unwrap();
    let input = directory
        .path()
        .join(OsString::from_vec(b"in\xffput.csv".to_vec()));
    fs::write(&input, ORIGINAL).unwrap();
    let server = Loopback::noul(0.99).unwrap();
    let result = support::command()
        .args(["decide", "Ready?", "--from", "csv"])
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
        .output()
        .unwrap();
    assert_eq!(result.status.code(), Some(2), "{result:?}");
    assert_eq!(support::reported(&result), "usage.option");
    assert_eq!(server.count(), 0);
    assert_eq!(fs::read(&input).unwrap(), ORIGINAL);
}

#[test]
fn a_destination_created_during_the_request_keeps_the_finished_output_beside_it() {
    let directory = tempfile::tempdir().unwrap();
    let input = directory.path().join("input.csv");
    fs::write(&input, ORIGINAL).unwrap();
    let elsewhere = directory.path().join("elsewhere");
    fs::create_dir(&elsewhere).unwrap();
    let output = elsewhere.join("result.csv");
    let taken = output.clone();
    let server = Loopback::answering(move |_| {
        fs::write(&taken, b"another program's file").unwrap();
        json!({"type":"noul","noul":0.99})
    })
    .unwrap();
    let result = support::command()
        .args(["decide", "Ready?", "--from", "csv"])
        .arg(&input)
        .args(["--column", "Body", "--into", "ready", "--output"])
        .arg(&output)
        .args(["--url", server.base()])
        .env("THINKTHEN_API_KEY", KEY)
        .output()
        .unwrap();
    assert_kept(&result, &elsewhere.join("result.thinkthen.csv"));
    assert_eq!(fs::read(&output).unwrap(), b"another program's file");
    assert_eq!(fs::read(&input).unwrap(), ORIGINAL);
    assert_eq!(fs::read_dir(&elsewhere).unwrap().count(), 2);
}

#[cfg(unix)]
#[test]
fn a_failed_in_place_swap_keeps_the_finished_output_and_the_backup() {
    use crate::cancellation::wait_for;
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::process::{Command, Stdio};
    use std::sync::mpsc;

    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    let input = root.join("input.csv");
    let saved = root.join("source.csv");
    fs::write(&input, ORIGINAL).unwrap();
    let (changed, received) = mpsc::channel();
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
        json!({"type":"noul","noul":0.99})
    })
    .unwrap();
    let mut child = support::command()
        .args(["decide", "Ready?", "--from", "csv"])
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
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    wait_for(&mut child, || received.try_recv().ok());
    let (opened, writer) = mpsc::channel();
    let fifo = input.clone();
    std::thread::spawn(move || {
        let _ = opened.send(OpenOptions::new().write(true).open(fifo));
    });
    let mut feed = wait_for(&mut child, || writer.try_recv().ok()).unwrap();
    feed.write_all(ORIGINAL).unwrap();
    // The hash ends when the feed closes, so the swap then meets a directory.
    fs::rename(&input, root.join("fifo")).unwrap();
    fs::create_dir(&input).unwrap();
    drop(feed);
    let result = child.wait_with_output().unwrap();
    assert_kept(&result, &root.join("input.thinkthen.csv"));
    assert_eq!(fs::read(root.join("input.backup.csv")).unwrap(), ORIGINAL);
    assert_eq!(fs::read(saved).unwrap(), ORIGINAL);
}

#[cfg(unix)]
#[test]
fn a_failed_backup_keeps_the_finished_output_and_the_source() {
    use crate::cancellation::wait_for;
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::process::{Command, Stdio};
    use std::sync::mpsc;

    let directory = tempfile::tempdir().unwrap();
    let root = directory.path().join("files");
    let scratch = directory.path().join("scratch");
    fs::create_dir(&root).unwrap();
    fs::create_dir(&scratch).unwrap();
    let input = root.join("input.csv");
    fs::write(&input, ORIGINAL).unwrap();
    let (changed, received) = mpsc::channel();
    let spool = scratch.clone();
    let server = Loopback::answering(move |_| {
        // The snapshot is the scratch file holding the source's bytes. Its
        // FIFO holds the output write, and the backup then finds no snapshot.
        let snapshot = fs::read_dir(&spool)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .find(|path| fs::read(path).unwrap() == ORIGINAL)
            .unwrap();
        fs::remove_file(&snapshot).unwrap();
        assert!(
            Command::new("mkfifo")
                .arg(&snapshot)
                .status()
                .unwrap()
                .success()
        );
        changed.send(snapshot).unwrap();
        json!({"type":"noul","noul":0.99})
    })
    .unwrap();
    let mut child = support::command()
        .args(["decide", "Ready?", "--from", "csv"])
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
        .env("TMPDIR", &scratch)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let snapshot = wait_for(&mut child, || received.try_recv().ok());
    let (opened, writer) = mpsc::channel();
    let fifo = snapshot.clone();
    std::thread::spawn(move || {
        let _ = opened.send(OpenOptions::new().write(true).open(fifo));
    });
    let mut feed = wait_for(&mut child, || writer.try_recv().ok()).unwrap();
    feed.write_all(ORIGINAL).unwrap();
    fs::rename(&snapshot, directory.path().join("fifo")).unwrap();
    drop(feed);
    let result = child.wait_with_output().unwrap();
    assert_kept(&result, &root.join("input.thinkthen.csv"));
    assert_eq!(fs::read(&input).unwrap(), ORIGINAL);
    assert_eq!(fs::read_dir(&root).unwrap().count(), 2);
}

#[cfg(windows)]
#[test]
fn a_held_source_keeps_the_finished_output_and_the_source() {
    use std::fs::{File, OpenOptions};
    use std::os::windows::fs::OpenOptionsExt;
    use std::sync::{Arc, Mutex};

    const FILE_SHARE_READ: u32 = 1;
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    let input = root.join("input.csv");
    fs::write(&input, ORIGINAL).unwrap();
    make_private(&input);
    let list = access_list(&input);
    let held: Arc<Mutex<Option<File>>> = Arc::default();
    let holder = Arc::clone(&held);
    let target = input.clone();
    let server = Loopback::answering(move |_| {
        // Readers still pass the final hash, but the target cannot be replaced.
        let file = OpenOptions::new()
            .read(true)
            .share_mode(FILE_SHARE_READ)
            .open(&target)
            .unwrap();
        *holder.lock().unwrap() = Some(file);
        json!({"type":"noul","noul":0.99})
    })
    .unwrap();
    let result = support::command()
        .args(["decide", "Ready?", "--from", "csv"])
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
        .output()
        .unwrap();
    let kept = root.join("input.thinkthen.csv");
    assert_kept(&result, &kept);
    assert_eq!(fs::read(&input).unwrap(), ORIGINAL);
    assert_eq!(fs::read(root.join("input.backup.csv")).unwrap(), ORIGINAL);
    assert_eq!(access_list(&kept), list);
}

#[cfg(windows)]
fn grant_only(path: &Path, rights: &str) {
    let grant = format!("{}:{rights}", std::env::var("USERNAME").unwrap());
    let status = std::process::Command::new("icacls")
        .arg(path)
        .args(["/inheritance:r", "/grant:r", &grant])
        .status()
        .unwrap();
    assert!(status.success());
}

/// A runner's temp folder can hand down nothing, which leaves a new file with
/// explicit entries from the token. This folder hands down one entry, so a new
/// file's list is inherited and `grant_only` can replace all of it.
#[cfg(windows)]
fn hand_down_access(folder: &Path) {
    grant_only(folder, "(OI)(CI)F");
}

#[cfg(windows)]
fn make_private(path: &Path) {
    grant_only(path, "F");
}

/// The list as `icacls` prints it, without the path it names or the padding
/// that path's length sets.
#[cfg(windows)]
fn access_list(path: &Path) -> String {
    let output = std::process::Command::new("icacls")
        .arg(path)
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    String::from_utf8_lossy(&output.stdout)
        .replace(path.to_str().unwrap(), "")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(windows)]
fn run_in_place(input: &Path) -> Output {
    support::command()
        .args(["decide", "Ready?", "--backend", "stub", "--from", "csv"])
        .arg(input)
        .args(["--column", "Body", "--into", "ready", "--in-place"])
        .output()
        .unwrap()
}

#[cfg(windows)]
#[test]
fn in_place_keeps_the_access_list_on_the_result_and_the_backup() {
    for private in [true, false] {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path();
        hand_down_access(root);
        let input = root.join("input.csv");
        fs::write(&input, ORIGINAL).unwrap();
        if private {
            make_private(&input);
        }
        // `(I)` marks an entry that follows the folder, so an ordinary file
        // that stopped following it would read differently.
        let list = access_list(&input);
        assert_eq!(list.contains("(I)"), !private, "{list}");
        let result = run_in_place(&input);
        assert!(result.status.success(), "{result:?}");
        assert_ne!(fs::read(&input).unwrap(), ORIGINAL);
        assert_eq!(access_list(&input), list);
        assert_eq!(access_list(&root.join("input.backup.csv")), list);
    }
}

#[cfg(windows)]
#[test]
fn a_private_file_moved_in_from_another_folder_stays_private() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    let private = root.join("private");
    fs::create_dir(&private).unwrap();
    grant_only(&private, "(OI)(CI)F");
    let moved = private.join("input.csv");
    fs::write(&moved, ORIGINAL).unwrap();
    let input = root.join("input.csv");
    fs::rename(&moved, &input).unwrap();
    // Its entries still say inherited, so this folder would replace them. The
    // mark may clear; the entries must not change.
    let list = access_list(&input);
    assert!(list.contains("(I)"), "{list}");
    let result = run_in_place(&input);
    assert!(result.status.success(), "{result:?}");
    let list = list.replace("(I)", "");
    assert_eq!(access_list(&input).replace("(I)", ""), list);
    let backup = root.join("input.backup.csv");
    assert_eq!(access_list(&backup).replace("(I)", ""), list);
}

#[cfg(windows)]
#[test]
fn a_list_that_would_block_the_swap_stops_before_any_call() {
    let directory = tempfile::tempdir().unwrap();
    hand_down_access(directory.path());
    let input = directory.path().join("input.csv");
    fs::write(&input, ORIGINAL).unwrap();
    grant_only(&input, "R");
    let server = Loopback::noul(0.99).unwrap();
    let result = support::command()
        .args(["decide", "Ready?", "--from", "csv"])
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
        .output()
        .unwrap();
    assert_eq!(result.status.code(), Some(5), "{result:?}");
    assert_eq!(support::reported(&result), "local.io");
    assert_eq!(server.count(), 0);
    assert_eq!(fs::read(&input).unwrap(), ORIGINAL);
}

#[cfg(windows)]
#[test]
fn in_place_keeps_a_read_only_source_read_only() {
    let directory = tempfile::tempdir().unwrap();
    let input = directory.path().join("input.csv");
    fs::write(&input, ORIGINAL).unwrap();
    let mut permissions = fs::metadata(&input).unwrap().permissions();
    permissions.set_readonly(true);
    fs::set_permissions(&input, permissions).unwrap();
    let result = run_in_place(&input);
    assert!(result.status.success(), "{result:?}");
    assert_ne!(fs::read(&input).unwrap(), ORIGINAL);
    assert!(fs::metadata(&input).unwrap().permissions().readonly());
}

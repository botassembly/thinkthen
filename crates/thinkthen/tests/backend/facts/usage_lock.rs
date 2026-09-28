//! A held advisory lock cannot leave a completed command waiting at exit.

use std::fs;
use std::os::unix::fs::{OpenOptionsExt as _, PermissionsExt as _};
use std::path::Path;
use std::thread;
use std::time::{Duration, Instant};

use serde_json::Value;

use crate::harness::{Canned, Listener, finish, start};

#[test]
fn a_foreign_usage_lock_loses_only_advisory_counts_before_exit() {
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join("facts-held-usage-lock");
    let _absent = fs::remove_dir_all(&root);
    let usage = root.join("thinkthen-usage");
    fs::create_dir_all(&usage).expect("usage folder");
    fs::set_permissions(&usage, fs::Permissions::from_mode(0o700)).expect("private folder");
    let lock = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(usage.join(".lock"))
        .expect("private lock");
    lock.lock().expect("foreign holder");
    let listener = Listener::serving(vec![Canned::ok(
        r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.92}},"usage":{"input_tokens":8,"output_tokens":2}}"#,
    )])
    .expect("listener");
    let mut child = start(
        &[
            "decide",
            "Is this valid?",
            "--facts",
            "--no-cache",
            "--url",
            listener.base(),
        ],
        &[
            ("THINKTHEN_API_KEY", "secret-key"),
            ("XDG_CACHE_HOME", root.to_str().expect("UTF-8 root")),
        ],
        b"one record",
    )
    .expect("compiled command");
    let until = Instant::now() + Duration::from_secs(3);
    let exited = loop {
        if child.try_wait().expect("child state").is_some() {
            break true;
        }
        if Instant::now() >= until {
            break false;
        }
        thread::sleep(Duration::from_millis(10));
    };
    if !exited {
        drop(lock);
        let _finished = finish(child, "held usage lock");
        panic!("the command waited for the still-held advisory usage lock");
    }
    let output = finish(child, "held usage lock").expect("command output");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(output.stdout, b"true\n");
    let stderr = String::from_utf8_lossy(&output.stderr);
    let lines: Vec<_> = stderr.lines().collect();
    assert_eq!(lines.len(), 2, "{stderr}");
    assert_eq!(
        lines[0],
        "thinkthen: usage counters could not be updated; check the usage folder permissions and free space"
    );
    let facts: Value = serde_json::from_str(lines[1]).expect("final run facts");
    assert_eq!(facts["schema"], "thinkthen.run/1");
    assert_eq!(facts["records"], 1);
    assert_eq!(facts["requests_sent"], 1);
    assert_eq!(listener.requests().len(), 1);
    let mut names = fs::read_dir(&usage)
        .expect("usage folder")
        .map(|entry| {
            entry
                .expect("entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect::<Vec<_>>();
    names.sort();
    assert_eq!(names, [".lock"], "no month count or partial write");
    drop(lock);
}

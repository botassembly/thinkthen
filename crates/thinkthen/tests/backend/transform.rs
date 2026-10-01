//! The read-only transform catalog prints embedded bytes and touches nothing.
#![cfg(feature = "cli")]
#![allow(
    clippy::expect_used,
    clippy::panic,
    reason = "a failed fixture stops the proof"
)]

use crate::child::ChildEnvironment as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
// The guarded catalog case plants executable traps and FIFOs, which need Unix.
#[cfg(unix)]
use std::{
    io::{ErrorKind, Read as _},
    net::TcpListener,
    os::unix::fs::PermissionsExt as _,
    process::Stdio,
    time::{Duration, Instant},
};

#[cfg(unix)]
use crate::child;
use crate::run;

const LIST: &str =
    "band\ncalibration\ncompare\ncost\ncounts\nmonitor\nscore\nsweep\ntriage\ntrials\n";
const UNKNOWN: &str =
    "thinkthen: transform: unknown name; run `thinkthen transform list` to see the catalog\n";

/// Each public name. `show` must print its packaged and repository source.
const MEMBERS: [&str; 10] = [
    "band",
    "calibration",
    "compare",
    "cost",
    "counts",
    "monitor",
    "score",
    "sweep",
    "triage",
    "trials",
];

/// A fresh empty folder under the system temporary folder.
fn folder(label: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!("thinkthen-0083-{label}-{}", std::process::id()));
    let _absent = fs::remove_dir_all(&path);
    fs::create_dir(&path).expect("a fresh folder");
    path
}

fn catalog(arguments: &[&str], cwd: &Path) -> Output {
    run::output(
        Command::new(env!("CARGO_BIN_EXE_thinkthen"))
            .args(arguments)
            .clear_environment()
            .current_dir(cwd),
    )
    .expect("the compiled binary runs")
}

#[test]
fn list_prints_the_closed_names_in_byte_order_everywhere() {
    let empty = folder("list");
    for cwd in [empty.as_path(), Path::new("/")] {
        for locale in ["C", "POSIX", "tr_TR.UTF-8", "de_DE.UTF-8"] {
            for _repeat in 0..2 {
                let output = run::output(
                    Command::new(env!("CARGO_BIN_EXE_thinkthen"))
                        .args(["transform", "list"])
                        .clear_environment()
                        .env("LC_ALL", locale)
                        .env("LANG", locale)
                        .current_dir(cwd),
                )
                .expect("the compiled binary runs");
                assert_eq!(String::from_utf8_lossy(&output.stdout), LIST, "{locale}");
                assert!(output.stderr.is_empty(), "{locale}");
                assert_eq!(output.status.code(), Some(0), "{locale}");
            }
        }
    }
    fs::remove_dir(&empty).expect("the folder stays empty");
}

#[test]
fn show_prints_each_member_byte_for_byte() {
    let empty = folder("show");
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    for name in MEMBERS {
        let output = catalog(&["transform", "show", name], &empty);
        assert_eq!(output.status.code(), Some(0), "{name}");
        assert!(output.stderr.is_empty(), "{name}");
        let packaged = fs::read(root.join(format!("transforms/{name}.jq"))).expect("packaged");
        let source = fs::read(root.join(format!("../../transforms/{name}/{name}.jq")));
        assert_eq!(output.stdout, packaged, "{name}");
        assert_eq!(output.stdout, source.expect("repository source"), "{name}");
        assert!(output.stdout.ends_with(b"\n") && !output.stdout.ends_with(b"\n\n"));
        assert!(!output.stdout.contains(&b'\r') && !output.stdout.starts_with(b"\xef\xbb\xbf"));
    }
    fs::remove_dir(&empty).expect("the folder stays empty");
}

#[test]
fn an_unknown_name_is_refused_without_echoing_it() {
    let empty = folder("unknown");
    for name in [
        "Band",
        "BAND",
        "band.jq",
        "./band",
        "../transforms/band",
        "/band",
        "transforms/band",
        "ban",
        "bands",
        "",
        " band",
        "band\n",
        "bänd",
        "トランス",
    ] {
        let output = catalog(&["transform", "show", name], &empty);
        assert!(output.stdout.is_empty(), "{name:?}");
        assert_eq!(String::from_utf8_lossy(&output.stderr), UNKNOWN, "{name:?}");
        assert_eq!(output.status.code(), Some(2), "{name:?}");
    }
    for arguments in [
        &["transform", "show"][..],
        &["transform", "show", "band", "cost"],
        &["transform", "list", "band"],
        &["transform", "show", "--input", "band"],
    ] {
        let output = catalog(arguments, &empty);
        assert!(output.stdout.is_empty(), "{arguments:?}");
        assert!(!output.stderr.is_empty(), "{arguments:?}");
        assert_eq!(output.status.code(), Some(2), "{arguments:?}");
    }
    fs::remove_dir(&empty).expect("the folder stays empty");
}

/// Every path under a folder with its size, so a change anywhere shows.
#[cfg(unix)]
fn tree(root: &Path) -> Vec<(PathBuf, u64)> {
    let mut found = Vec::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(path) = pending.pop() {
        let metadata = fs::symlink_metadata(&path).expect("metadata");
        found.push((path.clone(), metadata.len()));
        if metadata.is_dir() {
            pending.extend(
                fs::read_dir(&path)
                    .expect("a folder")
                    .map(|entry| entry.expect("an entry").path()),
            );
        }
    }
    found.sort();
    found
}

/// Run one catalog command with every recognized variable set to a canary,
/// only trap programs on `PATH`, standard input held open, and named pipes as
/// decoy files. Opening a decoy or reading input would hang the command.
#[cfg(unix)]
fn guarded(arguments: &[&str], root: &Path, url: &str) -> Output {
    let locked = root.join("locked");
    let mut command = Command::new(env!("CARGO_BIN_EXE_thinkthen"));
    command
        .args(arguments)
        .clear_environment()
        .current_dir(root.join("decoys"));
    for variable in [
        "HOME",
        "XDG_CONFIG_HOME",
        "XDG_CACHE_HOME",
        "THINKTHEN_CACHE",
        "THINKTHEN_TEST_IDENTITY_FOLDER",
    ] {
        command.env(variable, locked.join(variable));
    }
    for variable in [
        "THINKTHEN_SIGINT_CHILD",
        "THINKTHEN_SIGINT_PREFIX",
        "THINKTHEN_TEST_IDENTITY_PAUSE",
        "THINKTHEN_TEST_IDENTITY_READY",
        "THINKTHEN_TEST_IDENTITY_RESUME",
        "THINKTHEN_TEST_RETRY_WAIT_MS",
        "THINKTHEN_TEST_SIGINT_ACK",
    ] {
        command.env(variable, root.join("traps/canary-0083"));
    }
    command
        .env("THINKTHEN_API_KEY", "canary-0083")
        .env("THINKTHEN_BASE_URL", url)
        .env("RUST_BACKTRACE", "full")
        .env("PATH", root.join("traps"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = command.spawn().expect("the compiled binary runs");
    let _held_open = child.stdin.take();
    let deadline = Instant::now() + Duration::from_secs(20);
    while child.try_wait().expect("the child").is_none() {
        if Instant::now() > deadline {
            let _killed = child.kill();
            panic!("{arguments:?} blocked on input or a decoy");
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    let (mut stdout, mut stderr) = (Vec::new(), Vec::new());
    child
        .stdout
        .take()
        .expect("stdout")
        .read_to_end(&mut stdout)
        .expect("stdout");
    child
        .stderr
        .take()
        .expect("stderr")
        .read_to_end(&mut stderr)
        .expect("stderr");
    Output {
        status: child.wait().expect("the child"),
        stdout,
        stderr,
    }
}

#[cfg(unix)]
#[test]
fn the_catalog_reads_no_setting_input_or_file_and_sends_and_runs_nothing() {
    let root = folder("guarded");
    for part in ["locked", "decoys", "traps"] {
        fs::create_dir(root.join(part)).expect("a part");
    }
    let marker = root.join("ran");
    for program in ["jq", "sh", "bash", "python3", "cargo", "env", "cat"] {
        let path = root.join("traps").join(program);
        fs::write(&path, format!("#!/bin/sh\n: > '{}'\n", marker.display())).expect("a trap");
        fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).expect("executable");
    }
    for name in MEMBERS {
        for decoy in [name.to_owned(), format!("{name}.jq")] {
            let made = child::command("mkfifo", &[])
                .arg(root.join("decoys").join(decoy))
                .status();
            assert!(made.expect("mkfifo runs").success());
        }
    }
    let listener = TcpListener::bind("127.0.0.1:0").expect("a loopback listener");
    listener.set_nonblocking(true).expect("nonblocking");
    let url = format!("http://{}/v1", listener.local_addr().expect("an address"));
    let before = tree(&root);
    fs::set_permissions(root.join("locked"), fs::Permissions::from_mode(0o000)).expect("locked");

    let mut outputs = vec![(
        guarded(&["transform", "list"], &root, &url),
        LIST.as_bytes().to_vec(),
        0,
    )];
    for name in MEMBERS {
        let packaged =
            fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("transforms/{name}.jq")));
        outputs.push((
            guarded(&["transform", "show", name], &root, &url),
            packaged.expect("packaged"),
            0,
        ));
    }
    let refused = guarded(&["transform", "show", "canary-0083"], &root, &url);
    fs::set_permissions(root.join("locked"), fs::Permissions::from_mode(0o755)).expect("unlocked");

    assert_eq!(String::from_utf8_lossy(&refused.stderr), UNKNOWN);
    assert_eq!(refused.status.code(), Some(2));
    outputs.push((refused, Vec::new(), 2));
    for (output, expected, code) in &outputs {
        assert_eq!(&output.stdout, expected);
        assert_eq!(output.status.code(), Some(*code));
        for channel in [&output.stdout, &output.stderr] {
            assert!(!String::from_utf8_lossy(channel).contains("canary-0083"));
        }
    }
    assert!(
        outputs
            .iter()
            .take(11)
            .all(|(output, _, _)| output.stderr.is_empty())
    );
    assert!(!marker.exists(), "a catalog command started a program");
    assert_eq!(
        listener.accept().map(|_| ()).map_err(|error| error.kind()),
        Err(ErrorKind::WouldBlock)
    );
    assert_eq!(
        tree(&root),
        before,
        "a catalog command created or changed a file"
    );
    fs::remove_dir_all(&root).expect("cleanup");
}

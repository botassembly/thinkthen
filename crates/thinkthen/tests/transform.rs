//! The read-only transform catalog prints embedded bytes and touches nothing.
#![cfg(feature = "cli")]
#![allow(
    clippy::expect_used,
    clippy::panic,
    reason = "a failed fixture stops the proof"
)]

use std::fs;
use std::io::{ErrorKind, Read as _};
use std::net::TcpListener;
use std::os::unix::fs::PermissionsExt as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::time::{Duration, Instant};

use sha2::{Digest as _, Sha256};

#[path = "../src/test_deadline/child.rs"]
mod child;
#[path = "../src/test_deadline/run.rs"]
mod run;
#[path = "../src/test_deadline/wait.rs"]
mod wait;

const LIST: &str =
    "band\ncalibration\ncompare\ncost\ncounts\nmonitor\nscore\nsweep\ntriage\ntrials\n";
const UNKNOWN: &str =
    "thinkthen: transform: unknown name; run `thinkthen transform list` to see the catalog\n";

/// Each public name, its byte count, and its SHA-256 at ticket 0083's base.
const MEMBERS: [(&str, usize, &str); 10] = [
    (
        "band",
        3_425,
        "bd4934353f3d07da55f2642b5090a96d77788a778095bdf37607c8c07729b176",
    ),
    (
        "calibration",
        2_893,
        "23f91a4231f910c567939e6bd0dbeded6bcb3e9be785991d58091365ca5747f5",
    ),
    (
        "compare",
        12_218,
        "1909601f3d5325111e6f2293521211c692bf0e2a4015369d89b84830acde110f",
    ),
    (
        "cost",
        2_295,
        "4b00fc0c180bd0464a0e84d0a221cca5204e9a456da41383f43f1f3cda7c209e",
    ),
    (
        "counts",
        1_735,
        "de58fa6c5646432e3947d404a1355ebd0724f4829e234f76472b4fe8e6e0132f",
    ),
    (
        "monitor",
        3_453,
        "18e9a22e298a9819c0817672fa8c8a62e99800176e5f66357e5e2a0024906b27",
    ),
    (
        "score",
        3_495,
        "6df93483f3d017c3d6c91e8920c66ba636099fa767019f46ba792eee34a5f8d5",
    ),
    (
        "sweep",
        26_692,
        "8100a565923890f16bf338dda397360c7ae55a300648ac972116d4c2212f4871",
    ),
    (
        "triage",
        1_788,
        "6ec36edd30443f39a2de5f8e24e09e8aba9cc33446128381f4f5830856744736",
    ),
    (
        "trials",
        8_334,
        "aa361137f146a33bf1547e6f0276c5a312a20d9469178f3c28474932d629e2dd",
    ),
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
            .env_clear()
            .current_dir(cwd),
    )
    .expect("the compiled binary runs")
}

fn sha256(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
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
                        .env_clear()
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
    for (name, bytes, digest) in MEMBERS {
        let output = catalog(&["transform", "show", name], &empty);
        assert_eq!(output.status.code(), Some(0), "{name}");
        assert!(output.stderr.is_empty(), "{name}");
        assert_eq!(output.stdout.len(), bytes, "{name}");
        assert_eq!(sha256(&output.stdout), digest, "{name}");
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

#[test]
fn transform_help_pins_its_three_introductions() {
    const ROOT: &str = "List or print the built-in jq transforms without running them";
    const LIST_HELP: &str = "List the names of the built-in jq transforms";
    const SHOW_HELP: &str = "Print one built-in jq transform exactly as shipped";
    let empty = folder("help");
    let root = catalog(&["--help"], &empty);
    let root = String::from_utf8_lossy(&root.stdout);
    assert!(root.contains(&format!("\n  cache      Inspect and maintain answer-cache folders without sending a request\n  transform  {ROOT}\n  audit      ")), "{root}");
    for flag in ["-h", "--help"] {
        for (arguments, sentence) in [
            (vec!["transform", flag], ROOT),
            (vec!["transform", "list", flag], LIST_HELP),
            (vec!["transform", "show", flag], SHOW_HELP),
        ] {
            let output = catalog(&arguments, &empty);
            assert_eq!(output.status.code(), Some(0), "{arguments:?}");
            let help = String::from_utf8_lossy(&output.stdout);
            assert!(
                help.starts_with(&format!("{sentence}\n\n")),
                "{arguments:?}: {help}"
            );
        }
    }
    fs::remove_dir(&empty).expect("the folder stays empty");
}

/// Every path under a folder with its size, so a change anywhere shows.
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
fn guarded(arguments: &[&str], root: &Path, url: &str) -> Output {
    let locked = root.join("locked");
    let mut command = Command::new(env!("CARGO_BIN_EXE_thinkthen"));
    command
        .args(arguments)
        .env_clear()
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
    for (name, _, _) in MEMBERS {
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
    for (name, _, _) in MEMBERS {
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

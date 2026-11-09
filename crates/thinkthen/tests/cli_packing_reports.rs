#![cfg(test)]
//! Optional packing reports remain separate across public callers in different processes.
#![cfg(feature = "command")]

#[path = "../../loopback/src/lib.rs"]
pub mod loopback;
mod support;

use std::collections::HashSet;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use std::process::{Child, ChildStdout, Stdio};

use loopback::{KEY, Loopback};
use serde_json::Value;
use thinkthen::{Decide, Key, Outcome, Record, Settings};

const RUN: &str = "0123456789abcdef0123456789abcdef";

struct OwnedChild(Child, Option<BufReader<ChildStdout>>);

impl Drop for OwnedChild {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn report_file(path: &Path) -> File {
    OpenOptions::new()
        .read(true)
        .append(true)
        .open(path)
        .unwrap()
}

fn worker(path: &Path, server: &Loopback, role: &str, calls: usize) -> OwnedChild {
    let mut child = support::isolated(std::env::current_exe().unwrap());
    child
        .args(["--exact", "packing_worker", "--ignored", "--nocapture"])
        .env("PACKING_TEST_ROLE", role)
        .env("PACKING_TEST_CALLS", calls.to_string())
        .env("PACKING_TEST_URL", server.base())
        .env("THINKTHEN_INPUT_REPORT", path)
        .env("THINKTHEN_INPUT_REPORT_RUN", RUN)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped());
    let mut child = OwnedChild(child.spawn().unwrap(), None);
    let mut output = BufReader::new(child.0.stdout.take().unwrap());
    loop {
        let mut line = String::new();
        assert_ne!(
            output.read_line(&mut line).unwrap(),
            0,
            "worker exited before ready"
        );
        if line.trim_end().ends_with("PACKING_READY") {
            break;
        }
    }
    child.1 = Some(output);
    child
}

fn start(child: &mut OwnedChild) {
    child.0.stdin.as_mut().unwrap().write_all(b"G").unwrap();
}

fn reports(path: &Path, expected: &HashSet<(u32, usize)>) {
    let text = fs::read_to_string(path).unwrap();
    let mut actual = HashSet::new();
    for line in text.lines() {
        let row: Value = serde_json::from_str(line).unwrap();
        assert_eq!(row["run"], RUN);
        assert_eq!(row["complete"], true);
        assert_eq!(row["records"], 1);
        assert_eq!(row["packed_requests"], 1);
        assert_eq!(row["idle_flushes"], 0);
        assert_eq!(row["idle_added_requests"], 0);
        assert!(actual.insert((
            row["pid"].as_u64().unwrap() as u32,
            row["call"].as_u64().unwrap() as usize
        )));
    }
    assert_eq!(&actual, expected);
}

#[test]
#[ignore = "Separate-process helper, invoked only by the owning report tests"]
fn packing_worker() {
    let path = std::env::var_os("THINKTHEN_INPUT_REPORT").unwrap();
    let holder = std::env::var("PACKING_TEST_ROLE").unwrap() == "holder";
    let file = report_file(Path::new(&path));
    if holder {
        file.lock().unwrap();
    }
    std::io::stdout().write_all(b"PACKING_READY\n").unwrap();
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_exact(&mut [0]).unwrap();
    assert!(!holder, "holder must be terminated while owning its lock");
    let engine = Settings::default()
        .url(std::env::var("PACKING_TEST_URL").unwrap())
        .api_key(Key::new(KEY).unwrap())
        .engine(|_| None)
        .unwrap();
    let calls: usize = std::env::var("PACKING_TEST_CALLS")
        .unwrap()
        .parse()
        .unwrap();
    for _ in 0..calls {
        let rows: Vec<_> = engine
            .rows(&Decide::new("Is it?").unwrap(), [Record::text("record")])
            .unwrap()
            .map(Result::unwrap)
            .collect();
        assert_eq!(rows.len(), 1);
        assert!(matches!(rows[0].outcome(), Outcome::Answered { .. }));
    }
}

#[test]
fn separate_processes_keep_every_report_identity_and_result() {
    let file = tempfile::NamedTempFile::new().unwrap();
    let server = Loopback::noul(0.9).unwrap();
    let mut workers: Vec<_> = (0..4)
        .map(|_| worker(file.path(), &server, "writer", 16))
        .collect();
    let expected: HashSet<_> = workers
        .iter()
        .flat_map(|child| (0..16).map(move |call| (child.0.id(), call)))
        .collect();
    for child in &mut workers {
        start(child);
    }
    for child in &mut workers {
        assert!(child.0.wait().unwrap().success());
    }
    assert_eq!(server.count(), 64);
    reports(file.path(), &expected);
}

#[test]
fn terminating_a_lock_holder_releases_the_report_file() {
    let file = tempfile::NamedTempFile::new().unwrap();
    let server = Loopback::noul(0.9).unwrap();
    let mut holder = worker(file.path(), &server, "holder", 0);
    let mut survivor = worker(file.path(), &server, "writer", 1);
    start(&mut survivor);
    holder.0.kill().unwrap();
    holder.0.wait().unwrap();
    assert!(survivor.0.wait().unwrap().success());
    reports(file.path(), &HashSet::from([(survivor.0.id(), 0)]));
}

#[cfg(target_os = "linux")]
#[test]
fn a_public_command_waits_for_the_other_process_report_lock() {
    use std::time::{Duration, Instant};

    let file = tempfile::NamedTempFile::new().unwrap();
    let locked = report_file(file.path());
    locked.lock().unwrap();
    let server = Loopback::noul(0.9).unwrap();
    let mut command = support::command();
    command
        .args(["decide", "Is it?", "--url", server.base()])
        .env("THINKTHEN_API_KEY", KEY)
        .env("THINKTHEN_INPUT_REPORT", file.path())
        .env("THINKTHEN_INPUT_REPORT_RUN", RUN)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped());
    let mut child = OwnedChild(command.spawn().unwrap(), None);
    child.0.stdin.take().unwrap().write_all(b"record").unwrap();
    let pid = child.0.id().to_string();
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        let locks = fs::read_to_string("/proc/locks").unwrap();
        if locks.lines().any(|line| {
            let fields: Vec<_> = line.split_whitespace().collect();
            fields.get(1) == Some(&"->")
                && fields.get(2) == Some(&"FLOCK")
                && fields.get(5) == Some(&pid.as_str())
        }) {
            break;
        }
        assert!(
            child.0.try_wait().unwrap().is_none(),
            "writer ignored the other process's report lock"
        );
        assert!(
            Instant::now() < deadline,
            "report writer never reached a kernel lock wait"
        );
        std::thread::yield_now();
    }
    assert_eq!(fs::metadata(file.path()).unwrap().len(), 0);
    drop(locked);
    assert!(child.0.wait().unwrap().success());
    let mut output = String::new();
    child
        .0
        .stdout
        .take()
        .unwrap()
        .read_to_string(&mut output)
        .unwrap();
    assert_eq!(output, "true\n");
    reports(file.path(), &HashSet::from([(child.0.id(), 0)]));
}

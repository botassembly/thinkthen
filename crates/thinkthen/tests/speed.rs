//! The speed test's gate (ticket 0145): `probes/speed/measure.py gate` against the loopback backend.
#![cfg(feature = "cli")]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Output;

use conformance_backend::Backend;

#[path = "../src/test_deadline/child.rs"]
mod child;
#[path = "../src/test_deadline/run.rs"]
mod run;
#[path = "../src/test_deadline/wait.rs"]
mod wait;

fn repository() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Run the gate over one list file against the generic arm.
fn gate(backend: &Backend, list: &Path) -> Output {
    run::output(
        child::command("python3", &[])
            .arg(repository().join("probes/speed/measure.py"))
            .arg("gate")
            .arg(list)
            .env("THINKTHEN_BIN", env!("CARGO_BIN_EXE_thinkthen"))
            .env(
                "THINKTHEN_BASE_URL",
                format!("{}/generic/v1", backend.origin()),
            ),
    )
    .expect("the gate runs")
}

/// The committed table with `from` replaced by `to` in one function's row, written under the target folder.
fn planted(function: &str, from: &str, to: &str) -> PathBuf {
    let table =
        fs::read_to_string(repository().join("probes/speed/functions.jsonl")).expect("table");
    let key = format!("{{\"function\":\"{function}\",");
    let mut out = String::new();
    for line in table.lines() {
        out += &if line.starts_with(&key) {
            line.replacen(from, to, 1)
        } else {
            line.to_owned()
        };
        out.push('\n');
    }
    assert_ne!(out, table, "the plant for {function} changed nothing");
    let path = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("speed-{function}.jsonl"));
    fs::write(&path, out).expect("planted table");
    path
}

#[test]
fn the_speed_gate_holds_and_names_each_fault() {
    let backend = Backend::start().expect("backend");
    let committed = gate(&backend, &repository().join("probes/speed/functions.jsonl"));
    let stderr = String::from_utf8_lossy(&committed.stderr);
    assert_eq!(committed.status.code(), Some(0), "{stderr}");
    assert_eq!(stderr, "");
    let reported: usize = String::from_utf8_lossy(&committed.stdout)
        .lines()
        .map(|line| {
            line.split('\t')
                .nth(2)
                .and_then(|n| n.parse::<usize>().ok())
                .expect("a count")
        })
        .sum();
    assert_eq!(
        reported,
        backend.count(),
        "status must count what the socket read"
    );

    let listed = r#","list":{"ticket":"B4","number":null}"#;
    let cases = [
        (
            planted("filter", listed, ""),
            "speed: filter sent 12 requests for 12 items where 1 fits. Batch it, or list it with its ticket.\n",
        ),
        (
            planted(
                "find",
                r#""items":1,"floor":1}"#,
                r#""items":12,"floor":1,"list":{"ticket":"B99","number":null}}"#,
            ),
            "speed: find is listed for ticket B99 but sent 1 request for 12 items. Remove its entry.\n",
        ),
        (
            planted("decide", r#""number":null"#, r#""number":"0141""#),
            "speed: decide is listed for ticket 0141, which has landed. Remove its entry.\n",
        ),
    ];
    for (list, sentence) in cases {
        let output = gate(&backend, &list);
        assert_eq!(output.status.code(), Some(1), "{}", list.display());
        assert_eq!(String::from_utf8_lossy(&output.stderr), sentence);
    }
}

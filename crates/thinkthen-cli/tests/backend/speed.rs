//! The speed test's gate (ticket 0145): `probes/speed/measure.py gate` against the loopback backend.

use crate::harness::stderr_is;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Output;

use conformance_backend::Backend;

use crate::child;
use crate::run;

fn repository() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Run the gate over one list file against the generic arm.
fn gate(backend: &Backend, list: &Path) -> io::Result<Output> {
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
}

/// The committed table with `from` replaced by `to` in one function's row, written under the target folder.
fn planted(function: &str, from: &str, to: &str) -> io::Result<PathBuf> {
    let table = fs::read_to_string(repository().join("probes/speed/functions.jsonl"))?;
    let key = format!("{{\"function\":\"{function}\",");
    let mut out = String::new();
    let line = table
        .lines()
        .find(|line| line.starts_with(&key))
        .expect("function row");
    let mut out = line.replacen(from, to, 1);
    assert_ne!(out, line, "the plant for {function} changed nothing");
    out.push('\n');
    let path = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("speed-{function}.jsonl"));
    fs::write(&path, out)?;
    Ok(path)
}

#[test]
fn the_speed_gate_holds_and_names_each_fault() {
    let backend = Backend::start().expect("backend");
    let committed =
        gate(&backend, &repository().join("probes/speed/functions.jsonl")).expect("the gate runs");
    let reported: usize = crate::harness::printed(&committed)
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
    let stderr = crate::harness::said(&committed);
    assert_eq!(committed.status.code(), Some(0), "{stderr}");
    assert_eq!(stderr, "");

    let cases = [
        (
            planted("filter", r#""0.95""#, r#""0.95","--batch","1""#),
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
            planted("recognize", r#""number":null"#, r#""number":"0141""#),
            "speed: recognize is listed for ticket 0141, which has landed. Remove its entry.\n",
        ),
    ];
    for (list, sentence) in cases {
        let before = backend.count();
        let list = list.expect("planted table");
        let output = gate(&backend, &list).expect("the gate runs");
        let reported: usize = crate::harness::printed(&output)
            .lines()
            .map(|line| line.split('\t').nth(2).unwrap().parse::<usize>().unwrap())
            .sum();
        assert_eq!(
            reported,
            backend.count() - before,
            "status must count the planted row's socket reads"
        );
        fs::remove_file(list).expect("remove planted table");
        assert_eq!(output.status.code(), Some(1), "{sentence}");
        stderr_is(&output, sentence);
    }
}

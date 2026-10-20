//! The core session packets are the shared, current library oracle.
#![allow(clippy::expect_used, reason = "fixture failures stop the oracle comparison")]
use std::io::{self, Write};
use std::path::Path;

#[path = "../src/test_deadline/child.rs"]
mod child;
#[path = "packet_oracle/dump.rs"]
mod dump;

#[test]
#[ignore = "repository packet currency gate requires shared conformance fixtures"]
fn shared_packet_oracle_is_current() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let worker = std::env::current_exe().expect("core worker binary");
    let backend = worker.parent().and_then(Path::parent).expect("target debug").join("conformance-backend");
    let mut compare = child::command("python3", &[]);
    compare.arg(root.join("conformance/children/packet_dump.py"))
        .arg("--check").arg("--worker").arg(worker)
        .env("THINKTHEN_TEST_BACKEND", backend);
    if std::env::var_os("THINKTHEN_UPDATE_PACKETS").is_some() {
        compare.arg("--update");
    }
    assert!(compare.status().expect("shared packet comparison").success(),
        "regenerate with THINKTHEN_UPDATE_PACKETS=1 cargo test -p thinkthen --test packet_oracle -- --ignored --exact shared_packet_oracle_is_current");
}

#[test]
#[ignore = "repository packet dumper subprocess; ordinary Rust users need no fixture tooling"]
fn packet_dumper_worker() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("fixture input");
    let value = dump::run(&input).expect("native packet dump");
    writeln!(io::stdout().lock(), "packet_dump: {value}").expect("fixture output");
}

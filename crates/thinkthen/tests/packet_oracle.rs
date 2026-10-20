//! The core session packets are the shared, current library oracle.
#![allow(clippy::expect_used, clippy::unwrap_used, reason = "fixture failures stop the oracle comparison")]
use serde_json::{Value, json};
use std::io::Write;
use std::path::Path;
use std::process::Stdio;

#[path = "../src/test_deadline/child.rs"]
mod child;
#[path = "../../../libraries/rust/consumer/src/dump.rs"]
mod dump;

#[test]
fn shared_packet_oracle_is_current() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let script = root.join("conformance/children/packet_dump.py");
    let home = std::env::temp_dir().join(format!("thinkthen-packet-oracle-{}", std::process::id()));
    std::fs::create_dir(&home).expect("owned oracle home");
    use child::ChildEnvironment as _;
    let output = child::command("python3", &[])
        .arg(&script).arg("--requests")
        .isolated_home(&home)
        .output().expect("shared case requests");
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let mut cases: Value = serde_json::from_slice(&output.stdout).expect("case requests JSON");
    let backend = conformance_backend::Backend::start().expect("loopback backend");
    for case in cases.get_mut("cases").and_then(Value::as_array_mut).expect("cases") {
        let arm = case.get("arm").and_then(Value::as_str).expect("case arm");
        if let Some(files) = case.get("files").and_then(Value::as_object) {
            for (name, text) in files {
                std::fs::write(home.join(name), text.as_str().expect("file text")).expect("fixture file");
            }
        }
        let request: Value = serde_json::from_str(&case.get("request").expect("request").to_string().replace("$FOLDER", &home.to_string_lossy())).expect("resolved request");
        let input = json!({"request": request, "base_url":format!("{}/{arm}", backend.origin()), "cancel": case.get("cancel")});
        case.as_object_mut().expect("case object").insert("dump".into(), dump::run(&input).expect("native packets"));
    }
    let mut compare = child::command("python3", &[]);
    compare.arg(script).arg("--check");
    if std::env::var_os("THINKTHEN_UPDATE_PACKETS").is_some() {
        compare.arg("--update");
    }
    let mut process = compare.isolated_home(&home)
        .stdin(Stdio::piped()).spawn().expect("packet comparison");
    process.stdin.take().expect("comparison input").write_all(cases.to_string().as_bytes()).expect("write packets");
    std::fs::remove_dir_all(&home).expect("remove owned oracle home");
    assert!(process.wait().expect("comparison finished").success(), "regenerate with THINKTHEN_UPDATE_PACKETS=1 cargo test -p thinkthen --test packet_oracle");
}

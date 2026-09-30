//! ADR 0113: a C engine built from the environment adds its sends to the
//! command's usage totals when the host frees it.

use super::*;
use cases::{Script, replies};

#[test]
fn a_freed_c_engine_adds_its_send_to_the_usage_totals() {
    let driver = compile(&crate_dir().join("tests/c/driver.c"));
    let backend = Backend::start().expect("backend");
    let base = format!("{}/generic/v1", backend.origin());
    let home = scratch("usage-home");
    let mut script = Script::default();
    script.ask("settings", &[&base, r#"{"cache":false}"#]);
    script.ask(
        "decide",
        &[&base, r#"{"decide":"asks for a refund"}"#, "Refund me."],
    );
    let output = run_with(
        &driver,
        &base,
        &script.0,
        &[("HOME", &home), ("XDG_CACHE_HOME", &home)],
    );
    let said = replies(&output.stdout).expect("replies");
    assert_eq!((said[0].0, said[1].0), (0, 0), "{said:?}");
    assert_eq!(backend.count(), 1);
    let usage = if cfg!(target_os = "macos") {
        home.join("Library/Caches/thinkthen-usage")
    } else {
        home.join("thinkthen-usage")
    };
    let month = std::fs::read_dir(&usage)
        .expect("the usage folder")
        .filter_map(Result::ok)
        .find(|entry| entry.file_name().to_string_lossy().ends_with(".json"))
        .expect("a month file");
    let counts: serde_json::Value =
        serde_json::from_slice(&std::fs::read(month.path()).expect("the month")).expect("JSON");
    assert_eq!(counts["requests_sent"], 1, "{counts}");
}

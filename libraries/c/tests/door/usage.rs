//! ADR 0113: a C engine built from the environment adds its sends and its
//! cache answers to the command's usage totals when the host frees it.

use super::*;
use cases::{Script, replies};

/// Run the driver once: build an engine with `settings`, decide one refund
/// request, and free the engine. Return both reply codes.
fn decide_once(driver: &Path, base: &str, settings: &str, home: &Path) -> (i32, i32) {
    let mut script = Script::default();
    script.ask("settings", &[base, settings]);
    script.ask(
        "decide",
        &[base, r#"{"decide":"asks for a refund"}"#, "Refund me."],
    );
    let output = run_with(
        driver,
        base,
        &script.0,
        &[("HOME", home), ("XDG_CACHE_HOME", home)],
    );
    let said = replies(&output.stdout).expect("replies");
    (said[0].0, said[1].0)
}

/// The one month file under this home's usage folder.
fn month_counts(home: &Path) -> serde_json::Value {
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
    serde_json::from_slice(&std::fs::read(month.path()).expect("the month")).expect("JSON")
}

#[test]
fn a_freed_c_engine_adds_its_send_to_the_usage_totals() {
    let driver = compile(&crate_dir().join("tests/c/driver.c"));
    let backend = Backend::start().expect("backend");
    let base = format!("{}/generic/v1", backend.origin());
    let home = scratch("usage-home");
    let said = decide_once(&driver, &base, r#"{"cache":false}"#, &home);
    assert_eq!(said, (0, 0));
    assert_eq!(backend.count(), 1);
    let counts = month_counts(&home);
    assert_eq!(counts["requests_sent"], 1, "{counts}");
}

#[test]
fn a_cached_rerun_sends_nothing_and_adds_a_cache_answer() {
    let driver = compile(&crate_dir().join("tests/c/driver.c"));
    let backend = Backend::start().expect("backend");
    let base = format!("{}/generic/v1", backend.origin());
    let home = scratch("usage-cached-home");
    let cache = scratch("usage-cached-folder");
    let settings = serde_json::json!({"cache": cache}).to_string();
    for _ in 0..2 {
        assert_eq!(decide_once(&driver, &base, &settings, &home), (0, 0));
    }
    assert_eq!(backend.count(), 1);
    let counts = month_counts(&home);
    assert_eq!(
        (&counts["requests_sent"], &counts["cache_answers"]),
        (&1.into(), &1.into()),
        "{counts}"
    );
}

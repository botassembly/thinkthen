//! The read-only process status surface.
#![cfg(feature = "cli")]

use std::fs;
use std::process::Command;

#[path = "../src/test_deadline/run.rs"]
mod run;
#[path = "../src/test_deadline/wait.rs"]
mod wait;

fn command(home: &std::path::Path) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_thinkthen"));
    command
        .env_clear()
        .env("HOME", home)
        .env("PATH", std::env::var_os("PATH").unwrap_or_default());
    command
}

#[test]
fn absent_state_has_one_exact_closed_json_shape_and_changes_nothing() {
    let home = std::env::temp_dir().join(format!("thinkthen-status-absent-{}", std::process::id()));
    let _absent = fs::remove_dir_all(&home);
    fs::create_dir(&home).expect("home");

    let output = run::output(command(&home).args(["status", "--json"])).expect("status runs");

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).expect("status JSON");
    let month = value["usage"]["month"].as_str().expect("month");
    assert_eq!(
        value,
        serde_json::json!({
            "schema": "thinkthen.status/1",
            "version": env!("CARGO_PKG_VERSION"),
            "configuration": {"path": home.join(".config/thinkthen/config.json"), "present": false},
            "backend": {"url": "https://api.typesafe.ai/v1/systemone", "url_source": "built_in", "model": "jev-1.13.0", "model_source": "built_in", "api_key_set": false},
            "cache": {"enabled": true, "enabled_source": "built_in", "path": home.join(".cache/thinkthen"), "path_source": "platform", "entries": 0, "bytes": 0, "bad_entries": 0, "temporary_entries": 0, "temporary_bytes": 0, "prune_target_bytes": 100000000, "prune_target_source": "built_in"},
            "usage": {"path": home.join(".cache/thinkthen-usage"), "month": month, "this_month": {"requests_sent":0, "retries":0, "input_tokens":0, "output_tokens":0, "cache_answers":0}, "total": {"requests_sent":0, "retries":0, "input_tokens":0, "output_tokens":0, "cache_answers":0}}
        })
    );
    assert_eq!(output.stdout.last(), Some(&b'\n'));
    assert_eq!(fs::read_dir(&home).expect("home remains").count(), 0);

    let human = run::output(command(&home).arg("status")).expect("human status");
    assert!(human.status.success());
    let expected = format!(
        "version {}\nconfiguration_path {}\nconfiguration_present false\nurl https://api.typesafe.ai/v1/systemone\nurl_source built_in\nmodel jev-1.13.0\nmodel_source built_in\napi_key_set false\ncache_enabled true\ncache_enabled_source built_in\ncache_path {}\ncache_path_source platform\ncache_entries 0\ncache_bytes 0\ncache_bad_entries 0\ncache_temporary_entries 0\ncache_temporary_bytes 0\ncache_prune_target_bytes 100000000\ncache_prune_target_source built_in\nusage_path {}\nusage_month {}\nmonth_requests_sent 0\nmonth_retries 0\nmonth_input_tokens 0\nmonth_output_tokens 0\nmonth_cache_answers 0\ntotal_requests_sent 0\ntotal_retries 0\ntotal_input_tokens 0\ntotal_output_tokens 0\ntotal_cache_answers 0\n",
        env!("CARGO_PKG_VERSION"),
        home.join(".config/thinkthen/config.json").display(),
        home.join(".cache/thinkthen").display(),
        home.join(".cache/thinkthen-usage").display(),
        month,
    );
    assert_eq!(
        String::from_utf8(human.stdout).expect("human text"),
        expected
    );
}

#[test]
fn status_without_an_absolute_home_uses_the_exact_unavailable_shape() {
    let output = run::output(command(std::path::Path::new("relative")).args(["status", "--json"]))
        .expect("status runs");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).expect("status JSON");
    assert_eq!(value["configuration"]["path"], serde_json::Value::Null);
    assert_eq!(value["cache"]["path"], serde_json::Value::Null);
    assert_eq!(value["cache"]["entries"], serde_json::Value::Null);
    assert_eq!(value["cache"]["bad_entries"], serde_json::Value::Null);
    assert_eq!(value["cache"]["temporary_entries"], serde_json::Value::Null);
    assert_eq!(value["cache"]["temporary_bytes"], serde_json::Value::Null);
    assert_eq!(value["usage"]["path"], serde_json::Value::Null);
    assert_eq!(value["usage"]["this_month"], serde_json::Value::Null);
    assert_eq!(value["usage"]["total"], serde_json::Value::Null);
    let human = run::output(command(std::path::Path::new("relative")).arg("status"))
        .expect("human status runs");
    assert!(human.status.success());
    assert!(String::from_utf8_lossy(&human.stdout).contains("cache_bad_entries unavailable\n"));
    assert!(
        String::from_utf8_lossy(&human.stdout).contains("cache_temporary_entries unavailable\n")
    );
}

#[test]
fn environment_and_configuration_provenance_are_independent_and_hide_the_key() {
    let home =
        std::env::temp_dir().join(format!("thinkthen-status-sources-{}", std::process::id()));
    let _absent = fs::remove_dir_all(&home);
    let config_home = home.join("config");
    let cache_home = home.join("cache");
    let named_cache = home.join("named");
    fs::create_dir_all(config_home.join("thinkthen")).expect("config folder");
    fs::write(
        config_home.join("thinkthen/config.json"),
        r#"{"schema":"thinkthen.config/1","url":"https://configured.example/v1","model":"fixed","cache":false,"cache_bytes":42}"#,
    ).expect("configuration");
    let mut status = command(&home);
    let output = run::output(
        status
            .args(["status", "--json"])
            .env("XDG_CONFIG_HOME", &config_home)
            .env("XDG_CACHE_HOME", &cache_home)
            .env("THINKTHEN_BASE_URL", "https://environment.example/v1")
            .env("THINKTHEN_CACHE", &named_cache)
            .env("THINKTHEN_API_KEY", "secret-status-marker"),
    )
    .expect("status");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = String::from_utf8(output.stdout).expect("status text");
    assert!(!text.contains("secret-status-marker"));
    let value: serde_json::Value = serde_json::from_str(&text).expect("status JSON");
    assert_eq!(
        value["backend"]["url"],
        "https://environment.example/v1/systemone"
    );
    assert_eq!(value["backend"]["url_source"], "environment");
    assert_eq!(value["backend"]["model"], "fixed");
    assert_eq!(value["backend"]["model_source"], "configuration");
    assert_eq!(value["backend"]["api_key_set"], true);
    assert_eq!(value["cache"]["enabled"], true);
    assert_eq!(value["cache"]["enabled_source"], "environment");
    assert_eq!(
        value["cache"]["path"],
        named_cache.to_string_lossy().as_ref()
    );
    assert_eq!(value["cache"]["path_source"], "environment");
    assert_eq!(value["cache"]["prune_target_bytes"], 42);
    assert_eq!(value["cache"]["prune_target_source"], "configuration");
    assert_eq!(
        value["usage"]["path"],
        cache_home
            .join("thinkthen-usage")
            .to_string_lossy()
            .as_ref()
    );
    assert!(!home.join("named/thinkthen-usage").exists());
}

#[cfg(unix)]
#[test]
fn a_malformed_recognized_usage_month_fails_without_partial_output_or_repair() {
    use std::os::unix::fs::PermissionsExt as _;

    let home = std::env::temp_dir().join(format!("thinkthen-status-strict-{}", std::process::id()));
    let _absent = fs::remove_dir_all(&home);
    let usage = home.join(".cache/thinkthen-usage");
    fs::create_dir_all(&usage).expect("usage folder");
    fs::set_permissions(&usage, fs::Permissions::from_mode(0o700)).expect("private folder");
    let lock = usage.join(".lock");
    fs::write(&lock, []).expect("lock");
    fs::set_permissions(&lock, fs::Permissions::from_mode(0o600)).expect("private lock");
    let month = usage.join("2026-09.json");
    fs::write(&month, b"").expect("zero-byte month");
    fs::set_permissions(&month, fs::Permissions::from_mode(0o600)).expect("private month");
    let before = fs::read(&month).expect("before");

    let output = run::output(command(&home).args(["status", "--json"])).expect("status");
    assert_eq!(output.status.code(), Some(5));
    assert!(output.stdout.is_empty());
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "thinkthen: status could not read local usage file 2026-09.json: invalid contents\n"
    );
    assert_eq!(fs::read(month).expect("after"), before);
    assert!(!usage.join(".update.tmp").exists());
}

#[cfg(unix)]
#[test]
fn an_unsafe_cache_entry_is_counted_without_leaking_local_bytes() {
    use std::os::unix::fs::{PermissionsExt as _, symlink};

    let home = std::env::temp_dir().join(format!("thinkthen-status-cache-{}", std::process::id()));
    let _absent = fs::remove_dir_all(&home);
    let cache = home.join(".cache/thinkthen");
    fs::create_dir_all(&cache).expect("cache folder");
    fs::set_permissions(&cache, fs::Permissions::from_mode(0o700)).expect("private folder");
    let target = cache.join("private-target");
    fs::write(&target, b"private-cache-marker").expect("target");
    let entry = cache.join(format!("{}.json", "a".repeat(64)));
    symlink(&target, &entry).expect("entry symlink");
    let temporary = cache.join(format!(".123.0.{}.json", "b".repeat(64)));
    fs::write(&temporary, b"private-temporary-marker").expect("temporary file");
    let unsafe_temporary = cache.join(format!(".123.1.{}.json", "c".repeat(64)));
    symlink(&target, &unsafe_temporary).expect("temporary symlink");
    let allocated = {
        use std::os::unix::fs::MetadataExt as _;
        fs::metadata(&temporary)
            .expect("temporary metadata")
            .blocks()
            * 512
    };

    let output = run::output(command(&home).args(["status", "--json"])).expect("status");
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
    let text = String::from_utf8(output.stdout).expect("status text");
    let status: serde_json::Value = serde_json::from_str(&text).expect("status JSON");
    assert_eq!(status["cache"]["entries"], 0);
    assert_eq!(status["cache"]["bad_entries"], 1);
    assert_eq!(status["cache"]["temporary_entries"], 1);
    assert_eq!(status["cache"]["temporary_bytes"], allocated);
    assert!(!text.contains("private-cache-marker"));
    assert!(!text.contains("private-temporary-marker"));
    assert_eq!(
        fs::read(target).expect("target remains"),
        b"private-cache-marker"
    );
}

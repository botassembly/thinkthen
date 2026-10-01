//! The read-only process status surface.
#![cfg(feature = "cli")]

use crate::child::ChildEnvironment as _;
#[cfg(unix)]
use std::fs;
use std::process::Command;

use crate::run;

fn command(home: &std::path::Path) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_thinkthen"));
    command
        .clear_environment()
        .home(home)
        .env("PATH", std::env::var_os("PATH").unwrap_or_default());
    command
}

// It pins the default folders under `HOME`, the Linux folder (sdlc/planning/windows.md).
#[cfg(unix)]
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
            "schema": "thinkthen.status/2",
            "version": env!("CARGO_PKG_VERSION"),
            "configuration": {"path": home.join(".config/thinkthen/config.json"), "present": false},
            "backend": {"name": null, "url": "https://api.typesafe.ai/v1/systemone", "url_source": "built_in", "model": "jev-1.13.0", "model_source": "built_in", "key_variable": "THINKTHEN_API_KEY", "api_key_set": false},
            "cache": {"enabled": true, "enabled_source": "built_in", "path": home.join(".cache/thinkthen"), "path_source": "platform", "entries": 0, "bytes": 0, "old_entries": 0, "prune_target_bytes": 100000000, "prune_target_source": "built_in"},
            "usage": {"path": home.join(".local/state/thinkthen"), "month": month, "this_month": {"requests_sent":0, "retries":0, "input_tokens":0, "output_tokens":0, "cache_answers":0}, "total": {"requests_sent":0, "retries":0, "input_tokens":0, "output_tokens":0, "cache_answers":0}}
        })
    );
    assert_eq!(output.stdout.last(), Some(&b'\n'));
    assert_eq!(fs::read_dir(&home).expect("home remains").count(), 0);

    let human = run::output(command(&home).arg("status")).expect("human status");
    assert!(human.status.success());
    let expected = format!(
        "version {}\nconfiguration_path {}\nconfiguration_present false\nbackend none\nurl https://api.typesafe.ai/v1/systemone\nurl_source built_in\nmodel jev-1.13.0\nmodel_source built_in\nkey_variable THINKTHEN_API_KEY\napi_key_set false\ncache_enabled true\ncache_enabled_source built_in\ncache_path {}\ncache_path_source platform\ncache_entries 0\ncache_bytes 0\ncache_old_entries 0\ncache_prune_target_bytes 100000000\ncache_prune_target_source built_in\nusage_path {}\nusage_month {}\nmonth_requests_sent 0\nmonth_retries 0\nmonth_input_tokens 0\nmonth_output_tokens 0\nmonth_cache_answers 0\ntotal_requests_sent 0\ntotal_retries 0\ntotal_input_tokens 0\ntotal_output_tokens 0\ntotal_cache_answers 0\n",
        env!("CARGO_PKG_VERSION"),
        home.join(".config/thinkthen/config.json").display(),
        home.join(".cache/thinkthen").display(),
        home.join(".local/state/thinkthen").display(),
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
    assert_eq!(value["cache"]["bytes"], serde_json::Value::Null);
    assert_eq!(value["cache"]["old_entries"], serde_json::Value::Null);
    assert_eq!(value["usage"]["path"], serde_json::Value::Null);
    assert_eq!(value["usage"]["this_month"], serde_json::Value::Null);
    assert_eq!(value["usage"]["total"], serde_json::Value::Null);
    let human = run::output(command(std::path::Path::new("relative")).arg("status"))
        .expect("human status runs");
    assert!(human.status.success());
    let human = String::from_utf8_lossy(&human.stdout);
    assert!(human.contains("cache_entries unavailable\n"), "{human}");
    assert!(human.contains("cache_old_entries unavailable\n"), "{human}");
}

#[cfg(unix)]
fn private_cache(label: &str) -> std::io::Result<(std::path::PathBuf, std::path::PathBuf)> {
    use std::os::unix::fs::PermissionsExt as _;

    let home = std::env::temp_dir().join(format!(
        "thinkthen-status-store-{}-{label}",
        std::process::id()
    ));
    let _absent = fs::remove_dir_all(&home);
    let cache = home.join(".cache/thinkthen");
    fs::create_dir_all(&cache)?;
    fs::set_permissions(&cache, fs::Permissions::from_mode(0o700))?;
    Ok((home, cache))
}

#[cfg(unix)]
#[allow(
    clippy::expect_used,
    reason = "a failed fixture step should stop the boundary test"
)]
fn inspect(home: &std::path::Path) -> (Option<i32>, serde_json::Value, Vec<u8>) {
    let output = run::output(command(home).args(["status", "--json"])).expect("read-only status");
    let value = serde_json::from_slice(&output.stdout).unwrap_or(serde_json::Value::Null);
    (output.status.code(), value, output.stderr)
}

/// `status` counts the answers in `thinkthen.sqlite`, its bytes, and the old
/// digest-named entries beside it, and writes nothing. The two usage paths
/// scripts read stay where status v2 put them.
#[cfg(unix)]
#[test]
fn status_counts_the_live_store_and_old_entries_without_writes() {
    use conformance_backend::{Canned, Listener};

    let (home, cache) = private_cache("counts").expect("private cache");
    let (code, value, stderr) = inspect(&home);
    assert_eq!(code, Some(0));
    assert!(stderr.is_empty());
    assert_eq!(value["cache"]["entries"], 0);
    assert_eq!(value["cache"]["old_entries"], 0);
    assert_eq!(fs::read_dir(&cache).expect("empty cache").count(), 0);

    let old = cache.join(format!("{}.json", "a".repeat(64)));
    fs::write(&old, b"an old entry").expect("old entry");
    let listener = Listener::serving(vec![Canned::ok(concat!(
        r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.92}},"#,
        r#""usage":{"input_tokens":312,"output_tokens":48}}"#,
    ))])
    .expect("listener");
    let input = home.join("record.txt");
    fs::write(&input, b"Refund me please.").expect("record");
    let asked = run::output(
        command(&home)
            .args(["decide", "asks for a refund", "--input"])
            .arg(&input)
            .env("THINKTHEN_BASE_URL", listener.base())
            .env("THINKTHEN_API_KEY", "test-key"),
    )
    .expect("cached run");
    assert_eq!(asked.status.code(), Some(0), "{asked:?}");
    let store = fs::read(cache.join("thinkthen.sqlite")).expect("live store");
    let (code, value, stderr) = inspect(&home);
    assert_eq!(code, Some(0));
    assert!(stderr.is_empty());
    assert_eq!(value["schema"], "thinkthen.status/2");
    assert_eq!(value["cache"]["entries"], 1);
    assert_eq!(value["cache"]["bytes"], store.len());
    assert_eq!(value["cache"]["old_entries"], 1);
    assert_eq!(value["usage"]["total"]["requests_sent"], 1);
    assert_eq!(value["usage"]["total"]["input_tokens"], 312);
    assert_eq!(
        fs::read(cache.join("thinkthen.sqlite")).expect("store"),
        store
    );
    assert_eq!(
        fs::read(&old).expect("unchanged old entry"),
        b"an old entry"
    );

    fs::write(cache.join("thinkthen.sqlite"), b"private damaged store").expect("damaged");
    let (code, value, stderr) = inspect(&home);
    assert_eq!(code, Some(5));
    assert_eq!(value, serde_json::Value::Null);
    assert_eq!(stderr, b"thinkthen: status could not read the local cache or usage state; check its permissions and contents\n");
}

/// The folder marker is retired by ADR 0111, so status ignores one a
/// former version left, and a disabled cache still reports its folder.
#[cfg(unix)]
#[test]
fn a_retired_marker_is_ignored_and_a_disabled_cache_reports_off() {
    let (home, cache) = private_cache("marker-disabled").expect("private cache");
    let marker = cache.join(".thinkthen-backend.json");
    let invalid = b"private invalid marker";
    fs::write(&marker, invalid).expect("invalid marker");
    let config = home.join(".config/thinkthen/config.json");
    fs::create_dir_all(config.parent().expect("config parent")).expect("config folder");
    for disabled in [false, true] {
        if disabled {
            fs::write(&config, br#"{"schema":"thinkthen.config/1","cache":false}"#)
                .expect("disabled cache setting");
        }
        let (code, value, stderr) = inspect(&home);
        assert_eq!(code, Some(0));
        assert!(stderr.is_empty());
        assert_eq!(value["cache"]["enabled"], !disabled);
        assert_eq!(value["cache"]["entries"], 0);
        assert_eq!(value["cache"]["old_entries"], 0);
        assert!(!value.to_string().contains("private"));
        assert_eq!(fs::read(&marker).expect("unchanged marker"), invalid);
    }
}

// It names the XDG folders. Windows reads APPDATA and LOCALAPPDATA (sdlc/planning/windows.md).
#[cfg(unix)]
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
            .env("XDG_STATE_HOME", home.join("state"))
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
        home.join("state/thinkthen").to_string_lossy().as_ref()
    );
    assert!(!cache_home.join("thinkthen-usage").exists());
}

/// Plant a private usage folder under `home`'s state folder with these
/// files, each private.
#[cfg(unix)]
#[allow(clippy::expect_used, reason = "a failed fixture stops the proof")]
fn planted(label: &str, files: &[(&str, &[u8])]) -> (std::path::PathBuf, std::path::PathBuf) {
    use std::os::unix::fs::PermissionsExt as _;

    let home =
        std::env::temp_dir().join(format!("thinkthen-status-{label}-{}", std::process::id()));
    let _absent = fs::remove_dir_all(&home);
    let usage = home.join(".local/state/thinkthen");
    fs::create_dir_all(&usage).expect("usage folder");
    fs::set_permissions(&usage, fs::Permissions::from_mode(0o700)).expect("private folder");
    for (name, bytes) in files {
        fs::write(usage.join(name), bytes).expect("usage file");
        fs::set_permissions(usage.join(name), fs::Permissions::from_mode(0o600)).expect("private");
    }
    (home, usage)
}

/// Main refused a folder without `.lock` with exit 5 until ticket 0360 read
/// it as zero. Ticket 0367 reads its months without the lock, so a month
/// counts and status still creates and changes nothing.
#[cfg(unix)]
#[test]
fn a_missing_usage_lock_reads_its_months_without_creating_or_changing_state() {
    let bytes = b"{\"schema\":\"thinkthen.usage/1\",\"requests_sent\":1,\"input_tokens\":2,\"output_tokens\":3,\"cache_answers\":0}\n";
    let (home, usage) = planted("no-lock", &[("2026-09.json", bytes)]);
    let output = run::output(command(&home).args(["status", "--json"])).expect("status");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(output.stderr, b"");
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).expect("JSON");
    assert_eq!(value["usage"]["total"]["requests_sent"], 1);
    assert_eq!(value["usage"]["total"]["output_tokens"], 3);
    assert_eq!(
        fs::read(usage.join("2026-09.json")).expect("unchanged month"),
        bytes
    );
    assert_eq!(
        fs::read_dir(&usage).expect("unchanged directory").count(),
        1
    );
}

/// Main failed with exit 5 and printed no report. Status now reports the
/// rest, shows the counts as unavailable, and names the file and the fix.
/// A folder without `.lock` read as zero before ticket 0367; it reports the
/// same way.
#[cfg(unix)]
#[test]
fn a_malformed_usage_month_reports_unavailable_counts_and_the_fix() {
    let with_lock: &[(&str, &[u8])] = &[(".lock", b""), ("2026-09.json", b"")];
    let without_lock: &[(&str, &[u8])] = &[("2026-09.json", b"garbage\n")];
    for (label, files) in [("strict", with_lock), ("strict-no-lock", without_lock)] {
        let (home, usage) = planted(label, files);
        let month = files.last().map(|(_, bytes)| *bytes).unwrap_or_default();
        let output = run::output(command(&home).args(["status", "--json"])).expect("status");
        assert_eq!(output.status.code(), Some(0), "{label}");
        let sentence = format!(
            "thinkthen: cannot read the usage totals: {} has invalid contents. Move it aside, and counting starts again.\n",
            usage.join("2026-09.json").display()
        );
        assert_eq!(String::from_utf8_lossy(&output.stderr), sentence, "{label}");
        let value: serde_json::Value = serde_json::from_slice(&output.stdout).expect("JSON");
        assert_eq!(value["usage"]["this_month"], serde_json::Value::Null);
        assert_eq!(value["usage"]["total"], serde_json::Value::Null);
        assert_eq!(
            value["cache"]["path"],
            home.join(".cache/thinkthen").to_string_lossy().as_ref()
        );
        let human = run::output(command(&home).arg("status")).expect("human status");
        assert_eq!(human.status.code(), Some(0));
        assert_eq!(String::from_utf8_lossy(&human.stderr), sentence);
        let text = String::from_utf8(human.stdout).expect("text");
        assert!(text.ends_with("month_requests_sent unavailable\nmonth_retries unavailable\nmonth_input_tokens unavailable\nmonth_output_tokens unavailable\nmonth_cache_answers unavailable\ntotal_requests_sent unavailable\ntotal_retries unavailable\ntotal_input_tokens unavailable\ntotal_output_tokens unavailable\ntotal_cache_answers unavailable\n"), "{text}");
        assert_eq!(fs::read(usage.join("2026-09.json")).expect("after"), month);
        assert!(!usage.join(".update.tmp").exists());
        assert_eq!(
            usage.join(".lock").exists(),
            files.iter().any(|(name, _)| *name == ".lock"),
            "{label}: status made a lock"
        );
    }
}

/// Another process holding the usage lock makes status wait at most one
/// second, then report the busy sentence and exit 0 (ticket 0360).
#[cfg(unix)]
#[test]
fn a_held_usage_lock_reports_busy_within_a_second_and_exits_zero() {
    let month = b"{\"schema\":\"thinkthen.usage/1\",\"requests_sent\":1,\"input_tokens\":0,\"output_tokens\":0,\"cache_answers\":0}\n";
    let (home, usage) = planted("busy", &[(".lock", b""), ("2026-09.json", month)]);
    let held = fs::File::open(usage.join(".lock")).expect("lock file");
    held.lock().expect("hold the lock");
    let started = std::time::Instant::now();
    let output = run::output(command(&home).args(["status", "--json"])).expect("status");
    let elapsed = started.elapsed();
    drop(held);
    assert_eq!(output.status.code(), Some(0));
    assert!(elapsed < std::time::Duration::from_secs(3), "{elapsed:?}");
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        format!(
            "thinkthen: cannot read the usage totals: {} is locked by another process. Try again when it finishes.\n",
            usage.join(".lock").display()
        )
    );
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).expect("JSON");
    assert_eq!(value["usage"]["this_month"], serde_json::Value::Null);
    assert_eq!(value["usage"]["total"], serde_json::Value::Null);
}

/// QA's case: an older build wrote retries into the month file and a newer
/// one wrote a `retries-` file with another value. Main refused to choose
/// (exit 5, "retry totals differ"). The month file alone holds the counts.
#[cfg(unix)]
#[test]
fn an_old_retries_file_beside_a_month_with_retries_is_ignored() {
    let (home, _usage) = planted(
        "two-retries",
        &[
            (".lock", b""),
            ("2026-09.json", b"{\"schema\":\"thinkthen.usage/1\",\"requests_sent\":4,\"retries\":2,\"input_tokens\":7,\"output_tokens\":3,\"cache_answers\":0}\n"),
            ("retries-2026-09.json", b"{\"schema\":\"thinkthen.usage.retries/1\",\"retries\":5}\n"),
        ],
    );
    let output = run::output(command(&home).args(["status", "--json"])).expect("status");
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.stderr, b"");
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).expect("JSON");
    assert_eq!(value["usage"]["total"]["requests_sent"], 4);
    assert_eq!(value["usage"]["total"]["retries"], 2);
}

/// An unsafe month mode gives the unsafe sentence with the full path.
#[cfg(unix)]
#[test]
fn an_unsafe_usage_month_names_the_unsafe_fix() {
    use std::os::unix::fs::PermissionsExt as _;

    let month = b"{\"schema\":\"thinkthen.usage/1\",\"requests_sent\":1,\"input_tokens\":0,\"output_tokens\":0,\"cache_answers\":0}\n";
    let (home, usage) = planted("unsafe", &[(".lock", b""), ("2026-09.json", month)]);
    fs::set_permissions(
        usage.join("2026-09.json"),
        fs::Permissions::from_mode(0o644),
    )
    .expect("mode");
    let output = run::output(command(&home).args(["status"])).expect("status");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        format!(
            "thinkthen: cannot read the usage totals: {} has unsafe or unreadable state. Make it private to your user (folder 0700, files 0600), or move it aside.\n",
            usage.join("2026-09.json").display()
        )
    );
}

#[cfg(unix)]
#[test]
fn old_entries_are_counted_by_name_without_leaking_local_bytes() {
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

    let output = run::output(command(&home).args(["status", "--json"])).expect("status");
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
    let text = String::from_utf8(output.stdout).expect("status text");
    let status: serde_json::Value = serde_json::from_str(&text).expect("status JSON");
    assert_eq!(status["cache"]["entries"], 0);
    assert_eq!(status["cache"]["old_entries"], 1);
    assert!(!text.contains("private-cache-marker"));
    assert!(!text.contains("private-temporary-marker"));
    assert_eq!(
        fs::read(target).expect("target remains"),
        b"private-cache-marker"
    );
}

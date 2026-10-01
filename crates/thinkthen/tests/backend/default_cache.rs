use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Output;
use std::thread;

use crate::child::Folder;
use crate::harness::{Canned, Listener, spawn};
use crate::support::{
    DEFAULT_BASE, DEFAULT_MODEL, ENDPOINT_PATH, encoded_decide, keys, plant_fixture,
};

const ANSWERED: &str = concat!(
    r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.92}},"#,
    r#""usage":{"input_tokens":312,"output_tokens":48}}"#,
);
const EVIDENCE: &str = "Refund me please.";

fn folder(name: &str) -> PathBuf {
    let path = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _absent = fs::remove_dir_all(&path);
    path
}

fn run(arguments: &[&str], environment: &[(&str, &str)]) -> io::Result<Output> {
    spawn(arguments, environment, EVIDENCE.as_bytes())
}

fn url() -> String {
    format!("{DEFAULT_BASE}/{ENDPOINT_PATH}")
}

fn body() -> Vec<u8> {
    encoded_decide(EVIDENCE, DEFAULT_MODEL, "asks for a refund")
}

fn usage(status: &Output, name: &str) -> Option<u64> {
    serde_json::from_slice::<serde_json::Value>(&status.stdout)
        .ok()?
        .get("usage")?
        .get("this_month")?
        .get(name)?
        .as_u64()
}

#[test]
fn the_platform_cache_is_used_by_default_and_no_cache_disables_it() {
    let root = folder("default-cache-home");
    let (cache, state) = (Folder::Cache.variable(&root), Folder::Usage.variable(&root));
    let listener = Listener::serving(vec![Canned::ok(ANSWERED)]).expect("listener");
    let base = listener.base().to_owned();
    let environment = [
        ("THINKTHEN_BASE_URL", base.as_str()),
        ("THINKTHEN_API_KEY", "secret-key"),
        (cache.0, cache.1.as_str()),
        (state.0, state.1.as_str()),
    ];
    for (run_number, expected) in [(1, 1), (2, 0)] {
        let output = run(&["decide", "asks for a refund", "--details"], &environment).expect("run");
        assert_eq!(output.status.code(), Some(0));
        let printed = String::from_utf8_lossy(&output.stdout);
        assert!(
            printed.contains(&format!(r#""requests_sent":{expected},"cached":"#)),
            "run {run_number}: {printed}"
        );
    }
    assert_eq!(listener.requests().len(), 1);
    assert!(Folder::Cache.under(&root).is_dir());

    let listener =
        Listener::serving(vec![Canned::ok(ANSWERED), Canned::ok(ANSWERED)]).expect("listener");
    let base = listener.base().to_owned();
    let environment = [
        ("THINKTHEN_BASE_URL", base.as_str()),
        ("THINKTHEN_API_KEY", "secret-key"),
        (cache.0, cache.1.as_str()),
        (state.0, state.1.as_str()),
    ];
    for _ in 0..2 {
        assert_eq!(
            run(
                &["decide", "asks for another refund", "--no-cache"],
                &environment
            )
            .expect("run")
            .status
            .code(),
            Some(0)
        );
    }
    assert_eq!(listener.requests().len(), 2);

    let status = run(&["status", "--json"], &environment).expect("status");
    assert_eq!(status.status.code(), Some(0));
    let value: serde_json::Value = serde_json::from_slice(&status.stdout).expect("status JSON");
    assert_eq!(value["usage"]["this_month"]["requests_sent"], 3);
    assert_eq!(value["usage"]["this_month"]["input_tokens"], 936);
    assert_eq!(value["usage"]["this_month"]["output_tokens"], 144);
    assert_eq!(value["usage"]["this_month"]["cache_answers"], 1);
}

#[cfg(unix)]
#[test]
fn cache_prune_removes_every_answer_under_a_one_byte_target() {
    let folder = folder("cache-prune");
    prune::fill(
        &folder,
        EVIDENCE,
        &[(DEFAULT_MODEL, "asks for a refund", ANSWERED)],
    );
    fs::write(folder.join("sentinel"), "keep me").expect("sentinel");
    let output = run(
        &[
            "cache",
            "prune",
            folder.to_str().expect("folder"),
            "--max-size",
            "1",
        ],
        &[],
    )
    .expect("prune");
    assert_eq!(output.status.code(), Some(0));
    let line = String::from_utf8(output.stdout).expect("summary");
    assert!(line.starts_with("removed 1 answers and "), "{line}");
    assert!(line.contains("; 0 answers and "), "{line}");
    let status = run(
        &["status", "--json"],
        &[("THINKTHEN_CACHE", folder.to_str().expect("folder"))],
    )
    .expect("status");
    let value: serde_json::Value = serde_json::from_slice(&status.stdout).expect("status JSON");
    assert_eq!(value["cache"]["entries"], 0);
    assert_eq!(
        fs::read_to_string(folder.join("sentinel")).expect("sentinel"),
        "keep me"
    );
}

#[test]
fn an_enabled_default_cache_without_a_home_has_the_fixed_local_failure() {
    let relative_home = Path::new("ticket-0062-relative-home");
    let relative_cache = Path::new("ticket-0062-relative-cache");
    let relative_config = Path::new("ticket-0062-relative-config");
    let _absent = fs::remove_dir_all(relative_home);
    let _absent = fs::remove_dir_all(relative_cache);
    let _absent = fs::remove_dir_all(relative_config);
    let output = run(
        &["decide", "asks for a refund"],
        &[
            ("HOME", "ticket-0062-relative-home"),
            ("XDG_CACHE_HOME", "ticket-0062-relative-cache"),
            ("XDG_CONFIG_HOME", "ticket-0062-relative-config"),
        ],
    )
    .expect("run");
    assert_eq!(output.status.code(), Some(5));
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "thinkthen: no default cache folder is available; set THINKTHEN_CACHE or use --no-cache\n"
    );
    assert!(!relative_home.exists());
    assert!(!relative_cache.exists());
    assert!(!relative_config.exists());
}

#[test]
fn rejected_input_creates_no_default_cache() {
    let root = folder("rejected-default-cache");
    let cache = Folder::Cache.variable(&root);
    let environment = [(cache.0, cache.1.as_str())];
    let one_unit =
        spawn(&["find", "Which unit?"], &environment, b"only one\n").expect("find refusal");
    assert_eq!(one_unit.status.code(), Some(2));
    assert!(!Folder::Cache.under(&root).exists());

    let malformed = spawn(
        &["decide", "asks?", "--jsonl"],
        &environment,
        b"{not json}\n",
    )
    .expect("record refusal");
    assert_eq!(malformed.status.code(), Some(2));
    assert!(!Folder::Cache.under(&root).exists());
}

#[path = "default_cache/usage.rs"]
mod usage_tests;

#[test]
fn replay_of_a_missing_directory_is_a_miss_and_creates_nothing() {
    let missing = folder("missing-replay-directory");
    let output = run(
        &[
            "decide",
            "asks for a refund",
            "--replay",
            missing.to_str().expect("folder"),
        ],
        &[],
    )
    .expect("replay refusal");
    assert_eq!(output.status.code(), Some(5));
    let [key] = keys(&url(), &body()).try_into().expect("one question");
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        format!(
            "thinkthen: the decide request for one document: the replay folder holds no answer \
             for question `{key}`; the key is the SHA-256 of the adapter, address, model, \
             shared state and question as sent\n"
        )
    );
    assert!(!missing.exists());
}

#[cfg(unix)]
#[path = "default_cache/prune.rs"]
mod prune;

#[cfg(unix)]
#[path = "default_cache/unused.rs"]
mod unused;

#[cfg(unix)]
#[test]
fn replay_reads_a_read_only_directory_without_changing_it() {
    use std::os::unix::fs::PermissionsExt as _;

    // A failed earlier run may have left the folder read-only.
    let scratch = Path::new(env!("CARGO_TARGET_TMPDIR")).join("read-only-replay");
    let _writable = fs::set_permissions(&scratch, fs::Permissions::from_mode(0o700));
    let folder = folder("read-only-replay");
    plant_fixture(
        &folder,
        &url(),
        &body(),
        &[r#"{"type":"noul","noul":0.92}"#],
        None,
    )
    .expect("fixture");
    fs::set_permissions(
        folder.join("thinkthen.jsonl"),
        fs::Permissions::from_mode(0o400),
    )
    .expect("fixture mode");
    fs::set_permissions(&folder, fs::Permissions::from_mode(0o500)).expect("folder mode");
    let before = fs::metadata(&folder)
        .expect("before metadata")
        .modified()
        .expect("before mtime");
    let names = fs::read_dir(&folder).expect("before names").count();
    let output = run(
        &[
            "decide",
            "asks for a refund",
            "--replay",
            folder.to_str().expect("folder"),
        ],
        &[],
    )
    .expect("replay");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(fs::read_dir(&folder).expect("after names").count(), names);
    assert_eq!(
        fs::metadata(&folder)
            .expect("after metadata")
            .modified()
            .expect("after mtime"),
        before
    );
    fs::set_permissions(&folder, fs::Permissions::from_mode(0o700)).expect("restore mode");
}

#[cfg(unix)]
#[test]
fn an_existing_wide_default_cache_is_refused_without_changing_its_mode() {
    use std::os::unix::fs::PermissionsExt as _;
    let root = folder("wide-default-cache");
    let cache = Folder::Cache.under(&root);
    fs::create_dir_all(&cache).expect("cache");
    fs::set_permissions(&cache, fs::Permissions::from_mode(0o755)).expect("mode");
    let moved = Folder::Cache.variable(&root);
    let output = run(
        &["decide", "asks for a refund"],
        &[(moved.0, moved.1.as_str())],
    )
    .expect("run");
    assert_eq!(output.status.code(), Some(5));
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "thinkthen: the default cache folder is not private; set its permissions to 0700 or use --no-cache\n"
    );
    assert_eq!(
        fs::metadata(cache).expect("metadata").permissions().mode() & 0o777,
        0o755
    );
}

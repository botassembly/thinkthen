use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Output;
use std::thread;

use crate::harness::{Canned, Listener, spawn};
use crate::support::{
    DEFAULT_BASE, DEFAULT_MODEL, ENDPOINT_PATH, encoded_decide, plant_backend_identity,
    plant_recording,
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

fn plant(folder: &Path, response: &str) -> Option<String> {
    let request = encoded_decide(EVIDENCE, DEFAULT_MODEL, "asks for a refund");
    let url = format!("{DEFAULT_BASE}/{ENDPOINT_PATH}");
    let name = plant_recording(folder, &url, &request, response)?;
    plant_backend_identity(folder, &url)?;
    Some(name)
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
    let listener = Listener::serving(vec![Canned::ok(ANSWERED)]).expect("listener");
    let base = listener.base().to_owned();
    let environment = [
        ("THINKTHEN_BASE_URL", base.as_str()),
        ("THINKTHEN_API_KEY", "secret-key"),
        ("XDG_CACHE_HOME", root.to_str().expect("cache root")),
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
    assert!(root.join("thinkthen").is_dir());

    let listener =
        Listener::serving(vec![Canned::ok(ANSWERED), Canned::ok(ANSWERED)]).expect("listener");
    let base = listener.base().to_owned();
    let environment = [
        ("THINKTHEN_BASE_URL", base.as_str()),
        ("THINKTHEN_API_KEY", "secret-key"),
        ("XDG_CACHE_HOME", root.to_str().expect("cache root")),
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

#[test]
fn cache_prune_removes_valid_entries_to_the_explicit_target() {
    let folder = folder("cache-prune");
    let name = plant(&folder, ANSWERED).expect("entry");
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
    assert!(line.starts_with("removed 1 entries and "), "{line}");
    assert!(line.ends_with("; 0 entries and 0 bytes remain\n"), "{line}");
    assert!(!folder.join(name).exists());
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
    let environment = [("XDG_CACHE_HOME", root.to_str().expect("root"))];
    let one_unit =
        spawn(&["find", "Which unit?"], &environment, b"only one\n").expect("find refusal");
    assert_eq!(one_unit.status.code(), Some(2));
    assert!(!root.join("thinkthen").exists());

    let malformed = spawn(
        &["decide", "asks?", "--jsonl"],
        &environment,
        b"{not json}\n",
    )
    .expect("record refusal");
    assert_eq!(malformed.status.code(), Some(2));
    assert!(!root.join("thinkthen").exists());
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
    let message = String::from_utf8_lossy(&output.stderr);
    assert!(message.starts_with(
        "thinkthen: the decide request for one document: the replay folder holds no entry named `"
    ));
    assert!(
        message.ends_with("`; the entry name covers the backend interface, address, and request\n")
    );
    assert!(!missing.exists());
}

#[cfg(unix)]
#[path = "default_cache/prune.rs"]
mod prune;

#[cfg(unix)]
#[test]
fn replay_locks_a_read_only_directory_without_changing_it() {
    use std::os::unix::fs::PermissionsExt as _;

    let folder = folder("read-only-replay");
    let name = plant(&folder, ANSWERED).expect("entry");
    fs::set_permissions(folder.join(name), fs::Permissions::from_mode(0o400)).expect("entry mode");
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
    let cache = root.join("thinkthen");
    fs::create_dir_all(&cache).expect("cache");
    fs::set_permissions(&cache, fs::Permissions::from_mode(0o755)).expect("mode");
    let output = run(
        &["decide", "asks for a refund"],
        &[("XDG_CACHE_HOME", root.to_str().expect("root"))],
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

#[cfg(target_os = "linux")]
fn prune_opened_folder(prune: &mut std::process::Child, folder: &Path) -> io::Result<bool> {
    use std::time::{Duration, Instant};

    let metadata = fs::metadata(folder)?;
    let deadline = Instant::now() + Duration::from_secs(2);
    loop {
        if crate::harness::process_has_file(prune.id(), &metadata)? {
            return Ok(prune.try_wait()?.is_none());
        }
        if Instant::now() >= deadline || prune.try_wait()?.is_some() {
            return Ok(false);
        }
        thread::yield_now();
    }
}

#[cfg(target_os = "linux")]
#[test]
fn prune_waits_for_a_live_partial_and_preserves_its_installed_entry() {
    use std::process::{Command, Stdio};
    use std::sync::{Arc, mpsc};
    use std::time::Duration;

    use crate::harness::Observed;
    use conformance_backend::Rendezvous;

    let cache = folder("cache-prune-live-writer");
    let named = cache.to_str().expect("cache path");
    let release = Arc::new(Rendezvous::new(2));
    let (events, observed) = mpsc::channel();
    let listener = Listener::answering_with_events(
        {
            let release = Arc::clone(&release);
            move |_| Canned::ok(ANSWERED).after_release(Arc::clone(&release))
        },
        events,
    )
    .expect("listener");
    let base = listener.base().to_owned();
    thread::scope(|scope| {
        let writer = scope.spawn(|| {
            run(
                &["decide", "asks for a refund", "--cache", named, "--details"],
                &[
                    ("THINKTHEN_API_KEY", "sk-test-value"),
                    ("THINKTHEN_BASE_URL", &base),
                ],
            )
            .expect("writer")
        });
        assert!(matches!(
            observed.recv_timeout(Duration::from_secs(2)),
            Ok(Observed::Request)
        ));
        let partials = fs::read_dir(&cache)
            .expect("cache exists")
            .map(|item| {
                item.expect("item")
                    .file_name()
                    .to_string_lossy()
                    .into_owned()
            })
            .filter(|name| {
                name.ends_with(".json") && name.as_bytes().get(1).is_some_and(u8::is_ascii_digit)
            })
            .collect::<Vec<_>>();
        assert_eq!(partials.len(), 1, "writer's actual partial: {partials:?}");
        let status =
            run(&["status", "--json"], &[("THINKTHEN_CACHE", named)]).expect("status during write");
        let value: serde_json::Value = serde_json::from_slice(&status.stdout).expect("status JSON");
        assert_eq!(value["cache"]["temporary_entries"], 1);
        let mut prune = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
            .env_clear()
            .args(["cache", "prune", named, "--max-size", "100000000"])
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("prune process");
        let waiting = prune_opened_folder(&mut prune, &cache).expect("folder wait");
        release.wait();
        let answered = writer.join().expect("writer thread");
        let pruned = prune.wait_with_output().expect("prune completes");
        assert!(waiting, "prune must wait on the open folder gate");
        assert_eq!(answered.status.code(), Some(0));
        assert_eq!(pruned.status.code(), Some(0));
        assert_eq!(listener.requests().len(), 1);
        let replayed = run(
            &[
                "decide",
                "asks for a refund",
                "--replay",
                named,
                "--url",
                &base,
                "--details",
            ],
            &[],
        )
        .expect("replay installed entry");
        assert_eq!(replayed.status.code(), Some(0));
        let (written, _, _) =
            crate::result_assertions::normalized_details(&answered).expect("writer details");
        let (saved, cached, sent) =
            crate::result_assertions::normalized_details(&replayed).expect("replayed details");
        assert_eq!(written, saved);
        assert!(cached);
        assert_eq!(sent, 0);
        assert!(!cache.join(&partials[0]).exists());
    });
}

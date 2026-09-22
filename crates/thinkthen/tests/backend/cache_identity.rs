//! A recording folder stays bound to the backend that first wrote it.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::thread;

use crate::harness::{Canned, Listener, spawn};
use crate::support::{encoded_decide, plant_recording};

const QUESTION: &str = "asks for a refund";
const EVIDENCE: &str = "Refund me please.";
const ANSWER: &str = concat!(
    r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9}},"#,
    r#""usage":{"input_tokens":10,"output_tokens":2}}"#,
);
const MISMATCH: &str = concat!(
    "thinkthen: the recording folder belongs to another backend interface or address; ",
    "restore its backend settings or choose another folder\n",
);
const LEGACY: &str = concat!(
    "thinkthen: the recording folder predates backend binding; ",
    "replay it read-only or choose a new folder\n",
);
type FolderFiles = Vec<(std::ffi::OsString, Vec<u8>)>;

fn folder(name: &str) -> PathBuf {
    let path = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _absent = fs::remove_dir_all(&path);
    path
}

fn decide(base: &str, folder: &Path, mode: &str, key: bool) -> io::Result<std::process::Output> {
    decide_model(base, folder, mode, key, "local-1")
}

fn decide_model(
    base: &str,
    folder: &Path,
    mode: &str,
    key: bool,
    model: &str,
) -> io::Result<std::process::Output> {
    let environment = key
        .then_some(("THINKTHEN_API_KEY", "sk-test-value"))
        .into_iter()
        .collect::<Vec<_>>();
    spawn(
        &[
            "decide",
            QUESTION,
            "--url",
            base,
            "--model",
            model,
            mode,
            &folder.to_string_lossy(),
        ],
        &environment,
        EVIDENCE.as_bytes(),
    )
}

fn files(folder: &Path) -> io::Result<FolderFiles> {
    let mut found = fs::read_dir(folder)?
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_ok_and(|kind| kind.is_file()))
        .map(|entry| Ok((entry.file_name(), fs::read(entry.path())?)))
        .collect::<io::Result<Vec<_>>>()?;
    found.sort_by(|left, right| left.0.cmp(&right.0));
    Ok(found)
}

fn default_cache(home: &Path) -> PathBuf {
    if cfg!(target_os = "macos") {
        home.join("Library/Caches/thinkthen")
    } else {
        home.join(".cache/thinkthen")
    }
}

#[test]
fn a_cache_refuses_another_address_before_key_lookup_or_send() {
    let cache = folder("cache-backend-mismatch");
    let first = Listener::answering(|_| Canned::ok(ANSWER)).expect("first listener");
    let filled = decide(first.base(), &cache, "--cache", true).expect("fill runs");
    assert_eq!(filled.status.code(), Some(0));
    let before = files(&cache).expect("cache files");
    assert!(cache.join(".thinkthen-backend.json").is_file());

    let second = Listener::answering(|_| Canned::ok(ANSWER)).expect("second listener");
    let refused = decide(second.base(), &cache, "--cache", false).expect("refusal runs");
    assert_eq!(refused.status.code(), Some(5));
    assert!(refused.stdout.is_empty());
    assert_eq!(String::from_utf8_lossy(&refused.stderr), MISMATCH);
    assert!(second.requests().is_empty());
    let after = files(&cache).expect("cache files");
    assert_eq!(after, before);
}

#[test]
fn concurrent_first_users_at_different_addresses_choose_one_backend() {
    let cache = folder("cache-backend-first-use-race");
    let first = Listener::answering(|_| Canned::ok(ANSWER).after(30)).expect("first listener");
    let second = Listener::answering(|_| Canned::ok(ANSWER).after(30)).expect("second listener");
    let first_base = first.base().to_owned();
    let second_base = second.base().to_owned();
    let first_cache = cache.clone();
    let second_cache = cache.clone();
    let (first_output, second_output) = thread::scope(|scope| {
        let first_run = scope
            .spawn(move || decide(&first_base, &first_cache, "--cache", true).expect("first runs"));
        let second_run = scope.spawn(move || {
            decide(&second_base, &second_cache, "--cache", true).expect("second runs")
        });
        (
            first_run.join().expect("first joins"),
            second_run.join().expect("second joins"),
        )
    });

    let mut codes = [first_output.status.code(), second_output.status.code()];
    codes.sort();
    assert_eq!(codes, [Some(0), Some(5)]);
    assert_eq!(first.requests().len() + second.requests().len(), 1);
    assert_eq!(
        [first_output.stderr, second_output.stderr]
            .iter()
            .filter(|bytes| bytes.as_slice() == MISMATCH.as_bytes())
            .count(),
        1
    );
    let marker = fs::read(cache.join(".thinkthen-backend.json")).expect("complete marker");
    assert!(serde_json::from_slice::<serde_json::Value>(&marker).is_ok());
}

#[test]
fn normalized_address_spellings_and_models_share_one_folder() {
    let cache = folder("cache-backend-normalized-and-models");
    let listener = Listener::answering(|_| Canned::ok(ANSWER)).expect("listener");
    let first_base = listener.base().replacen("http://", "HTTP://", 1);
    let first = decide_model(
        &format!("{first_base}/"),
        &cache,
        "--record",
        true,
        "local-1",
    )
    .expect("first runs");
    assert_eq!(first.status.code(), Some(0));
    let second =
        decide_model(listener.base(), &cache, "--record", true, "local-2").expect("second runs");
    assert_eq!(second.status.code(), Some(0));
    assert_eq!(listener.requests().len(), 2);
    assert_eq!(
        files(&cache)
            .expect("cache files")
            .iter()
            .filter(|(name, _)| !name.to_string_lossy().starts_with('.'))
            .count(),
        2
    );
}

#[test]
fn an_old_exact_replay_stays_read_only_and_a_miss_explains_the_digest() {
    let recording = folder("legacy-replay");
    let base = "http://127.0.0.1:1/v1";
    let url = format!("{base}/systemone");
    let request = encoded_decide(EVIDENCE, "local-1", QUESTION);
    let name = plant_recording(&recording, &url, &request, ANSWER).expect("legacy entry");
    let before = fs::read(recording.join(&name)).expect("entry");

    let replayed = decide(base, &recording, "--replay", false).expect("replay runs");
    assert_eq!(replayed.status.code(), Some(0));
    assert_eq!(fs::read(recording.join(&name)).expect("entry"), before);
    assert!(!recording.join(".thinkthen-backend.json").exists());

    let missed = decide("http://127.0.0.1:2/v1", &recording, "--replay", false).expect("miss runs");
    assert_eq!(missed.status.code(), Some(5));
    let message = String::from_utf8_lossy(&missed.stderr);
    assert!(message.starts_with("thinkthen: the replay folder holds no entry named `"));
    assert!(
        message.ends_with("`; the entry name covers the backend interface, address, and request\n")
    );
    assert!(!recording.join(".thinkthen-backend.json").exists());
}

#[test]
fn a_write_capable_old_folder_refuses_before_send() {
    let recording = folder("legacy-write-refusal");
    let old_base = "http://127.0.0.1:1/v1";
    let request = encoded_decide(EVIDENCE, "local-1", QUESTION);
    plant_recording(
        &recording,
        &format!("{old_base}/systemone"),
        &request,
        ANSWER,
    )
    .expect("legacy entry");
    let listener = Listener::answering(|_| Canned::ok(ANSWER)).expect("listener");

    let refused = decide(listener.base(), &recording, "--cache", true).expect("refusal runs");
    assert_eq!(refused.status.code(), Some(5));
    assert_eq!(String::from_utf8_lossy(&refused.stderr), LEGACY);
    assert!(listener.requests().is_empty());
    assert!(!recording.join(".thinkthen-backend.json").exists());
}

#[test]
fn malformed_identity_is_a_secret_safe_storage_failure() {
    let cache = folder("malformed-cache-identity");
    fs::create_dir_all(&cache).expect("cache folder");
    fs::write(
        cache.join(".thinkthen-backend.json"),
        b"private evidence that is not json",
    )
    .expect("marker");
    let listener = Listener::answering(|_| Canned::ok(ANSWER)).expect("listener");

    let refused = decide(listener.base(), &cache, "--cache", true).expect("refusal runs");
    assert_eq!(refused.status.code(), Some(5));
    assert_eq!(
        String::from_utf8_lossy(&refused.stderr),
        "thinkthen: the recording folder could not be read or written; check its permissions and free space\n"
    );
    assert!(listener.requests().is_empty());
    assert!(!String::from_utf8_lossy(&refused.stderr).contains("private evidence"));
}

#[test]
fn default_cache_binds_replays_and_stays_out_of_status_and_prune_counts() {
    let home = folder("default-cache-identity-home");
    let listener = Listener::serving(vec![Canned::ok(ANSWER)]).expect("listener");
    let environment = [
        ("HOME", home.to_str().expect("home")),
        ("THINKTHEN_API_KEY", "sk-test-value"),
    ];
    let arguments = [
        "decide",
        QUESTION,
        "--url",
        listener.base(),
        "--model",
        "local-1",
    ];
    for _ in 0..2 {
        let output = spawn(&arguments, &environment, EVIDENCE.as_bytes()).expect("cached run");
        assert_eq!(output.status.code(), Some(0));
    }
    assert_eq!(listener.requests().len(), 1);
    let cache = default_cache(&home);
    let marker = cache.join(".thinkthen-backend.json");
    let before = fs::read(&marker).expect("bound default cache");

    let cache_text = cache.to_str().expect("cache");
    let pruned = spawn(
        &["cache", "prune", cache_text, "--max-size", "99999999"],
        &[],
        &[],
    )
    .expect("prune runs");
    assert_eq!(pruned.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&pruned.stdout).starts_with("removed 0 entries"));
    assert_eq!(fs::read(&marker).expect("marker after prune"), before);

    let status = spawn(
        &["status", "--json"],
        &[("THINKTHEN_CACHE", cache_text)],
        &[],
    )
    .expect("status runs");
    assert_eq!(status.status.code(), Some(0));
    let value: serde_json::Value = serde_json::from_slice(&status.stdout).expect("status JSON");
    assert_eq!(value["cache"]["entries"], 1);
}

#[test]
fn paired_record_replay_binds_then_replays() {
    let cache = folder("paired-record-replay-identity");
    let listener = Listener::serving(vec![Canned::ok(ANSWER)]).expect("listener");
    let cache_text = cache.to_str().expect("cache");
    let arguments = [
        "decide",
        QUESTION,
        "--url",
        listener.base(),
        "--model",
        "local-1",
        "--record",
        cache_text,
        "--replay",
        cache_text,
    ];
    for _ in 0..2 {
        let output = spawn(
            &arguments,
            &[("THINKTHEN_API_KEY", "sk-test-value")],
            EVIDENCE.as_bytes(),
        )
        .expect("paired run");
        assert_eq!(output.status.code(), Some(0));
    }
    assert_eq!(listener.requests().len(), 1);
    assert!(cache.join(".thinkthen-backend.json").is_file());
}

#[test]
fn a_bound_cache_mixes_a_hit_and_a_miss_without_rebinding() {
    let cache = folder("mixed-hit-miss-identity");
    let listener =
        Listener::serving(vec![Canned::ok(ANSWER), Canned::ok(ANSWER)]).expect("listener");
    let cache_text = cache.to_str().expect("cache");
    let arguments = [
        "decide",
        QUESTION,
        "--url",
        listener.base(),
        "--model",
        "local-1",
        "--cache",
        cache_text,
        "--lines",
    ];
    let first = spawn(
        &arguments,
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        format!("{EVIDENCE}\n").as_bytes(),
    )
    .expect("first run");
    assert_eq!(first.status.code(), Some(0));
    let second = spawn(
        &arguments,
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        format!("{EVIDENCE}\nA new record\n").as_bytes(),
    )
    .expect("mixed run");
    assert_eq!(second.status.code(), Some(0));
    assert_eq!(listener.requests().len(), 2);
    assert_eq!(String::from_utf8_lossy(&second.stdout).lines().count(), 2);
}

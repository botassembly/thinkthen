//! A named folder's writers decide answers; the command warns before a hit.

use std::fs;
use std::os::unix::fs::PermissionsExt as _;
use std::path::{Path, PathBuf};

use crate::harness::{Canned, Listener, spawn};
use crate::support::{encoded_decide, plant_backend_identity, plant_recording};

const QUESTION: &str = "asks for a refund";
const EVIDENCE: &str = "Refund me please.";
const MODEL: &str = "local-1";
const ANSWER: &str = r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9}}}"#;
const WARNING: &str = "thinkthen: warning: another user may change this named cache or recording folder; its writers decide the answers read from it\n";

fn folder(name: &str) -> PathBuf {
    let path = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    if path.is_dir() {
        let _restored = fs::set_permissions(&path, fs::Permissions::from_mode(0o700));
    }
    let _absent = fs::remove_dir_all(&path);
    path
}

fn plant(folder: &Path, base: &str) -> Option<PathBuf> {
    let url = format!("{base}/systemone");
    let request = encoded_decide(EVIDENCE, MODEL, QUESTION);
    let name = plant_recording(folder, &url, &request, ANSWER)?;
    plant_backend_identity(folder, &url)?;
    Some(folder.join(name))
}

#[test]
fn named_folder_mode_warns_once_before_keyless_cache_and_replay_hits() {
    let listener = Listener::answering(|_| Canned::ok(ANSWER)).expect("listener");
    for (label, mode, selector, quiet, warning) in [
        ("private-cache", 0o700, "cache", false, false),
        ("group-cache", 0o770, "cache", false, true),
        ("world-replay", 0o777, "replay", false, true),
        ("environment-cache", 0o777, "environment", false, true),
        ("private-read-only-replay", 0o500, "replay", false, false),
        ("quiet-group-cache", 0o770, "cache", true, true),
    ] {
        let cache = folder(&format!("named-trust-{label}"));
        let entry = plant(&cache, listener.base()).expect("bound entry");
        let before = fs::read(&entry).expect("entry before replay");
        fs::set_permissions(&cache, fs::Permissions::from_mode(mode)).expect("folder mode");
        let named = cache.to_str().expect("folder path");
        let mut arguments = vec![
            "decide",
            QUESTION,
            "--url",
            listener.base(),
            "--model",
            MODEL,
        ];
        let environment = if selector == "environment" {
            vec![("THINKTHEN_CACHE", named)]
        } else {
            arguments.extend([
                if selector == "cache" {
                    "--cache"
                } else {
                    "--replay"
                },
                named,
            ]);
            Vec::new()
        };
        if quiet {
            arguments.push("--quiet");
        }
        let output = spawn(&arguments, &environment, EVIDENCE.as_bytes()).expect("keyless hit");
        assert_eq!(output.status.code(), Some(0), "{label}");
        assert_eq!(
            output.stdout,
            if quiet { &b""[..] } else { &b"true\n"[..] },
            "{label}"
        );
        assert_eq!(
            output.stderr,
            if warning { WARNING.as_bytes() } else { b"" },
            "{label}"
        );
        assert_eq!(
            fs::read(&entry).expect("entry after replay"),
            before,
            "{label}"
        );
        assert!(listener.requests().is_empty(), "{label} sent a request");
        fs::set_permissions(&cache, fs::Permissions::from_mode(0o700))
            .expect("restore fixture mode");
    }
}

#[test]
fn a_named_recording_warns_before_the_missing_key_failure_without_sending() {
    // A loopback backend intentionally permits a blank key. Use the reserved
    // invalid domain so the missing-key refusal precedes any possible send.
    let base = "https://example.invalid/v1";
    let cache = folder("named-trust-record-no-key");
    let entry = plant(&cache, base).expect("bound entry");
    let before = fs::read(&entry).expect("entry before run");
    fs::set_permissions(&cache, fs::Permissions::from_mode(0o777)).expect("folder mode");
    let output = spawn(
        &[
            "decide",
            QUESTION,
            "--url",
            base,
            "--model",
            MODEL,
            "--record",
            cache.to_str().expect("folder"),
        ],
        &[],
        EVIDENCE.as_bytes(),
    )
    .expect("record refusal");
    assert_eq!(output.status.code(), Some(4));
    assert!(output.stdout.is_empty());
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        format!(
            "{WARNING}thinkthen: the environment variable `THINKTHEN_API_KEY` is unset or blank, so no key is sent\n"
        )
    );
    assert_eq!(fs::read(entry).expect("entry after refusal"), before);
}

#[test]
fn trust_warning_precedes_the_refresh_cost_warning() {
    let listener = Listener::answering(|_| Canned::ok(ANSWER)).expect("listener");
    let cache = folder("named-trust-refresh-order");
    plant(&cache, listener.base()).expect("bound entry");
    fs::set_permissions(&cache, fs::Permissions::from_mode(0o770)).expect("folder mode");
    let output = spawn(
        &[
            "decide",
            QUESTION,
            "--url",
            listener.base(),
            "--model",
            MODEL,
            "--cache",
            cache.to_str().expect("folder"),
            "--refresh-cache",
        ],
        &[("THINKTHEN_API_KEY", "test-key")],
        EVIDENCE.as_bytes(),
    )
    .expect("refresh");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(output.stdout, b"true\n");
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        format!(
            "{WARNING}thinkthen: warning: a mutable model alias or --refresh-cache sends each planned cache request live and may incur a charge\n"
        )
    );
    assert_eq!(listener.requests().len(), 1);
}

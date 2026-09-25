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
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .starts_with("thinkthen: the replay folder holds no entry named `")
    );
    assert!(!missing.exists());
}

#[test]
fn prune_model_selection_and_scan_before_delete_hold() {
    let folder = folder("cache-prune-model");
    let current = plant(&folder, ANSWERED).expect("current");
    let url = format!("{DEFAULT_BASE}/{ENDPOINT_PATH}");
    let old_response = ANSWERED.replace("jev-1.13.0", "jev-old");
    let old = plant_recording(
        &folder,
        &url,
        &encoded_decide(EVIDENCE, DEFAULT_MODEL, "another question"),
        &old_response,
    )
    .expect("old");
    let output = run(
        &[
            "cache",
            "prune",
            folder.to_str().expect("folder"),
            "--max-size",
            "100000000",
            "--answered-by-other-than",
            "jev-1.13.0",
        ],
        &[],
    )
    .expect("prune");
    assert_eq!(output.status.code(), Some(0));
    assert!(folder.join(current).exists());
    assert!(!folder.join(old).exists());

    let removable = plant_recording(
        &folder,
        &url,
        &encoded_decide(EVIDENCE, DEFAULT_MODEL, "third question"),
        &old_response,
    )
    .expect("old");
    fs::write(
        folder.join("0".repeat(64) + ".json"),
        "private malformed marker",
    )
    .expect("malformed");
    let output = run(
        &[
            "cache",
            "prune",
            folder.to_str().expect("folder"),
            "--answered-by-other-than",
            "jev-1.13.0",
        ],
        &[],
    )
    .expect("refusal");
    assert_eq!(output.status.code(), Some(5));
    assert!(output.stdout.is_empty());
    assert!(folder.join(removable).exists());
    assert!(!String::from_utf8_lossy(&output.stderr).contains("private malformed marker"));
}

/// The allocated bytes of the named entries, as prune counts them.
fn allocated(folder: &Path, names: &[&String]) -> u64 {
    use std::os::unix::fs::MetadataExt as _;
    names
        .iter()
        .map(|name| fs::metadata(folder.join(name)).expect("entry").blocks() * 512)
        .sum()
}

#[test]
fn prune_refuses_the_alias_and_keeps_the_upgrade() {
    const REFUSAL: &str = concat!(
        "thinkthen: --answered-by-other-than names the model the requests asked for, ",
        "and no reply names it, so prune removed nothing; ",
        "name the version a result's meta.model shows, not the alias passed to --model\n",
    );
    let url = format!("{DEFAULT_BASE}/{ENDPOINT_PATH}");
    let echoed = ANSWERED.replace("jev-1.13.0", DEFAULT_MODEL);
    // Row: MODEL, other options, the second entry's reply, and which of the
    // two entries leave. `None` means the alias refusal.
    let rows: [(&str, &[&str], &str, Option<[bool; 2]>); 7] = [
        ("jev-latest", &[], ANSWERED, None),
        ("jev-latest", &["--older-than", "1d"], ANSWERED, None),
        ("jev-1.14.0", &[], ANSWERED, Some([true, true])),
        ("no-such-model", &[], ANSWERED, Some([true, true])),
        ("JEV-LATEST", &[], ANSWERED, Some([true, true])),
        ("jev-1.13.0", &[], ANSWERED, Some([false, false])),
        ("jev-latest", &[], &echoed, Some([true, false])),
    ];
    for (row, (model, options, second_reply, removed)) in rows.into_iter().enumerate() {
        let folder = folder(&format!("prune-alias-{row}"));
        let first = plant(&folder, ANSWERED).expect("first entry");
        let request = encoded_decide(EVIDENCE, DEFAULT_MODEL, "another question");
        let second = plant_recording(&folder, &url, &request, second_reply).expect("second");
        let names = [&first, &second];
        let bytes = [
            allocated(&folder, &[&first]),
            allocated(&folder, &[&second]),
        ];
        let before = [
            fs::read(folder.join(&first)),
            fs::read(folder.join(&second)),
        ]
        .map(|read| read.expect("entry"));
        let mut arguments = vec!["cache", "prune", folder.to_str().expect("folder")];
        arguments.extend(["--answered-by-other-than", model]);
        arguments.extend_from_slice(options);
        let output = run(&arguments, &[]).expect("prune");
        let Some(removed) = removed else {
            assert_eq!(output.status.code(), Some(2), "{row}");
            assert!(output.stdout.is_empty(), "{row}");
            assert_eq!(String::from_utf8_lossy(&output.stderr), REFUSAL, "{row}");
            for (name, bytes) in names.iter().zip(&before) {
                assert_eq!(&fs::read(folder.join(name)).expect("kept"), bytes, "{row}");
            }
            continue;
        };
        let out = removed.iter().filter(|leaves| **leaves).count();
        let out_bytes: u64 = (0..2)
            .filter(|index| removed[*index])
            .map(|index| bytes[index])
            .sum();
        let (kept, kept_bytes) = (2 - out, bytes.iter().sum::<u64>() - out_bytes);
        assert_eq!(output.status.code(), Some(0), "{row}");
        assert_eq!(
            String::from_utf8_lossy(&output.stdout),
            format!(
                "removed {out} entries and {out_bytes} bytes; {kept} entries and {kept_bytes} bytes remain\n"
            ),
            "{row}"
        );
        for (index, name) in names.iter().enumerate() {
            assert_eq!(folder.join(name).exists(), !removed[index], "{row}");
        }
    }

    let empty = folder("prune-alias-empty");
    fs::create_dir_all(&empty).expect("empty folder");
    plant_backend_identity(&empty, &url).expect("marker");
    let output = run(
        &[
            "cache",
            "prune",
            empty.to_str().expect("folder"),
            "--answered-by-other-than",
            "jev-latest",
        ],
        &[],
    )
    .expect("prune");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "removed 0 entries and 0 bytes; 0 entries and 0 bytes remain\n"
    );
}

#[test]
fn prune_refuses_digest_mismatch_blank_model_and_nonregular_entries_before_deletion() {
    let url = format!("{DEFAULT_BASE}/{ENDPOINT_PATH}");
    for (case, malformed) in ["mismatch", "blank-model", "directory"]
        .into_iter()
        .enumerate()
    {
        let folder = folder(malformed);
        let keep = plant(&folder, ANSWERED).expect("removable entry");
        match case {
            0 => {
                let planted = plant_recording(
                    &folder,
                    &url,
                    &encoded_decide(EVIDENCE, DEFAULT_MODEL, "different"),
                    ANSWERED,
                )
                .expect("mismatched source");
                fs::rename(folder.join(planted), folder.join("0".repeat(64) + ".json"))
                    .expect("mismatched name");
            }
            1 => {
                let response = ANSWERED.replace("jev-1.13.0", " ");
                plant_recording(
                    &folder,
                    &url,
                    &encoded_decide(EVIDENCE, DEFAULT_MODEL, "blank model"),
                    &response,
                )
                .expect("blank-model entry");
            }
            _ => fs::create_dir(folder.join("0".repeat(64) + ".json"))
                .expect("digest-shaped directory"),
        }
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
        .expect("prune refusal");
        assert_eq!(output.status.code(), Some(5), "{malformed}");
        assert!(output.stdout.is_empty(), "{malformed}");
        assert!(folder.join(keep).exists(), "{malformed}");
    }
}

#[cfg(unix)]
#[test]
fn prune_refuses_a_digest_shaped_symlink_without_following_it() {
    use std::os::unix::fs::symlink;

    let folder = folder("cache-prune-symlink");
    let keep = plant(&folder, ANSWERED).expect("removable entry");
    let outside = folder.with_extension("outside");
    fs::write(&outside, "outside marker").expect("outside target");
    symlink(&outside, folder.join("0".repeat(64) + ".json")).expect("symlink");
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
    .expect("prune refusal");
    assert_eq!(output.status.code(), Some(5));
    assert!(folder.join(keep).exists());
    assert_eq!(
        fs::read_to_string(&outside).expect("outside target"),
        "outside marker"
    );
    fs::remove_file(outside).expect("outside cleanup");
}

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

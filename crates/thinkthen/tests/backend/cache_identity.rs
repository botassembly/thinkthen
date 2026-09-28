//! A recording folder stays bound to the backend that first wrote it.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::thread;

use crate::harness::{Canned, Listener, spawn_one as spawn};
use crate::support::{encoded_decide, plant_recording};

const QUESTION: &str = "asks for a refund";
const EVIDENCE: &str = "Refund me please.";
const ANSWER: &str = concat!(
    r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9}},"#,
    r#""usage":{"input_tokens":10,"output_tokens":2}}"#,
);
/// The refusal a folder the user named prints for this run's endpoint.
fn mismatch(url: &str) -> String {
    format!(
        "thinkthen: the recording folder is bound to a backend address other than `{url}`; \
         restore its backend settings or choose another folder\n"
    )
}
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
fn explicit_refresh_replaces_a_complete_cache_answer_and_replay_stays_offline() {
    let old = r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9}}}"#;
    let new = r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.1}}}"#;
    let listener = Listener::serving(vec![Canned::ok(old), Canned::ok(new)]).expect("listener");
    let cache = folder("explicit-refresh-complete-answer");
    let path = cache.to_str().expect("cache path");
    let ask = |extra: &[&str]| {
        let mut args = vec![
            "decide",
            QUESTION,
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--cache",
            path,
        ];
        args.extend_from_slice(extra);
        spawn(
            &args,
            &[("THINKTHEN_API_KEY", "sk-test-value")],
            EVIDENCE.as_bytes(),
        )
        .expect("command")
    };
    let first = ask(&[]);
    assert_eq!(first.status.code(), Some(0));
    let before = files(&cache).expect("old files");
    let refreshed = ask(&["--refresh-cache"]);
    assert_eq!(refreshed.status.code(), Some(1));
    let after = files(&cache).expect("new files");
    assert_eq!(before.len(), 2, "marker and one entry");
    assert_eq!(after.len(), 2, "same marker and one entry");
    fn selected(items: &FolderFiles, marker: bool) -> &(std::ffi::OsString, Vec<u8>) {
        items
            .iter()
            .find(|(name, _)| (name == ".thinkthen-backend.json") == marker)
            .expect("marker or digest entry")
    }
    assert_eq!(
        selected(&before, true),
        selected(&after, true),
        "marker stays unchanged"
    );
    assert_eq!(
        selected(&before, false).0,
        selected(&after, false).0,
        "same digest name"
    );
    assert_ne!(
        selected(&before, false).1,
        selected(&after, false).1,
        "answer replaced"
    );
    assert!(
        String::from_utf8_lossy(&refreshed.stderr)
            .contains("sends each planned cache request live")
    );
    let replay = spawn(
        &[
            "decide",
            QUESTION,
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--replay",
            path,
        ],
        &[],
        EVIDENCE.as_bytes(),
    )
    .expect("offline replay");
    assert_eq!(replay.status.code(), Some(1));
    assert_eq!(listener.requests().len(), 2, "replay sends nothing");
}

#[test]
fn a_missing_key_leaves_an_unbound_folder_for_the_next_address() {
    const NO_KEY: &str = "thinkthen: the environment variable `THINKTHEN_API_KEY` is unset or blank, so no key is sent\n";
    let second = Listener::answering(|_| Canned::ok(ANSWER)).expect("listener");
    for (row, named, present) in [
        ("default-absent", false, false),
        ("cache-absent", true, false),
        ("cache-empty", true, true),
    ] {
        let home = folder(&format!("no-key-{row}-home"));
        let cache = if named {
            folder(&format!("no-key-{row}-cache"))
        } else {
            default_cache(&home)
        };
        if present {
            fs::create_dir_all(&cache).expect("empty folder");
        }
        let before = if present {
            Some(fs::read_dir(&cache).expect("folder").count())
        } else {
            None
        };
        let mut arguments = vec![
            "decide",
            QUESTION,
            "--url",
            "https://example.invalid/v1",
            "--model",
            "local-1",
        ];
        let cache_text = cache.to_str().expect("cache path");
        if named {
            arguments.extend(["--cache", cache_text]);
        }
        let home_text = home.to_str().expect("home path");
        let failed = spawn(&arguments, &[("HOME", home_text)], EVIDENCE.as_bytes())
            .expect("missing-key run");
        assert_eq!(failed.status.code(), Some(4), "{row}");
        assert!(failed.stdout.is_empty(), "{row}");
        assert_eq!(String::from_utf8_lossy(&failed.stderr), NO_KEY, "{row}");
        assert_eq!(
            cache.exists(),
            present,
            "{row}: the failed run changed folder presence"
        );
        if let Some(count) = before {
            assert_eq!(
                fs::read_dir(&cache).expect("folder").count(),
                count,
                "{row}"
            );
        }

        arguments[3] = second.base();
        let answered = spawn(
            &arguments,
            &[("HOME", home_text), ("THINKTHEN_API_KEY", "sk-test-value")],
            EVIDENCE.as_bytes(),
        )
        .expect("second-address run");
        assert_eq!(answered.status.code(), Some(0), "{row}");
        assert!(cache.join(".thinkthen-backend.json").is_file(), "{row}");
    }
    assert_eq!(second.requests().len(), 3);
}

#[cfg(unix)]
#[test]
fn a_dangling_cache_path_is_storage_failure_before_key_lookup() {
    use std::os::unix::fs::symlink;

    let cache = folder("dangling-cache-admission");
    let _absent = fs::remove_file(&cache);
    symlink(cache.with_extension("missing"), &cache).expect("dangling cache link");
    let refused =
        decide("https://example.invalid/v1", &cache, "--cache", false).expect("dangling-path run");
    assert_eq!(refused.status.code(), Some(5));
    assert_eq!(
        String::from_utf8_lossy(&refused.stderr),
        "thinkthen: the recording folder could not be read or written; check its permissions and free space\n"
    );
    assert!(refused.stdout.is_empty());
    assert!(fs::symlink_metadata(&cache).is_ok());
}

#[test]
fn a_mismatch_names_the_folder_and_the_address() {
    let first = Listener::answering(|_| Canned::ok(ANSWER)).expect("first listener");
    let second = Listener::answering(|_| Canned::ok(ANSWER)).expect("second listener");
    let default_sentence = format!(
        "thinkthen: the default cache is bound to a backend address other than `{}`; \
         go back to that address, use --no-cache, or set THINKTHEN_CACHE to another folder\n",
        second.url()
    );
    let named_sentence = mismatch(second.url());
    // Row: the option that names the folder, whether THINKTHEN_CACHE names it,
    // and the sentence. `--replay` is filled through `--cache`, since a
    // replay alone writes nothing.
    for (row, option, environment_names, expected) in [
        ("default", None, false, &default_sentence),
        ("environment", None, true, &named_sentence),
        ("cache", Some("--cache"), false, &named_sentence),
        ("record", Some("--record"), false, &named_sentence),
        ("replay", Some("--replay"), false, &named_sentence),
    ] {
        let home = folder(&format!("mismatch-{row}-home"));
        let named = folder(&format!("mismatch-{row}-folder"));
        let chosen = if option.is_some() || environment_names {
            named.clone()
        } else {
            default_cache(&home)
        };
        let named_text = named.to_str().expect("folder");
        let home_text = home.to_str().expect("home");
        let run = |base: &str, option: Option<&str>, extra: &[&str], key: bool| {
            let mut arguments = vec!["decide", QUESTION, "--url", base, "--model", "local-1"];
            if let Some(option) = option {
                arguments.extend([option, named_text]);
            }
            arguments.extend_from_slice(extra);
            let mut environment = vec![("HOME", home_text)];
            if environment_names {
                environment.push(("THINKTHEN_CACHE", named_text));
            }
            if key {
                environment.push(("THINKTHEN_API_KEY", "sk-test-value"));
            }
            spawn(&arguments, &environment, EVIDENCE.as_bytes()).expect("run")
        };
        let fill = if option == Some("--replay") {
            Some("--cache")
        } else {
            option
        };
        assert_eq!(
            run(first.base(), fill, &[], true).status.code(),
            Some(0),
            "{row}"
        );
        let before = files(&chosen).expect("folder files");

        let refused = run(second.base(), option, &[], false);
        assert_eq!(refused.status.code(), Some(5), "{row}");
        assert!(refused.stdout.is_empty(), "{row}");
        assert_eq!(
            String::from_utf8_lossy(&refused.stderr),
            expected.as_str(),
            "{row}"
        );
        assert!(second.requests().is_empty(), "{row}");
        assert_eq!(files(&chosen).expect("folder files"), before, "{row}");

        if row == "default" {
            let stepped = run(second.base(), None, &["--no-cache"], true);
            assert_eq!(stepped.status.code(), Some(0));
            assert_eq!(second.requests().len(), 1);
            assert_eq!(files(&chosen).expect("folder files"), before);
        }
    }
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
        [
            (first_output.stderr, first.url()),
            (second_output.stderr, second.url()),
        ]
        .iter()
        .filter(|(bytes, url)| bytes.as_slice() == mismatch(url).as_bytes())
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
    assert!(message.starts_with(
        "thinkthen: the decide request for one document: the replay folder holds no entry named `"
    ));
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

    let refused = decide(listener.base(), &recording, "--cache", false).expect("refusal runs");
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

    let refused = decide(listener.base(), &cache, "--cache", false).expect("refusal runs");
    assert_eq!(refused.status.code(), Some(5));
    assert_eq!(
        String::from_utf8_lossy(&refused.stderr),
        "thinkthen: the recording folder could not be read or written; check its permissions and free space\n"
    );
    assert!(listener.requests().is_empty());
    assert!(!String::from_utf8_lossy(&refused.stderr).contains("private evidence"));
}

mod default;

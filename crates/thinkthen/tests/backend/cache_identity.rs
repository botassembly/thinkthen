//! A cache folder holds answers from any address, because the address is in
//! every question key, by ADR 0111 section 3. No folder binds to one backend.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::thread;

use crate::harness::{Canned, Listener, spawn_one as spawn};
use crate::support::{encoded_decide, plant_recording, stored};

const QUESTION: &str = "asks for a refund";
const EVIDENCE: &str = "Refund me please.";
const ANSWER: &str = concat!(
    r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9}},"#,
    r#""usage":{"input_tokens":10,"output_tokens":2}}"#,
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
    } else if cfg!(windows) {
        // The test home sets LOCALAPPDATA to `AppData/Local` under it.
        home.join("AppData/Local/thinkthen/cache")
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
    let before = stored(&cache).expect("the first answer");
    let refreshed = ask(&["--refresh-cache"]);
    assert_eq!(refreshed.status.code(), Some(1));
    let after = stored(&cache).expect("the refreshed answer");
    assert_eq!((before.len(), after.len()), (1, 1), "one answer each time");
    assert_eq!(before[0]["key"], after[0]["key"], "the same question key");
    assert_eq!(before[0]["answer"], r#"{"type":"noul","noul":0.9}"#);
    assert_eq!(
        after[0]["answer"], r#"{"type":"noul","noul":0.1}"#,
        "answer replaced"
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
fn a_missing_key_writes_nothing_and_the_next_address_fills_the_folder() {
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
        assert!(cache.join("thinkthen.sqlite").is_file(), "{row}");
    }
    assert_eq!(second.requests().len(), 3);
}

#[test]
fn a_control_character_key_writes_nothing_and_a_hit_reads_no_key() {
    const REFUSAL: &str = "thinkthen: the API key contains a control character\n";
    let first = Listener::answering(|_| Canned::ok(ANSWER)).expect("first listener");
    let second = Listener::answering(|_| Canned::ok(ANSWER)).expect("second listener");
    for (row, named, present, key) in [
        ("default-lf", false, false, "first\nsecond"),
        ("named-cr", true, true, "first\rsecond"),
        ("default-esc", false, false, "first\u{1b}second"),
        ("named-del", true, true, "first\u{7f}second"),
    ] {
        let home = folder(&format!("control-character-{row}-home"));
        let cache = if named {
            folder(&format!("control-character-{row}-cache"))
        } else {
            default_cache(&home)
        };
        if present {
            fs::create_dir_all(&cache).expect("empty folder");
        }
        let mut arguments = vec![
            "decide",
            QUESTION,
            "--url",
            first.base(),
            "--model",
            "local-1",
        ];
        let cache_text = cache.to_str().expect("cache path");
        if named {
            arguments.extend(["--cache", cache_text]);
        }
        let home_text = home.to_str().expect("home path");
        let failed = spawn(
            &arguments,
            &[("HOME", home_text), ("THINKTHEN_API_KEY", key)],
            EVIDENCE.as_bytes(),
        )
        .expect("control-character-key run");
        assert_eq!(failed.status.code(), Some(2), "{row}");
        assert!(failed.stdout.is_empty(), "{row}");
        assert_eq!(String::from_utf8_lossy(&failed.stderr), REFUSAL, "{row}");
        assert_eq!(cache.exists(), present, "{row}: folder presence");
        if present {
            assert_eq!(fs::read_dir(&cache).expect("folder").count(), 0, "{row}");
        }
        arguments[3] = second.base();
        let answered = spawn(
            &arguments,
            &[("HOME", home_text), ("THINKTHEN_API_KEY", "sk-test-value")],
            EVIDENCE.as_bytes(),
        )
        .expect("second-address run");
        assert_eq!(answered.status.code(), Some(0), "{row}");
        assert!(cache.join("thinkthen.sqlite").is_file(), "{row}");
        let hit = spawn(
            &arguments,
            &[("HOME", home_text), ("THINKTHEN_API_KEY", key)],
            EVIDENCE.as_bytes(),
        )
        .expect("bound hit with control-character key");
        assert_eq!(hit.status.code(), Some(0), "{row}");
    }
    assert_eq!(first.requests().len(), 0);
    assert_eq!(second.requests().len(), 4);
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
fn a_cache_pointed_at_a_new_address_resends_and_keeps_both_answers() {
    let first = Listener::answering(|_| Canned::ok(ANSWER)).expect("first listener");
    let second = Listener::answering(|_| Canned::ok(ANSWER)).expect("second listener");
    // Row: the option that names the folder, and whether THINKTHEN_CACHE names it.
    for (row, option, environment_names) in [
        ("default", None, false),
        ("environment", None, true),
        ("cache", Some("--cache"), false),
        ("record", Some("--record"), false),
    ] {
        let home = folder(&format!("new-address-{row}-home"));
        let named = folder(&format!("new-address-{row}-folder"));
        let chosen = if option.is_some() || environment_names {
            named.clone()
        } else {
            default_cache(&home)
        };
        let named_text = named.to_str().expect("folder");
        let home_text = home.to_str().expect("home");
        let run = |base: &str| {
            let mut arguments = vec!["decide", QUESTION, "--url", base, "--model", "local-1"];
            if let Some(option) = option {
                arguments.extend([option, named_text]);
            }
            let mut environment = vec![("HOME", home_text), ("THINKTHEN_API_KEY", "sk-test-value")];
            if environment_names {
                environment.push(("THINKTHEN_CACHE", named_text));
            }
            spawn(&arguments, &environment, EVIDENCE.as_bytes()).expect("run")
        };
        let sent = second.requests().len();
        assert_eq!(run(first.base()).status.code(), Some(0), "{row}");
        let answered = run(second.base());
        assert_eq!(answered.status.code(), Some(0), "{row}");
        assert!(answered.stderr.is_empty(), "{row}");
        assert_eq!(
            second.requests().len(),
            sent + 1,
            "{row}: the new address was asked"
        );
        let urls: Vec<_> = stored(&chosen)
            .expect("the store")
            .iter()
            .map(|answer| answer["url"].as_str().unwrap_or_default().to_owned())
            .collect();
        let mut expected = vec![first.url().to_owned(), second.url().to_owned()];
        let mut urls = urls;
        urls.sort();
        expected.sort();
        assert_eq!(urls, expected, "{row}");
    }
}

#[test]
fn two_processes_write_one_store_at_once_and_both_succeed() {
    let cache = folder("cache-two-writers");
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

    for output in [&first_output, &second_output] {
        assert_eq!(
            output.status.code(),
            Some(0),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    assert_eq!((first.requests().len(), second.requests().len()), (1, 1));
    assert_eq!(stored(&cache).expect("the store").len(), 2);
}

#[test]
fn normalized_address_spellings_share_one_key_and_models_do_not() {
    let cache = folder("cache-backend-normalized-and-models");
    let listener = Listener::answering(|_| Canned::ok(ANSWER)).expect("listener");
    let first_base = listener.base().replacen("http://", "HTTP://", 1);
    for (base, model) in [
        (format!("{first_base}/"), "local-1"),
        (listener.base().to_owned(), "local-1"),
        (listener.base().to_owned(), "local-2"),
    ] {
        let output = decide_model(&base, &cache, "--record", true, model).expect("run");
        assert_eq!(output.status.code(), Some(0));
    }
    assert_eq!(listener.requests().len(), 3);
    let answers = stored(&cache).expect("the store");
    assert_eq!(answers.len(), 2, "the two spellings replace one answer");
    assert!(answers.iter().all(|answer| answer["url"] == listener.url()));
}

#[test]
fn an_old_entry_is_ignored_until_convert_and_a_replay_writes_nothing() {
    let recording = folder("legacy-replay");
    let base = "http://127.0.0.1:1/v1";
    let url = format!("{base}/systemone");
    let request = encoded_decide(EVIDENCE, "local-1", QUESTION);
    let name = plant_recording(&recording, &url, &request, ANSWER).expect("legacy entry");
    let before = files(&recording).expect("old folder");

    let missed = decide(base, &recording, "--replay", false).expect("miss runs");
    assert_eq!(missed.status.code(), Some(5));
    let message = String::from_utf8_lossy(&missed.stderr);
    assert!(message.starts_with(
        "thinkthen: the decide request for one document: the replay folder holds no answer for question `"
    ));
    assert_eq!(files(&recording).expect("old folder"), before);

    let converted = spawn(
        &["cache", "convert", &recording.to_string_lossy()],
        &[],
        b"",
    )
    .expect("convert runs");
    assert_eq!(converted.status.code(), Some(0));
    let replayed = decide(base, &recording, "--replay", false).expect("replay runs");
    assert_eq!(replayed.status.code(), Some(0));
    assert!(recording.join(&name).is_file(), "the old entry stays");
}

#[test]
fn a_cache_over_an_old_folder_resends_and_keeps_its_old_entry() {
    let recording = folder("legacy-write");
    let request = encoded_decide(EVIDENCE, "local-1", QUESTION);
    let listener = Listener::answering(|_| Canned::ok(ANSWER)).expect("listener");
    let name = plant_recording(&recording, listener.url(), &request, ANSWER).expect("legacy entry");
    let old = fs::read(recording.join(&name)).expect("old entry");

    let answered = decide(listener.base(), &recording, "--cache", true).expect("cache runs");
    assert_eq!(answered.status.code(), Some(0));
    assert_eq!(listener.requests().len(), 1, "an old cache is not read");
    assert_eq!(stored(&recording).expect("the store").len(), 1);
    assert_eq!(fs::read(recording.join(&name)).expect("old entry"), old);
}

#[test]
fn a_store_that_is_no_database_is_a_secret_safe_storage_failure() {
    let cache = folder("malformed-cache-store");
    fs::create_dir_all(&cache).expect("cache folder");
    fs::write(
        cache.join("thinkthen.sqlite"),
        b"private evidence that is not a database, padded well past one header of bytes",
    )
    .expect("store");
    let listener = Listener::answering(|_| Canned::ok(ANSWER)).expect("listener");

    let refused = decide(listener.base(), &cache, "--cache", false).expect("refusal runs");
    assert_eq!(refused.status.code(), Some(5));
    assert_eq!(
        String::from_utf8_lossy(&refused.stderr),
        "thinkthen: the recording folder could not be read or written; check its permissions and free space\n"
    );
    assert!(listener.requests().is_empty());
}

mod default;

use super::*;

#[test]
fn a_live_reply_counts_valid_usage_when_its_only_answer_is_refused() {
    let root = folder("refused-answer-usage");
    let refused = concat!(
        r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"choice","probabilities":{"a":1.0}}},"#,
        r#""usage":{"input_tokens":17,"output_tokens":3}}"#,
    );
    let listener = Listener::serving(vec![Canned::ok(refused)]).expect("listener");
    let environment = [
        ("THINKTHEN_BASE_URL", listener.base()),
        ("THINKTHEN_API_KEY", "secret-key"),
        ("XDG_STATE_HOME", root.to_str().expect("state root")),
    ];
    let refused =
        run(&["decide", "asks for a refund", "--no-cache"], &environment).expect("refused answer");
    assert_eq!(refused.status.code(), Some(4));
    let status = run(&["status", "--json"], &environment).expect("status");
    let value: serde_json::Value = serde_json::from_slice(&status.stdout).expect("status JSON");
    assert_eq!(value["usage"]["this_month"]["requests_sent"], 1);
    assert_eq!(value["usage"]["this_month"]["input_tokens"], 17);
    assert_eq!(value["usage"]["this_month"]["output_tokens"], 3);
}

#[test]
fn retries_terminal_failures_and_explicit_replay_have_the_ruled_counts() {
    let root = folder("retry-failure-replay-usage");
    let listener = Listener::serving(vec![
        Canned::status(500, "{}"),
        Canned::ok(ANSWERED),
        Canned::status(422, "{}"),
    ])
    .expect("listener");
    let base = listener.base().to_owned();
    let environment = [
        ("THINKTHEN_BASE_URL", base.as_str()),
        ("THINKTHEN_API_KEY", "secret-key"),
        ("XDG_STATE_HOME", root.to_str().expect("state root")),
    ];
    let retried = run(
        &["decide", "retry", "--no-cache", "--details"],
        &environment,
    )
    .expect("retry");
    assert_eq!(retried.status.code(), Some(0));
    let details = String::from_utf8_lossy(&retried.stdout);
    assert!(
        details.contains(r#""requests_sent":2,"cached":false"#),
        "{details}"
    );
    assert_eq!(
        run(
            &["decide", "failure", "--no-cache", "--max-retries", "0"],
            &environment,
        )
        .expect("failure")
        .status
        .code(),
        Some(4)
    );
    let replay = root.join("explicit-replay");
    plant_fixture(
        &replay,
        &url(),
        &body(),
        &[r#"{"type":"noul","noul":0.92}"#],
        None,
    )
    .expect("fixture");
    let replay_environment = [("XDG_STATE_HOME", root.to_str().expect("state root"))];
    assert_eq!(
        run(
            &[
                "decide",
                "asks for a refund",
                "--replay",
                replay.to_str().expect("replay"),
            ],
            &replay_environment,
        )
        .expect("replay")
        .status
        .code(),
        Some(0)
    );
    let status = run(&["status", "--json"], &environment).expect("status");
    assert_eq!(usage(&status, "requests_sent"), Some(3));
    assert_eq!(usage(&status, "input_tokens"), Some(312));
    assert_eq!(usage(&status, "output_tokens"), Some(48));
    assert_eq!(usage(&status, "cache_answers"), Some(0));
}

#[test]
fn two_processes_update_one_month_without_losing_a_cache_answer() {
    let root = folder("two-process-usage");
    let cache = root.join("answers");
    let cache_name = cache.to_string_lossy().into_owned();
    let root_name = root.to_string_lossy().into_owned();
    // The fill counts its usage under another root, so this month holds
    // only the two cache runs.
    let fill_root = folder("two-process-usage-fill");
    let listener = Listener::serving(vec![Canned::ok(ANSWERED)]).expect("listener");
    let base = listener.base().to_owned();
    let filled = run(
        &["decide", "asks for a refund", "--record", &cache_name],
        &[
            ("THINKTHEN_BASE_URL", &base),
            ("THINKTHEN_API_KEY", "secret-key"),
            ("XDG_STATE_HOME", fill_root.to_str().expect("fill root")),
        ],
    )
    .expect("fill");
    assert_eq!(filled.status.code(), Some(0));
    assert_eq!(listener.requests().len(), 1);
    let environment = [
        ("XDG_STATE_HOME", root_name.as_str()),
        ("THINKTHEN_BASE_URL", &base),
    ];
    thread::scope(|scope| {
        let first = scope.spawn(|| {
            run(
                &["decide", "asks for a refund", "--cache", &cache_name],
                &environment,
            )
            .expect("first")
        });
        let second = scope.spawn(|| {
            run(
                &["decide", "asks for a refund", "--cache", &cache_name],
                &environment,
            )
            .expect("second")
        });
        assert_eq!(first.join().expect("first thread").status.code(), Some(0));
        assert_eq!(second.join().expect("second thread").status.code(), Some(0));
    });
    let status = run(&["status", "--json"], &[("XDG_STATE_HOME", &root_name)]).expect("status");
    assert_eq!(usage(&status, "cache_answers"), Some(2));
    assert_eq!(usage(&status, "requests_sent"), Some(0));
    assert_eq!(listener.requests().len(), 0);
}

#[test]
fn packed_annotate_counts_one_exchange_and_one_usage_object() {
    let root = folder("packed-annotate-usage");
    let set = root.join("questions.json");
    fs::create_dir_all(&root).expect("root");
    fs::write(
        &set,
        r#"{"version":1,"questions":{"a":{"decide":"First?"},"b":{"decide":"Second?"}}}"#,
    )
    .expect("set");
    let answer = r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.8}},"usage":{"input_tokens":20,"output_tokens":4}}"#;
    let listener = Listener::serving(vec![Canned::ok(answer)]).expect("listener");
    let base = listener.base().to_owned();
    let environment = [
        ("THINKTHEN_BASE_URL", base.as_str()),
        ("THINKTHEN_API_KEY", "secret-key"),
        ("XDG_STATE_HOME", root.to_str().expect("state root")),
    ];
    assert_eq!(
        run(
            &["annotate", set.to_str().expect("set"), "--no-cache"],
            &environment,
        )
        .expect("annotate")
        .status
        .code(),
        Some(0)
    );
    assert_eq!(listener.requests().len(), 1);
    let status = run(&["status", "--json"], &environment).expect("status");
    assert_eq!(usage(&status, "requests_sent"), Some(1));
    assert_eq!(usage(&status, "input_tokens"), Some(20));
    assert_eq!(usage(&status, "output_tokens"), Some(4));
    let persisted = fs::read_dir(root.join("thinkthen"))
        .expect("usage folder")
        .filter_map(Result::ok)
        .filter(|entry| {
            entry
                .path()
                .extension()
                .is_some_and(|extension| extension == "json")
        })
        .map(|entry| fs::read_to_string(entry.path()).expect("usage row"))
        .collect::<String>();
    assert!(!persisted.contains("secret-key"));
    assert!(!persisted.contains("First?"));
    assert!(!persisted.contains("Second?"));
}

/// A usage folder thinkthen cannot read refused nothing on main: the run
/// answered and stopped counting. Now the first send refuses, the whole run
/// ends with exit 5 before any request, and no row fails alone (ticket 0360).
#[cfg(unix)]
fn refused_before_any_send(root: &std::path::Path, key: &str) -> (std::process::Output, usize) {
    let listener = Listener::serving(vec![Canned::ok(ANSWERED)]).expect("listener");
    let records: String = (1..=6)
        .map(|place| format!("{{\"body\":\"record {place}\"}}\n"))
        .collect();
    let output = crate::harness::spawn(
        &[
            "decide",
            "asks for a refund",
            "--jsonl",
            "--field",
            "/body",
            "--jobs",
            "4",
            "--batch",
            "1",
            "--no-cache",
        ],
        &[
            ("THINKTHEN_BASE_URL", listener.base()),
            ("THINKTHEN_API_KEY", key),
            ("XDG_STATE_HOME", root.to_str().expect("state root")),
        ],
        records.as_bytes(),
    )
    .expect("judgment");
    (output, listener.requests().len())
}

#[cfg(unix)]
#[test]
fn an_unsafe_usage_folder_refuses_the_run_before_any_send() {
    use std::os::unix::fs::PermissionsExt as _;

    let root = folder("usage-unsafe-folder");
    let usage_folder = root.join("thinkthen");
    fs::create_dir_all(&usage_folder).expect("usage folder");
    fs::set_permissions(&usage_folder, fs::Permissions::from_mode(0o755)).expect("unsafe mode");
    let (output, requests) = refused_before_any_send(&root, "secret-warning-key");
    assert_eq!((output.status.code(), requests), (Some(5), 0));
    assert_eq!(output.stdout, b"");
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "thinkthen: cannot read the usage totals: the usage folder that thinkthen status names has unsafe or unreadable state. Make it private to your user (folder 0700, files 0600), or move it aside.\nthinkthen: stopped at record 1; 0 records finished\n"
    );
}

#[cfg(unix)]
#[test]
fn a_malformed_month_refuses_the_run_and_keeps_its_bytes() {
    use std::os::unix::fs::PermissionsExt as _;

    let root = folder("zero-byte-usage-month");
    let usage_folder = root.join("thinkthen");
    fs::create_dir_all(&usage_folder).expect("usage folder");
    fs::set_permissions(&usage_folder, fs::Permissions::from_mode(0o700)).expect("private folder");
    let lock = usage_folder.join(".lock");
    let month = usage_folder.join("2026-08.json");
    fs::write(&lock, b"").expect("stable lock");
    fs::write(&month, b"").expect("interrupted month");
    for file in [&lock, &month] {
        fs::set_permissions(file, fs::Permissions::from_mode(0o600)).expect("private file");
    }
    let (output, requests) = refused_before_any_send(&root, "secret-zero-byte-key");
    assert_eq!((output.status.code(), requests), (Some(5), 0));
    assert_eq!(output.stdout, b"");
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "thinkthen: cannot read the usage totals: 2026-08.json has invalid contents. Move it out of the usage folder that thinkthen status names, and counting starts again.\nthinkthen: stopped at record 1; 0 records finished\n"
    );
    assert_eq!(fs::read(&month).expect("unchanged month"), b"");
    let planned = crate::harness::spawn(
        &["decide", "asks for a refund", "--plan"],
        &[("XDG_STATE_HOME", root.to_str().expect("state root"))],
        b"evidence",
    )
    .expect("plan");
    assert_eq!(
        planned.status.code(),
        Some(0),
        "a plan sends nothing, so it never refuses"
    );
}

/// Another process holding the usage lock stops no request: all 16 jobs reach
/// the listener while the lock is held, and the totals land once it lets go.
#[cfg(unix)]
#[test]
fn requests_go_out_while_another_process_holds_the_usage_lock() {
    use std::os::unix::fs::{OpenOptionsExt as _, PermissionsExt as _};
    use std::time::{Duration, Instant};

    use crate::harness::{Gathering, finish, start};

    let root = folder("usage-lock-held");
    let usage_folder = root.join("thinkthen");
    fs::create_dir_all(&usage_folder).expect("usage folder");
    fs::set_permissions(&usage_folder, fs::Permissions::from_mode(0o700)).expect("private");
    let lock = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(usage_folder.join(".lock"))
        .expect("usage lock");
    lock.lock()
        .expect("the lock, held as a second process would");
    let gathering = Gathering::new(16);
    let listener = Listener::answering(move |_| {
        gathering.hold();
        Canned::ok(concat!(
            r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.92}},"#,
            r#""usage":{"input_tokens":88,"output_tokens":12}}"#,
        ))
    })
    .expect("listener");
    let records: String = (1..=16)
        .map(|place| format!("{{\"body\":\"record {place}\"}}\n"))
        .collect();
    let environment = [
        ("THINKTHEN_BASE_URL", listener.base()),
        ("THINKTHEN_API_KEY", "secret-key"),
        ("XDG_STATE_HOME", root.to_str().expect("state root")),
    ];
    let arguments = ["decide", "asks for a refund", "--jsonl", "--field", "/body"];
    let arguments = [
        &arguments[..],
        &["--jobs", "16", "--batch", "1", "--no-cache"],
    ]
    .concat();
    let mut child =
        start(&arguments, &environment, records.as_bytes()).expect("the command starts");

    let arrived = Instant::now() + Duration::from_secs(30);
    while listener.count() < 16 && Instant::now() < arrived {
        thread::sleep(Duration::from_millis(10));
    }
    let (count, peak) = (listener.count(), listener.peak());
    let settled = Instant::now() + Duration::from_millis(500);
    let mut exited = child.try_wait().expect("the child's state").is_some();
    while !exited && Instant::now() < settled {
        thread::sleep(Duration::from_millis(10));
        exited = child.try_wait().expect("the child's state").is_some();
    }
    drop(lock);
    let output = finish(child, "decide --jobs 16").expect("the command ends");

    assert_eq!((count, peak), (16, 16), "in flight while the lock was held");
    assert!(!exited, "the command exited before its totals were written");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(String::from_utf8_lossy(&output.stdout).lines().count(), 16);
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    let status = run(&["status", "--json"], &environment).expect("status");
    assert_eq!(usage(&status, "requests_sent"), Some(16));
    assert_eq!(usage(&status, "input_tokens"), Some(1408));
    assert_eq!(usage(&status, "output_tokens"), Some(192));
    assert_eq!(usage(&status, "cache_answers"), Some(0));
}

/// The totals are state, not cache: removing the cache home keeps them, and
/// one month file holds all five counts with no `retries-` file (ticket 0360).
#[test]
fn removing_the_cache_home_keeps_the_count_in_one_month_file() {
    let root = folder("usage-survives-the-cache");
    let (cache, state) = (root.join("cache"), root.join("state"));
    let listener = Listener::serving(vec![Canned::ok(ANSWERED)]).expect("listener");
    let environment = [
        ("THINKTHEN_BASE_URL", listener.base()),
        ("THINKTHEN_API_KEY", "secret-key"),
        ("XDG_CACHE_HOME", cache.to_str().expect("cache root")),
        ("XDG_STATE_HOME", state.to_str().expect("state root")),
    ];
    let output = run(&["decide", "asks for a refund"], &environment).expect("run");
    assert_eq!(output.status.code(), Some(0));
    assert!(cache.join("thinkthen").is_dir());
    fs::remove_dir_all(&cache).expect("the cache cleared");
    let status = run(&["status", "--json"], &environment).expect("status");
    assert_eq!(usage(&status, "requests_sent"), Some(1));
    assert_eq!(usage(&status, "input_tokens"), Some(312));
    let mut names: Vec<String> = fs::read_dir(state.join("thinkthen"))
        .expect("usage folder")
        .map(|entry| {
            entry
                .expect("entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    names.sort();
    assert_eq!(names.len(), 2, "{names:?}");
    assert_eq!(names[0], ".lock");
    let row: serde_json::Value =
        serde_json::from_slice(&fs::read(state.join("thinkthen").join(&names[1])).expect("month"))
            .expect("month JSON");
    assert_eq!(
        row.as_object()
            .expect("object")
            .keys()
            .cloned()
            .collect::<Vec<_>>(),
        [
            "cache_answers",
            "input_tokens",
            "output_tokens",
            "requests_sent",
            "retries",
            "schema"
        ]
    );
}

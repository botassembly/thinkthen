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
        ("XDG_CACHE_HOME", root.to_str().expect("cache root")),
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
        ("XDG_CACHE_HOME", root.to_str().expect("cache root")),
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
    plant(&replay, ANSWERED).expect("recording");
    let replay_environment = [("XDG_CACHE_HOME", root.to_str().expect("cache root"))];
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
    plant(&cache, ANSWERED).expect("recording");
    let cache_name = cache.to_string_lossy().into_owned();
    let root_name = root.to_string_lossy().into_owned();
    thread::scope(|scope| {
        let first = scope.spawn(|| {
            run(
                &["decide", "asks for a refund", "--cache", &cache_name],
                &[("XDG_CACHE_HOME", &root_name)],
            )
            .expect("first")
        });
        let second = scope.spawn(|| {
            run(
                &["decide", "asks for a refund", "--cache", &cache_name],
                &[("XDG_CACHE_HOME", &root_name)],
            )
            .expect("second")
        });
        assert_eq!(first.join().expect("first thread").status.code(), Some(0));
        assert_eq!(second.join().expect("second thread").status.code(), Some(0));
    });
    let status = run(&["status", "--json"], &[("XDG_CACHE_HOME", &root_name)]).expect("status");
    assert_eq!(usage(&status, "cache_answers"), Some(2));
    assert_eq!(usage(&status, "requests_sent"), Some(0));
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
        ("XDG_CACHE_HOME", root.to_str().expect("cache root")),
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
    let persisted = fs::read_dir(root.join("thinkthen-usage"))
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

#[cfg(unix)]
#[test]
fn persistence_failure_warns_once_after_the_unchanged_judgment() {
    use std::os::unix::fs::PermissionsExt as _;

    let root = folder("usage-warning");
    let usage_folder = root.join("thinkthen-usage");
    fs::create_dir_all(&usage_folder).expect("usage folder");
    fs::set_permissions(&usage_folder, fs::Permissions::from_mode(0o755)).expect("unsafe mode");
    let listener = Listener::serving(vec![Canned::ok(ANSWERED)]).expect("listener");
    let base = listener.base().to_owned();
    let output = run(
        &["decide", "asks for a refund", "--no-cache"],
        &[
            ("THINKTHEN_BASE_URL", base.as_str()),
            ("THINKTHEN_API_KEY", "secret-warning-key"),
            ("XDG_CACHE_HOME", root.to_str().expect("cache root")),
        ],
    )
    .expect("judgment");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(String::from_utf8_lossy(&output.stdout), "true\n");
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "thinkthen: usage counters could not be updated; check the usage folder permissions and free space\n"
    );
    assert!(!String::from_utf8_lossy(&output.stderr).contains("secret-warning-key"));
}

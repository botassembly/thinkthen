//! The compiled command's one run line counts work hidden by row output.

use std::sync::atomic::{AtomicUsize, Ordering};
#[cfg(unix)]
use std::{fs, path::Path};

use serde_json::{Map, Value, json};

use crate::batching::{self, KEY, QUESTION};
use crate::harness::{Canned, Listener, spawn};

// Its cases name the XDG folders. Windows reads APPDATA and LOCALAPPDATA (sdlc/planning/windows.md).
#[cfg(unix)]
mod priced;
#[cfg(unix)]
mod usage_lock;

fn answered(body: &[u8]) -> Canned {
    let answers: Map<String, Value> = batching::places(body)
        .into_iter()
        .map(|(name, at)| {
            (
                name,
                json!({"type":"noul", "noul": if at <= 11 { 0.9 } else { 0.1 }}),
            )
        })
        .collect();
    Canned::ok(
        &json!({"model":"jev-1.13.0", "answers":answers,
            "usage":{"input_tokens":100,"output_tokens":10}})
        .to_string(),
    )
}

fn line(output: &std::process::Output) -> Value {
    let stderr = String::from_utf8_lossy(&output.stderr);
    let last = stderr.lines().last().expect("run facts line");
    let facts: Value = serde_json::from_str(last).expect("valid run facts");
    assert_eq!(facts["schema"], "thinkthen.run/1");
    assert!(
        facts["seconds"]
            .as_f64()
            .is_some_and(|seconds| seconds >= 0.0)
    );
    facts
}

// It names the XDG folders. Windows reads APPDATA and LOCALAPPDATA (sdlc/planning/windows.md).
#[cfg(unix)]
#[test]
fn the_facts_line_counts_filtered_records_and_matches_status() {
    let home = Path::new(env!("CARGO_TARGET_TMPDIR")).join("facts-filter-home");
    let _removed = fs::remove_dir_all(&home);
    fs::create_dir_all(&home).expect("private home");
    let home = home.to_str().expect("UTF-8 home");
    let listener = Listener::answering(answered).expect("loopback");
    let input = batching::lines(1..=25);
    let arguments = [
        "filter",
        QUESTION,
        "--lines",
        "--batch",
        "10",
        "--facts",
        "--no-cache",
        "--url",
        listener.base(),
    ];
    let output = spawn(
        &arguments,
        &[KEY, ("XDG_STATE_HOME", home)],
        input.as_bytes(),
    )
    .expect("compiled filter");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        output.stdout.iter().filter(|&&byte| byte == b'\n').count(),
        11
    );
    assert_eq!(listener.count(), 3);
    let facts = line(&output);
    let mut pinned = facts.clone();
    pinned
        .as_object_mut()
        .expect("facts object")
        .remove("seconds");
    assert_eq!(
        pinned,
        json!({"schema":"thinkthen.run/1","records":25,"requests_sent":3,
            "retries":0,"cache_answers":0,"input_tokens":300,"output_tokens":30,
            "model":"jev-1.13.0"})
    );
    assert_eq!(String::from_utf8_lossy(&output.stderr).lines().count(), 1);
    let status =
        spawn(&["status", "--json"], &[("XDG_STATE_HOME", home)], b"").expect("compiled status");
    let status: Value = serde_json::from_slice(&status.stdout).expect("status JSON");
    for field in [
        "requests_sent",
        "retries",
        "cache_answers",
        "input_tokens",
        "output_tokens",
    ] {
        assert_eq!(facts[field], status["usage"]["total"][field], "{field}");
    }
    let ordinary = spawn(
        &[
            "filter",
            QUESTION,
            "--lines",
            "--batch",
            "10",
            "--no-cache",
            "--url",
            listener.base(),
        ],
        &[KEY],
        input.as_bytes(),
    )
    .expect("ordinary filter");
    assert_eq!(ordinary.status.code(), Some(0));
    assert_eq!(ordinary.stdout, output.stdout);
    assert!(ordinary.stderr.is_empty());
}

#[test]
fn top_dropped_records_still_count() {
    let listener = Listener::answering(answered).expect("rank loopback");
    let ranked = spawn(
        &[
            "rank",
            QUESTION,
            "--lines",
            "--batch",
            "10",
            "--top",
            "5",
            "--facts",
            "--no-cache",
            "--url",
            listener.base(),
        ],
        &[KEY],
        batching::lines(1..=20).as_bytes(),
    )
    .expect("ranked run");
    assert_eq!(ranked.status.code(), Some(0));
    assert_eq!(
        ranked.stdout.iter().filter(|&&byte| byte == b'\n').count(),
        5
    );
    assert_eq!(line(&ranked)["records"], 20);
}

/// A usage folder the writer cannot make passes the read before the first
/// send, since it is missing, then fails the write: the warning stays.
#[cfg(unix)]
pub(crate) fn unwritable_state(name: &str) -> std::path::PathBuf {
    use std::os::unix::fs::PermissionsExt as _;
    let blocked = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _absent = fs::set_permissions(&blocked, fs::Permissions::from_mode(0o700));
    let _absent = fs::remove_dir_all(&blocked);
    fs::create_dir_all(&blocked).expect("state home");
    fs::set_permissions(&blocked, fs::Permissions::from_mode(0o500)).expect("read-only");
    blocked
}

#[cfg(unix)]
#[test]
fn a_usage_warning_precedes_the_facts_line() {
    let listener = Listener::answering(answered).expect("warning loopback");
    let blocked = unwritable_state("facts-usage-unwritable");
    let warned = spawn(
        &[
            "decide",
            QUESTION,
            "--facts",
            "--no-cache",
            "--url",
            listener.base(),
        ],
        &[
            KEY,
            ("XDG_STATE_HOME", blocked.to_str().expect("UTF-8 path")),
        ],
        b"line 1",
    )
    .expect("usage warning command");
    assert_eq!(warned.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&warned.stderr);
    assert_eq!(stderr.lines().count(), 2);
    assert!(
        stderr
            .lines()
            .next()
            .is_some_and(|line| line.starts_with("thinkthen: usage counters could not be updated"))
    );
    assert_eq!(line(&warned)["records"], 1);
}

#[test]
fn facts_omit_tokens_and_models_nobody_agreed_to_report() {
    let recording = batching::folder("facts-mixed-recording");
    let mixed = Listener::answering(|body| {
        let at = batching::places(body).first().map_or(0, |(_, at)| *at);
        let model = if at == 1 {
            "first-model"
        } else {
            "second-model"
        };
        let mut response = json!({"model":model,"answers":{"q1":{"type":"noul","noul":0.9}}});
        if at == 1 {
            response["usage"] = json!({"input_tokens":7,"output_tokens":2});
        }
        Canned::ok(&response.to_string())
    })
    .expect("mixed loopback");
    let input = batching::lines(1..=2);
    let fixed = [
        "decide",
        QUESTION,
        "--lines",
        "--batch",
        "1",
        "--facts",
        "--url",
        mixed.base(),
    ];
    let recorded = spawn(
        &[&fixed[..], &["--record", &recording]].concat(),
        &[KEY],
        input.as_bytes(),
    )
    .expect("recorded command");
    assert_eq!(recorded.status.code(), Some(0));
    assert_eq!(mixed.count(), 2);
    let facts = line(&recorded);
    assert_eq!(facts["records"], 2);
    assert_eq!(facts["requests_sent"], 2);
    for absent in ["input_tokens", "output_tokens", "model"] {
        assert!(facts.get(absent).is_none(), "{absent}");
    }
    let replayed = spawn(
        &[&fixed[..], &["--replay", &recording]].concat(),
        &[],
        input.as_bytes(),
    )
    .expect("replayed command");
    assert_eq!(replayed.status.code(), Some(0));
    assert_eq!(replayed.stdout, recorded.stdout);
    assert_eq!(mixed.count(), 2, "replay sends nothing");
    let facts = line(&replayed);
    assert_eq!(facts["requests_sent"], 0);
    assert_eq!(facts["cache_answers"], 0);
    for absent in ["input_tokens", "output_tokens", "model"] {
        assert!(facts.get(absent).is_none(), "{absent}");
    }
}

#[test]
fn a_cached_run_has_no_live_tokens() {
    let input = batching::lines(1..=2);
    let cache = batching::folder("facts-cache");
    let uniform = Listener::answering(answered).expect("cache loopback");
    let fixed = [
        "decide",
        QUESTION,
        "--lines",
        "--batch",
        "1",
        "--facts",
        "--url",
        uniform.base(),
        "--cache",
        &cache,
    ];
    let first = spawn(&fixed, &[KEY], input.as_bytes()).expect("first cache run");
    assert_eq!(first.status.code(), Some(0));
    assert_eq!(uniform.count(), 2);
    let second = spawn(&fixed, &[], input.as_bytes()).expect("cached run");
    assert_eq!(second.status.code(), Some(0));
    assert_eq!(uniform.count(), 2, "cache sends nothing new");
    let facts = line(&second);
    assert_eq!(facts["records"], 2);
    assert_eq!(facts["requests_sent"], 0);
    assert_eq!(facts["cache_answers"], 2);
    assert_eq!(facts["model"], "jev-1.13.0");
    assert!(facts.get("input_tokens").is_none());
    assert!(facts.get("output_tokens").is_none());
}

#[test]
fn a_stopped_run_names_the_cause_after_the_human_line() {
    let seen = AtomicUsize::new(0);
    let second_batch = Listener::answering(move |body| {
        if seen.fetch_add(1, Ordering::SeqCst) == 0 {
            answered(body)
        } else {
            Canned::status(503, "busy")
        }
    })
    .expect("second-batch listener");
    let second = spawn(
        &[
            "decide",
            QUESTION,
            "--lines",
            "--batch",
            "10",
            "--jobs",
            "1",
            "--facts",
            "--no-cache",
            "--max-retries",
            "1",
            "--url",
            second_batch.base(),
        ],
        &[KEY],
        batching::lines(1..=20).as_bytes(),
    )
    .expect("second-batch command");
    assert_eq!(second.status.code(), Some(4));
    assert_eq!(
        second.stdout.iter().filter(|&&byte| byte == b'\n').count(),
        10
    );
    assert_eq!(second_batch.count(), 3);
    let facts = line(&second);
    assert_eq!(facts["records"], 10);
    assert_eq!(facts["requests_sent"], 3);
    assert_eq!(facts["retries"], 1);
    assert_eq!(
        facts["stopped"],
        json!({"at":11,"cause":"status","status":503,"retryable":true})
    );
}

#[test]
fn retried_permanent_and_too_large_statuses_name_their_causes() {
    let cases = [
        (
            429,
            2,
            json!({"at":1,"cause":"status","status":429,"retryable":true}),
        ),
        (
            520,
            2,
            json!({"at":1,"cause":"status","status":520,"retryable":true}),
        ),
        (
            521,
            2,
            json!({"at":1,"cause":"status","status":521,"retryable":true}),
        ),
        (
            522,
            2,
            json!({"at":1,"cause":"status","status":522,"retryable":true}),
        ),
        (
            523,
            2,
            json!({"at":1,"cause":"status","status":523,"retryable":true}),
        ),
        (
            524,
            2,
            json!({"at":1,"cause":"status","status":524,"retryable":true}),
        ),
        (
            503,
            2,
            json!({"at":1,"cause":"status","status":503,"retryable":true}),
        ),
        (
            401,
            1,
            json!({"at":1,"cause":"status","status":401,"retryable":false}),
        ),
        (
            413,
            1,
            json!({"at":1,"cause":"too_large","retryable":false}),
        ),
    ];
    for (status, sends, stopped) in cases {
        let listener =
            Listener::serving((0..sends).map(|_| Canned::status(status, "busy")).collect())
                .expect("status listener");
        let output = spawn(
            &[
                "decide",
                QUESTION,
                "--facts",
                "--no-cache",
                "--max-retries",
                "1",
                "--url",
                listener.base(),
            ],
            &[KEY],
            b"line 1",
        )
        .expect("status command");
        assert_eq!(output.status.code(), Some(4), "{status}");
        assert!(output.stdout.is_empty(), "{status}");
        assert_eq!(
            listener.requests().len(),
            sends,
            "{status}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert_eq!(stderr.lines().count(), 2, "{status}: {stderr}");
        assert!(
            stderr
                .lines()
                .next()
                .is_some_and(|line| line.starts_with("thinkthen: "))
        );
        let facts = line(&output);
        assert_eq!(facts["stopped"], stopped, "{status}");
        assert_eq!(facts["records"], 0, "{status}");
        assert_eq!(facts["requests_sent"], sends, "{status}");
        assert_eq!(facts["retries"], sends - 1, "{status}");
    }
}

#[test]
fn a_missing_key_and_a_refused_connection_have_distinct_causes() {
    let output = spawn(
        &["decide", QUESTION, "--facts", "--no-cache"],
        &[],
        b"line 1",
    )
    .expect("no-key command");
    assert_eq!(output.status.code(), Some(4));
    assert_eq!(
        line(&output)["stopped"],
        json!({"at":1,"cause":"no_key","retryable":false})
    );
    assert_eq!(line(&output)["requests_sent"], 0);

    let refused = spawn(
        &[
            "decide",
            QUESTION,
            "--facts",
            "--no-cache",
            "--max-retries",
            "0",
            "--url",
            "http://127.0.0.1:0/v1/systemone",
        ],
        &[KEY],
        b"line 1",
    )
    .expect("refused command");
    assert_eq!(refused.status.code(), Some(4));
    assert_eq!(
        line(&refused)["stopped"],
        json!({"at":1,"cause":"transport","retryable":false})
    );
    assert_eq!(line(&refused)["requests_sent"], 1);
}

// It names the XDG folders. Windows reads APPDATA and LOCALAPPDATA (sdlc/planning/windows.md).
#[cfg(unix)]
#[test]
fn dry_run_and_early_setup_failures_report_zero_work() {
    let dry =
        spawn(&["decide", QUESTION, "--facts", "--plan"], &[], b"line 1").expect("dry-run command");
    assert_eq!(dry.status.code(), Some(0));
    assert_eq!(line(&dry)["records"], 0);
    assert_eq!(line(&dry)["requests_sent"], 0);
    assert!(line(&dry).get("stopped").is_none());

    let config = Path::new(env!("CARGO_TARGET_TMPDIR")).join("facts-invalid-config");
    fs::create_dir_all(config.join("thinkthen")).expect("configuration folder");
    fs::write(config.join("thinkthen/config.json"), b"not JSON").expect("invalid configuration");
    let early = spawn(
        &["decide", QUESTION, "--facts", "--plan"],
        &[("XDG_CONFIG_HOME", config.to_str().expect("UTF-8 config"))],
        b"line 1",
    )
    .expect("early failure command");
    assert_eq!(early.status.code(), Some(5));
    assert_eq!(String::from_utf8_lossy(&early.stderr).lines().count(), 2);
    assert_eq!(
        line(&early)["stopped"],
        json!({"at":1,"cause":"local","retryable":false})
    );
    assert_eq!(line(&early)["requests_sent"], 0);
}

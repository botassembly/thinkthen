//! Read-only configuration and command-family precedence.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Output;

use crate::harness::{Canned, Listener, spawn};

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

fn command(words: &[&str]) -> Vec<String> {
    words.iter().map(ToString::to_string).collect()
}

fn families(set: &Path) -> [Vec<String>; 8] {
    [
        command(&["decide", "asks?", "--dry-run"]),
        command(&["choose", "which?", "a", "b", "--dry-run"]),
        command(&["tag", "tags?", "a", "--dry-run"]),
        command(&["score", "score?", "low", "high", "--dry-run"]),
        command(&["filter", "asks?", "--lines", "--dry-run"]),
        command(&["rank", "asks?", "--lines", "--dry-run"]),
        command(&["annotate", &set.to_string_lossy(), "--dry-run"]),
        command(&["find", "which?", "--dry-run"]),
    ]
}

#[test]
fn configuration_supplies_address_model_and_cache_switch_without_a_home() {
    let root = folder("configuration");
    fs::create_dir_all(root.join("thinkthen")).expect("configuration directory");
    fs::write(root.join("thinkthen/config.json"), r#"{"schema":"thinkthen.config/1","url":"http://127.0.0.1:1/v1","model":"configured-1","cache":false}"#).expect("configuration");
    let output = run(
        &["decide", "asks for a refund", "--dry-run"],
        &[
            ("XDG_CONFIG_HOME", root.to_str().expect("root")),
            ("HOME", ""),
        ],
    )
    .expect("dry run");
    assert_eq!(output.status.code(), Some(0));
    let plan = String::from_utf8(output.stdout).expect("plan");
    for expected in [
        r#""url":"http://127.0.0.1:1/v1/systemone""#,
        r#""model":"configured-1""#,
        r#""from":{"question":"command line""#,
        r#""model":"configuration""#,
    ] {
        assert!(plan.contains(expected), "{plan}");
    }
}

#[test]
fn command_and_environment_precedence_crosses_all_eight_command_families() {
    let root = folder("configuration-precedence");
    fs::create_dir_all(root.join("thinkthen")).expect("configuration directory");
    fs::write(
        root.join("thinkthen/config.json"),
        r#"{"schema":"thinkthen.config/1","url":"http://127.0.0.1:1/v1","model":"configured-1","cache":false}"#,
    )
    .expect("configuration");
    let set = root.join("questions.json");
    fs::write(
        &set,
        r#"{"version":1,"questions":{"answer":{"decide":"asks?"}}}"#,
    )
    .expect("question set");
    let families = families(&set);
    let environment = [
        ("XDG_CONFIG_HOME", root.to_str().expect("root")),
        ("THINKTHEN_BASE_URL", "http://127.0.0.1:2/v1"),
    ];
    for arguments in families {
        let input: &[u8] = if arguments[0] == "find" {
            b"one\ntwo\n"
        } else {
            EVIDENCE.as_bytes()
        };
        let refs: Vec<_> = arguments.iter().map(String::as_str).collect();
        let from_environment = spawn(&refs, &environment, input).expect("configured plan");
        assert_eq!(from_environment.status.code(), Some(0), "{}", arguments[0]);
        let plan = String::from_utf8(from_environment.stdout).expect("plan");
        assert!(
            plan.contains(r#""url":"http://127.0.0.1:2/v1/systemone""#),
            "{}: {plan}",
            arguments[0]
        );
        assert!(
            plan.contains(r#""model":"configured-1""#),
            "{}: {plan}",
            arguments[0]
        );

        let mut typed = arguments.clone();
        typed.extend([
            "--url".to_owned(),
            "http://127.0.0.1:3/v1".to_owned(),
            "--model".to_owned(),
            "typed-1".to_owned(),
        ]);
        let refs: Vec<_> = typed.iter().map(String::as_str).collect();
        let from_command = spawn(&refs, &environment, input).expect("typed plan");
        assert_eq!(from_command.status.code(), Some(0), "{}", arguments[0]);
        let plan = String::from_utf8(from_command.stdout).expect("plan");
        assert!(
            plan.contains(r#""url":"http://127.0.0.1:3/v1/systemone""#),
            "{}: {plan}",
            arguments[0]
        );
        assert!(
            plan.contains(r#""model":"typed-1""#),
            "{}: {plan}",
            arguments[0]
        );
    }

    let question = root.join("question.json");
    fs::write(&question, r#"{"decide":"asks?","model":"saved-1"}"#).expect("question file");
    let named = format!("@{}", question.display());
    let saved = spawn(
        &["decide", &named, "--dry-run"],
        &environment,
        EVIDENCE.as_bytes(),
    )
    .expect("saved question plan");
    assert!(String::from_utf8_lossy(&saved.stdout).contains(r#""model":"saved-1""#));
}

#[test]
fn a_named_cache_enables_storage_over_disabled_configuration() {
    let root = folder("named-cache-precedence");
    let config = root.join("config");
    fs::create_dir_all(config.join("thinkthen")).expect("configuration directory");
    fs::write(
        config.join("thinkthen/config.json"),
        r#"{"schema":"thinkthen.config/1","cache":false}"#,
    )
    .expect("configuration");
    let cache = root.join("named");
    let listener = Listener::serving(vec![Canned::ok(ANSWERED)]).expect("listener");
    let environment = [
        ("XDG_CONFIG_HOME", config.to_str().expect("config")),
        ("THINKTHEN_CACHE", cache.to_str().expect("cache")),
        ("THINKTHEN_BASE_URL", listener.base()),
        ("THINKTHEN_API_KEY", "key"),
    ];
    for _ in 0..2 {
        assert_eq!(
            run(&["decide", "asks for a refund"], &environment)
                .expect("cached")
                .status
                .code(),
            Some(0)
        );
    }
    assert_eq!(listener.requests().len(), 1);
    assert!(cache.is_dir());
}

/// A configuration another user can write decides where the key goes, so the
/// run warns. One its owner alone can write stays quiet.
#[test]
fn a_configuration_another_user_can_write_is_warned_about() {
    use std::os::unix::fs::PermissionsExt as _;
    const WARNING: &str = "thinkthen: the configuration file is writable by another user; it decides where the key and evidence go\n";
    for (mode, says) in [(0o666, WARNING), (0o620, WARNING), (0o644, ""), (0o600, "")] {
        let root = folder(&format!("configuration-mode-{mode:o}"));
        let path = root.join("thinkthen/config.json");
        fs::create_dir_all(root.join("thinkthen")).expect("configuration directory");
        fs::write(&path, r#"{"schema":"thinkthen.config/1"}"#).expect("configuration");
        fs::set_permissions(&path, fs::Permissions::from_mode(mode)).expect("mode");
        let output = run(
            &["decide", "asks for a refund", "--dry-run"],
            &[("XDG_CONFIG_HOME", root.to_str().expect("root"))],
        )
        .expect("dry run");
        assert_eq!(output.status.code(), Some(0), "{mode:o}");
        assert!(!output.stdout.is_empty(), "{mode:o}");
        assert_eq!(String::from_utf8_lossy(&output.stderr), says, "{mode:o}");
    }
}

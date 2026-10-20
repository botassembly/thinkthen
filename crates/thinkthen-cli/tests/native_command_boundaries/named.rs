use conformance_backend::{Canned, Listener};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use super::child;
use child::ChildEnvironment as _;

struct Folder(PathBuf);
impl Folder {
    fn new() -> std::io::Result<Self> {
        static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let root = std::env::temp_dir().join(format!(
            "thinkthen-names-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        ));
        fs::create_dir(&root)?;
        Ok(Self(root))
    }
    fn path(&self) -> &Path {
        &self.0
    }
}
impl Drop for Folder {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn config(root: &Path) -> PathBuf {
    child::Folder::Config.under(root)
}
fn child_environment(command: &mut Command, root: &Path) {
    command.isolated_home(root).current_dir(root);
}

#[test]
fn cli_named_selected_input_is_admitted_before_lookup_or_send_with_safe_fixed_errors() {
    let root = Folder::new().unwrap();
    fs::create_dir_all(config(root.path()).join("questions")).unwrap();
    fs::write(config(root.path()).join("questions/refund.json"), r#"{"name":"refund","wording_version":2,"decide":"Refund?","on":"/message","item_schema":{"type":"object","properties":{"body":{"type":"string"},"ready":{"type":"boolean"}},"required":["body"]}}"#).unwrap();
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.7}}}"#)
    })
    .unwrap();
    let input = root.path().join("input.jsonl");
    let run = || {
        let mut command = Command::new(env!("CARGO_BIN_EXE_thinkthen"));
        command.clear_environment();
        child_environment(&mut command, root.path());
        command
            .env("THINKTHEN_API_KEY", "named-fixture-private")
            .args([
                "decide",
                "@refund",
                "--jsonl",
                "--url",
                listener.base(),
                "--model",
                "fixed",
                "--no-cache",
                "--max-retries",
                "0",
            ])
            .stdin(fs::File::open(&input).unwrap())
            .output()
            .unwrap()
    };
    fs::write(
        &input,
        "{\"message\":{\"body\":\"Refund me.\",\"ready\":null},\"private\":\"unprinted-secret\"}\n",
    )
    .unwrap();
    let refused = run();
    assert_eq!(refused.status.code(), Some(2));
    assert!(refused.stdout.is_empty());
    let stderr = String::from_utf8(refused.stderr).unwrap();
    assert!(
        stderr.contains("the item does not match item_schema"),
        "{stderr}"
    );
    for private in [
        "unprinted-secret",
        "named-fixture-private",
        "ready",
        "message",
    ] {
        assert!(!stderr.contains(private));
    }
    assert_eq!(listener.count(), 0);
    fs::write(
        &input,
        "{\"message\":{\"body\":\"Refund me.\",\"ready\":false},\"private\":\"unsent-secret\"}\n",
    )
    .unwrap();
    let accepted = run();
    assert_eq!(
        accepted.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&accepted.stderr)
    );
    assert_eq!(listener.count(), 1);
    assert!(
        child::Folder::Usage.under(root.path()).is_dir(),
        "the live call writes only owned usage"
    );
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&listener.requests()[0].body).unwrap(),
        serde_json::json!({"state":"Each question quotes the text it asks about.","model":"fixed","questions":{"q1":{"type":"noul","instructions":r#"The text is {"body":"Refund me.","ready":false}. Refund?"#}}})
    );
}

#[test]
fn cli_declared_batches_refuse_before_any_lookup_and_keep_only_prior_wire_batches() {
    let root = Folder::new().unwrap();
    fs::create_dir_all(config(root.path()).join("questions")).unwrap();
    fs::write(
        config(root.path()).join("questions/refund.json"),
        r#"{"decide":"Refund?","item_schema":{"type":"string"}}"#,
    )
    .unwrap();
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.7},"q2":{"type":"noul","noul":0.2}},"usage":{"input_tokens":887}}"#)).unwrap();
    let input = root.path().join("input.jsonl");
    let run = |function: &str| {
        let mut command = Command::new(env!("CARGO_BIN_EXE_thinkthen"));
        command.clear_environment();
        child_environment(&mut command, root.path());
        command
            .env("THINKTHEN_API_KEY", "named-fixture-private")
            .args([
                function,
                "@refund",
                "--jsonl",
                "--batch",
                "2",
                "--url",
                listener.base(),
                "--model",
                "fixed",
                "--no-cache",
                "--max-retries",
                "0",
            ])
            .stdin(fs::File::open(&input).unwrap())
            .output()
            .unwrap()
    };
    fs::write(&input, "\"Valid.\"\n12\n\"Unread-private.\"\n").unwrap();
    let refused = run("decide");
    assert_eq!(refused.status.code(), Some(2));
    assert!(refused.stdout.is_empty());
    let stderr = String::from_utf8(refused.stderr).unwrap();
    assert_eq!(
        stderr,
        "thinkthen: the item does not match item_schema\nthinkthen: stopped at record 2; 0 records finished\n"
    );
    assert!(!stderr.contains("private"));
    assert_eq!(listener.count(), 0);
    fs::write(
        &input,
        "\"A.\"\n\"B.\"\n\"C.\"\nfalse\n\"Unread-private.\"\n",
    )
    .unwrap();
    let stopped = run("decide");
    assert_eq!(stopped.status.code(), Some(2));
    assert_eq!(
        String::from_utf8(stopped.stdout).unwrap(),
        "{\"input\":\"A.\",\"value\":true}\n{\"input\":\"B.\",\"value\":false}\n"
    );
    assert_eq!(
        String::from_utf8(stopped.stderr).unwrap(),
        "thinkthen: the item does not match item_schema\nthinkthen: stopped at record 4; 2 records finished\n"
    );
    assert_eq!(listener.count(), 1);
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&listener.requests()[0].body).unwrap(),
        serde_json::json!({"state":"Each question quotes the text it asks about.","model":"fixed","questions":{"q1":{"type":"noul","instructions":"The text is \"A.\". Refund?"},"q2":{"type":"noul","instructions":"The text is \"B.\". Refund?"}}})
    );
    let rank = run("rank");
    assert_eq!(rank.status.code(), Some(2));
    assert!(rank.stdout.is_empty());
    assert_eq!(listener.count(), 2);
    assert!(
        child::Folder::Usage.under(root.path()).is_dir(),
        "the completed prefix writes only owned usage"
    );
}

#[test]
fn cli_whole_set_functions_validate_typed_selected_values_before_sending() {
    let root = Folder::new().unwrap();
    let input = root.path().join("input.jsonl");
    fs::write(
        &input,
        "{\"body\":false,\"private\":\"unprinted-secret\"}\n",
    )
    .unwrap();
    let listener = Listener::answering(|_| Canned::ok("{}")).unwrap();
    let cases = [
        (
            "find",
            r#"{"find":"Which?","on":"/body","item_schema":{"type":"string"}}"#,
        ),
        (
            "recognize",
            r#"{"version":1,"recognize":{},"on":"/body","item_schema":{"type":"string"}}"#,
        ),
        (
            "relate",
            r#"{"version":1,"relate":{"relations":[{"name":"knows","source":"person","target":"person","reads":"knows"}]},"item_schema":{"type":"string"}}"#,
        ),
    ];
    for (function, question) in cases {
        let path = root.path().join("question.json");
        fs::write(&path, question).unwrap();
        let mut command = Command::new(env!("CARGO_BIN_EXE_thinkthen"));
        command.clear_environment();
        child_environment(&mut command, root.path());
        let output = command
            .env("THINKTHEN_API_KEY", "named-fixture-private")
            .args([
                function,
                "@question.json",
                "--jsonl",
                "--url",
                listener.base(),
                "--model",
                "fixed",
                "--no-cache",
                "--max-retries",
                "0",
            ])
            .stdin(fs::File::open(&input).unwrap())
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(2),
            "{function}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(output.stdout.is_empty());
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(
            stderr.contains("the item does not match item_schema"),
            "{function}: {stderr}"
        );
        assert!(!stderr.contains("private"));
        assert!(!stderr.contains("body"));
        assert_eq!(listener.count(), 0);
    }
}

#[test]
fn cli_rank_sets_admit_every_member_declaration_before_lookup_or_send() {
    let root = Folder::new().unwrap();
    let set = root.path().join("rank.json");
    fs::write(&set, r#"{"version":1,"questions":{"first":{"decide":"Ready?"},"second":{"decide":"Useful?","item_schema":{"type":"object","properties":{}}}}}"#).unwrap();
    let listener = Listener::answering(|_| Canned::ok("{}")).unwrap();
    let input = root.path().join("items.txt");
    fs::write(&input, "alpha\nbeta\n").unwrap();
    let cache = root.path().join("saved");
    fs::create_dir(&cache).unwrap();
    // An empty replay returns a miss if declaration admission reaches lookup.
    fs::write(cache.join("thinkthen.jsonl"), "").unwrap();
    for mode in ["--no-cache", "--replay"] {
        let mut command = Command::new(env!("CARGO_BIN_EXE_thinkthen"));
        command.clear_environment();
        child_environment(&mut command, root.path());
        command.args([
            "rank",
            &format!("@{}", set.display()),
            "--batch",
            "2",
            "--url",
            listener.base(),
            "--model",
            "fixed",
            "--max-retries",
            "0",
        ]);
        if mode == "--replay" {
            command.arg(mode).arg(&cache);
        } else {
            command.arg(mode);
        }
        let output = command
            .env("THINKTHEN_API_KEY", "rank-private")
            .stdin(fs::File::open(&input).unwrap())
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
        assert_eq!(
            String::from_utf8(output.stderr).unwrap(),
            format!(
                "thinkthen: the item does not match item_schema\nthinkthen: stopped at record 1; 0 records finished{}, and nothing was printed because an order needs every record\n",
                if mode == "--replay" {
                    ", 0 records from a recording"
                } else {
                    ""
                }
            )
        );
        assert_eq!(listener.count(), 0);
        assert!(!cache.join("thinkthen.sqlite").exists());
        assert_eq!(
            fs::read_to_string(cache.join("thinkthen.jsonl")).unwrap(),
            ""
        );
    }
}

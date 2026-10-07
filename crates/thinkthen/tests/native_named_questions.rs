//! Native name resolution and CLI admission use isolated ordinary config folders.
use conformance_backend::{Canned, Listener};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};
use thinkthen::{ErrorKind, Question};

#[path = "../src/test_deadline/child.rs"]
mod child;
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
    if cfg!(target_os = "macos") {
        root.join("Library/Application Support/thinkthen")
    } else {
        root.join("thinkthen")
    }
}
fn child_environment(command: &mut Command, root: &Path) {
    command
        .home(root)
        .env("XDG_CONFIG_HOME", root)
        .env("APPDATA", root)
        .current_dir(root);
    for folder in [child::Folder::Cache, child::Folder::Usage] {
        let (name, value) = folder.variable(root);
        command.env(name, value);
    }
}

#[test]
fn named_native_loaders_preserve_path_collisions_and_confine_symlinks_without_config_json() {
    const CHILD: &str = "THINKTHEN_TEST_NAMED_QUESTION_ROOT";
    if let Some(root) = std::env::var_os(CHILD) {
        let root = PathBuf::from(root);
        check_named(&root);
        return;
    }
    let root = Folder::new().unwrap();
    fs::create_dir_all(config(root.path()).join("questions")).unwrap();
    fs::create_dir(root.path().join("directory")).unwrap();
    let authored = r#"{"name":"refund","decide":"Refund?","wording_version":2}"#;
    fs::write(config(root.path()).join("questions/refund.json"), authored).unwrap();
    fs::write(
        config(root.path()).join("questions/directory.json"),
        authored,
    )
    .unwrap();
    fs::write(
        config(root.path()).join("questions/mismatch.json"),
        authored,
    )
    .unwrap();
    fs::write(
        root.path().join("refund"),
        r#"{"decide":"Local collision?"}"#,
    )
    .unwrap();
    fs::write(root.path().join("@literal.json"), authored).unwrap();
    let mut command = Command::new(std::env::current_exe().unwrap());
    command.clear_environment();
    child_environment(&mut command, root.path());
    let output = command.env(CHILD, root.path()).args(["--exact", "named_native_loaders_preserve_path_collisions_and_confine_symlinks_without_config_json", "--nocapture"]).output().unwrap();
    assert!(
        output.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!config(root.path()).join("config.json").exists());
}

#[cfg(feature = "cli")]
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

#[cfg(test)]
fn check_named(root: &Path) {
    check_roles(root);
    let named = Question::load_named("refund").unwrap();
    assert_eq!(named.name().unwrap().as_str(), "refund");
    assert!(
        Question::load_reference("@refund")
            .unwrap()
            .name()
            .is_none()
    );
    assert_eq!(
        Question::load_reference("refund").unwrap_err().kind(),
        ErrorKind::Usage
    );
    for name in ["", "../private", "a/b", "a\\b", "Refund", "."] {
        assert_eq!(
            Question::load_named(name).unwrap_err().kind(),
            ErrorKind::Usage
        );
    }
    assert_eq!(
        Question::load_named("mismatch")
            .unwrap_err()
            .detail()
            .message(),
        "the named question name does not match the file"
    );
    assert_eq!(
        Question::load_named("missing")
            .unwrap_err()
            .detail()
            .message(),
        "the named question is unavailable"
    );
    assert_eq!(
        Question::load_reference("@directory").unwrap_err().kind(),
        ErrorKind::Local
    );
    assert_eq!(
        Question::load(root.join("@literal.json"))
            .unwrap()
            .name()
            .unwrap()
            .as_str(),
        "refund"
    );
    fs::remove_file(root.join("refund")).unwrap();
    assert_eq!(
        Question::load_reference("@refund")
            .unwrap()
            .name()
            .unwrap()
            .as_str(),
        "refund"
    );
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(root.join("absent"), root.join("refund")).unwrap();
        assert_eq!(
            Question::load_reference("@refund").unwrap_err().kind(),
            ErrorKind::Local
        );
        fs::remove_file(root.join("refund")).unwrap();
        std::os::unix::fs::symlink(
            root.join("@literal.json"),
            config(root).join("questions/escape.json"),
        )
        .unwrap();
        assert_eq!(
            Question::load_named("escape")
                .unwrap_err()
                .detail()
                .message(),
            "the named question is unavailable"
        );
        fs::rename(config(root).join("questions"), root.join("outside")).unwrap();
        std::os::unix::fs::symlink(root.join("outside"), config(root).join("questions")).unwrap();
        assert_eq!(
            Question::load_named("refund")
                .unwrap_err()
                .detail()
                .message(),
            "the named question is unavailable"
        );
    }
}

#[cfg(feature = "cli")]
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

#[cfg(test)]
fn check_roles(root: &Path) {
    for (name, body) in [
        ("finder", r#"{"find":"Which?","wording_version":2}"#),
        ("recognizer", r#"{"version":1,"recognize":{}}"#),
        ("ranker", r#"{"decide":"Ready?","wording_version":2}"#),
        (
            "connections",
            r#"{"version":1,"relate":{"fields":{"name":"/person/name","kind":"/person/kind"},"relations":[{"name":"knows","source":"person","target":"person","reads":"knows"}]}}"#,
        ),
    ] {
        fs::write(config(root).join(format!("questions/{name}.json")), body).unwrap();
        fs::write(root.join(format!("{name}.json")), body).unwrap();
    }
    for error in [
        Question::load_named("finder").unwrap_err(),
        thinkthen::QuestionSet::load_named("recognizer").unwrap_err(),
        thinkthen::RecognizeQuestionFile::load_named("finder").unwrap_err(),
        thinkthen::Relate::load_named("recognizer").unwrap_err(),
        thinkthen::RecordChooseQuestion::load_named("ranker").unwrap_err(),
        thinkthen::RankSet::load_reference("@finder.json").unwrap_err(),
        Question::load_rank_reference("@recognizer.json").unwrap_err(),
    ] {
        assert_eq!(error.kind(), ErrorKind::Usage);
        assert_eq!(
            error.detail().message(),
            "the question file uses another function"
        );
        assert!(error.facts().is_none());
    }
    // Released literal-path loader keeps its file/content error boundary.
    assert_eq!(
        Question::load(root.join("finder.json")).unwrap_err().kind(),
        ErrorKind::Local
    );
    assert_eq!(
        Question::load_rank_named("ranker").unwrap().kind(),
        thinkthen::QuestionKind::Rank
    );
    assert_eq!(
        Question::load_rank_reference("@ranker.json")
            .unwrap()
            .kind(),
        thinkthen::QuestionKind::Rank
    );
    assert_eq!(
        Question::load_find_named("finder").unwrap().kind(),
        thinkthen::QuestionKind::Find
    );
    assert_eq!(
        Question::load_find_reference("@finder.json")
            .unwrap()
            .kind(),
        thinkthen::QuestionKind::Find
    );
    assert!(thinkthen::Relate::load_records_named("connections").is_ok());
    assert!(thinkthen::Relate::load_records_reference("@connections.json").is_ok());
    assert_eq!(
        thinkthen::Relate::load_named("connections")
            .unwrap_err()
            .kind(),
        ErrorKind::Local
    );
}

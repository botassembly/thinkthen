//! Native name resolution and CLI admission use isolated ordinary config folders.
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
    child::Folder::Config.under(root)
}
fn child_environment(command: &mut Command, root: &Path) {
    command.isolated_home(root).current_dir(root);
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

#[cfg(test)]
fn check_named(root: &Path) {
    check_roles(root);
    reference::check(root, &config(root));
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

#[cfg(test)]
#[path = "native_named_questions/reference.rs"]
mod reference;

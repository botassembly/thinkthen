//! Caller-authorized reading precedes native content admission.
use std::{fs, path::Path};
use thinkthen::{Error, ErrorKind, Question, QuestionFileReference, QuestionFileRole};

pub(super) fn check(root: &Path, config: &Path) {
    selection(root, config);
    unread_content_and_links(config);
    #[cfg(unix)]
    unix_links(root, config);
    content(root, config);
}

fn selection(root: &Path, config: &Path) {
    let named = QuestionFileReference::named("refund").unwrap();
    assert_eq!(named.path(), config.join("questions/refund.json"));
    assert_eq!(
        named.named_root(),
        Some(config.join("questions").canonicalize().unwrap().as_path())
    );
    assert_eq!(format!("{named:?}"), "QuestionFileReference { .. }");
    assert_eq!(
        QuestionFileReference::reference("refund")
            .unwrap_err()
            .kind(),
        ErrorKind::Usage
    );
    for name in ["", "../private", "a/b", "a\\b", "Refund", "."] {
        assert_eq!(
            QuestionFileReference::named(name).unwrap_err().kind(),
            ErrorKind::Usage
        );
    }
    let before = std::env::current_dir().unwrap();
    let local = QuestionFileReference::reference("@refund").unwrap();
    assert_eq!(local.path(), root.join("refund"));
    assert!(local.named_root().is_none());
    let directory = QuestionFileReference::reference_in("@directory", root).unwrap();
    assert_eq!(directory.path(), root.join("directory"));
    assert!(directory.named_root().is_none());
    assert!(
        QuestionFileReference::named("directory")
            .unwrap()
            .named_root()
            .is_some()
    );
    fs::create_dir(root.join("lookup")).unwrap();
    fs::write(root.join("lookup/refund"), "private malformed JSON").unwrap();
    let relative = QuestionFileReference::reference_in("@refund", Path::new("lookup")).unwrap();
    assert_eq!(relative.path(), root.join("lookup/refund"));
    assert!(relative.named_root().is_none());
    let spelling = root.join("lookup/../@literal.json");
    let absolute = QuestionFileReference::reference_in(
        &format!("@{}", spelling.display()),
        Path::new("unused"),
    )
    .unwrap();
    assert_eq!(absolute.path(), spelling);
    assert_eq!(
        QuestionFileReference::reference_in("@lookup/../@literal.json", root)
            .unwrap()
            .path(),
        spelling
    );
    assert_eq!(
        QuestionFileReference::reference_in("@finder", root)
            .unwrap()
            .named_root(),
        Some(config.join("questions").canonicalize().unwrap().as_path())
    );
    assert_eq!(std::env::current_dir().unwrap(), before);
}

fn unread_content_and_links(config: &Path) {
    let relative = QuestionFileReference::reference_in("@refund", Path::new("lookup")).unwrap();
    let authorized_read = |_path: &Path| -> Result<String, Error> {
        Err(Error::new(ErrorKind::Usage, "the host refuses this file"))
    };
    let parsed = std::cell::Cell::new(false);
    let refused = authorized_read(relative.path()).and_then(|text| {
        relative.parse(&text, QuestionFileRole::Atomic, |_| {
            parsed.set(true);
            Ok(())
        })
    });
    assert_eq!(
        refused.unwrap_err().detail().message(),
        "the host refuses this file"
    );
    assert!(!parsed.get());
    let large_path = config.join("questions/large.json");
    fs::File::create(&large_path)
        .unwrap()
        .set_len(1_048_577)
        .unwrap();
    assert_eq!(
        QuestionFileReference::named("large").unwrap().path(),
        large_path
    );
}

#[cfg(unix)]
fn unix_links(root: &Path, config: &Path) {
    let named = QuestionFileReference::named("refund").unwrap();
    let large_path = config.join("questions/large.json");
    {
        use std::os::unix::fs::{PermissionsExt as _, symlink};
        fs::set_permissions(&large_path, fs::Permissions::from_mode(0o0)).unwrap();
        assert!(QuestionFileReference::named("large").is_ok());
        symlink(root.join("absent"), root.join("dangling")).unwrap();
        assert!(
            QuestionFileReference::reference("@dangling")
                .unwrap()
                .named_root()
                .is_none()
        );
        symlink(
            config.join("questions/refund.json"),
            config.join("questions/alias.json"),
        )
        .unwrap();
        let alias = QuestionFileReference::named("alias").unwrap();
        assert_eq!(alias.path(), config.join("questions/alias.json"));
        assert_eq!(alias.named_root(), named.named_root());
        // The ordinary native reader still admits a confined symlink.
        fs::write(
            config.join("questions/unnamed.json"),
            r#"{"decide":"Ready?"}"#,
        )
        .unwrap();
        symlink(
            config.join("questions/unnamed.json"),
            config.join("questions/link.json"),
        )
        .unwrap();
        assert!(Question::load_named("link").is_ok());
        let captured = named.named_root().unwrap().to_path_buf();
        fs::rename(config.join("questions"), config.join("original-questions")).unwrap();
        symlink(root, config.join("questions")).unwrap();
        assert_eq!(named.named_root(), Some(captured.as_path()));
        fs::remove_file(config.join("questions")).unwrap();
        fs::rename(config.join("original-questions"), config.join("questions")).unwrap();
    }
}

fn content(root: &Path, config: &Path) {
    let named = QuestionFileReference::named("refund").unwrap();
    // Parsing uses only supplied original text after the selected file disappears.
    let authored = fs::read_to_string(named.path()).unwrap();
    fs::remove_file(named.path()).unwrap();
    for text in [
        r#"{"decide":"Ready?"}"#,
        r#"{"name":"refund","decide":"Ready?"}"#,
    ] {
        let calls = std::cell::Cell::new(0);
        let parsed = named
            .parse(text, QuestionFileRole::Atomic, |original| {
                calls.set(calls.get() + 1);
                assert_eq!(original, text);
                Question::from_json(original)
            })
            .unwrap();
        assert_eq!(calls.get(), 1);
        assert_eq!(
            parsed.name().map(|name| name.as_str()),
            if text.contains("name") {
                Some("refund")
            } else {
                None
            }
        );
    }
    let cases = [
        (
            r#"{"name":"private-name","find":"Which?"}"#,
            ErrorKind::Local,
        ),
        ("private malformed JSON", ErrorKind::Local),
        (
            r#"{"decide":"private","decide":"duplicate"}"#,
            ErrorKind::Local,
        ),
        (r#"{"name":42,"decide":"Ready?"}"#, ErrorKind::Local),
        (r#"{"name":"Refund","decide":"Ready?"}"#, ErrorKind::Local),
        (r#"{"decide":"Ready?","find":"Which?"}"#, ErrorKind::Local),
        (
            r#"{"decide":"Ready?","item_schema":{"type":"private-type"}}"#,
            ErrorKind::Local,
        ),
        (r#"{"find":"Which?"}"#, ErrorKind::Usage),
    ];
    for (text, expected) in cases {
        let error = named
            .parse(text, QuestionFileRole::Atomic, Question::from_json)
            .unwrap_err();
        assert_eq!(error.kind(), expected, "{text}");
        assert!(error.facts().is_none());
        assert!(!error.detail().message().contains("private"));
        assert!(!format!("{error:?}").contains("private"));
    }
    let large = "é".repeat(524_289);
    let error = named
        .parse(&large, QuestionFileRole::Atomic, |_| -> Result<(), Error> {
            panic!("over-cap content reached parser")
        })
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Local);
    assert_eq!(error.detail().message(), "the question file is too large");
    let explicit = QuestionFileReference::reference_in("@selected.json", root).unwrap();
    assert!(explicit.named_root().is_none());
    assert_eq!(
        explicit
            .parse(
                "private malformed JSON",
                QuestionFileRole::Atomic,
                Question::from_json
            )
            .unwrap_err()
            .kind(),
        ErrorKind::Local
    );
    assert_eq!(
        explicit
            .parse(
                r#"{"find":"Which?"}"#,
                QuestionFileRole::Atomic,
                Question::from_json
            )
            .unwrap_err()
            .kind(),
        ErrorKind::Usage
    );
    fs::write(config.join("questions/refund.json"), authored).unwrap();
}

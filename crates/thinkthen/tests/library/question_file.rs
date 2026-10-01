//! The one capped question-file reader, through the public loaders (ticket 0345).
//!
//! Each loader reads at most 1 MiB and one byte. A file of exactly 1 MiB
//! loads, one byte more and `/dev/zero` are too large, and a missing file or
//! invalid UTF-8 could not be read. Every refusal is local.

#![allow(clippy::expect_used, reason = "a failed fixture stops the proof")]

use std::path::{Path, PathBuf};

use thinkthen::{ErrorKind, Question, QuestionFileError, QuestionSet, Recognize, Relate};

/// The cap in bytes: 1 MiB.
const LIMIT: usize = 1_048_576;

/// A loader, a valid file for it, and the words its sentences name the file by.
type Loader = fn(&Path) -> Result<(), thinkthen::Error>;

const LOADERS: [(&str, Loader, &str); 4] = [
    (
        "question file",
        |path| Question::load(path).map(drop),
        r#"{"decide":"Is it?"}"#,
    ),
    (
        "relate file",
        |path| Relate::load(path).map(drop),
        r#"{"version":1,"relate":{"relations":[{"name":"r","source":"a","target":"b"}]}}"#,
    ),
    (
        "recognize file",
        |path| Recognize::load(path).map(drop),
        r#"{"version":1,"recognize":{}}"#,
    ),
    (
        "question set",
        |path| QuestionSet::load(path).map(drop),
        r#"{"version":1,"questions":{"q":{"decide":"Is it?"}}}"#,
    ),
];

fn written(folder: &Path, name: &str, bytes: &[u8]) -> PathBuf {
    let path = folder.join(name);
    std::fs::write(&path, bytes).expect("a fixture file");
    path
}

/// `text` padded with trailing spaces to `size` bytes.
fn padded(text: &str, size: usize) -> Vec<u8> {
    format!("{text}{}", " ".repeat(size - text.len())).into_bytes()
}

#[test]
fn each_loader_reads_one_mib_and_refuses_more_invalid_utf8_or_a_missing_file() {
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("question-file-cap");
    std::fs::create_dir_all(&folder).expect("folder");
    for (role, load, text) in LOADERS {
        let fits = written(&folder, &format!("{role}-fits"), &padded(text, LIMIT));
        assert_eq!(
            load(&fits).map_err(|error| error.to_string()),
            Ok(()),
            "{role}"
        );
        let over = written(&folder, &format!("{role}-over"), &padded(text, LIMIT + 1));
        let invalid = written(
            &folder,
            &format!("{role}-invalid"),
            b"{\"decide\":\"\xff\"}",
        );
        let mut cases: Vec<(&Path, String)> = vec![
            (&over, format!("the {role} is too large")),
            (&invalid, format!("the {role} could not be read")),
            (
                &folder.join("absent.json"),
                format!("the {role} could not be read"),
            ),
        ];
        // Windows has no endless file like `/dev/zero`.
        if cfg!(unix) {
            cases.push((Path::new("/dev/zero"), format!("the {role} is too large")));
        }
        for (path, sentence) in cases {
            let error = load(path).expect_err("a refusal");
            assert_eq!(
                (error.kind(), error.to_string()),
                (ErrorKind::Local, sentence),
                "{role} {}",
                path.display()
            );
        }
    }
}

#[test]
fn the_reader_names_each_reason() {
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("question-file-reasons");
    std::fs::create_dir_all(&folder).expect("folder");
    let invalid = written(&folder, "invalid", b"\xff");
    let text = written(&folder, "text", b"hello");
    assert_eq!(
        thinkthen::read_question_file(&text).ok().as_deref(),
        Some("hello")
    );
    // Windows has no endless file like `/dev/zero`.
    if cfg!(unix) {
        assert!(matches!(
            thinkthen::read_question_file("/dev/zero"),
            Err(QuestionFileError::TooLarge)
        ));
    }
    assert!(matches!(
        thinkthen::read_question_file(&invalid),
        Err(QuestionFileError::NotUtf8)
    ));
    assert!(matches!(
        thinkthen::read_question_file(folder.join("absent")),
        Err(QuestionFileError::Unreadable(_))
    ));
}

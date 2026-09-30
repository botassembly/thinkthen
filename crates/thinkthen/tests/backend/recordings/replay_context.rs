//! One strict replay miss at each distinct command-owned request shape.

use std::fs;
use std::io;
use std::path::Path;

use crate::harness::{Listener, spawn};
use crate::support::{ENDPOINT_PATH, encoded_decide, keys};

use super::{EVIDENCE, folder, judge, recorded};

type Case<'a> = (&'static str, Vec<&'a str>, &'static [u8], &'static str);

#[test]
fn missing_entries_name_only_proved_sources_and_keep_replay_local() {
    let folder = super::folder("context-missing");
    let folder_text = folder.to_string_lossy().into_owned();
    let set = Path::new(env!("CARGO_TARGET_TMPDIR")).join("context-set.json");
    fs::write(
        &set,
        r#"{"version":1,"questions":{"first":{"decide":"private-question"},"second":{"decide":"another private-question"}}}"#,
    )
    .expect("question set");
    let set_text = set.to_string_lossy().into_owned();
    let listener = Listener::serving(Vec::new()).expect("counted listener");
    let base = listener.base();
    let cases = cases(base, &folder_text, &set_text);
    for (name, args, input, source) in cases {
        let output = spawn(&args, &[], input).expect(name);
        assert_eq!(
            output.status.code(),
            Some(5),
            "{name}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(output.stdout.is_empty(), "{name}");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains(source), "{name}: {stderr}");
        assert!(stderr.contains(miss(name)), "{name}: {stderr}");
        if name == "streaming row" {
            assert!(
                stderr.contains("stopped at record 1; 0 records finished"),
                "{stderr}"
            );
        }
        for secret in [
            "private-question",
            "private-evidence",
            &folder_text,
            &set_text,
            base,
        ] {
            assert!(
                !stderr.contains(secret),
                "{name} exposed a source: {stderr}"
            );
        }
        assert!(!folder.exists(), "{name} created the replay folder");
        if name == "shared batch and facts" {
            let lines: Vec<_> = stderr.lines().collect();
            // A miss is found at lookup, before any request holds record 1,
            // so it names the record alone.
            assert_eq!(lines.len(), 3, "{stderr}");
            assert!(
                lines[1].starts_with("thinkthen: stopped at record 1; "),
                "{stderr}"
            );
            let facts: serde_json::Value = serde_json::from_str(lines[2]).expect("final facts");
            assert_eq!(facts["stopped"]["cause"], "local");
        }
    }
    assert!(
        listener.requests().is_empty(),
        "strict replay opened a connection"
    );
}

/// The sentence end every miss prints: a question key under ADR 0111.
fn miss(_name: &str) -> &'static str {
    "; the key is the SHA-256 of the adapter, address, model, shared state and question as sent"
}

fn cases<'a>(base: &'a str, folder: &'a str, set: &'a str) -> [Case<'a>; 7] {
    [
        (
            "one document",
            vec!["decide", "private-question", "--url", base, "--replay", folder],
            b"private-evidence",
            "the decide request for one document: the replay folder holds no answer",
        ),
        (
            "streaming row",
            vec!["filter", "private-question", "--lines", "--url", base, "--replay", folder],
            b"private-evidence\n",
            "the filter request: the replay folder holds no answer",
        ),
        (
            "shared batch and facts",
            vec!["decide", "private-question", "--lines", "--batch", "2", "--facts", "--url", base, "--replay", folder],
            b"private-evidence\nsecond private-evidence\n",
            "the decide request: the replay folder holds no answer",
        ),
        (
            "annotate group",
            vec!["annotate", set, "--url", base, "--replay", folder],
            br#"{"body":"private-evidence"}"#,
            "the annotate request: the replay folder holds no answer",
        ),
        (
            "complete find set",
            vec!["find", "private-question", "--url", base, "--replay", folder],
            b"private-evidence\nsecond private-evidence\n",
            "the complete find set of 2 units: the replay folder holds no answer",
        ),
        (
            "recognize command fallback",
            vec!["recognize", "person", "--url", base, "--replay", folder],
            b"private-evidence",
            "the recognize request: the replay folder holds no answer",
        ),
        (
            "relate command fallback",
            vec!["relate", "works_for=person:organization", "--url", base, "--replay", folder],
            br#"[{"name":"private-evidence","kind":"person"},{"name":"Acme","kind":"organization"}]"#,
            "the relate request: the replay folder holds no answer",
        ),
    ]
}

#[test]
fn a_replay_miss_is_a_local_failure_that_names_the_question_key() {
    let folder = folder("missed");
    let (listener, _) = recorded(&folder).expect("one recorded answer");
    assert_eq!(
        listener.requests().len(),
        1,
        "the recording run called once"
    );
    let snapshot = || {
        let mut pending = vec![folder.clone()];
        let mut entries = Vec::new();
        while let Some(path) = pending.pop() {
            let metadata = fs::metadata(&path)?;
            let contents = if metadata.is_dir() {
                pending.extend(
                    fs::read_dir(&path)?
                        .map(|entry| entry.map(|entry| entry.path()))
                        .collect::<io::Result<Vec<_>>>()?,
                );
                None
            } else {
                Some(fs::read(&path)?)
            };
            entries.push((
                path.strip_prefix(&folder)
                    .expect("snapshot stays in folder")
                    .to_path_buf(),
                contents,
                metadata.modified()?,
                metadata.permissions().readonly(),
            ));
        }
        entries.sort_by(|left, right| left.0.cmp(&right.0));
        Ok::<_, io::Error>(entries)
    };
    let before = snapshot().expect("folder before miss");

    // Another question makes another question key, which this folder does
    // not hold.
    let question = "asks for something else";
    let output = judge(
        question,
        listener.base(),
        &["--replay", &folder.to_string_lossy()],
        None,
    )
    .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(5));
    assert!(output.stdout.is_empty());
    let message = String::from_utf8_lossy(&output.stderr);
    let [expected] = keys(
        &format!("{}/{ENDPOINT_PATH}", listener.base()),
        &encoded_decide(EVIDENCE, "local-1", question),
    )
    .try_into()
    .expect("one question");
    assert_eq!(
        message,
        format!(
            "thinkthen: the decide request for one document: the replay folder holds no answer \
             for question `{expected}`; the key is the SHA-256 of the adapter, address, model, \
             shared state and question as sent\n"
        )
    );
    for secret in [
        question,
        EVIDENCE,
        "sk-test-value",
        listener.base(),
        folder.to_str().expect("UTF-8 folder"),
    ] {
        assert!(!message.contains(secret), "{message}");
    }
    assert_eq!(listener.requests().len(), 0, "a replay opens no connection");
    assert_eq!(snapshot().expect("folder after miss"), before);
}

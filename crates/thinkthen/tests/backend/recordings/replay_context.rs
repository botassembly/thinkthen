//! One strict replay miss at each distinct command-owned request shape.

use std::fs;
use std::path::Path;

use crate::harness::{Listener, spawn};

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
        assert!(stderr.contains(".json`"), "{name}: {stderr}");
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
            assert_eq!(lines.len(), 2, "{stderr}");
            assert!(
                lines[0].starts_with("thinkthen: stopped at record 1; "),
                "{stderr}"
            );
            let facts: serde_json::Value = serde_json::from_str(lines[1]).expect("final facts");
            assert_eq!(facts["stopped"]["cause"], "local");
        }
    }
    assert!(
        listener.requests().is_empty(),
        "strict replay opened a connection"
    );
}

fn cases<'a>(base: &'a str, folder: &'a str, set: &'a str) -> [Case<'a>; 7] {
    [
        (
            "one document",
            vec!["decide", "private-question", "--url", base, "--replay", folder],
            b"private-evidence",
            "the decide request: the replay folder holds no entry",
        ),
        (
            "streaming row",
            vec!["filter", "private-question", "--lines", "--url", base, "--replay", folder],
            b"private-evidence\n",
            "the filter request: the replay folder holds no entry",
        ),
        (
            "shared batch and facts",
            vec!["decide", "private-question", "--lines", "--batch", "2", "--facts", "--url", base, "--replay", folder],
            b"private-evidence\nsecond private-evidence\n",
            "request for records 1 to 2 failed: the decide request: the replay folder holds no entry",
        ),
        (
            "annotate group",
            vec!["annotate", set, "--url", base, "--replay", folder],
            br#"{"body":"private-evidence"}"#,
            "annotate group 1 with 2 members: the replay folder holds no entry",
        ),
        (
            "complete find set",
            vec!["find", "private-question", "--url", base, "--replay", folder],
            b"private-evidence\nsecond private-evidence\n",
            "the complete find set of 2 units: the replay folder holds no entry",
        ),
        (
            "recognize command fallback",
            vec!["recognize", "person", "--url", base, "--replay", folder],
            b"private-evidence",
            "the recognize request: the replay folder holds no entry",
        ),
        (
            "relate command fallback",
            vec!["relate", "works_for=person:organization", "--url", base, "--replay", folder],
            br#"[{"name":"private-evidence","kind":"person"},{"name":"Acme","kind":"organization"}]"#,
            "the relate request: the replay folder holds no entry",
        ),
    ]
}

//! Relate question-file grammar, precedence, and exit classes.

use serde_json::Value;

use super::harness::{CLOSED, run, written};

#[test]
fn file_backed_dry_run_reports_every_independent_precedence_source() {
    let file = written(
        "relate-precedence",
        r#"{"version":1,"relate":{"fields":{"name":"/title","kind":"/type"},"relations":[{"name":"works_for","source":"person","target":"organization","reads":"works for","either":false}]},"threshold":0.6,"model":"file-model","profile":"measured"}"#,
    );
    let output = run(
        &[
            "relate",
            &file,
            "--field",
            "/name",
            "--threshold",
            "0.7",
            "--model",
            "cli-model",
            "--plan",
            "--url",
            CLOSED,
            "--no-cache",
        ],
        br#"[{"name":"Ada","type":"person"},{"name":"Acme","type":"organization"}]"#,
    )
    .expect("binary runs");
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let plan: Value = serde_json::from_slice(
        output
            .stdout
            .split(|byte| *byte == b'\n')
            .next()
            .expect("plan line"),
    )
    .expect("plan");
    assert_eq!(plan["model"], "cli-model");
    assert_eq!(plan["backend_profile"], Value::Null);
    assert_eq!(
        plan["fields"],
        serde_json::json!({"name":"/name","kind":"/type"})
    );
    assert_eq!(
        plan["from"],
        serde_json::json!({
            "question":"file","threshold":"command line","model":"command line",
            "field":"command line","kind_field":"file","profile":"file"
        })
    );

    // A runtime backend profile is reported apart and leaves the saved
    // calibration identity and its provenance alone.
    let runtime = written(
        "relate-runtime-profile",
        r#"{"schema":"thinkthen.backend-profile/1","name":"runtime","max_questions":8}"#,
    );
    let runtime = runtime.strip_prefix('@').unwrap_or(&runtime);
    let profiled = run(
        &[
            "relate",
            &file,
            "--plan",
            "--url",
            CLOSED,
            "--no-cache",
            "--profile",
            runtime,
        ],
        br#"[{"title":"Ada","type":"person"},{"title":"Acme","type":"organization"}]"#,
    )
    .expect("binary runs");
    assert_eq!(
        profiled.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&profiled.stderr)
    );
    let profiled: Value = serde_json::from_slice(
        profiled
            .stdout
            .split(|byte| *byte == b'\n')
            .next()
            .expect("plan line"),
    )
    .expect("plan");
    assert_eq!(profiled["backend_profile"], "runtime");
    assert_eq!(profiled["from"]["profile"], "file");
}

#[test]
fn invalid_unreadable_and_wrong_verb_files_keep_their_ruled_exit_codes() {
    let invalid = written(
        "relate-invalid",
        r#"{"version":1,"relate":{"relations":[{"name":"x","source":"a","target":"b"}]},"extra":true}"#,
    );
    let wrong = written("relate-wrong", r#"{"decide":"Is this valid?"}"#);
    let wrong_recognize = written(
        "relate-wrong-recognize",
        r#"{"version":1,"recognize":{"kinds":{"person":"A person's name."}}}"#,
    );
    let invalid_decide = written(
        "relate-invalid-decide",
        r#"{"decide":"Is this valid?","relations":[]}"#,
    );
    let invalid_recognize = written(
        "relate-invalid-recognize",
        r#"{"version":1,"recognize":{"kinds":{}},"extra":true}"#,
    );
    for (file, code) in [
        (invalid, 5),
        (wrong, 2),
        (wrong_recognize, 2),
        (invalid_decide, 5),
        (invalid_recognize, 5),
        ("@/path/that/does/not/exist.json".to_owned(), 5),
    ] {
        let output = run(&["relate", &file, "--plan"], b"[]").expect("binary runs");
        assert_eq!(output.status.code(), Some(code), "{file}");
        assert!(output.stdout.is_empty());
    }
}

#[test]
#[cfg(unix)]
fn named_recognize_and_relate_keep_saved_fields_and_refuse_bad_records_before_sends() {
    let root = crate::input_sources::folder("named-aggregate-admission").expect("folder");
    let questions = root.join("thinkthen/questions");
    std::fs::create_dir_all(&questions).expect("questions");
    for (verb, name, definition, valid, invalid, flags) in [
        (
            "recognize",
            "names-native",
            r#"{"version":1,"recognize":{"kinds":{"person":null}},"on":"/body","name":"names-native","wording_version":3,"item_schema":{"type":"string"}}"#,
            br#"{"body":"Ada met Grace."}"#.as_slice(),
            br#"{"body":7}"#.as_slice(),
            vec!["--jsonl"],
        ),
        (
            "relate",
            "links-native",
            r#"{"version":1,"relate":{"fields":{"name":"/title","kind":"/type"},"relations":[{"name":"met","source":"person","target":"person"}]},"name":"links-native","wording_version":4,"item_schema":{"type":"object","properties":{"title":{"type":"string"},"type":{"type":"string"}}}}"#,
            br#"[{"title":"Ada","type":"person"},{"title":"Grace","type":"person"}]"#.as_slice(),
            br#"[{"title":7,"type":"person"}]"#.as_slice(),
            vec![],
        ),
    ] {
        let path = questions.join(format!("{name}.json"));
        std::fs::write(&path, definition).expect("saved question");
        let listener = crate::harness::Listener::serving(Vec::new()).expect("listener");
        let named = format!("@{name}");
        let file = format!("@{}", path.display());
        let run = |selector: &str, evidence: &[u8]| {
            let mut args = vec![
                verb,
                selector,
                "--plan",
                "--url",
                listener.base(),
                "--no-cache",
            ];
            args.extend(&flags);
            crate::harness::spawn(
                &args,
                &[("XDG_CONFIG_HOME", root.to_str().expect("path"))],
                evidence,
            )
            .expect("command")
        };
        let direct = run(&file, valid);
        let lookup = run(&named, valid);
        assert_eq!(
            direct.status.code(),
            Some(0),
            "{}",
            String::from_utf8_lossy(&direct.stderr)
        );
        assert_eq!(
            lookup.status.code(),
            Some(0),
            "{}",
            String::from_utf8_lossy(&lookup.stderr)
        );
        assert_eq!(lookup.stdout, direct.stdout);
        let rejected = run(&named, invalid);
        assert_eq!(
            rejected.status.code(),
            Some(2),
            "{}",
            String::from_utf8_lossy(&rejected.stderr)
        );
        assert!(rejected.stdout.is_empty());
        assert_eq!(listener.connections(), 0);
    }
}

#[test]
fn native_relation_admission_keeps_the_cli_complete_set_count_sentence() {
    let listener = crate::harness::Listener::serving(Vec::new()).expect("listener");
    let entities = (0..256)
        .map(|at| format!("entity-{at}\n"))
        .collect::<String>();
    let output = crate::harness::spawn(
        &[
            "relate",
            "met",
            "--lines",
            "--plan",
            "--url",
            listener.base(),
        ],
        &[],
        entities.as_bytes(),
    )
    .expect("command");
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "thinkthen: relate takes at most 255 entities; this set has 256. If all 256 were distinct, an unordered all-kind rule would have 32640 candidate pairs; split the set or narrow by kind\n"
    );
    let empty = crate::harness::spawn(
        &[
            "relate",
            "met=person:person",
            "--lines",
            "--plan",
            "--url",
            listener.base(),
        ],
        &[],
        b"",
    )
    .expect("command");
    assert_eq!(empty.status.code(), Some(2));
    assert_eq!(
        String::from_utf8_lossy(&empty.stderr),
        "thinkthen: --lines takes only bare relation names or NAME=*:*\n"
    );
    assert!(empty.stdout.is_empty());
    assert_eq!(listener.connections(), 0);
}

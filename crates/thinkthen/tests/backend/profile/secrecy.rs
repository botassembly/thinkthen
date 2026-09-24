//! Profile file and limit failures across every command family.

use std::fs;
use std::path::PathBuf;

use crate::harness::spawn;

use super::{file, profile};

#[test]
fn every_command_family_keeps_profile_failures_free_of_keys_and_evidence() {
    const KEY: &str = "PROFILE-KEY-MARKER";
    const EVIDENCE: &str = "PROFILE-EVIDENCE-MARKER";
    let set = file(
        "profile-secrecy-set.json",
        r#"{"version":1,"questions":{"ready":{"decide":"Ready?"}}}"#,
    );
    let invalid = file(
        "profile-secrecy-invalid.json",
        r#"{"schema":"thinkthen.backend-profile/1","name":"safe","max_questions":1,"PROFILE-KEY-MARKER":"PROFILE-EVIDENCE-MARKER"}"#,
    );
    let tiny = profile("profile-secrecy-tiny", r#""max_evidence_bytes":1"#);
    let missing = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join("profiles")
        .join("profile-secrecy-missing.json");
    let _absent = fs::remove_file(&missing);
    let commands = [
        vec!["decide".to_owned(), "Ready?".to_owned()],
        vec![
            "choose".to_owned(),
            "Which?".to_owned(),
            "a".to_owned(),
            "b".to_owned(),
        ],
        vec!["tag".to_owned(), "Which?".to_owned(), "a".to_owned()],
        vec![
            "score".to_owned(),
            "How much?".to_owned(),
            "low".to_owned(),
            "high".to_owned(),
        ],
        vec![
            "filter".to_owned(),
            "Ready?".to_owned(),
            "--lines".to_owned(),
        ],
        vec!["rank".to_owned(), "Ready?".to_owned(), "--lines".to_owned()],
        vec!["find".to_owned(), "Which line?".to_owned()],
        vec!["annotate".to_owned(), set.to_string_lossy().into_owned()],
        vec![
            "relate".to_owned(),
            "linked".to_owned(),
            "--either".to_owned(),
        ],
    ];
    let routes = [
        (missing, 5, "could not be opened"),
        (invalid, 5, "holds no unknown keys"),
        (tiny, 2, "evidence bytes"),
    ];
    for command in commands {
        for (path, code, phrase) in &routes {
            let mut arguments = command.clone();
            // A closed loopback port keeps a refusal that stopped refusing off
            // every real backend.
            arguments.extend([
                "--profile".to_owned(),
                path.to_string_lossy().into_owned(),
                "--url".to_owned(),
                crate::secrecy::CLOSED.to_owned(),
            ]);
            let input = match command.first().map(String::as_str) {
                Some("find") => format!("{EVIDENCE}\nother\n"),
                Some("relate") => format!(
                    r#"[{{"name":"{EVIDENCE}","kind":"record"}},{{"name":"Acme","kind":"record"}}]"#
                ),
                _ => EVIDENCE.to_owned(),
            };
            let borrowed = arguments.iter().map(String::as_str).collect::<Vec<_>>();
            let output =
                spawn(&borrowed, &[("THINKTHEN_API_KEY", KEY)], input.as_bytes()).expect("command");
            assert_eq!(output.status.code(), Some(*code), "{arguments:?}");
            assert!(output.stdout.is_empty(), "{arguments:?}");
            let error = String::from_utf8_lossy(&output.stderr);
            assert!(error.contains(phrase), "{arguments:?}: {error}");
            assert!(!error.contains(KEY), "{arguments:?}: {error}");
            assert!(!error.contains(EVIDENCE), "{arguments:?}: {error}");
        }
    }
}

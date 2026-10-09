//! The command retains whole output and reuses boundary answers across modes.
use super::*;
use serde_json::json;

#[test]
fn saved_boundary_mode_replays_whole_questions_without_later_sends_and_preserves_defaults() {
    let listener = Listener::answering(automatic).unwrap();
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("boundary-mode-{}", std::process::id()));
    fs::create_dir_all(&root).unwrap();
    let recording = root.join("whole-recording");
    let recording = recording.to_string_lossy();
    let whole = local(
        &listener,
        &["person", "--details", "--no-cache", "--record", &recording],
        Some("fake"),
        b"Nobody",
    );
    assert_eq!(whole.status.code(), Some(0));
    assert_eq!(listener.count(), 2);
    assert_eq!(json(&whole)["value"], json!({"entities":[]}));
    let default = local(
        &listener,
        &["person", "--details", "--no-cache", "--replay", &recording],
        None,
        b"Nobody",
    );
    let explicit = local(
        &listener,
        &[
            "person",
            "--mode",
            "whole",
            "--details",
            "--no-cache",
            "--replay",
            &recording,
        ],
        None,
        b"Nobody",
    );
    assert_eq!(default.status.code(), Some(0));
    assert_eq!(explicit.status.code(), Some(0));
    assert_eq!(default.stdout, explicit.stdout);
    assert_eq!(listener.count(), 2);
    let saved = root.join("boundary.json");
    fs::write(
        &saved,
        r#"{"version":1,"recognize":{"mode":"boundary_only","kinds":{"person":null}}}"#,
    )
    .unwrap();
    let operand = format!("@{}", saved.display());
    let proposed = local(
        &listener,
        &[&operand, "--details", "--no-cache", "--replay", &recording],
        None,
        b"Nobody",
    );
    assert_eq!(
        proposed.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&proposed.stderr)
    );
    let proposal = json(&proposed);
    assert_eq!(
        proposal["value"],
        json!({"mode":"boundary_only","proposals":[{"text":"Nobody","start":0,"end":6,"length":6,"probability":1.0}]})
    );
    assert_eq!(
        proposal["answer"]
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        vec!["pieces", "proposals"]
    );
    assert_ne!(proposal["answer_id"], json(&default)["answer_id"]);
    assert_eq!(
        proposal["meta"]["observations"][0],
        json(&default)["meta"]["observations"][0]
    );
    assert_eq!(listener.count(), 2);
    let overridden = local(
        &listener,
        &[
            &operand,
            "--mode",
            "whole",
            "--details",
            "--no-cache",
            "--replay",
            &recording,
        ],
        None,
        b"Nobody",
    );
    assert_eq!(overridden.status.code(), Some(0));
    assert_eq!(overridden.stdout, default.stdout);
    let only_recording = root.join("boundary-recording");
    let only_recording = only_recording.to_string_lossy();
    let fresh = local(
        &listener,
        &[
            "person",
            "--mode",
            "boundary_only",
            "--no-cache",
            "--record",
            &only_recording,
        ],
        Some("fake"),
        b"Ada",
    );
    assert_eq!(fresh.status.code(), Some(0));
    assert_eq!(listener.count(), 3);
    let missing = local(
        &listener,
        &["person", "--no-cache", "--replay", &only_recording],
        None,
        b"Ada",
    );
    assert_eq!(missing.status.code(), Some(5));
    assert!(String::from_utf8_lossy(&missing.stderr).contains("replay"));
    assert_eq!(listener.count(), 3);
    let plan = local(
        &listener,
        &["person", "--mode", "boundary_only", "--plan"],
        None,
        b"Ada",
    );
    assert_eq!(plan.status.code(), Some(0));
    let plan = plan_json(&plan);
    assert_eq!(plan["name_requests_upper_bound"], 0);
    assert_eq!(plan["relation_pairs_upper_bound"], 0);
    assert_eq!(plan["relation_requests_upper_bound"], 0);
    assert_eq!(listener.count(), 3);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn forbidden_boundary_controls_and_bad_mode_refuse_before_inputs_or_sends() {
    let listener = Listener::answering(automatic).unwrap();
    for control in [
        vec!["--relation", "works_for"],
        vec!["--relation-threshold", "0.5"],
        vec!["--kind-edge-context", ""],
        vec!["--relation-context", ""],
    ] {
        let arguments = [
            &[
                "person",
                "--mode",
                "boundary_only",
                "--input",
                "missing-boundary-evidence",
            ][..],
            &control,
        ]
        .concat();
        let refused = local(&listener, &arguments, Some("fake"), b"");
        assert_eq!(refused.status.code(), Some(2));
        assert!(String::from_utf8_lossy(&refused.stderr).contains("boundary_only recognition takes no relations, relation threshold, kind_edge context or relation context"));
    }
    let bad = local(
        &listener,
        &["--mode", "unknown", "--input", "missing-boundary-evidence"],
        Some("fake"),
        b"",
    );
    assert_eq!(bad.status.code(), Some(2));
    assert_eq!(listener.count(), 0);
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("boundary-invalid-{}", std::process::id()));
    fs::create_dir_all(&root).unwrap();
    for (at, controls) in [
        r#""relations":[]"#,
        r#""stage_context":{"kind_edge":""}"#,
        r#""stage_context":{"relation":""}"#,
    ]
    .iter()
    .enumerate()
    {
        let file = root.join(format!("{at}.json"));
        fs::write(
            &file,
            format!(r#"{{"version":1,"recognize":{{"mode":"boundary_only",{controls}}}}}"#),
        )
        .unwrap();
        let operand = format!("@{}", file.display());
        let refused = local(
            &listener,
            &[&operand, "--input", "missing-boundary-evidence"],
            Some("fake"),
            b"",
        );
        assert_eq!(
            refused.status.code(),
            Some(5),
            "{}",
            String::from_utf8_lossy(&refused.stderr)
        );
        assert!(
            String::from_utf8_lossy(&refused.stderr)
                .contains("boundary_only recognition takes no relations")
        );
    }
    assert_eq!(listener.count(), 0);
    fs::remove_dir_all(root).unwrap();
}

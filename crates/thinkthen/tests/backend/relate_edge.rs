//! The public relate command boundary.
#![cfg(feature = "cli")]

use crate::child::ChildEnvironment as _;
use std::process::Command;

use crate::run;

/// A name `recognize` found carries `text` in place of `name`, and `relate`
/// reads it as the name. `name` wins when a record holds both.
#[test]
fn relate_reads_the_names_recognize_found() {
    let demo =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../demos/44-recognize-names");
    let folder = std::env::temp_dir().join(format!("thinkthen-0147-relate-{}", std::process::id()));
    std::fs::create_dir_all(&folder).expect("folder");
    let message = std::fs::read_to_string(demo.join("message.txt")).expect("message");
    std::fs::write(folder.join("text"), message.replace('\n', "")).expect("text");
    // The kinds match demo 44's recording byte for byte, so the replay answers.
    let recording = demo.join("recording");
    let recognize = [
        "recognize",
        "--replay",
        recording.to_str().expect("path"),
        "--kind",
        "PER=Part of a person's name.",
        "--kind",
        "ORG=Part of the name of an organization: a company, band, team, agency, government body, or media outlet.",
        "--kind",
        "LOC=Part of the name of a place: a country, region, city, or geographic feature.",
        "--kind",
        "MISC=Part of another named entity: a nationality, an event, a product, or the name of a creative work.",
    ];
    let thinkthen = |arguments: &[&str], input: &str| {
        let output = run::output(
            Command::new(env!("CARGO_BIN_EXE_thinkthen"))
                .clear_environment()
                .args(arguments)
                .args(["--url", "https://api.typesafe.ai/v1", "--input"])
                .arg(folder.join(input)),
        )
        .expect("binary runs");
        let error = String::from_utf8_lossy(&output.stderr);
        assert!(output.status.success(), "{error}");
        // A plan's second line counts records; the first holds the request.
        let first = output.stdout.split(|byte| *byte == b'\n').next();
        serde_json::from_slice::<serde_json::Value>(first.unwrap_or_default()).expect("json")
    };
    let found = thinkthen(&recognize, "text");
    let mut lines: Vec<String> = found["entities"]
        .as_array()
        .expect("entities")
        .iter()
        .map(ToString::to_string)
        .collect();
    lines.push(r#"{"name":"Ana Lima","text":"not this","kind":"PER"}"#.to_owned());
    std::fs::write(folder.join("entities"), lines.join("\n")).expect("entities");
    let plan = thinkthen(
        &["relate", "works_for=PER:ORG", "--plan", "--jsonl"],
        "entities",
    );
    let body = plan["requests"][0]["body_utf8"].as_str().expect("body");
    let state: serde_json::Value = serde_json::from_str(body).expect("json");
    let names = state["state"]["entities"].as_array().expect("entities");
    let names: Vec<&str> = names
        .iter()
        .filter_map(|one| one["name"].as_str())
        .collect();
    // The state holds only names whose kinds a rule names, so the place stays out.
    let want = ["Maria Chen", "Northwind Freight", "Ana Lima"];
    assert_eq!(names, want);
    std::fs::remove_dir_all(&folder).expect("cleanup");
}

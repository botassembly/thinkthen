//! The public relate command boundary.
#![cfg(feature = "cli")]

use std::process::Command;

#[path = "../src/test_deadline/run.rs"]
mod run;
#[path = "../src/test_deadline/wait.rs"]
mod wait;

#[test]
fn help_names_the_beta_complete_set_and_secrecy_contract() {
    let output = run::output(
        Command::new(env!("CARGO_BIN_EXE_thinkthen"))
            .env_clear()
            .args(["relate", "--help"]),
    )
    .expect("binary runs");
    assert_eq!(output.status.code(), Some(0));
    let help = String::from_utf8_lossy(&output.stdout);
    for required in [
        "Relations are beta",
        "complete entity set",
        "NAME=SOURCE_KIND:TARGET_KIND",
        "--kind-field",
        "Entries contain the judged text",
    ] {
        assert!(help.contains(required), "{required}\n{help}");
    }
    assert!(
        help.contains(concat!(
            "\n\nA run makes paid requests. A relation between two kinds asks one question for every ",
            "entity of the larger kind, or of the source kind when the counts are equal. A same-kind ",
            "relation asks one yes-or-no question for every pair, in both directions unless --either. ",
            "--plan prints the questions and requests and sends nothing.\n\n",
        )),
        "{help}"
    );
    assert!(
        help.contains(concat!(
            "It acts in record mode, on `annotate`, where a single text can make several grouped ",
            "requests, and on `relate`, where each relation makes its own requests. Output follows ",
            "the order the command defines.",
        )),
        "{help}"
    );
}

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
    let readme = std::fs::read_to_string(demo.join("README.md")).expect("readme");
    let kinds: Vec<&str> = readme
        .lines()
        .find(|line| line.contains(" recognize "))
        .expect("the command")
        .split(" --kind ")
        .skip(1)
        .collect();
    let mut recognize = format!(
        "--replay {} --kind {}",
        demo.join("recording").display(),
        kinds.join(" --kind ")
    );
    recognize.truncate(recognize.find(" | jq").expect("the pipe"));
    let thinkthen = |command: &str, input: &str| {
        let output = run::output(
            Command::new("sh")
                .env_clear()
                .arg("-c")
                .arg(format!(
                    "\"$0\" {command} --url https://api.typesafe.ai/v1 --input \"$1\""
                ))
                .args([
                    env!("CARGO_BIN_EXE_thinkthen"),
                    folder.join(input).to_str().expect("path"),
                ]),
        )
        .expect("binary runs");
        let error = String::from_utf8_lossy(&output.stderr);
        assert!(output.status.success(), "{error}");
        serde_json::from_slice::<serde_json::Value>(&output.stdout).expect("json")
    };
    let found = thinkthen(&format!("recognize {recognize}"), "text");
    let mut lines: Vec<String> = found["entities"]
        .as_array()
        .expect("entities")
        .iter()
        .map(ToString::to_string)
        .collect();
    lines.push(r#"{"name":"Ana Lima","text":"not this","kind":"PER"}"#.to_owned());
    std::fs::write(folder.join("entities"), lines.join("\n")).expect("entities");
    let plan = thinkthen("relate works_for=PER:ORG --plan --jsonl", "entities");
    let body = plan["requests"][0]["body_utf8"].as_str().expect("body");
    let state: serde_json::Value = serde_json::from_str(body).expect("json");
    let names = state["state"]["entities"].as_array().expect("entities");
    let names: Vec<&str> = names
        .iter()
        .filter_map(|one| one["name"].as_str())
        .collect();
    let want = ["Maria Chen", "Northwind Freight", "Chicago", "Ana Lima"];
    assert_eq!(names, want);
    std::fs::remove_dir_all(&folder).expect("cleanup");
}

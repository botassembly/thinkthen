#![cfg(test)]
#![cfg(feature = "command")]
//! Help and file diagnostics teach the current command grammar.

mod support;

#[test]
fn help_lists_current_options_without_retired_flags_or_settings() {
    for name in [
        None,
        Some("decide"),
        Some("filter"),
        Some("choose"),
        Some("tag"),
        Some("score"),
        Some("rank"),
        Some("find"),
        Some("annotate"),
        Some("recognize"),
        Some("relate"),
        Some("run"),
    ] {
        let mut command = support::command();
        if let Some(name) = name {
            command.arg(name);
        }
        let output = command.arg("--help").output().unwrap();
        assert!(output.status.success(), "{name:?}: {output:?}");
        let help = String::from_utf8(output.stdout).unwrap();
        for current in ["--backend", "--evidence", "--from", "--json"] {
            assert!(help.contains(current), "{name:?}: missing {current}");
        }
        for retired in [
            "--on",
            "--details",
            "--adapter",
            "--input",
            "--lines",
            "--csv",
            "--tsv",
            "--xlsx",
            "--option",
            "THINKTHEN_ON",
            "THINKTHEN_DETAILS",
            "THINKTHEN_ADAPTER",
            "THINKTHEN_INPUT",
            "THINKTHEN_OPTIONS",
        ] {
            assert!(!help.contains(retired), "{name:?}: advertises {retired}");
        }
    }
}

#[test]
fn single_column_diagnostic_names_the_current_evidence_option() {
    let directory = tempfile::tempdir().unwrap();
    let input = directory.path().join("requests.csv");
    std::fs::write(&input, "Body\nready\n").unwrap();
    let output = support::command()
        .args(["decide", "Ready?", "--backend", "stub"])
        .arg(&input)
        .args(["--column", "Body", "--into", "ready", "--evidence", "/Body"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2), "{output:?}");
    assert!(output.stdout.is_empty());
    let diagnostic = String::from_utf8(output.stderr).unwrap();
    assert!(diagnostic.contains("usage.question"), "{diagnostic}");
    assert!(diagnostic.contains("omit --evidence"), "{diagnostic}");
    assert!(!diagnostic.contains("--on"), "{diagnostic}");
    assert!(!diagnostic.contains("--details"), "{diagnostic}");
    assert!(!directory.path().join("requests.thinkthen.csv").exists());
}

#[test]
fn top_level_help_describes_every_function() {
    let output = support::command().arg("--help").output().unwrap();
    assert!(output.status.success());
    let help = String::from_utf8(output.stdout).unwrap();
    for function in thinkthen::FunctionName::ALL {
        let name = serde_json::to_value(function).unwrap();
        let name = name.as_str().unwrap();
        let line = help
            .lines()
            .find(|line| line.trim_start().starts_with(&format!("{name} ")))
            .unwrap();
        assert!(line.split_whitespace().count() > 2, "{line}");
    }
}

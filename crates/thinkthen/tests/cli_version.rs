#![cfg(test)]
//! The command reports its version.
#![cfg(feature = "command")]

mod support;

#[test]
fn version_prints_name_and_version() {
    let output = support::command().arg("--version").output().unwrap();
    assert_eq!(output.status.code(), Some(0));
    let banner = String::from_utf8(output.stdout).unwrap();
    assert_eq!(banner, format!("thinkthen {}\n", thinkthen::VERSION));
    assert!(output.stderr.is_empty());
    let help = support::command().arg("--help").output().unwrap();
    let help = String::from_utf8(help.stdout).unwrap();
    assert!(help.contains("Backends:"));
    assert!(!help.contains("stub"));
    for backend in [
        thinkthen::BackendName::SystemOne,
        thinkthen::BackendName::OpenAi,
    ] {
        assert!(help.contains(backend.as_str()));
        assert!(help.contains(&backend.default_model()));
    }
}

#[test]
fn unknown_backend_lists_only_advertised_providers() {
    let output = support::command()
        .args(["decide", "Ready?", "x", "--backend", "unknown"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    let error = String::from_utf8(output.stderr).unwrap();
    assert!(error.contains("usage.backend"));
    assert!(error.contains("systemone") && error.contains("openai"));
    assert!(!error.contains("stub"));
}

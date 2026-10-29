#![cfg(test)]
#![cfg(feature = "command")]
//! Global command options retain function behavior and reject incompatible forms.
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;
mod support;
use serde_json::json;
use std::io::Write;
use std::process::{Command, Output, Stdio};

fn run(command: &mut Command, input: &[u8]) -> Output {
    let mut child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let _ = child.stdin.take().unwrap().write_all(input);
    child.wait_with_output().unwrap()
}
fn functions(directory: &std::path::Path) -> Vec<Vec<String>> {
    let annotation = directory.join("questions.json");
    std::fs::write(
        &annotation,
        json!({"questions":{"ready":{"function":"decide","question":"Ready?"}}}).to_string(),
    )
    .unwrap();
    [
        vec!["decide", "Ready?"],
        vec!["filter", "Ready?"],
        vec!["choose", "Which?", "--choice", "a", "--choice", "b"],
        vec!["tag", "Which?", "--label", "a"],
        vec!["score", "How?", "--level", "low", "--level", "high"],
        vec!["rank", "How?", "--level", "low", "--level", "high"],
        vec!["find", "Where?"],
        vec!["recognize", "--kind", "person"],
        vec!["relate", "--rule", "works_for", "person", "organization"],
        vec!["annotate", annotation.to_str().unwrap()],
    ]
    .into_iter()
    .map(|args| args.into_iter().map(str::to_owned).collect())
    .collect()
}
#[test]
fn quiet_and_short_verbose_apply_to_every_function() {
    let directory = tempfile::tempdir().unwrap();
    for args in functions(directory.path()) {
        let baseline = run(
            support::command()
                .args(["--backend", "stub", "--json"])
                .args(&args),
            b"Ada joined Acme.",
        );
        assert_eq!(baseline.status.code(), Some(0), "{args:?}: {baseline:?}");
        let quiet = run(
            support::command()
                .args(["--backend", "stub", "--json", "--quiet"])
                .args(&args)
                .env("THINKTHEN_VERBOSE", "true"),
            b"Ada joined Acme.",
        );
        assert_eq!(quiet.status.code(), Some(0));
        assert_eq!(quiet.stdout, baseline.stdout);
        assert!(quiet.stderr.is_empty());
        let verbose = run(
            support::command()
                .args(["--backend", "stub", "--json", "-v"])
                .args(&args)
                .env("THINKTHEN_QUIET", "true"),
            b"Ada joined Acme.",
        );
        assert_eq!(verbose.status.code(), Some(0));
        assert_eq!(verbose.stdout, baseline.stdout);
        assert!(
            String::from_utf8_lossy(&verbose.stderr).contains("1 rows, 0 failed,"),
            "{args:?}: {verbose:?}"
        );
    }
}
#[test]
fn threshold_and_band_conflicts_fail_without_a_paid_request() {
    let server = loopback::Loopback::noul(0.9).unwrap();
    for args in functions(tempfile::tempdir().unwrap().path())
        .into_iter()
        .filter(|a| matches!(a[0].as_str(), "decide" | "filter" | "choose"))
    {
        for band in [
            vec!["--low", "0.2"],
            vec!["--high", "0.8"],
            vec!["--low", "0.2", "--high", "0.8"],
        ] {
            let output = run(
                support::command()
                    .args(&args)
                    .args(["--threshold", "0.5"])
                    .args(band)
                    .env("THINKTHEN_BASE_URL", server.base())
                    .env("THINKTHEN_API_KEY", loopback::KEY),
                b"evidence",
            );
            assert_eq!(output.status.code(), Some(2), "{args:?}: {output:?}");
            assert!(output.stdout.is_empty());
        }
    }
    assert_eq!(server.count(), 0);
}
#[test]
fn unsupported_check_functions_refuse_the_option_before_sending() {
    let directory = tempfile::tempdir().unwrap();
    let server = loopback::Loopback::noul(0.9).unwrap();
    let mut checked = 0;
    for args in functions(directory.path())
        .into_iter()
        .filter(|a| !matches!(a[0].as_str(), "decide" | "filter" | "choose" | "find"))
    {
        let output = run(
            support::command()
                .args(&args)
                .arg("--check")
                .env("THINKTHEN_BASE_URL", server.base())
                .env("THINKTHEN_API_KEY", loopback::KEY),
            b"evidence",
        );
        assert_eq!(output.status.code(), Some(2), "{args:?}: {output:?}");
        assert!(output.stdout.is_empty());
        checked += 1;
    }
    assert_eq!(checked, 6);
    assert_eq!(server.count(), 0);
}
#[test]
fn filter_check_reduces_yes_no_and_unsure() {
    for (probability, expected) in [(0.9, 0), (0.1, 1), (0.5, 3)] {
        let server = loopback::Loopback::noul(probability).unwrap();
        let output = run(
            support::command()
                .args([
                    "filter", "Ready?", "--low", "0.2", "--high", "0.8", "--check",
                ])
                .env("THINKTHEN_BASE_URL", server.base())
                .env("THINKTHEN_API_KEY", loopback::KEY),
            b"evidence\n",
        );
        assert_eq!(output.status.code(), Some(expected), "{output:?}");
        assert_eq!(server.count(), 1);
    }
}
#[test]
fn scalar_environment_options_match_their_explicit_forms() {
    for (variable, value, flags, input) in [
        (
            "THINKTHEN_BACKEND",
            "stub",
            vec!["--backend", "stub"],
            "evidence",
        ),
        (
            "THINKTHEN_MODEL",
            "fixture",
            vec!["--model", "fixture"],
            "evidence",
        ),
        ("THINKTHEN_BATCH", "1", vec!["--batch", "1"], "evidence"),
        (
            "THINKTHEN_FROM",
            "jsonl",
            vec!["--from", "jsonl"],
            "\"evidence\"\n",
        ),
        (
            "THINKTHEN_EVIDENCE",
            "/body",
            vec!["--evidence", "/body"],
            "{\"body\":\"evidence\"}",
        ),
        (
            "THINKTHEN_FORMAT",
            "json",
            vec!["--format", "json"],
            "evidence",
        ),
        ("THINKTHEN_JSON", "true", vec!["--json"], "evidence"),
        (
            "THINKTHEN_THRESHOLD",
            "0.8",
            vec!["--threshold", "0.8"],
            "evidence",
        ),
        (
            "THINKTHEN_LOW",
            "0.2",
            vec!["--low", "0.2", "--high", "0.8"],
            "evidence",
        ),
        (
            "THINKTHEN_HIGH",
            "0.8",
            vec!["--low", "0.2", "--high", "0.8"],
            "evidence",
        ),
        ("THINKTHEN_CHECK", "true", vec!["--check"], "evidence"),
        (
            "THINKTHEN_NAME",
            "named",
            vec!["--name", "named"],
            "evidence",
        ),
        (
            "THINKTHEN_COLOR",
            "never",
            vec!["--color", "never"],
            "evidence",
        ),
        ("THINKTHEN_QUIET", "true", vec!["--quiet"], "evidence"),
    ] {
        let mut base = vec!["decide", "Ready?"];
        if variable != "THINKTHEN_BACKEND" {
            base.extend(["--backend", "stub"]);
        }
        if !matches!(variable, "THINKTHEN_FORMAT" | "THINKTHEN_JSON") {
            base.push("--json");
        }
        let expected = run(support::command().args(&base).args(flags), input.as_bytes());
        let mut actual = support::command();
        actual.args(base).env(variable, value);
        if variable == "THINKTHEN_LOW" {
            actual.env("THINKTHEN_HIGH", "0.8");
        }
        if variable == "THINKTHEN_HIGH" {
            actual.env("THINKTHEN_LOW", "0.2");
        }
        let actual = run(&mut actual, input.as_bytes());
        assert_eq!(
            actual.status.code(),
            expected.status.code(),
            "{variable}: {actual:?}"
        );
        assert_eq!(actual.stdout, expected.stdout, "{variable}");
        assert_eq!(actual.stderr, expected.stderr, "{variable}");
    }
}

#[test]
fn function_environment_options_retain_their_typed_values() {
    for (args, variable, value, flags) in [
        (
            vec![
                "rank", "How?", "--level", "low", "--level", "high", "--from", "lines",
            ],
            "THINKTHEN_TOP",
            "1",
            vec!["--top", "1"],
        ),
        (
            vec!["relate", "--rule", "works_for", "person", "organization"],
            "THINKTHEN_EITHER",
            "true",
            vec!["--either"],
        ),
    ] {
        let input = if variable == "THINKTHEN_TOP" {
            b"first\nsecond\nthird\n".as_slice()
        } else {
            b"Ada joined Acme.".as_slice()
        };
        let expected = run(
            support::command()
                .args(["--backend", "stub", "--json"])
                .args(&args)
                .args(flags),
            input,
        );
        let actual = run(
            support::command()
                .args(["--backend", "stub", "--json"])
                .args(args)
                .env(variable, value),
            input,
        );
        assert_eq!(actual.status.code(), Some(0), "{variable}: {actual:?}");
        assert_eq!(actual.stdout, expected.stdout, "{variable}");
    }
    let output = run(
        support::command()
            .args(["decide", "Ready?", "--backend", "stub"])
            .env("THINKTHEN_VERBOSE", "true"),
        b"evidence",
    );
    assert_eq!(output.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&output.stderr).contains("1 rows, 0 failed,"));
    let conflict = run(
        support::command()
            .args(["decide", "Ready?", "--backend", "stub"])
            .env("THINKTHEN_QUIET", "true")
            .env("THINKTHEN_VERBOSE", "true"),
        b"evidence",
    );
    assert_eq!(conflict.status.code(), Some(2));
    assert!(conflict.stdout.is_empty());
    let directory = tempfile::tempdir().unwrap();
    let saved = directory.path().join("saved.json");
    std::fs::write(&saved,json!({"entries":{"ready":{"question":{"function":"decide","question":"Ready?"}},"other":{"question":{"function":"decide","question":"Other?"}}}}).to_string()).unwrap();
    let expected = run(
        support::command()
            .args(["run", "--backend", "stub", "--json"])
            .arg(&saved)
            .args(["--entry", "ready"]),
        b"\"evidence\"\n",
    );
    let actual = run(
        support::command()
            .args(["run", "--backend", "stub", "--json"])
            .arg(&saved)
            .env("THINKTHEN_ENTRY", "ready"),
        b"\"evidence\"\n",
    );
    assert_eq!(actual.status.code(), Some(0), "{actual:?}");
    assert_eq!(actual.stdout, expected.stdout);
}
#[test]
fn saved_run_transport_setup_failure_exits_four() {
    let refused = loopback::UnavailableBackend::new().unwrap();
    let directory = tempfile::tempdir().unwrap();
    let saved = directory.path().join("saved.json");
    std::fs::write(
        &saved,
        json!({"question":{"function":"decide","question":"Ready?"}}).to_string(),
    )
    .unwrap();
    let output = run(
        support::command()
            .arg("run")
            .arg(&saved)
            .env("THINKTHEN_BASE_URL", refused.base())
            .env("THINKTHEN_API_KEY", loopback::KEY),
        b"\"evidence\"\n",
    );
    assert_eq!(output.status.code(), Some(4), "{output:?}");
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("backend.transport"));
}
#[test]
fn certificate_file_environment_is_checked_before_tls_or_credentials_are_sent() {
    use std::io::Read;
    let listener = loopback::listener().unwrap();
    let address = listener.local_addr().unwrap();
    let (sent, received) = std::sync::mpsc::channel();
    let peer = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        stream
            .set_read_timeout(Some(std::time::Duration::from_secs(30)))
            .unwrap();
        sent.send(stream.read(&mut [0; 4096]).unwrap()).unwrap();
    });
    let directory = tempfile::tempdir().unwrap();
    let output = run(
        support::command()
            .args(["decide", "Ready?"])
            .env("THINKTHEN_BASE_URL", format!("https://{address}/v1"))
            .env("THINKTHEN_API_KEY", loopback::KEY)
            .env("THINKTHEN_CA_FILE", directory.path().join("missing.pem")),
        b"evidence",
    );
    assert_eq!(output.status.code(), Some(4), "{output:?}");
    assert!(String::from_utf8_lossy(&output.stderr).contains("THINKTHEN_CA_FILE"));
    assert_eq!(
        received
            .recv_timeout(std::time::Duration::from_secs(30))
            .unwrap(),
        0
    );
    peer.join().unwrap();
}

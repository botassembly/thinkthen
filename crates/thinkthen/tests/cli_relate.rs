#![cfg(feature = "command")]
#![cfg(test)]
//! Relation command rules, framing and output.
mod support;
use loopback::{KEY, Loopback};
use serde_json::{Value, json};
use std::io::Write;
use std::process::{Output, Stdio};
fn run(server: &Loopback, args: &[&str], input: &str) -> Output {
    let mut child = support::command()
        .arg("relate")
        .args(args)
        .env("THINKTHEN_URL", server.base())
        .env("THINKTHEN_API_KEY", KEY)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    child.wait_with_output().unwrap()
}
fn server(p: f64) -> Loopback {
    Loopback::answering(move |q| {
        let words = q["instructions"].as_str().unwrap();
        if q["criteria"].get("BEGIN").is_some() {
            let named = words.contains("[[Ada]]") || words.contains("[[Acme]]");
            json!({"type":"choice","probabilities":{"BEGIN":0.,"INSIDE":0.,"END":0.,"SINGLE":if named {1.} else {0.},"OUT":if named {0.} else {1.}}})
        } else if q.get("criteria").is_some() {
            let company = words.contains("[[Acme]]");
            json!({"type":"choice","probabilities":{"person":if company {0.} else {1.},"company":if company {1.} else {0.},"none of these":0.}})
        } else {json!({"type":"noul","noul":p})}
    }).unwrap()
}
#[test]
fn bare_output_has_one_relation_per_line_and_empty_success() {
    for (p, args, expected) in [
        (0.8, vec!["--rule", "works_for", "person", "company"], true),
        (0.4, vec!["--rule", "works_for", "person", "company"], false),
        (
            0.8,
            vec!["--rule", "works_for", "person", "company", "--either"],
            true,
        ),
    ] {
        let server = server(p);
        let output = run(&server, &args, "Ada joined Acme");
        assert_eq!(
            output.status.code(),
            Some(0),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(output.stderr.is_empty());
        if expected {
            let relation: Value = serde_json::from_slice(&output.stdout).unwrap();
            assert_eq!(relation["relation"], "works_for");
            assert_eq!(relation["source"]["text"], "Ada");
            assert_eq!(relation["target"]["text"], "Acme");
            assert_eq!(relation["probability"], p);
            assert_eq!(
                relation.get("either"),
                args.contains(&"--either").then_some(&json!(true))
            );
        } else {
            assert!(output.stdout.is_empty());
        }
    }
}
#[test]
fn details_and_record_streams_keep_rows() {
    for (args, input, count) in [
        (vec!["--json"], "Ada joined Acme", 1),
        (
            vec!["--from", "lines", "--json"],
            "Ada joined Acme\nnobody\n",
            2,
        ),
        (
            vec!["--from", "jsonl", "--json", "--evidence", "/body"],
            "{\"body\":\"Ada joined Acme\"}\n",
            1,
        ),
    ] {
        let server = server(0.8);
        let mut command = vec!["--rule", "works_for", "person", "company"];
        command.extend(args);
        let output = run(&server, &command, input);
        assert_eq!(
            output.status.code(),
            Some(0),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let rows: Vec<Value> = String::from_utf8(output.stdout)
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
        assert_eq!(rows.len(), count);
        assert!(rows[0].get("probability").is_none());
        assert_eq!(rows[0]["answer"][0]["relation"], "works_for");
        assert_eq!(rows[0]["detail"]["pairs"][0]["probability"], 0.8);
    }
}
#[test]
fn invalid_rules_fail_before_sending() {
    for args in [
        vec!["works_for"],
        vec![
            "--rule",
            "works_for",
            "person",
            "company",
            "--rule",
            "works_for",
            "person",
            "company",
        ],
        vec![
            "--rule",
            "works_for",
            "person",
            "company",
            "also=company:person",
            "--threshold",
            "bad",
        ],
        vec!["works_for=*:company"],
        vec![
            "--rule",
            "works_for",
            "person",
            "company",
            "--threshold",
            "0.2:0.8",
        ],
    ] {
        let server = self::server(0.8);
        let output = run(&server, &args, "Ada joined Acme");
        assert_eq!(output.status.code(), Some(2));
        assert_eq!(server.count(), 0);
    }
}

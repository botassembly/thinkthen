//! Relation-specific secrecy across complete-set framings and failure routes.

use std::fs;

use serde_json::Value;

use crate::harness::{Canned, Listener, spawn};
use crate::secrecy::{EVIDENCE, KEY, environment, folder, nothing_leaked};

fn answer(body: &[u8]) -> Canned {
    let request: Value = serde_json::from_slice(body).expect("request");
    let answers = request["questions"]
        .as_object()
        .expect("questions")
        .iter()
        .map(|(name, question)| {
            let answer = if question["type"] == "noul" {
                serde_json::json!({"type":"noul","noul":0.9})
            } else {
                let labels = question["criteria"].as_object().expect("criteria");
                let picked = labels
                    .keys()
                    .find(|label| label.starts_with('i'))
                    .expect("entity option");
                let probabilities = labels
                    .keys()
                    .map(|label| (label.clone(), Value::from(f64::from(label == picked))))
                    .collect::<serde_json::Map<_, _>>();
                serde_json::json!({"type":"choice","choice":picked,"probabilities":probabilities})
            };
            (name.clone(), answer)
        })
        .collect::<serde_json::Map<_, _>>();
    Canned::ok(&serde_json::json!({"model":"local-1","answers":answers}).to_string())
}

fn framing(name: &str) -> (&'static str, &'static [&'static str], Vec<u8>) {
    match name {
        "document" => (
            "linked=person:organization",
            &[],
            format!(r#"[{{"name":"{EVIDENCE}","kind":"person"}},{{"name":"Acme","kind":"organization"}}]"#).into_bytes(),
        ),
        "lines" => ("linked", &["--lines"], format!("{EVIDENCE}\nAcme\n").into_bytes()),
        "jsonl" => (
            "linked=person:organization",
            &["--jsonl"],
            format!("{{\"name\":\"{EVIDENCE}\",\"kind\":\"person\"}}\n{{\"name\":\"Acme\",\"kind\":\"organization\"}}\n").into_bytes(),
        ),
        "csv" => (
            "linked=person:organization",
            &["--csv"],
            format!("name,kind\n{EVIDENCE},person\nAcme,organization\n").into_bytes(),
        ),
        "tsv" => (
            "linked=person:organization",
            &["--tsv"],
            format!("name\tkind\n{EVIDENCE}\tperson\nAcme\torganization\n").into_bytes(),
        ),
        _ => unreachable!("known framing"),
    }
}

#[test]
fn every_complete_set_framing_keeps_the_key_to_authorization_and_replays_safely() {
    for name in ["document", "lines", "jsonl", "csv", "tsv"] {
        let (relation, flags, input) = framing(name);
        let into = folder(&format!("relate-{name}")).expect("folder");
        let recording = into.join("recording");
        let listener = Listener::answering(answer).expect("listener");
        let mut arguments = vec![
            "relate",
            relation,
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--record",
            recording.to_str().expect("path"),
            "--no-cache",
            "--details",
        ];
        arguments.extend_from_slice(flags);
        let output = spawn(&arguments, &environment(true), &input).expect("record run");
        assert_eq!(output.status.code(), Some(0), "{name}");
        let requests = listener.requests();
        assert!(!requests.is_empty(), "{name}");
        for request in requests {
            assert_eq!(
                request.header("authorization"),
                Some(format!("Bearer {KEY}").as_str())
            );
        }
        nothing_leaked(name, &output, &into);

        let replay = arguments
            .iter()
            .map(|argument| {
                if *argument == "--record" {
                    "--replay"
                } else {
                    argument
                }
            })
            .collect::<Vec<_>>();
        let output = spawn(&replay, &[], &input).expect("replay run");
        assert_eq!(output.status.code(), Some(0), "{name}");
        assert!(listener.requests().is_empty(), "{name}");
        nothing_leaked(&format!("{name} replay"), &output, &into);
    }
}

#[test]
fn backend_and_local_failures_never_quote_relation_evidence() {
    let input = format!(
        r#"[{{"name":"{EVIDENCE}","kind":"person"}},{{"name":"Acme","kind":"organization"}}]"#
    );
    let routes = [
        (
            Canned::status(400, &format!(r#"{{"error":"{EVIDENCE}"}}"#)),
            4,
        ),
        (
            Canned::ok(&format!(r#"{{"model":"local-1","answers":"{EVIDENCE}"}}"#)),
            4,
        ),
    ];
    for (place, (response, code)) in routes.into_iter().enumerate() {
        let into = folder(&format!("relate-failure-{place}")).expect("folder");
        let listener = Listener::serving(vec![response]).expect("listener");
        let output = spawn(
            &[
                "relate",
                "linked=person:organization",
                "--url",
                listener.base(),
                "--model",
                "local-1",
                "--no-cache",
                "--details",
            ],
            &environment(true),
            input.as_bytes(),
        )
        .expect("failed run");
        assert_eq!(output.status.code(), Some(code));
        nothing_leaked(&format!("relate failure {place}"), &output, &into);
    }

    let into = folder("relate-replay-miss").expect("folder");
    let replay = into.join("recording");
    fs::create_dir(&replay).expect("recording folder");
    let output = spawn(
        &[
            "relate",
            "linked=person:organization",
            "--url",
            "https://api.typesafe.ai/v1",
            "--model",
            "local-1",
            "--replay",
            replay.to_str().expect("path"),
            "--no-cache",
        ],
        &[],
        input.as_bytes(),
    )
    .expect("replay miss");
    assert_eq!(output.status.code(), Some(5));
    nothing_leaked("relate replay miss", &output, &into);

    let into = folder("relate-partial").expect("folder");
    let listener = Listener::answering(|body| {
        let request: Value = serde_json::from_slice(body).expect("request");
        let names = request["questions"]
            .as_object()
            .expect("questions")
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        Canned::ok(
            &serde_json::json!({"model":"local-1","answers":{
                names[0].clone(): {"type":"noul","noul":0.9},
                names[1].clone(): {"type":"choice","probabilities":{"wrong":1.0}}
            }})
            .to_string(),
        )
    })
    .expect("listener");
    let output = spawn(
        &[
            "relate",
            "linked=person:person",
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--no-cache",
            "--details",
        ],
        &environment(true),
        format!(r#"[{{"name":"{EVIDENCE}","kind":"person"}},{{"name":"Ada","kind":"person"}}]"#)
            .as_bytes(),
    )
    .expect("partial run");
    assert_eq!(output.status.code(), Some(6));
    nothing_leaked("relate partial", &output, &into);
}

#[test]
fn relation_refusals_and_plans_send_nothing_and_read_no_key() {
    let listener = Listener::answering(answer).expect("listener");
    let input = format!(r#"[{{"name":"{EVIDENCE}","kind":"person"}}]"#);
    for additions in [
        vec!["linked=person:organization", "--field", "/missing"],
        vec!["linked=person:organization:extra"],
    ] {
        let mut arguments = vec!["relate"];
        arguments.extend(additions);
        arguments.extend(["--url", listener.base(), "--model", "local-1", "--no-cache"]);
        let output = spawn(&arguments, &[], input.as_bytes()).expect("local route");
        assert_eq!(output.status.code(), Some(2), "{arguments:?}");
        assert!(!String::from_utf8_lossy(&output.stderr).contains(EVIDENCE));
    }
    let complete = format!(
        r#"[{{"name":"{EVIDENCE}","kind":"person"}},{{"name":"Acme","kind":"organization"}}]"#
    );
    let common = [
        "relate",
        "linked=person:organization",
        "--url",
        listener.base(),
        "--model",
        "local-1",
        "--no-cache",
    ];
    let mut plan = common.to_vec();
    plan.push("--dry-run");
    let output = spawn(&plan, &[], complete.as_bytes()).expect("plan");
    assert_eq!(output.status.code(), Some(0));
    assert!(!String::from_utf8_lossy(&output.stderr).contains(EVIDENCE));
    let output = spawn(&common, &[], complete.as_bytes()).expect("missing key");
    assert_eq!(output.status.code(), Some(4));
    assert!(!String::from_utf8_lossy(&output.stderr).contains(EVIDENCE));
    assert_eq!(listener.connections(), 0);
}

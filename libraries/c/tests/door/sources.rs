//! Explicit files keep source coordinates beside every native answer.
use crate::cases::{Script, replies};
use crate::{compile, crate_dir, run, scratch, text};
use conformance_backend::Backend;
use serde_json::{Value, json};

fn ask(backend: &Backend, requests: &[Value]) -> Vec<(i32, Value)> {
    let base = format!("{}/arm/full/v1", backend.origin());
    let mut script = Script::default();
    script.ask("settings", &[&base, r#"{"cache":false,"max_retries":0}"#]);
    for request in requests {
        script.ask("call", &[&base, &request.to_string()]);
    }
    let output = run(
        &compile(&crate_dir().join("tests/c/driver.c")),
        "",
        &script.0,
    );
    assert!(output.status.success(), "{}", text(&output.stderr));
    replies(&output.stdout)
        .expect("framed replies")
        .into_iter()
        .skip(1)
        .map(|(code, body)| {
            (
                code,
                serde_json::from_str(&body).unwrap_or(json!({"message":body})),
            )
        })
        .collect()
}

#[test]
fn every_function_reaches_the_shared_documents_without_changing_records() {
    let backend = Backend::start().expect("backend");
    let documents = crate_dir().join("../../specification/fixtures/files/documents");
    let source = json!({"paths":[documents],"unit":"file"});
    let set: Value = serde_json::from_str(include_str!(
        "../../../../specification/fixtures/files/questions.json"
    ))
    .expect("set");
    let mut requests = vec![
        json!({"decide":"Does this document need attention?"}),
        json!({"choose":"What is this document?","options":["policy","contract"]}),
        json!({"tag":"Which topics appear?","labels":["refund","support"]}),
        json!({"score":"How urgent is this document?","levels":["low","high"]}),
        json!({"filter":"Does this document contain a support contract?"}),
        json!({"rank":"Which document needs attention?"}),
        json!({"find":"Which document contains a refund policy?"}),
        json!({"annotate":set}),
        json!({"version":1,"recognize":{"kinds":{"person":null}}}),
        json!({"version":1,"relate":{"relations":[{"name":"supports","source":"*","target":"*"}]}}),
    ];
    for request in &mut requests {
        request["source"] = source.clone();
    }
    let replies = ask(&backend, &requests);
    assert_eq!(replies.len(), requests.len());
    let originals: Vec<String> = ["01-policy.txt", "02-contract.txt"]
        .iter()
        .map(|name| std::fs::read_to_string(documents.join(name)).expect("document"))
        .collect();
    for (index, (code, reply)) in replies.iter().enumerate() {
        assert_eq!(*code, 0, "verb {index}: {reply}");
        assert!(
            reply["facts"]["requests_sent"]
                .as_u64()
                .is_some_and(|n| n > 0)
        );
        let value = &reply["value"];
        if index == 9 {
            let edges = value["edges"].as_array().expect("edges");
            assert!(!edges.is_empty());
            for edge in edges {
                for endpoint in ["source", "target"] {
                    assert!(
                        originals
                            .iter()
                            .any(|record| edge[endpoint]["record"] == *record)
                    );
                    assert_eq!(edge[endpoint]["first_line"], 1);
                    assert_eq!(edge[endpoint]["last_line"], 4);
                }
            }
        } else {
            let rows: Vec<&Value> = if index == 6 {
                vec![value]
            } else {
                value.as_array().expect("rows").iter().collect()
            };
            assert!(!rows.is_empty());
            for row in rows {
                assert!(originals.iter().any(|record| row["record"] == *record));
                assert_eq!(row["first_line"], 1);
                assert_eq!(row["last_line"], 4);
                assert!(row["file"].as_str().is_some_and(|f| f.ends_with(".txt")));
                assert!(row.get("value").is_some());
            }
        }
    }
}

#[test]
fn source_options_and_mixtures_refuse_before_any_request() {
    let backend = Backend::start().expect("backend");
    let missing = scratch("missing-source").join("missing.txt");
    let requests = [
        json!({"decide":"Q?","source":{"paths":[missing],"unit":"line","window":2}}),
        json!({"decide":"Q?","source":{"paths":[missing],"unit":"window","window":0}}),
        json!({"decide":"Q?","source":{"paths":[missing],"unit":"file"},"evidence":"old"}),
        json!({"find":"Q?","source":{"paths":[missing]},"units":["old","other"]}),
        json!({"relate":{"relations":[]},"source":{"paths":[missing]},"records":[]}),
        json!({"decide":"Q?","source":{"paths":[missing]}}),
    ];
    let replies = ask(&backend, &requests);
    for (index, (code, reply)) in replies.iter().enumerate() {
        assert_eq!(*code, if index == 5 { 4 } else { 1 }, "{reply}");
        let message = reply["message"].as_str().expect("failure message");
        if index < 2 {
            assert!(message.contains("reader window"), "{message}");
        } else if index < 5 {
            assert_eq!(message, "source replaces evidence, records, and units");
        }
    }
    assert_eq!(backend.count(), 0);
}

#[test]
fn lines_and_windows_retain_physical_crlf_positions_and_duplicate_occurrences() {
    let backend = Backend::start().expect("backend");
    let folder = scratch("located-lines");
    std::fs::create_dir_all(&folder).expect("folder");
    let path = folder.join("notes.txt");
    std::fs::write(&path, "Ada\r\n\r\nBea\r\nAda\r\n").expect("notes");
    let requests = [
        json!({"decide":"Q?","source":{"paths":[path]}}),
        json!({"decide":"Q?","source":{"paths":[path],"unit":"window","window":3}}),
        json!({"version":1,"relate":{"relations":[{"name":"met","source":"*","target":"*"}]},"source":{"paths":[path]}}),
    ];
    let replies = ask(&backend, &requests);
    assert!(replies.iter().all(|(code, _)| *code == 0), "{replies:?}");
    let rows = &replies[0].1["value"];
    assert_eq!(rows.as_array().map(Vec::len), Some(3));
    assert_eq!(rows[0]["record"], "Ada");
    assert_eq!(rows[0]["first_line"], 1);
    assert_eq!(rows[1]["record"], "Bea");
    assert_eq!(rows[1]["first_line"], 3);
    assert_eq!(rows[2]["record"], "Ada");
    assert_eq!(rows[2]["first_line"], 4);
    let windows = &replies[1].1["value"];
    assert_eq!(windows[0]["record"], "Ada\r\n\r\nBea");
    assert_eq!(windows[0]["last_line"], 3);
    assert_eq!(windows[1]["first_line"], 4);
    assert_eq!(
        replies[2].1["value"]["edges"].as_array().map(Vec::len),
        Some(4)
    );
}

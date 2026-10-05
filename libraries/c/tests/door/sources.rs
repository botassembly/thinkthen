//! Explicit files keep source coordinates beside every native answer.
use crate::cases::{Script, replies};
use crate::{compile, crate_dir, run, scratch, text};
use conformance_backend::Backend;
use serde_json::{Value, json};

fn ask(backend: &Backend, requests: &[Value]) -> Vec<(i32, Value)> {
    ask_at(backend, requests, false)
}

fn ask_at(backend: &Backend, requests: &[Value], cache: bool) -> Vec<(i32, Value)> {
    let base = format!("{}/arm/full/capture/v1", backend.origin());
    let mut script = Script::default();
    script.ask(
        "settings",
        &[&base, &json!({"cache":cache,"max_retries":0}).to_string()],
    );
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
            for endpoint in edges
                .iter()
                .flat_map(|edge| [&edge["source"], &edge["target"]])
            {
                source_row(endpoint, &originals);
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

fn source_row(row: &Value, originals: &[String]) {
    assert!(originals.iter().any(|record| row["record"] == *record));
    assert_eq!(row["first_line"], 1);
    assert_eq!(row["last_line"], 4);
}

#[test]
fn source_plans_match_original_evidence_and_mixtures_preserve_outputs_without_sending() {
    use crate::child::ChildEnvironment as _;
    let backend = Backend::start().expect("backend");
    let folder = scratch("source-plan");
    std::fs::create_dir_all(&folder).expect("folder");
    let path = folder.join("notes.txt");
    std::fs::write(&path, "Refund me please.\n").expect("notes");
    let evidence =
        json!({"verb":"decide","question":"asks for a refund","input":["Refund me please."]});
    let source = json!({"verb":"decide","question":"asks for a refund","source":{"paths":[path]}});
    let mut mixture = source.clone();
    mixture["input"] = json!(["old"]);
    let output = std::process::Command::new(compile(&crate_dir().join("tests/c/source_plan.c")))
        .clear_environment()
        .env(
            "THINKTHEN_BASE_URL",
            format!("{}/generic/v1", backend.origin()),
        )
        .env("THINKTHEN_CACHE", folder.join("cache"))
        .args([
            evidence.to_string(),
            source.to_string(),
            mixture.to_string(),
        ])
        .output()
        .expect("plans");
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let plans: Vec<Value> = text(&output.stdout)
        .lines()
        .map(|line| serde_json::from_str(line).expect("plan"))
        .collect();
    assert_eq!(plans.len(), 2);
    assert_eq!(plans[0], plans[1]);
    assert_eq!(backend.count(), 0);
}

#[test]
fn moving_identical_evidence_changes_provenance_without_changing_cache_identity() {
    let backend = Backend::start().expect("backend");
    let folder = scratch("source-identity");
    std::fs::create_dir_all(&folder).expect("folder");
    let first = folder.join("first.txt");
    let second = folder.join("second.txt");
    for path in [&first, &second] {
        std::fs::write(path, "Refund me please.\n").expect("notes");
    }
    let requests = [
        json!({"decide":"Q?","source":{"paths":[first]}}),
        json!({"decide":"Q?","source":{"paths":[second]}}),
        json!({"decide":"Q?","evidence":"Refund me please."}),
    ];
    let replies = ask_at(&backend, &requests, true);
    assert!(replies.iter().all(|(code, _)| *code == 0), "{replies:?}");
    assert_eq!(
        replies[0].1["value"][0]["file"],
        first.to_string_lossy().as_ref()
    );
    assert_eq!(
        replies[1].1["value"][0]["file"],
        second.to_string_lossy().as_ref()
    );
    assert_eq!(backend.count(), 1);
    let capture: Value = serde_json::from_str(&backend.capture()).expect("capture");
    let bodies = capture["bodies"].as_array().expect("bodies");
    assert_eq!(bodies.len(), 1);
    let body = bodies[0].to_string();
    assert!(body.contains("Refund me please."));
    assert!(!body.contains("first.txt") && !body.contains("second.txt"));
    assert!(!body.contains("first_line") && !body.contains("last_line"));
}

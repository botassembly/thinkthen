//! Counted preview/runtime contracts at the default and explicit widths.
use super::super::rank_set_0401::{SET, call_at_width, saved};
use crate::harness::Listener;
use crate::intake_0401::text;
use serde_json::Value;
use std::io;

pub(super) fn text_members() -> io::Result<()> {
    for jobs in [None, Some("1"), Some("4"), Some("8")] {
        preview_matches_counted_requests("rank-set-preview-text-0400", SET, false, jobs)?;
    }
    Ok(())
}

pub(super) fn mixed_members() -> io::Result<()> {
    for (name, set) in [
        (
            "rank-set-preview-text-json-0400",
            r#"{"version":1,"questions":{"text":{"decide":"Ready?"},"json":{"decide":{"check":"Ready?"}}}}"#,
        ),
        (
            "rank-set-preview-json-text-0400",
            r#"{"version":1,"questions":{"json":{"decide":{"check":"Ready?"}},"text":{"decide":"Ready?"}}}"#,
        ),
    ] {
        for jobs in [None, Some("1"), Some("4"), Some("8")] {
            preview_matches_counted_requests(name, set, true, jobs)?;
        }
    }
    Ok(())
}

fn preview_matches_counted_requests(
    name: &str,
    set: &str,
    mixed: bool,
    jobs: Option<&str>,
) -> io::Result<()> {
    let question = saved(name, set)?;
    let listener = Listener::answering(crate::intake_0401::answer)?;
    let preview = call_at_width(
        &listener,
        &question,
        &["--batch", "1", "--plan"],
        b"alpha\nbeta\ngamma\n",
        jobs,
    )?;
    assert_eq!(preview.status.code(), Some(0), "{}", text(&preview.stderr));
    assert_eq!(listener.connections(), 0);
    assert!(listener.requests().is_empty());
    let lines: Vec<_> = text(&preview.stdout).lines().map(str::to_owned).collect();
    assert_eq!(lines.len(), 2);
    let document: serde_json::Map<String, Value> = serde_json::from_str(&lines[0])?;
    let raw: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>> =
        serde_json::from_str(&lines[0])?;
    assert!(contains_record(&document["request"], "alpha"));
    let expected = if mixed { "6" } else { "3" };
    let runtime = call_at_width(
        &listener,
        &question,
        &["--batch", "1", "--max-requests-total", expected, "--facts"],
        b"alpha\nbeta\ngamma\n",
        jobs,
    )?;
    assert_eq!(runtime.status.code(), Some(0), "{}", text(&runtime.stderr));
    assert_eq!(runtime.stdout, b"alpha\nbeta\ngamma\n");
    let requests = listener.requests();
    let per_record = if mixed { 2 } else { 1 };
    assert_eq!(requests.len(), 3 * per_record);
    let first = requests
        .iter()
        .find(|request| request.body == raw["request"].get().as_bytes())
        .expect("the exact preview body was sent");
    assert_eq!(
        document["request"],
        serde_json::from_slice::<Value>(&first.body)?
    );
    let bodies: Vec<Value> = requests
        .iter()
        .map(|request| serde_json::from_slice(&request.body))
        .collect::<Result<_, _>>()?;
    for record in ["alpha", "beta", "gamma"] {
        let own: Vec<Value> = bodies
            .iter()
            .filter(|body| contains_record(body, record))
            .cloned()
            .collect();
        assert_eq!(own.len(), per_record);
        assert_member_evidence(&own, record, mixed);
    }
    let facts: Value = serde_json::from_str(text(&runtime.stderr).trim())?;
    assert_eq!(facts["records"], 3);
    assert_eq!(facts["requests_sent"], 3 * per_record);
    let bytes: usize = requests.iter().map(|request| request.body.len()).sum();
    let summary: Value = serde_json::from_str(&lines[1])?;
    assert_eq!(
        summary,
        serde_json::json!({
            "records":3,"requests":3 * per_record,"estimated_bytes":bytes,
            "estimated_input_tokens":{"lower":bytes * 516 / 1000,"upper":(bytes * 908).div_ceil(1000)},
            "upper_bound":false
        })
    );
    Ok(())
}

fn contains_record(body: &Value, record: &str) -> bool {
    body["state"] == record
        || body["questions"]
            .as_object()
            .expect("questions")
            .values()
            .any(|q| {
                q["instructions"]
                    .as_str()
                    .is_some_and(|text| text.contains(&format!("\"{record}\"")))
            })
}

fn assert_member_evidence(bodies: &[Value], record: &str, mixed: bool) {
    let instructions: Vec<_> = bodies
        .iter()
        .flat_map(|body| {
            body["questions"]
                .as_object()
                .expect("questions")
                .values()
                .map(|q| &q["instructions"])
        })
        .collect();
    assert_eq!(
        instructions.len(),
        2,
        "both members judge each original record"
    );
    let texts: Vec<_> = instructions.iter().filter_map(|q| q.as_str()).collect();
    assert_eq!(texts.len(), if mixed { 1 } else { 2 });
    assert!(texts.iter().all(|q| q.contains(&format!("\"{record}\""))));
    if mixed {
        let structured = bodies
            .iter()
            .find(|body| {
                body["questions"]
                    .as_object()
                    .expect("questions")
                    .values()
                    .any(|q| q["instructions"].is_object())
            })
            .expect("structured request");
        assert_eq!(structured["state"], record);
        assert!(
            instructions
                .iter()
                .any(|q| **q == serde_json::json!({"check":"Ready?"}))
        );
        assert!(texts[0].contains("Ready?"));
    } else {
        for member in ["First?", "Second?"] {
            assert!(texts.iter().any(|q| q.contains(member)));
        }
    }
}

//! The 4,096-member close advances an oldest row held behind another group.

use std::sync::Arc;

use serde_json::{Value, json};

use super::set;
use crate::harness::{Canned, Gathering, Listener, spawn};

#[test]
fn the_member_cap_closes_a_later_group_while_the_first_reply_is_held() {
    let file = set(
        "batch-cap-groups",
        r#"{"version":1,"questions":{"wide_answer":{"decide":"Wide?","on":"/wide"},"narrow_answer":{"decide":"Narrow?","on":"/narrow"}}}"#,
    );
    let gathering = Arc::new(Gathering::new(2));
    let listener = Listener::answering({
        let gathering = Arc::clone(&gathering);
        move |body| {
            let request: Value = serde_json::from_slice(body).expect("request JSON");
            if request["state"] == "cut-12835" || request["state"] == "same-narrow" {
                gathering.hold();
            }
            Canned::ok(r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9}}}"#)
        }
    })
    .expect("listener");
    let mut input = String::from("{\"wide\":\"cut-12835\",\"narrow\":\"same-narrow\"}\n");
    for _ in 1..4096 {
        input.push_str("{\"wide\":\"same-wide\",\"narrow\":\"same-narrow\"}\n");
    }
    let output = spawn(
        &[
            "annotate",
            &file.to_string_lossy(),
            "--jsonl",
            "--batch",
            "max",
            "--jobs",
            "2",
            "--details",
            "--no-cache",
            "--url",
            listener.base(),
            "--model",
            "local-1",
        ],
        &[("THINKTHEN_API_KEY", "key")],
        input.as_bytes(),
    )
    .expect("annotate stream");
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 3);
    let mut quotes: Vec<String> = requests
        .iter()
        .map(|request| {
            let body = serde_json::from_slice::<Value>(&request.body).expect("request JSON");
            assert_eq!(
                body["state"],
                "Each question quotes the text it asks about."
            );
            let asked = body["questions"]["q1"]["instructions"]
                .as_str()
                .unwrap_or_default();
            asked.split(". ").next().unwrap_or_default().to_owned()
        })
        .collect();
    quotes.sort();
    assert_eq!(
        quotes,
        [
            r#"The text is "cut-12835""#,
            r#"The text is "same-narrow""#,
            r#"The text is "same-wide""#
        ]
    );
    let rows: Vec<Value> = output
        .stdout
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_slice(line).expect("row JSON"))
        .collect();
    assert_eq!(rows.len(), 4096);
    assert_eq!(rows[0]["meta"]["batches"][0]["closed"], "content");
    assert_eq!(rows[0]["meta"]["batches"][1]["records"], 4096);
    assert_eq!(rows[0]["meta"]["batches"][1]["closed"], "limit");
    assert_eq!(rows[4095]["meta"]["batches"][0]["records"], 4095);
    assert_eq!(rows[4095]["value"]["wide_answer"], true);
}
